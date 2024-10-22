import { PageContainer } from '@ant-design/pro-layout';
import { Input } from 'antd';
import React, { useState } from 'react';
import type { FC } from 'react';
import { history } from 'umi';

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
    key: 'kv',
    tab: '原始表单',
  },
  {
    key: 'tick',
    tab: '行情表单',
  },
  {
    key: 'kline',
    tab: 'K线表单',
  },
];

const Search: FC<SearchProps> = (props) => {
  const [search, setSearch] = useState<string>('');

  const handleTabChange = (key: string) => {
    // eslint-disable-next-line no-console
    console.log(key)
  };

  const handleFormSubmit = (value: string) => {
    const encoded = new TextEncoder().encode(value).toString();
    if (encoded.length > 0 && encoded !== search) {
      // eslint-disable-next-line no-console
      console.log("search:", value, encoded);
      setSearch(encoded)
    }
  };

  return (
    <PageContainer
      content={
        <div style={{ textAlign: 'center' }}>
          <Input.Search
            enterButton="搜索"
            size="large"
            onSearch={handleFormSubmit}
            style={{ maxWidth: 522, width: '100%' }}
          />
        </div>
      }
      tabList={tabList}
      onTabChange={handleTabChange}
    >
      {React.Children.map(props.children, (child: any) => {
        return React.cloneElement(child, {search,})
      })}
    </PageContainer>
  );
};

export default Search;
