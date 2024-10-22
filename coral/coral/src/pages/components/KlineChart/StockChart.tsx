import * as d3 from 'd3'
import { format } from "d3-format";
import { timeFormat } from "d3-time-format";
import zh_locale from "d3-time-format/locale/zh-CN.json";
import * as React from "react";
import {
    elderRay,
    macd,
    ema,
    discontinuousTimeScaleProviderBuilder,
    Chart,
    ChartCanvas,
    CurrentCoordinate,
    BarSeries,
    CandlestickSeries,
    MACDSeries,
    MACDTooltip,
    ElderRaySeries,
    LineSeries,
    MovingAverageTooltip,
    OHLCTooltip,
    SingleValueTooltip,
    lastVisibleItemBasedZoomAnchor,
    XAxis,
    YAxis,
    CrossHairCursor,
    EdgeIndicator,
    MouseCoordinateX,
    MouseCoordinateY,
    ZoomButtons,
    withDeviceRatio,
    withSize,
} from "react-financial-charts";
import { message } from 'antd';
import type { IOHLCData } from "@/pages/components/data"
import { withOHLCData } from "@/pages/components/data";
import MobileDetect from 'mobile-detect'

interface StockChartProps {
  readonly data: IOHLCData[];
  readonly height: number;
  readonly dateTimeFormat?: string;
  readonly width: number;
  readonly ratio: number;
  readonly id: string;
  readonly type: string;
  readonly refresh?: () => void;
  readonly long?: number;
  readonly short?: number;
}

class StockChart extends React.Component<StockChartProps, { long?: number; short?: number; }> {
  private readonly margin = { left: 0, right: 48, top: 0, bottom: 24 };
  private pricesDisplayFormat = format(".0f");
  private readonly xScaleProvider = discontinuousTimeScaleProviderBuilder().inputDateAccessor(
      (d: IOHLCData) => d.date,
  );

  private readonly macdAppearance = {
      fillStyle: {
          divergence: "#4682B4",
      },
      strokeStyle: {
          macd: "#0093FF",
          signal: "#D84315",
          zero: "rgba(0, 0, 0, 0.3)",
      },
  };

  private readonly macdCalculator = macd()
      .options({
          fast: 12,
          signal: 9,
          slow: 26,
      })
      .merge((d: any, c: any) => {
          d.macd = c;
      })
      .accessor((d: any) => d.macd);

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

      let long: number|undefined = undefined;
      let short: number|undefined = undefined;

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

    // macd
    const calculatedData = this.macdCalculator(initialData);

    const ema12 = ema()
        .id(1)
        .options({ windowSize: 12 })
        .merge((d: any, c: any) => {
            d.ema12 = c;
        })
        .accessor((d: any) => d.ema12);

    const ema26 = ema()
        .id(2)
        .options({ windowSize: 26 })
        .merge((d: any, c: any) => {
            d.ema26 = c;
        })
        .accessor((d: any) => d.ema26);

    // const elder = elderRay();
    // const calculatedData = elder(ema26(ema12(initialData)));

    const { margin, xScaleProvider } = this;

    const { data, xScale, xAccessor, displayXAccessor } = xScaleProvider(calculatedData);

    const md = new MobileDetect(window.navigator.userAgent);
    let offset = 160;
    if (md.phone()) { offset = 40; }

    const xMin = xAccessor(data[Math.max(0, data.length - offset)]);
    const xMax = xAccessor(data[data.length - 1]) + 3;

    const gridHeight = height - margin.top - margin.bottom;

    // const elderRayHeight = 100;
    // const elderRayOrigin = (_: number, h: number) => [0, h - elderRayHeight];

    const macdHeight = 150;
    const macdOrigin = (_: number, h: number) => [0, h - macdHeight];

    const barChartHeight = gridHeight / 4;
    const barChartOrigin = (_: number, h: number) => [0, h - barChartHeight - macdHeight];

    const chartHeight = gridHeight - macdHeight;

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

    if (/^T\d/.test(this.props.id)) {
      this.pricesDisplayFormat = format(".3f");
    }

    if (/^I/.test(this.props.id)) {
      this.pricesDisplayFormat = format(".1f");
    }

    const { long, short } = this.state;

    const yAccessor = this.macdCalculator.accessor();
    const options = this.macdCalculator.options();

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
            disablePan={false}
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
                <CandlestickSeries
                  fill={(d: any) => (d.close < d.open ? "#26a69a" : "#ef5350")}
                  wickStroke={(d: any) => (d.close < d.open ? "#26a69a" : "#ef5350")}
                />
                <LineSeries yAccessor={ema26.accessor()} strokeStyle={ema26.stroke()} />
                <CurrentCoordinate yAccessor={ema26.accessor()} fillStyle={ema26.stroke()} />
                <LineSeries yAccessor={ema12.accessor()} strokeStyle={ema12.stroke()} />
                <CurrentCoordinate yAccessor={ema12.accessor()} fillStyle={ema12.stroke()} />

