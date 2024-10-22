import { PageContainer, FooterToolbar } from '@ant-design/pro-layout';
import ProTable from '@ant-design/pro-table';
import type { ProDescriptionsItemProps } from '@ant-design/pro-descriptions';
import ProDescriptions from '@ant-design/pro-descriptions';
import {
  Card, Col, Row,
  Input, Button, message,
  Menu, Dropdown, Radio,
  Drawer, Tabs, AutoComplete,
} from 'antd';
import { Link } from 'umi';
import { useParams, useModel, useRequest } from 'umi';
import { useEffect, useState, useRef } from 'react';
import request from 'umi-request';
import _ from 'lodash';
import { Gauge, Liquid, RingProgress, Treemap } from '@ant-design/charts';
import ProForm, {
  ModalForm,
  ProFormText,
  ProFormDateRangePicker,
  ProFormSelect,
  ProFormDigit,
  ProFormSwitch,
} from '@ant-design/pro-form';
import {
  PlusOutlined, DeleteOutlined, EllipsisOutlined,
} from '@ant-design/icons';
import { GridContent } from '@ant-design/pro-layout';
import { SmallStockChart as StockChart } from '@/pages/components/KlineChart';
import type { FC } from 'react';
import type { RadioChangeEvent } from 'antd/es/radio';
import type { TableListItem, TableListPagination, CommissionItem, MarginItem, TickItem } from './data';
import type { ProColumns, ActionType } from '@ant-design/pro-table';
import { position, instrument as get_instrument, tick as get_tick } from './service';
import numeral from 'numeral';
import './style.less';

const { TabPane } = Tabs;
const ID_KEY = "PRODUCT_DEFAULT_ID"

enum PosiDirection {
  BUY,
  SELL,
}

type KlineType = 'minute' | '10minute' | 'hour' | 'day';

