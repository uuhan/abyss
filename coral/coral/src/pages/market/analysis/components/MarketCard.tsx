import * as d3 from 'd3'
import { format } from "d3-format";
import { timeFormat } from "d3-time-format";
import zh_locale from "d3-time-format/locale/zh-CN.json";
import React, { useEffect, useState } from 'react';
import { useRequest } from 'umi';
import request from 'umi-request';
import { Card, Col, Row, Tabs, Skeleton, Spin, Radio, Input } from 'antd';
import type { RadioChangeEvent } from 'antd/es/radio';
import type moment from 'moment';
import { Column } from '@ant-design/charts';
import { RawStockChart } from '@/pages/components/KlineChart';
import {
  ChartCanvas, lastVisibleItemBasedZoomAnchor,
  Chart, BarSeries, XAxis,
  YAxis, CrossHairCursor, ZoomButtons,
  MouseCoordinateY, LineSeries, CandlestickSeries,
  MouseCoordinateX, EdgeIndicator, OHLCTooltip,
  withSize, withDeviceRatio, discontinuousTimeScaleProviderBuilder,
} from 'react-financial-charts';
import MobileDetect from 'mobile-detect'
import { withOHLCData } from "@/pages/components/data";
import type { IOHLCData } from "@/pages/components/data"
import numeral from 'numeral';
import _ from 'lodash';
import type { DataItem } from '../data.d';
import styles from '../style.less';

type KlineType = 'minute' | '10minute' | 'hour' | 'day';
export type TimeType = 'today' | 'week' | 'month' | 'year';

const { TabPane } = Tabs;

interface StockChartProps {
  readonly data: IOHLCData[];
  readonly height: number;
  readonly dateTimeFormat?: string;
  readonly width: number;
  readonly ratio: number;
  readonly id: string;
  readonly type: string;
  readonly refresh?: () => void;
}

class LineChart extends React.Component<StockChartProps, { long?: number; short?: number; }> {
  private readonly margin = { left: 0, right: 48, top: 0, bottom: 24 };
  private pricesDisplayFormat = format(".0f");
  private readonly xScaleProvider = discontinuousTimeScaleProviderBuilder().inputDateAccessor(
      (d: IOHLCData) => d.date,
  );

  private socket?: WebSocket;

  private subscribePosition(props: StockChartProps) {
    if (!props.id) {
      return;
    }

    if (this.socket) {
      this.socket.close();
    }

    const TOKEN = localStorage.getItem("TOKEN");
    const SCHEMA = window.location.protocol === 'http:' ? 'ws' : 'wss';

    this.socket = new WebSocket(`${SCHEMA}://${window.location.host}/api/v1/position/of/${props.id}?token=${TOKEN}`)

    this.socket.onopen = () => {
      // eslint-disable-next-line no-console
      console.log(`${props.id} connected.`);
    }

    this.socket.onclose = (c) => {}

    this.socket.onerror = (e) => {
      // eslint-disable-next-line no-console
      console.error(`${props.id} err:`, e);
    }

    this.socket.onmessage = (msg) => {
      const positions = JSON.parse(msg.data);

      let long: number|undefined;
      let short: number|undefined;

      for (const position of positions) {
        if (position.direction === 'Short') {
          short = position.open;
        }

        if (position.direction === 'Long') {
          long = position.open;
        }
      }

      this.setState({
        long,
        short,
      })
    }
  }

  public constructor(props: StockChartProps) {
    super(props);

    this.state = {
      long: undefined,
      short: undefined,
    }
  }

  public componentDidMount() {
    this.subscribePosition(this.props);
  }

  public componentWillReceiveProps(nextProps: StockChartProps) {
    if (this.props.id !== nextProps.id || this.props.type !== nextProps.type) {
      this.setState({
        long: undefined,
        short: undefined,
      }, () => {
        this.subscribePosition(nextProps)
      })
    }
  }

