import { PageContainer, FooterToolbar } from '@ant-design/pro-layout';
import ProTable from '@ant-design/pro-table';
import type { ProDescriptionsItemProps } from '@ant-design/pro-descriptions';
import ProDescriptions from '@ant-design/pro-descriptions';
import {
  Card, Col, Row, Modal,
  Input, Button, message,
  Menu, Dropdown, Radio,
  Drawer, Tabs, Switch,
} from 'antd';
import { Link } from 'umi';
import type { RadioChangeEvent } from 'antd/es/radio';
import { useParams } from 'umi';
import { useEffect, useState, useRef } from 'react';
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
  PlusOutlined, AimOutlined, EllipsisOutlined,
  ExclamationCircleOutlined,
  CheckOutlined, CloseOutlined,
} from '@ant-design/icons';
import { GridContent } from '@ant-design/pro-layout';
import { Updating, StockChart, MinutesStockChart } from '@/pages/components/KlineChart';
import type { FC } from 'react';
import type { TableListItem, TableListPagination, CommissionItem, MarginItem, TickItem } from './data';
import type { ProColumns, ActionType } from '@ant-design/pro-table';
import { ctpTrade } from '@/services/abyss/api';
import { position as get_position, instrument as get_instrument, tick as get_tick } from './service';
import numeral from 'numeral';
import * as styles from './style.less';

const { TabPane } = Tabs;
const ID_KEY = "QUOTATION_DEFAULT_ID"

const menu = (
  <Menu onSelect={key => {
    console.log('menu:', key);
  }}>
    <Menu.Item key="top">置顶</Menu.Item>
    <Menu.Item key="refresh">刷新</Menu.Item>
  </Menu>
);

const dropdownGroup = (
  <span className="dropdown-group">
    <Dropdown overlay={menu} placement="bottomRight">
      <EllipsisOutlined />
    </Dropdown>
  </span>
);

type KlineType = 'minute' | '10minute' | 'hour' | 'day';

type FormObject = {
  volume?: number,
}

