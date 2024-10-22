import { tsvParse } from "d3-dsv";
import { timeParse } from "d3-time-format";
import { PageLoading } from '@ant-design/pro-layout'
import { Skeleton, message } from 'antd';
import * as React from "react";
import type { IOHLCData } from "./iOHLCData";
import { queryKline } from './service'
import _ from 'lodash'
import * as styles from './styles.less';

const parseDate = timeParse("%Y-%m-%d");

const parseData = () => {
    return (d: any) => {
        const date = parseDate(d.date);
        if (date === null) {
            d.date = new Date(Number(d.date));
        } else {
            d.date = new Date(date);
        }

        for (const key in d) {
            if (key !== "date" && Object.prototype.hasOwnProperty.call(d, key)) {
                d[key] = +d[key];
            }
        }

        return d as IOHLCData;
    };
};

interface WithOHLCDataProps {
    id: string,
    type: string,
    height?: number,
    readonly data: IOHLCData[];
}

interface WithOHLCState {
    loading: boolean;
    data?: IOHLCData[];
}

export function withOHLCData(dataSet?: string) {
    return <TProps extends WithOHLCDataProps>(OriginalComponent: React.ComponentClass<TProps>) => {
        return class WithOHLCData extends React.Component<Omit<TProps, "data">, WithOHLCState> {
          public socket?: WebSocket;

          public constructor(props: Omit<TProps, "data">) {
            super(props);

            this.state = {
              loading: false,
              data: [],
            };
          }

          private fetchData(props: Omit<TProps, "data">) {
            if (!props.id) {
              return;
            }

            let kind = "d";
            const type = props.type ?? "minute";

            switch (type) {
              case 'minute':
                kind = "m";
                break;
              case '10minute':
                kind = "10m";
                break;
              case 'hour':
                kind = "h";
                break;
              case 'day':
                kind = "d";
                break;
              default:
            }

            this.setState({
              loading: true,
            });

            if (dataSet === "MINUTES") {
              kind = "m"
            } else if (dataSet === "HOURS") {
              kind = "h"
            }

            // 郑商所
            // const isCZCE = /^[A-Z]/.test(props.id);

            queryKline(props.id, kind).then(klines => {
              let data = klines.map(k => {
                // if (isCZCE) {
                //   return {
                //     low: k.low,
                //     high: k.high,
                //     open: k.open,
                //     close: k.close,
                //     volume: k.volume,
                //     average: k.average,
                //     date: new Date(k.date),
                //   }
                // }

                const date = new Date(k.date);
                // 夜盘数据, 非郑商所为第二天时间
                if (date.getHours() >= 20) {
                  date.setDate(date.getDate() - 1);
                }
                return {
                  low: k.low,
                  high: k.high,
                  open: k.open,
                  close: k.close,
                  volume: k.volume,
                  average: k.average,
                  date,
                }
              })

              this.setState({
                loading: false,
              });

              // 不合法合约
              if (data.length === 0) {
                message.error("合约错误!");
                return;
              }

              data = _.sortBy(data, k => k.date);
              this.setState({
                data,
              });

              if (this.socket) {
                this.socket.close();
              }

              const TOKEN = localStorage.getItem("TOKEN");
              const SCHEMA = window.location.protocol === 'http:' ? 'ws' : 'wss';
              this.socket = new WebSocket(`${SCHEMA}://${window.location.host}/api/v1/kline/${props.id}?kind=${kind}&token=${TOKEN}`)

              this.socket.onopen = () => {
                // eslint-disable-next-line no-console
                console.log(`${props.id} connected.`);
              }

              this.socket.onclose = (c) => {
                // eslint-disable-next-line no-console
                console.log(`closed:`, c);
              }

              this.socket.onerror = (e) => {
                // eslint-disable-next-line no-console
                console.log(`${props.id} err:`, e);
              }

              this.socket.onmessage = (msg) => {
                const k = JSON.parse(msg.data);

                // 夜盘数据
                const date = new Date(k.date);
                // if (!isCZCE) {
                if (date.getHours() >= 20) {
                  date.setDate(date.getDate() - 1);
                }
                // }

                const kline = { ...k, date, };
                const last = (this.state.data || []).pop();
                if (last) {
                  if (last.date.getMinutes() === kline.date.getMinutes()) {
                    this.state.data!.push(kline)
                  } else {
                    this.state.data!.push(last);
                    this.state.data!.push(kline);
                  }
                }

                this.setState({
                  data: this.state.data
                })
              }
            }).catch(err => {
              this.setState({
                loading: false,
              });

              // eslint-disable-next-line no-console
              console.log("err: ", err);
              message.error(err);
            });
          }

          public componentWillReceiveProps(nextProps: TProps) {
            if (this.props.id !== nextProps.id || this.props.type !== nextProps.type) {
              this.setState({
                data: [],
              }, () => {
                // 注意: 这里等旧数据清理结束之后再进行更新
                // 否则会打乱 StockChart 的 xExtents 计算
                this.fetchData(nextProps)
              })
            }
          }

          public componentDidMount() {
            this.fetchData(this.props)
          }

          public componentWillUnmount() {
            this.socket?.close()
            // eslint-disable-next-line no-console
            console.log("[ws] closed");
          }

          public render() {
            const { data } = this.state;

            if (data?.length === 0 || this.state.loading) {
              return <Skeleton active className={styles.skeleton}/>
            }

            return (
              <OriginalComponent {...(this.props as TProps)} data={data} refresh={() => this.fetchData(this.props)} />
            )

          }
        };
    };
}