  public componentWillUnmount() {
    this.socket?.close()
  }


  public render() {
    const { data: initialData, dateTimeFormat = "%c", height, ratio, width, refresh } = this.props;

    const { margin, xScaleProvider } = this;

    const { data, xScale, xAccessor, displayXAccessor } = xScaleProvider(initialData);

    // 适配手机
    const md = new MobileDetect(window.navigator.userAgent);
    let offset = 160;
    if (md.phone()) { offset = 40; }

    const xMin = xAccessor(data[Math.max(0, data.length - offset)]);
    const xMax = xAccessor(data[data.length - 1]) + 3;

    const gridHeight = height - margin.top - margin.bottom;
    const barChartHeight = gridHeight / 4;
    const barChartOrigin = (_w: number, h: number) => [0, h - barChartHeight];

    const chartHeight = gridHeight;

    d3.timeFormatDefaultLocale(zh_locale as any);
    let timeDisplayFormat = timeFormat(dateTimeFormat);

    if (this.props.type === "day") {
      timeDisplayFormat = timeFormat("%x %a");
    }

    if (/^(sc|ZC)/.test(this.props.id)) {
      this.pricesDisplayFormat = format(".1f");
    }

    if (/^au/.test(this.props.id)) {
      this.pricesDisplayFormat = format(".2f");
    }

    const { long, short } = this.state;

    return (
        <ChartCanvas
            height={height}
            ratio={ratio}
            width={width}
            margin={margin}
            data={data}
            displayXAccessor={displayXAccessor}
            seriesName="Data"
            xScale={xScale}
            xAccessor={xAccessor}
            xExtents={() => {
              return [xMin, xMax];
            }}
            zoomAnchor={lastVisibleItemBasedZoomAnchor}
            disableZoom={true}
        >
            <Chart id={2} height={barChartHeight} origin={barChartOrigin} yExtents={this.barChartExtents}>
                <BarSeries fillStyle={this.volumeColor} yAccessor={this.volumeSeries} />
            </Chart>
            <Chart id={3} height={chartHeight} yExtents={this.candleChartExtents}>
                <XAxis
                  showGridLines
                  showTicks={false}
                  showTickLabel={false}
                />
                <YAxis showGridLines tickFormat={this.pricesDisplayFormat} />
                <MouseCoordinateX displayFormat={timeDisplayFormat} />
                <MouseCoordinateY rectWidth={margin.right} displayFormat={this.pricesDisplayFormat} />

                { long && <LineSeries yAccessor={() => long} strokeStyle="red" />}
                { short && <LineSeries yAccessor={() => short} strokeStyle="green" />}

                <EdgeIndicator
                    itemType="last"
                    rectWidth={margin.right}
                    fill={this.openCloseColor}
                    lineStroke={this.openCloseColor}
                    displayFormat={this.pricesDisplayFormat}
                    yAccessor={this.yEdgeIndicator}
                />

                {/* <LineSeries yAccessor={this.yAccessor} /> */}
                <CandlestickSeries
                  fill={(d: any) => (d.close < d.open ? "#26a69a" : "#ef5350")}
                  wickStroke={(d: any) => (d.close < d.open ? "#26a69a" : "#ef5350")}
                />
                <OHLCTooltip origin={[8, 16]} />
                <ZoomButtons onReset={refresh}/>
            </Chart>
            <CrossHairCursor />
        </ChartCanvas>
    );
  }

  private readonly yAccessor = (data: IOHLCData) => {
      return data.close;
  };

  private readonly yEdgeIndicator = (data: IOHLCData) => {
      return data.close;
  };

  private readonly barChartExtents = (data: IOHLCData) => {
      return data.volume;
  };

  private readonly candleChartExtents = (data: IOHLCData) => {
      return [data.high, data.low];
  };

