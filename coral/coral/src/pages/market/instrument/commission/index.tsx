import {
  Card, AutoComplete, Input,
  Col, Form, List,
  Row, Select, Typography,
  message, Menu, Skeleton,
} from 'antd';
import {
  GridContent,
} from '@ant-design/pro-layout';
import { Line, Column } from '@ant-design/charts';
import moment from 'moment';
import type { FC } from 'react';
import { useState, useEffect, useLayoutEffect, useRef } from 'react';
import { useRequest, useModel } from 'umi';
import _ from 'lodash';
import n from 'numeral';
import { getCommissions } from './service';
import AvatarList from './components/AvatarList';
import StandardFormRow from './components/StandardFormRow';
import TagSelect from './components/TagSelect';
import styles from './style.less';

const { Item } = Menu;
type SettingsStateKeys = 'instrument' | 'product';
type SettingsState = {
  mode: 'inline' | 'horizontal';
  selectKey: SettingsStateKeys;
};

const Commissions: FC = () => {
  // const { data, loading, run } = useRequest((values: any) => {
  //   return Promise.resolve({})
  // });
  const menuMap: Record<string, React.ReactNode> = {
    instrument: '合约',
    product: '品种',
  };

  const [initConfig, setInitConfig] = useState<SettingsState>({
    mode: 'inline',
    selectKey: 'instrument',
  });
  const { initialState, setInitialState } = useModel('@@initialState');
  const products = (initialState?.products || []).map(option => ({ value: option }));

  const [options, setOptions] = useState(products);
  const [data, setData] = useState<any[]>([]);
  const [columnData, setColumnData] = useState<any[]>([]);
  const [loading, setLoading] = useState<boolean>(false);

  const asyncFetch = () => {
  };
  useEffect(() => {
    asyncFetch();
  }, []);

  const lineConfig = {
    data,
    xField: 'day',
    yField: 'value',
    seriesField: 'type',
    label: {
      position: 'top' as any,
      formatter: (datum: any) => {
        return n(datum.value).format("0.0");
      }
    },
  };

  const columnConfig = {
    data: columnData,
    isGroup: true,
    xField: 'id',
    yField: 'value',
    seriesField: 'type',
    label: {
      position: 'top' as any,
      formatter: (datum: any) => {
        return n(datum.value).format("0.0");
      }
    },
  };

  const dom = useRef<HTMLDivElement>();

  const onSearchProduct = (value: any) => {
    setOptions(
      _.filter(products, option => option.value.toLowerCase().includes(value.toLowerCase()))
    )
  }

  const onSearchInstrument = (id: any) => {
    setLoading(true);
    getCommissions({id,})
      .then(fields => setData(fields))
      .catch((error) => {
        message.error(error.message ?? message)
      }).finally(() => {
        setLoading(false);
      });
  }

  const onSelectProduct = (pid: any) => {
    setLoading(true);
    getCommissions({pid,})
      .then(fields => setColumnData(fields))
      .catch((error) => {
        message.error(error.message ?? message)
      }).finally(() => {
        setLoading(false);
      });
  }

  const onSelectMenu = (key: any) => {
    console.log(key)
  }

  const getMenu = () => {
    return Object.keys(menuMap).map((item) => <Item key={item}>{menuMap[item]}</Item>);
  };

  const renderChildren = () => {
    const { selectKey } = initConfig;
    switch (selectKey) {
      case 'instrument':
        return <Card
            title={
              <div style={{ textAlign: 'center' }}>
                <Input.Search
                  size="large"
                  placeholder="合约"
                  enterButton="载入"
                  onSearch={onSearchInstrument}
                  style={{ maxWidth: 522, width: '100%' }}
                />
              </div>
            }
          >
          { loading ? <Skeleton active className={styles.skeleton}/> : <Line {...lineConfig} /> }
        </Card>;

      case 'product':
        return <Card
            title={
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
          { loading ? <Skeleton active className={styles.skeleton}/> : <Column {...columnConfig} /> }
        </Card>;
      default:
        return null;
    }
  }

  return <GridContent>
    <div
      ref={(ref) => { if (ref) { dom.current = ref; } }}
      className={styles.main}
    >
      <div className={styles.left}>
        <Menu
          mode={initConfig.mode}
          selectedKeys={[initConfig.selectKey]}
          onClick={({key}) => {
            setInitConfig({
              ...initConfig,
              selectKey: key as SettingsStateKeys,
            });
          }}
        >
          {getMenu()}
        </Menu>
      </div>
      <div className={styles.right}>
          <div className={styles.title}>{menuMap[initConfig.selectKey]}</div>
          {renderChildren()}
      </div>
    </div>
  </GridContent>
};

export default Commissions;
