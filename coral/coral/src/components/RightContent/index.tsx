import { Space } from 'antd';
import {
  QuestionCircleOutlined,
  TransactionOutlined,
} from '@ant-design/icons';
import React from 'react';
import { useModel, Link } from 'umi';
import Avatar from './AvatarDropdown';
import HeaderSearch from '../HeaderSearch';
import styles from './index.less';
import NoticeIconView from '../NoticeIcon';
export type SiderTheme = 'light' | 'dark';

const GlobalHeaderRight: React.FC = () => {
  const { initialState } = useModel('@@initialState');

  if (!initialState || !initialState.settings) {
    return null;
  }

  const { navTheme, layout } = initialState.settings;
  let className = styles.right;

  if ((navTheme === 'dark' && layout === 'top') || layout === 'mix') {
    className = `${styles.right}  ${styles.dark}`;
  }

  return (
    <Space className={className}>
      <Link className={styles.link} to="/portfolio/trade">
        <TransactionOutlined />
      </Link>
      <NoticeIconView />
      <Avatar menu />
    </Space>
  );
};

export default GlobalHeaderRight;