  private readonly volumeColor = (data: IOHLCData) => {
      return data.close < data.open ? "rgba(38, 166, 154, 0.3)" : "rgba(239, 83, 80, 0.3)";
  };

  private readonly volumeSeries = (data: IOHLCData) => {
      return data.volume;
  };

  private readonly openCloseColor = (data: IOHLCData) => {
      return data.close < data.open ? "#26a69a" : "#ef5350";
  };
}

const StockChart = withOHLCData()(withSize({
  style: { minHeight: 340 }
})(withDeviceRatio()(LineChart)));

const MarketCard = ({
}: {
}) => {
  const rankingListData: { title: string; total: number }[] = [];
  for (let i = 0; i < 7; i += 1) {
    rankingListData.push({
      title: `工专路 ${i} 号店`,
      total: 323234,
    });
  }

  const [instrument, setInstrument] = useState<string>('');
  const [klineType, setKlineType] = useState<KlineType>('minute');
  const [riseTicks, setRiseTicks] = useState<any[]>([]);
  const [fallTicks, setFallTicks] = useState<any[]>([]);
  const [datumTick, setDatumTick] = useState<any[]>([]);

  const { loading: tickLoading, ...rest } = useRequest(() => {
    return request('/api/v1/ticks/by-major', {
      method: 'GET',
    }).then(resp => {
      const ticks1 = [];
      for (const [name, value] of _.toPairs(resp) as any) {
        const tick: any = {};
        tick.name = name;
        const { close, open, id } = value;

        if (open === Number.MAX_VALUE || close === Number.MAX_VALUE || open === 0) {
          // 异常数据
        } else {
          tick.ratio = (close - open) / open;
          tick.id = id;
          ticks1.push(tick);
        }
      }

      const ticks2 = _.sortBy(ticks1, tick => tick.ratio);
      const fall = _.take(ticks2, 8);
      const rise = _.take(_.reverse(ticks2), 8);

      const first_id = rise?.[0]?.id;
      if (first_id && !instrument) { setInstrument(first_id) }

      setFallTicks(fall);
      setRiseTicks(rise);
      setDatumTick(ticks2);

      return resp;
    });
  }, {
    pollingInterval: 5000,
    pollingWhenHidden: false
  });

  const handleChangeKlineType = (e: RadioChangeEvent) => {
    setKlineType(e.target.value);
  };

  const onSearch = (value: any) => {
    if (value) {
      setInstrument(value)
    }
  }

  const tabLabel = instrument ? `行情(${instrument})` : "行情";

  return <Card bordered={false} bodyStyle={{ padding: 0 }}>
    <div className={styles.salesCard}>
      <Tabs
        tabBarExtraContent={
          <div className={styles.salesExtraWrap}>
            <div className={styles.salesExtra}>
              <Input.Search onSearch={onSearch} style={{ width: 200, marginRight: '5px' }} />
              <Radio.Group value={klineType} onChange={handleChangeKlineType}>
                <Radio.Button value="minute">分钟</Radio.Button>
                <Radio.Button value="10minute">十分</Radio.Button>
                <Radio.Button value="hour">小时</Radio.Button>
                <Radio.Button value="day">日线</Radio.Button>
              </Radio.Group>
            </div>
          </div>
        }
        size="large"
        tabBarStyle={{ marginBottom: 24 }}
      >
        <TabPane tab={tabLabel} key="sales">
          <Row>
            <Col xl={18} lg={12} md={12} sm={24} xs={24}>
              <div className={styles.salesBar}>
                { instrument && <StockChart id={instrument} type={klineType}/> }
              </div>
            </Col>
            <Col xl={3} lg={12} md={12} sm={24} xs={24}>
              <div className={styles.salesRank}>
                <Spin spinning={tickLoading}>
                  <h4 className={styles.rankingTitle}>市场涨幅排名</h4>
                  <ul className={styles.rankingList}>
                    {riseTicks.map((item, i) => (
                      <li key={item.name} onClick={() => setInstrument(item.id)}>
                        <span className={`${styles.rankingItemNumber} ${i < 3 ? styles.riseActive : ''}`}>
                          {i + 1}
                        </span>
                        <span className={styles.rankingItemTitle} title={item.name}>
                          {item.name}
                        </span>
                        <span className={styles.rankingItemValue}>
                          {numeral(item.ratio*100).format('+0.00')}%
                        </span>
                      </li>
                    ))}
                  </ul>
                </Spin>
              </div>
            </Col>
            <Col xl={3} lg={12} md={12} sm={24} xs={24}>
              <div className={styles.salesRank}>
                <Spin spinning={tickLoading}>
                  <h4 className={styles.rankingTitle}>市场跌幅排名</h4>
                  <ul className={styles.rankingList}>
                    {fallTicks.map((item, i) => (
                      <li key={item.name} onClick={() => setInstrument(item.id)}>
                        <span className={`${styles.rankingItemNumber} ${i < 3 ? styles.fallActive : ''}`}>
                          {i + 1}
                        </span>
                        <span className={styles.rankingItemTitle} title={item.name}>
                          {item.name}
                        </span>
                        <span className={styles.rankingItemValue}>
                          {numeral(item.ratio*100).format('+0.00')}%
                        </span>
                      </li>
                    ))}
                  </ul>
                </Spin>
              </div>
            </Col>
          </Row>
        </TabPane>
        <TabPane tab="涨跌" key="views">
          <Row>
            <Col xl={18} lg={12} md={12} sm={24} xs={24}>
              <div className={styles.salesBar}>
                <Column
                  height={330}
                  autoFit
                  data={datumTick.map(tick => ({ ratio: tick.ratio*100, name: tick.name }))}
                  xField="name"
                  yField="ratio"
                  xAxis={{}}
                  yAxis={{}}
                  seriesField="ratio"
                  animation={false}
                  color={data => {
                    if (data.ratio < 0) {
                      return 'green';
                    }
                    return 'red';
                  }}
                />
              </div>
            </Col>
            <Col xl={3} lg={12} md={12} sm={24} xs={24}>
              <div className={styles.salesRank}>
                <Spin spinning={tickLoading}>
                  <h4 className={styles.rankingTitle}>市场涨幅排名</h4>
                  <ul className={styles.rankingList}>
                    {riseTicks.map((item, i) => (
                      <li key={item.name} onClick={() => setInstrument(item.id)}>
                        <span className={`${styles.rankingItemNumber} ${i < 3 ? styles.riseActive : ''}`}>
                          {i + 1}
                        </span>
                        <span className={styles.rankingItemTitle} title={item.name}>
                          {item.name}
                        </span>
                        <span className={styles.rankingItemValue}>
                          {numeral(item.ratio*100).format('+0.00')}%
                        </span>
                      </li>
                    ))}
                  </ul>
                </Spin>
              </div>
            </Col>
            <Col xl={3} lg={12} md={12} sm={24} xs={24}>
              <div className={styles.salesRank}>
                <Spin spinning={tickLoading}>
                  <h4 className={styles.rankingTitle}>市场跌幅排名</h4>
                  <ul className={styles.rankingList}>
                    {fallTicks.map((item, i) => (
                      <li key={item.name} onClick={() => setInstrument(item.id)}>
                        <span className={`${styles.rankingItemNumber} ${i < 3 ? styles.fallActive : ''}`}>
                          {i + 1}
                        </span>
                        <span className={styles.rankingItemTitle} title={item.name}>
                          {item.name}
                        </span>
                        <span className={styles.rankingItemValue}>
                          {numeral(item.ratio*100).format('+0.00')}%
                        </span>
                      </li>
                    ))}
                  </ul>
                </Spin>
              </div>
            </Col>
          </Row>
        </TabPane>
      </Tabs>
    </div>
  </Card>
};

export default MarketCard;
