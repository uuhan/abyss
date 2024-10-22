import { Button, Drawer, Divider, Tabs, Row, Col, Skeleton, message } from 'antd';
import { CloseOutlined } from '@ant-design/icons'
import React, { useState, useRef } from 'react';
import { Link } from 'umi'
import { FooterToolbar } from '@ant-design/pro-layout';
import type { ProColumns, ActionType } from '@ant-design/pro-table';
import ProTable from '@ant-design/pro-table';
import type { ProDescriptionsItemProps } from '@ant-design/pro-descriptions';
import ProDescriptions from '@ant-design/pro-descriptions';
import { instruments, get_commission, get_margin, get_tick } from './service';
import type { TableListItem, TableListPagination, CommissionItem, MarginItem, TickItem } from './data';
import { ctp_s, ctp_u } from '@/utils'
import numeral from 'numeral'
import _ from 'lodash'
import styles from './style.less'
import { StockChart } from '@/pages/components/KlineChart'

const { TabPane } = Tabs;

const CONFIG_KEY = "DRAWER_TAB_KEY";

const TableList: React.FC = () => {
  const key = localStorage.getItem(CONFIG_KEY) || "info";
  const [showDetail, setShowDetail] = useState<boolean>(false);
  const actionRef = useRef<ActionType>();
  const [currentRow, setCurrentRow] = useState<TableListItem>();
  const [loading, setLoading] = useState<boolean>(false);
  const [currentTick, setCurrentTick] = useState<TickItem>();
  const [selectedRowsState, setSelectedRows] = useState<TableListItem[]>([]);
  const [tabKey, setTabKey] = useState<string>(key);
  /** 国际化配置 */

  const columns: ProColumns<TableListItem>[] = [
    {
      title: '合约ID',
      dataIndex: 'InstrumentID',
      render: (dom, entity) => {
        return (
          <a
            onClick={async () => {
              setLoading(true);
              setCurrentRow(entity);
              setShowDetail(true);
              try {
                const tick = await get_tick({ id: entity.InstrumentID as string });
                setCurrentTick(tick)
              } catch (err: any) {
                message.error(err?.message || err);
              }
              setLoading(false);
            }}
          >
            {dom}
          </a>
        );
      },
    },
    {
      title: '合约名称',
      dataIndex: 'InstrumentName',
      hideInSearch: true,
      render: (dom, entity) => {
        return (
          <a
            onClick={async () => {
              setLoading(true);
              setCurrentRow(entity);
              setShowDetail(true);
              try {
                const tick = await get_tick({ id: entity.InstrumentID as string });
                setCurrentTick(tick)
              } catch (err: any) {
                message.error(err?.message || err);
              }
              setLoading(false);
            }}
          >
            {dom}
          </a>
        );
      },
    },
    {
      title: '交易所',
      dataIndex: 'ExchangeID',
      render: (dom, _entity) => {
        return (
          <div>{dom}</div>
        )
      },
      valueEnum: {
        "SHFE": { text: '上期所', exchange: 'SHFE', },
        "CFFEX": { text: '上金所', exchange: 'CFFEX', },
        "INE": { text: '能源交易所', exchange: 'INE', },
        "DCE": { text: '大商所', exchange: 'DCE', },
        "CZCE": { text: '郑商所', exchange: 'CZCE', },
      },
    },
    {
      title: '价格跳动',
      dataIndex: 'PriceTick',
      hideInSearch: true,
      render: (dom, _a) => {
        return (
          <div>{dom}</div>
        )
      },
    },
    {
      title: '合约乘数',
      dataIndex: 'VolumeMultiple',
      hideInSearch: true,
      render: (dom, _a) => {
        return (
          <div>{dom}</div>
        )
      },
    },
    {
      title: '操作',
      dataIndex: 'option',
      valueType: 'option',
      width: "120px",
      fixed: true,
      render: (_k, entity) => [
        <a
          key="like"
          onClick={() => {
            // TODO: 关注
            setCurrentRow(entity);
            message.info("TODO");
          }}
        >关注</a>,
        <Link to={`/portfolio/trade/${entity.InstrumentID}`}>交易</Link>,
      ],
    },
  ];

  return (
    <div>
      <ProTable<TableListItem, TableListPagination>
        actionRef={actionRef}
        rowKey="InstrumentID"
        search={{}}
        request={(params) => {
          return instruments(params)
        }}
        options={{
          density: true,
        }}
        columns={columns}
      />

      <Drawer
        width="min(90vw, 600px)"
        visible={showDetail}
        onClose={() => {
          setCurrentRow(undefined);
          setShowDetail(false);
        }}
        closable={true}
        destroyOnClose={true}
        placement="top"
        height="min(90vh, 600px)"
        bodyStyle={{padding: "0 10px"}}
      >
      <Tabs
        size="large"
        tabBarStyle={{ marginBottom: 24 }}
        activeKey={tabKey}
        onChange={(activeKey) => {
          setTabKey(activeKey);
          window.localStorage.setItem(CONFIG_KEY, activeKey);
        }}
      >
        <TabPane tab="参数" key="info">
          <ProDescriptions<TableListItem>
            column={{xs: 2,sm: 2,md: 3}}
            title={currentRow?.InstrumentName}
            request={async () => ({
              data: currentRow || {},
            })}
            params={{
              id: currentRow?.InstrumentName,
            }}
            columns={columns as ProDescriptionsItemProps<TableListItem>[]}
          />
          <Divider/>
          { loading ? <Skeleton active /> :
            <ProDescriptions<CommissionItem>
              column={{xs: 2,sm: 2,md: 3}}
              title="手续费"
              request={async (params) => {
                if(params.id) {
                  const data = await get_commission({ id: params.id });
                  return data
                }
                return {}
              }}
              params={{
                id: currentRow?.InstrumentID
              }}
              columns={[
                // {
                //   title: '开仓手续费(率)',
                //   dataIndex: 'OpenRatioByMoney',
                // },
                // {
                //   title: '平今手续费(率)',
                //   dataIndex: 'CloseTodayRatioByMoney',
                // },
                // {
                //   title: '平昨手续费(率)',
                //   dataIndex: 'CloseRatioByMoney',
                // },
                // {
                //   title: '开仓手续费(手)',
                //   dataIndex: 'OpenRatioByVolume',
                // },
                // {
                //   title: '平今手续费(手)',
                //   dataIndex: 'CloseTodayRatioByVolume',
                // },
                // {
                //   title: '平昨手续费(手)',
                //   dataIndex: 'CloseRatioByVolume',
                // },
                {
                  title: '开仓手续费',
                  render: (_a, entity) => {
                    if (entity.hasOwnProperty("OpenRatioByMoney")) {
                      if (currentTick && currentRow) {
                        const price = currentTick.LastPrice * currentRow.VolumeMultiple * entity.OpenRatioByMoney
                        if (price) {
                          return numeral(price).format('0.0');
                        }
                      }
                    }

                    if (entity.hasOwnProperty("OpenRatioByVolume")) {
                      const price = entity.OpenRatioByVolume;
                      return price;
                    }

                    return "-";
                  }
                },
                {
                  title: '平今手续费',
                  render: (_a, entity) => {
                    if (entity.hasOwnProperty("CloseTodayRatioByMoney")) {
                      if (currentTick && currentRow) {
                        const price = currentTick.LastPrice * currentRow.VolumeMultiple * entity.CloseTodayRatioByMoney;
                        if (price) {
                          return numeral(price).format('0.0');
                        }
                      }
                    }

                    if (entity.hasOwnProperty("CloseTodayRatioByVolume")) {
                      const price = entity.CloseTodayRatioByVolume;
                      return price;
                    }

                    return "-";
                  }
                },
                {
                  title: '平昨手续费',
                  render: (dom, entity) => {
                    if (entity.hasOwnProperty("CloseRatioByMoney")) {
                      if (currentTick && currentRow) {
                        const price = currentTick.LastPrice * currentRow.VolumeMultiple * entity.CloseRatioByMoney;
                        if (price) {
                          return numeral(price).format('0.0');
                        }
                      }
                    }

                    if (entity.hasOwnProperty("CloseRatioByVolume")) {
                      const price = entity.CloseRatioByVolume;
                      return price;
                    }

                    return "-";
                  }
                },
              ]}
            />
          }
          <Divider/>
          { loading ? <Skeleton active /> :
            <ProDescriptions<MarginItem>
              column={{xs: 2,sm: 2,md: 3}}
              title="保证金"
              request={async (params) => {
                if(params.id) {
                  const data = await get_margin({ id: params.id });
                  return data
                }
                return {}
              }}
              params={{
                id: currentRow?.InstrumentID
              }}
              columns={[
                // {
                //   title: '多头保证金(率)',
                //   dataIndex: 'LongMarginRatioByMoney',
                // },
                // {
                //   title: '多头保证金(手)',
                //   dataIndex: 'LongMarginRatioByVolume',
                // },
                // {
                //   title: '空头保证金(率)',
                //   dataIndex: 'ShortMarginRatioByMoney',
                // },
                // {
                //   title: '空头保证金(手)',
                //   dataIndex: 'ShortMarginRatioByMoney',
                // },
                {
                  title: '多头保证金',
                  render: (_a, entity) => {
                    if (entity.hasOwnProperty("LongMarginRatioByMoney")) {
                      if (currentTick && currentRow) {
                        const price = currentTick.LastPrice * currentRow.VolumeMultiple * entity.LongMarginRatioByMoney;
                        return numeral(price).format('0.0');
                      }
                    }

                    if (entity.hasOwnProperty("LongMarginRatioByVolume")) {
                      const price = entity.LongMarginRatioByVolume;
                      return price;
                    }

                    return "-";
                  }
                },
                {
                  title: '空头保证金',
                  render: (_a, entity) => {
                    if (entity.hasOwnProperty("ShortMarginRatioByMoney")) {
                      if (currentTick && currentRow) {
                        const price = currentTick.LastPrice * currentRow.VolumeMultiple * entity.ShortMarginRatioByMoney;
                        if (price) {
                          return numeral(price).format('0.0');
                        }
                      }
                    }

                    if (entity.hasOwnProperty("ShortMarginRatioByVolume")) {
                      const price = entity.ShortMarginRatioByVolume;
                      return price;
                    }

                    return "-";
                  }
                },
              ]}
            />
          }
        </TabPane>
        <TabPane tab="行情" key="quotation">
          <StockChart id={currentRow?.InstrumentID as string} type="day"/>
        </TabPane>
      </Tabs>
      </Drawer>
    </div>
  );
};

export default TableList;
