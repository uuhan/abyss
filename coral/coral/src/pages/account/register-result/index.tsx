import { Button, Result } from 'antd';
import { Link } from 'umi';
import React from 'react';
import type { RouteChildrenProps } from 'react-router';

import styles from './style.less';

const actions = (
  <div className={styles.actions}>
    <a href="">
      <Button size="large" type="primary">
        <Link to="/user/login">前往登陆</Link>
      </Button>
    </a>
    <Link to="/">
      <Button size="large">返回首页</Button>
    </Link>
  </div>
);

export type LocationState = Record<string, unknown>;

const RegisterResult: React.FC<RouteChildrenProps> = ({ location }) => {
  const email = location.state
    ? (location.state as LocationState).account
    : 'AntDesign@example.com';
  return (
    <Result
      className={styles.registerResult}
      status="success"
      title={
        <div className={styles.title}>
          <span>注册成功</span>
        </div>
      }
      extra={actions}
    />
  );
};

export default RegisterResult;