const Subject: FC = () => {
  const route = useParams() as {id?: string};
  const DEFAULT_ID = localStorage.getItem(ID_KEY);
  const [instrument, setInstrument] = useState<string|null>(route?.id || DEFAULT_ID);
  // 买卖方向, true 为买入, false 为卖出
  const [posiDirection, setPosiDirection] = useState<boolean>(true);
  const [form, setForm] = useState<FormObject>({ volume: 1 });
  const [showOrderForm, setShowOrderForm] = useState<boolean>(false);
  const [showPosition, setShowPosition] = useState<boolean>(false);
  const [ctpLoading, setCtpLoading] = useState<boolean>(false);
  const [klineType, setKlineType] = useState<KlineType>('minute');
  const [currentPosition, setCurrentPosition] = useState<any>({});
  const actionRef = useRef<ActionType>();

  // 路由变更
  if (route?.id && instrument !== route?.id) {
    setInstrument(route?.id)
  }

  const handleFormSubmit = (value: string) => {
    if (value && value !== instrument) {
      localStorage.setItem(ID_KEY, value);
      setInstrument(value)
    }
  };

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

  const submit = async (body: Partial<API.TradeBody>) => {
    if (!instrument) {
      message.error('未配置交易合约!');
      return;
    }

    setCtpLoading(true);

    try {
      const result = await ctpTrade({ ...body, id: instrument } as any);
      if (result.success) {
        message.success(`报单成功: 价:${result.price}, 量:${result.volume}`);
        // Modal.confirm({
        //   title: '报单成功',
        //   icon: <CheckOutlined style={{color:"green"}}/>,
        //   content: `成交价格: ${result.price}, 成交手数: ${result.volume}`,
        //   okText: '确认',
        //   cancelText: null,
        // });
      } else {
        message.error(`报单失败: ${result.message}`);
        // Modal.confirm({
        //   title: '报单失败',
        //   icon: <CloseOutlined style={{color:"red"}}/>,
        //   content: result.message,
        //   okText: '确认',
        //   cancelText: null,
        // });
      }
      actionRef.current?.reload();
    } catch (err: any) {
      message.error(err?.message ?? err);
    } finally {
      setCtpLoading(false);
    }
  }

  const handleOpen = async (evt: any) => {
    const body: Partial<API.TradeBody> = { volume: form.volume };

    body.offset = '开仓';

    if (posiDirection) {
      body.direction = '买';
    } else {
      body.direction = '卖';
    }

    await submit(body);
  }

  const handleCloseYesterday = async (evt: any) => {
    const body: Partial<API.TradeBody> = { volume: form.volume };

    body.offset = '平昨';

    // NB: 平仓为当前方向的反方向
    if (posiDirection) {
      body.direction = '卖';
    } else {
      body.direction = '买';
    }

    await submit(body);
  }

  const handleCloseToday = async (evt: any) => {
    const body: Partial<API.TradeBody> = { volume: form.volume };

    body.offset = '平今';

    // NB: 平仓为当前方向的反方向
    if (posiDirection) {
      body.direction = '卖';
    } else {
      body.direction = '买';
    }

    await submit(body);
  }

  const handleConfig = (evt: any) => {
    setShowOrderForm(true);
  }

  const columns: ProColumns<TableListItem>[] = [
    {
      title: '合约ID',
      dataIndex: 'id',
      render: (dom, {id}) => {
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
      dataIndex: 'exchange',
      render: (dom, _) => {
        return dom;
      },
    },
    {
      title: '持仓',
      dataIndex: 'date',
      hideInSearch: true,
      render: (dom, { date, position, direction }) => {
        if (date === 'Today') {
          if (direction === 'Long') {
            return <>
              <div>今多:{position}</div>
              </>;
          }
          return <>
            <div>今空:{position}</div>
          </>;
        }

        if (direction === 'Long') {
          return <>
            <div>昨多:{position}</div>
            </>;
        }

        return <>
          <div>昨空:{position}</div>
        </>;
      },
    },
    {
      title: '开仓均价',
      dataIndex: 'open',
      hideInSearch: true,
      render: (dom, {open}) => {
        if (open) {
          return numeral(open).format(0);
        }
        return '-';
      },
    },
    {
      title: '盈亏',
      dataIndex: 'profit',
      hideInSearch: true,
      render: (dom, {profit}) => {
        if (profit < 0) {
          return <div style={{backgroundColor: "green", color: "white"}}>{numeral(profit).format("+0")}</div>;
        }

        if (profit > 0) {
          return <div style={{backgroundColor: "red", color: "white"}}>{numeral(profit).format("+0")}</div>;
        }

        return <div>{numeral(profit).format("+0")}</div>;
      },
    },
    {
      title: '保本',
      hideInSearch: true,
      render: (dom, entity) => {
        return <Switch checkedChildren="开启" unCheckedChildren="关闭"/>
      },
    },
    {
      title: '操作',
      dataIndex: 'option',
      valueType: 'option',
      render: (_k, entity) => {
        const buttons = [
          <a
            key="quotation"
            onClick={() => {
              setCurrentPosition(entity);
              setShowPosition(true);
            }}
          >行情</a>
        ];

        if (entity.position) {
          buttons.push(
            <a
              key="close"
              onClick={() => {
                Modal.confirm({
                  title: '确认平仓',
                  icon: <ExclamationCircleOutlined />,
                  content: `合约: ${entity.id}, 手数: ${entity.position}`,
                  okText: '确认',
                  cancelText: '取消',
                  onOk: async () => {
                    const { id, date, position, direction: d } = entity;
                    const body: Partial<API.TradeBody> = { id, volume: position };
                    if (d === 'Long') {
                      body.direction = '卖';
                    } else {
                      body.direction = '买';
                    }

                    if (date === 'Today') {
                      body.offset = '平今';
                    } else {
                      body.offset = '平昨';
                    }

                    try {
                      const result = await ctpTrade(body as any);
                      if (result.success) {
                        Modal.confirm({
                          title: '报单成功',
                          icon: <CheckOutlined style={{color:"green"}}/>,
                          content: `成交价格: ${result.price}, 成交手数: ${result.volume}`,
                          okText: '确认',
                          cancelText: null,
                        });
                      } else {
                        Modal.confirm({
                          title: '报单失败',
                          icon: <CloseOutlined style={{color:"red"}}/>,
                          content: result.message,
                          okText: '确认',
                          cancelText: null,
                        });
                      }
                      actionRef.current?.reload();
                    } catch (err: any) {
                      message.error(err?.message ?? err);
                    }
                  }
                });
              }}
            >平仓</a>
          )
        };

        return buttons;
      },
    },
  ];

  const show_search = !(route && route?.id)

  const handleChangeKlineType = (e: RadioChangeEvent) => {
    setKlineType(e.target.value);
  };

  const handlePosiDirectionChange = (checked: boolean) => {
    setPosiDirection(checked);
  }

  let closeTodayColor = 'green';
  if (!posiDirection) {
    closeTodayColor = 'deeppink';
  }

  return (
    <PageContainer
      header={{
        title: route?.id
      }}
      content={
        show_search &&
        <div style={{ textAlign: 'center' }}>
          <Input.Search
            placeholder="合约"
            enterButton="搜索"
            size="large"
            onSearch={handleFormSubmit}
            style={{ maxWidth: 522, width: '100%' }}
          />
        </div>
      }
      className={styles.container}
    >
      <GridContent>
        <Row gutter={24}>
          <Col xl={24} lg={24} sm={24} xs={24} style={{ marginBottom: 24 }}>
            <Card
              title={instrument}
              bordered={false}
              style={{
                height: '100%',
              }}
              extra={
                <div>
                  {dropdownGroup}
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
        <Row gutter={24}>
          <Col xl={24} lg={24} sm={24} xs={24} style={{ marginBottom: 24 }}>
            <Card title="当前持仓" bordered={false}
              headStyle={{padding: '0 24px'}}
              bodyStyle={{padding: '24px 0'}}
            >
              <ProTable<TableListItem, TableListPagination>
                actionRef={actionRef}
                rowKey="id"
                search={false}
                request={async (params) => {
                  const data = await get_position({});
                  return {
                    data,
                  }
                }}
                options={{
                  density: true,
                }}
                columns={columns}
              />
            </Card>
          </Col>
        </Row>
      </GridContent>

      <FooterToolbar className="subject-footer">
        <div style={{flex: 3, display: 'flex', justifyContent: 'start'}}>
          <Button
            type="primary"
            loading={ctpLoading}
            onClick={handleOpen}
            style={{marginRight: '10px'}}
            danger={posiDirection}
          >{ posiDirection ? '买' : '卖' }开仓</Button>

          <Button
            type="primary"
            loading={ctpLoading}
            onClick={handleCloseToday}
            style={{backgroundColor: closeTodayColor, borderColor: closeTodayColor}}
          >{ posiDirection ? '卖' : '买' }平今</Button>

          <Button
            type="primary"
            loading={ctpLoading}
            onClick={handleCloseYesterday}
            style={{marginLeft: '10px'}}
            danger={!posiDirection}
          >{ posiDirection ? '卖' : '买' }平昨</Button>
        </div>
        <div style={{flex: 2, display: 'flex', direction: 'rtl'}}>
          <Switch
            style={{ backgroundColor: posiDirection ? 'red' : 'green' }}
            checkedChildren="买"
            unCheckedChildren="卖"
            checked={posiDirection}
            onChange={handlePosiDirectionChange}
          />
        </div>
      </FooterToolbar>

      <ModalForm
        title="报单配置"
        visible={showOrderForm}
        modalProps={{
          destroyOnClose: true,
          onCancel: () => {
            setShowOrderForm(false);
          }
        }}
        onFinish={async (form1) => {
          setForm(form1);
          setShowOrderForm(false);
          return true;
        }}
      >
        <ProForm.Group>
          <ProFormText
            width="xs"
            name="id"
            label="合约"
            placeholder="合约ID"
            initialValue={instrument}
            disabled
          />
          <ProFormDigit
            width="xs"
            name="volume"
            initialValue={form.volume}
            label="手数"
          />
        </ProForm.Group>

        <ProForm.Group>
          <ProFormSelect
            options={[]}
            width="md"
            name="strategy"
            label="选用策略"
          />
        </ProForm.Group>
      </ModalForm>

      <Button className="config-button" onClick={handleConfig}><AimOutlined/></Button>

      <Drawer
        width="min(90vw, 600px)"
        visible={showPosition}
        onClose={() => {
          setCurrentPosition({});
          setShowPosition(false);
        }}
        closable={true}
        destroyOnClose={true}
        placement="top"
        height="min(90vh, 600px)"
        bodyStyle={{padding: "10px 10px"}}
      >
        { currentPosition.id && <StockChart id={currentPosition.id} type="day"/> }
      </Drawer>
    </PageContainer>
  );
};

export default Subject;
