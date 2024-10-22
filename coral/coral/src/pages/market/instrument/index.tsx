import { PageContainer } from '@ant-design/pro-layout';
import { Input, AutoComplete } from 'antd';
import type { FC } from 'react';
import { useEffect, useState } from 'react';
import { history, useModel } from 'umi';

type SearchProps = {
  match: {
    url: string;
    path: string;
  };
  location: {
    pathname: string;
  };
};

const tabList = [
  {
    key: 'info',
    tab: '详情页',
  },
  {
    key: 'commission',
    tab: '手续费',
  },
  {
    key: 'margin',
    tab: '保证金',
  },
];

const Search: FC<SearchProps> = (props) => {
  const getTabKey = () => {
    const { match, location } = props;
    const url = match.path === '/' ? '' : match.path;
    const tabKey = location.pathname.replace(`${url}/`, '');
    if (tabKey && tabKey !== '/') {
      return tabKey;
    }
    return 'info';
  };
  const tabKey = getTabKey();

  const handleTabChange = (key: string) => {
    const { match } = props;
    const url = match.url === '/' ? '' : match.url;
    switch (key) {
      case 'info':
        history.push(`${url}/info`);
        break;
      case 'commission':
        history.push(`${url}/commission`);
        break;
      case 'margin':
        history.push(`${url}/margin`);
        break;
      default:
        break;
    }
  };

  return (
    <PageContainer
      tabList={tabList}
      tabActiveKey={tabKey}
      onTabChange={handleTabChange}
    >
      {props.children}
    </PageContainer>
  );
};

export default Search;
