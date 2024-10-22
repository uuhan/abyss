import type { FC } from 'react';
import { Suspense, useState, useEffect } from 'react';
import { EllipsisOutlined } from '@ant-design/icons';
import { Card, Col, Dropdown, Menu, Row, Skeleton, message } from 'antd';
import { Liquid, RingProgress, Treemap } from '@ant-design/charts';
import { GridContent } from '@ant-design/pro-layout';
import request from 'umi-request';
import type { RadioChangeEvent } from 'antd/es/radio';
import type { RangePickerProps } from 'antd/es/date-picker/generatePicker';
import type moment from 'moment';
import MarketCard from './components/MarketCard';
import TopSearch from './components/TopSearch';
import ProportionSales from './components/ProportionSales';
import OfflineData from './components/OfflineData';
import { useRequest } from 'umi';
import PageLoading from './components/PageLoading';
import type { TimeType } from './components/MarketCard';
import { getTimeDistance } from './utils/utils';
import type { AnalysisData } from './data.d';
import styles from './style.less';

type RangePickerValue = RangePickerProps<moment.Moment>['value'];

type AnalysisProps = {
  dashboardAndanalysis: AnalysisData;
  loading: boolean;
};

type SalesType = 'all' | 'online' | 'stores';

const Analysis: FC<AnalysisProps> = () => {
  const [fetchData1, setData1] = useState([]);
  const [fetchData2, setData2] = useState([]);
  const [salesType, setSalesType] = useState<SalesType>('all');
  const [currentTabKey, setCurrentTabKey] = useState<string>('');

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

  const loading = false;
  const data: any = {}

  let salesPieData;
  if (salesType === 'all') {
    salesPieData = data?.salesTypeData;
  } else {
    salesPieData = salesType === 'online' ? data?.salesTypeDataOnline : data?.salesTypeDataOffline;
  }

  const menu = (
    <Menu>
      <Menu.Item>操作一</Menu.Item>
      <Menu.Item>操作二</Menu.Item>
    </Menu>
  );

  const dropdownGroup = (
    <span className={styles.iconGroup}>
      <Dropdown overlay={menu} placement="bottomRight">
        <EllipsisOutlined />
      </Dropdown>
    </span>
  );

  const handleChangeSalesType = (e: RadioChangeEvent) => {
    setSalesType(e.target.value);
  };

  const handleTabChange = (key: string) => {
    setCurrentTabKey(key);
  };

  const activeKey = currentTabKey || (data?.offlineData?.[0] && data?.offlineData?.[0].name) || '';

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
        <Suspense fallback={<Skeleton active/>}>
          <MarketCard/>
        </Suspense>

        <Row gutter={24}
          style={{
            marginTop: 24,
          }}
        >
          <Col xl={12} lg={24} md={24} sm={24} xs={24} style={{ marginBottom: 12 }}>
            <Card title="成交量" bordered={false}>
              <div className={styles.mapChart}>
                <Treemap {...config}/>
              </div>
            </Card>
          </Col>
          <Col xl={12} lg={24} md={24} sm={24} xs={24} style={{ marginBottom: 12 }}>
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

export default Analysis;