const Product: FC = () => {
  const route = useParams() as {id?: string};
  const DEFAULT_ID = localStorage.getItem(ID_KEY);
  let init = [];
  try {
    if (DEFAULT_ID) {
      init = JSON.parse(DEFAULT_ID);
    }
  } catch (err: any) {
    localStorage.removeItem(ID_KEY);
    message.error(err?.message ?? err);
  }

  const [instruments, setInstruments] = useState<string[]>(init);
  // false: BUY, true: SELL
  const { initialState, setInitialState } = useModel('@@initialState');
  const products = (initialState?.products || []).map((option: any) => ({ value: option }));
  const [options, setOptions] = useState(products);
  const [direction, setDirection] = useState<PosiDirection>(PosiDirection.BUY);
  const [fastOrder, setFastOrder] = useState<boolean>(false);
  const [closeOrder, setCloseOrder] = useState<boolean>(false);
  const [showOrderForm, setShowOrderForm] = useState<boolean>(false);
  const [showPosition, setShowPosition] = useState<boolean>(false);
  const [klineType, setKlineType] = useState<KlineType>('day');
  const [currentPosition, setCurrentPosition] = useState<any>({});
  const [loading, setLoading] = useState<boolean>(false);
  const actionRef = useRef<ActionType>();

  useEffect(() => {
    let id = 0;
    const loop = () => {
      id = window.setTimeout(() => {
        const { current } = actionRef;
        if (current) {
          current.reload();
        }
        loop();
      }, 5000);
    };
    loop();
    return () => {
      window.clearTimeout(id);
    };
  }, []);

  const handleBuy = (evt: any) => {
    if (!fastOrder) {
      setDirection(PosiDirection.BUY);
      setShowOrderForm(true);
    }

    // TODO: place order
  }

  const handleSell = (evt: any) => {
    if (!fastOrder) {
      setDirection(PosiDirection.SELL);
      setShowOrderForm(true);
    }

    // TODO: place order
  }

  const clearConfig = (evt: any) => {
    setFastOrder(false)
  }

  const config_label = closeOrder ? "平仓" : "开仓";
  const columns: ProColumns<TableListItem>[] = [
    {
      title: '合约ID',
      dataIndex: 'InstrumentID',
      render: (dom, entity) => {
        const id = entity.InstrumentID;
        if (id) {
          const to = `/portfolio/trade/${id}`;
          return <Link to={to}>
            {id}
            </Link>;
        }
        return dom;
      },
    },
    {
      title: '交易所',
      dataIndex: 'ExchangeID',
      render: (dom, _e) => {
        return dom;
      },
      valueEnum: {
        "SHFE": { text: '上期所' },
        "CFFEX": { text: '上金所' },
        "INE": { text: '能源交易所' },
        "DCE": { text: '大商所' },
        "CZCE": { text: '郑商所' },
      },
    },
    {
      title: '方向',
      dataIndex: 'PosiDirection',
      hideInSearch: true,
      valueEnum: {
        "Short": { text: '空头' },
        "Long": { text: '多头' },
      },
      render: (dom, entity) => {
        return dom;
      },
    },
    {
      title: '持仓',
      dataIndex: 'PositionDate',
      hideInSearch: true,
      render: (dom, entity: any) => {
        if (entity.PositionDate === 'Today') {
          return <>
            <div>今持仓:{entity.Position}</div>
          </>;
        }
        return <>
          <div>昨持仓:{entity.Position}</div>
        </>;
      },
    },
    {
      title: '开仓均价',
      dataIndex: 'OpenCost',
      hideInSearch: true,
      render: (dom, entity: any) => {
        const price = entity.OpenCost / (entity.Position * entity.VolumeMultiple);
        return numeral(price).format(0);
      },
    },
    {
      title: '盈亏',
      dataIndex: 'LastPrice',
      hideInSearch: true,
      render: (dom, entity: any) => {
        let price = entity.OpenCost - (entity.LastPrice * entity.Position * entity.VolumeMultiple);

        if (entity.PosiDirection === 'Long') {
          price = -price;
        }

        if (price < 0) {
          return <div style={{backgroundColor: "green", color: "white"}}>{numeral(price).format("+0")}</div>;
        }

        if (price > 0) {
          return <div style={{backgroundColor: "red", color: "white"}}>{numeral(price).format("+0")}</div>;
        }

        return <div>{numeral(price).format("+0")}</div>;
      },
    },
    {
      title: '操作',
      dataIndex: 'option',
      valueType: 'option',
      render: (_k, entity) => [
        <a
          key="quotation"
          onClick={() => {
            setCurrentPosition(entity);
            setShowPosition(true);
          }}
        >行情</a>,
        <a
          key="close"
          onClick={() => {
            // TODO
          }}
        >平仓</a>,
      ],
    },
  ];

  const show_search = !(route && route?.id)

  const handleChangeKlineType = (e: RadioChangeEvent) => {
    setKlineType(e.target.value);
  };

  const onSearchProduct = (value: any) => {
    setOptions(
      _.filter(products, option => option.value.toLowerCase().includes(value.toLowerCase()))
    )
  }

  const onSelectProduct = (pid: any) => {
    setLoading(true);
    request(`/api/v1/product/${pid}`, {
      method: 'GET',
    }).then(ids => {
      const sorted = _.sortBy(ids);
      setInstruments(sorted);
      localStorage.setItem(ID_KEY, JSON.stringify(sorted));
    })
  }

  return (
    <PageContainer
      header={{
        title: route?.id
      }}
      content={
        show_search &&
        <div style={{ textAlign: 'center' }}>
          <AutoComplete
            size="large"
            onSearch={onSearchProduct}
            onSelect={onSelectProduct}
            style={{ maxWidth: 522, width: '100%' }}
            options={options}
          >
            <Input.Search size="large" placeholder="商品" enterButton />
          </AutoComplete>
        </div>
      }
    >
      <GridContent>
        { instruments.map(instrument => <Row gutter={24}>
            <Col xl={24} lg={24} sm={24} xs={24} style={{ marginBottom: 24 }}>
              <Card
                title={instrument}
                bordered={false}
                style={{
                  height: '100%',
                }}
                extra={
                  <div>
                    <span className="dropdown-group">
                      <Dropdown
                        overlay={
                          <Menu onClick={select => {
                            console.log(select);
                            if (select.key === "delete") {
                              const filtered = _.filter(instruments, item => (item !== instrument));
                              setInstruments(filtered);
                              localStorage.setItem(ID_KEY, JSON.stringify(filtered));
                              return;
                            }

                            if (select.key === "top") {
                              const filtered = _.filter(instruments, item => (item !== instrument));
                              const ins = [instrument, ...filtered];
                              localStorage.setItem(ID_KEY, JSON.stringify(ins));
                              setInstruments(ins);
                            }
                          }}>
                            <Menu.Item key="top">置顶</Menu.Item>
                            <Menu.Item key="delete">移除</Menu.Item>
                          </Menu>
                        }
                        placement="bottomRight">
                        <EllipsisOutlined />
                      </Dropdown>
                    </span>
                    <div className="sales-type-radio">
                      <Radio.Group value={klineType} onChange={handleChangeKlineType}>
                        <Radio.Button value="minute">分钟</Radio.Button>
                        <Radio.Button value="10minute">十分</Radio.Button>
                        <Radio.Button value="hour">小时</Radio.Button>
                        <Radio.Button value="day">日线</Radio.Button>
                      </Radio.Group>
                    </div>
                  </div>
                }
              >
                { instrument && <StockChart id={instrument} type={klineType}/> }
              </Card>
            </Col>
          </Row>
        ) }
      </GridContent>
    </PageContainer>
  );
};

export default Product;