                {/* 均价 */}
                { this.props.type !== 'day' && <LineSeries yAccessor={(kline: any) => kline.average} strokeStyle={this.averageColor()} />}
                { this.props.type !== 'day' && <CurrentCoordinate yAccessor={(kline: any) => kline.average} fillStyle={this.averageColor()} />}

                { long && <LineSeries yAccessor={() => long} strokeStyle="red" />}
                { short && <LineSeries yAccessor={() => short} strokeStyle="green" />}

                <MouseCoordinateY rectWidth={margin.right} displayFormat={this.pricesDisplayFormat} />
                <EdgeIndicator
                    itemType="last"
                    rectWidth={margin.right}
                    fill={this.openCloseColor}
                    lineStroke={this.openCloseColor}
                    displayFormat={this.pricesDisplayFormat}
                    yAccessor={this.yEdgeIndicator}
                />
                <MovingAverageTooltip
                    origin={[8, 24]}
                    options={[
                        {
                            yAccessor: ema26.accessor(),
                            type: "EMA",
                            stroke: ema26.stroke(),
                            windowSize: ema26.options().windowSize,
                        },
                        {
                            yAccessor: ema12.accessor(),
                            type: "EMA",
                            stroke: ema12.stroke(),
                            windowSize: ema12.options().windowSize,
                        },
                        {
                            yAccessor: (kline: any) => kline.average,
                            type: "AVG",
                            stroke: this.averageColor(),
                            windowSize: 0,
                        },
                    ]}
                />

                <ZoomButtons onReset={refresh}/>
                <OHLCTooltip origin={[8, 16]} />
            </Chart>

            <Chart
              id={5}
              yExtents={yAccessor}
              height={macdHeight}
              origin={macdOrigin}
              padding={{ top: 8, bottom: 8 }}
            >
                <XAxis />
                <YAxis />

                <MACDSeries yAccessor={yAccessor} {...this.macdAppearance} />

                <MACDTooltip
                    origin={[8, 16]}
                    appearance={this.macdAppearance}
                    options={options}
                    yAccessor={yAccessor}
                />

                <MouseCoordinateX displayFormat={timeDisplayFormat} />
                <MouseCoordinateY rectWidth={margin.right} displayFormat={this.pricesDisplayFormat} />

            </Chart>

            {/*
            <Chart
                id={4}
                height={elderRayHeight}
                yExtents={[0, elder.accessor()]}
                origin={elderRayOrigin}
                padding={{ top: 8, bottom: 8 }}
            >
                <XAxis showGridLines gridLinesStrokeStyle="#e0e3eb" />
                <YAxis ticks={4} tickFormat={this.pricesDisplayFormat} />

                <ElderRaySeries
                  yAccessor={elder.accessor()}
                  fillStyle={{
                    bearPower: "rgba(38, 166, 153, 0.7)",
                    bullPower: "rgba(239, 83, 80, 0.7)",
                  }}
                />

                <SingleValueTooltip
                    yAccessor={elder.accessor()}
                    yLabel="Elder Ray"
                    yDisplayFormat={(d: any) =>
                        `${this.pricesDisplayFormat(d.bullPower)}, ${this.pricesDisplayFormat(d.bearPower)}`
                    }
                    origin={[8, 16]}
                />
            </Chart>
            */}
            <CrossHairCursor />
        </ChartCanvas>
    );
  }

  private readonly barChartExtents = (data: IOHLCData) => {
      return data.volume;
  };

  private readonly candleChartExtents = (data: IOHLCData) => {
      return [data.high, data.low];
  };

  private readonly yEdgeIndicator = (data: IOHLCData) => {
      return data.close;
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

  private readonly averageColor = () => {
    return "gold";
  };
}

export const RawStockChart = StockChart;

export default withOHLCData()(
  withSize({ style: { minHeight: 500 } })(withDeviceRatio()(StockChart))
);

export const SmallStockChart = withOHLCData()(
  withSize({ style: { minHeight: 300 } })(withDeviceRatio()(StockChart))
);

export const MinutesStockChart = withOHLCData("MINUTES")(
    withSize({ style: { minHeight: 500 } })(withDeviceRatio()(StockChart)),
);

export const SecondsStockChart = withOHLCData("SECONDS")(
    withSize({ style: { minHeight: 500 } })(withDeviceRatio()(StockChart)),
);
