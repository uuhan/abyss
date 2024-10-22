import { Card, Col, Row, message } from 'antd';
import type { FC } from 'react';
import { useEffect, useState } from 'react';
import request from 'umi-request';
import { Liquid, RingProgress, Treemap } from '@ant-design/charts';

import { GridContent } from '@ant-design/pro-layout';
import styles from './style.less';

const Monitor: FC = () => {
  const [fetchData1, setData1] = useState([]);
  const [fetchData2, setData2] = useState([]);
  const asyncFetch = () => {
    request('/api/v1/ticks/by-volume')
      .then(data => setData1(data))
      .catch((error) => {
        message.error(error?.message ?? error)
      });

    request('/api/v1/ticks/by-money')
      .then(data => setData2(data))
      .catch((error) => {
        message.error(error?.message ?? error)
      });
  };
  useEffect(() => {
    asyncFetch();
  }, []);

  const data1 = {
    name: 'root',
    children: fetchData1,
  };

  const data2 = {
    name: 'root',
    children: fetchData2,
  };

  const config = {
    data: data1,
    colorField: 'product',
    label: {
      style: {
        fill: 'black',
      }
    }
  };

  const config1 = {
    data: data2,
    colorField: 'product',
    label: {
      style: {
        fill: 'black',
      }
    }
  };

  return (
    <GridContent>
      <>
        <Row gutter={24}>
          <Col xl={24} lg={24} md={24} sm={24} xs={24} style={{ marginBottom: 24 }}>
            <Card title="成交量" bordered={false}>
              {/*
              <Row>
                <Col md={6} sm={12} xs={24}>
                  <Statistic
                    title="今日交易总额"
                    suffix="元"
                    value={numeral(124543233).format('0,0')}
                  />
                </Col>
                <Col md={6} sm={12} xs={24}>
                  <Statistic title="销售目标完成率" value="92%" />
                </Col>
                <Col md={6} sm={12} xs={24}>
                  <Countdown title="活动剩余时间" value={deadline} format="HH:mm:ss:SSS" />
                </Col>
                <Col md={6} sm={12} xs={24}>
                  <Statistic title="每秒交易总额" suffix="元" value={numeral(234).format('0,0')} />
                </Col>
              </Row>
              */}
              <div className={styles.mapChart}>
                <Treemap {...config}/>
              </div>
            </Card>
            <Card title="成交额" bordered={false}>
              <div className={styles.mapChart}>
                <Treemap {...config1}/>
              </div>
            </Card>
          </Col>
        </Row>
      </>
    </GridContent>
  );
};

export default Monitor;
