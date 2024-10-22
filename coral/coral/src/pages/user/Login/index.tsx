import {
  AlipayCircleOutlined,
  LockOutlined,
  KeyOutlined,
  TaobaoCircleOutlined,
  UserOutlined,
  WeiboCircleOutlined,
} from '@ant-design/icons';
import { Alert, Space, message, Tabs } from 'antd';
import React, { useState } from 'react';
import ProForm, { ProFormCaptcha, ProFormCheckbox, ProFormText } from '@ant-design/pro-form';
import { Link, history, useModel } from 'umi';
import { login, getProducts } from '@/services/abyss/api';
import { getFakeCaptcha } from '@/services/abyss/login';
import styles from './index.less';

const LoginMessage: React.FC<{
  content: string;
}> = ({ content }) => (
  <Alert
    style={{
      marginBottom: 24,
    }}
    message={content}
    type="error"
    showIcon
  />
);

const Login: React.FC = () => {
  const [submitting, setSubmitting] = useState(false);
  const [userLoginState, setUserLoginState] = useState<API.LoginResult>({});
  const [type, setType] = useState<string>('account');
  const { initialState, setInitialState } = useModel('@@initialState');

  const fetchUserInfo = async () => {
    const userInfo = await initialState?.fetchUserInfo?.();
    let products: string[] = [];
    try {
      products = await getProducts();
    } catch(err: any) { console.log(err.message) }

    if (userInfo) {
      await setInitialState((s) => ({ ...s, currentUser: userInfo, products, }));
    }
  };

  const handleSubmit = async (values: API.LoginParams) => {
    setSubmitting(true);

    try {
      // 登录
      const msg = await login({ ...values, type });

      if (msg.status === 'ok') {
        const defaultLoginSuccessMessage = '登录成功！';
        message.success(defaultLoginSuccessMessage);
        await fetchUserInfo();
        /** 此方法会跳转到 redirect 参数所在的位置 */

        if (!history) return;
        const { query } = history.location;
        const { redirect } = query as {
          redirect: string;
        };
        history.push(redirect || '/');
        return;
      }

      let failMessage;
      if (type === 'account') {
        failMessage = '验证失败，请检查账户密码!';
      } else {
        failMessage = '验证失败，无效的Token!';
      }
      message.error(failMessage);

      setUserLoginState(msg);
    } catch (error) {
      const defaultLoginFailureMessage = '登录失败，请重试！';
      message.error(defaultLoginFailureMessage);
    }

    setSubmitting(false);
  };

  const { status, type: loginType } = userLoginState;
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.top}>
          <div className={styles.header}>
            <Link to="/">
              <img alt="logo" className={styles.logo} src="/assets/logo.svg" />
              <span className={styles.title}>Abyss</span>
            </Link>
          </div>
        </div>

        <div className={styles.main}>
          <ProForm
            initialValues={{}}
            submitter={{
              searchConfig: {
                submitText: '登录',
              },
              render: (_, dom) => dom.pop(),
              submitButtonProps: {
                loading: submitting,
                size: 'large',
                style: {
                  width: '100%',
                },
              },
            }}
            isKeyPressSubmit={true}
            onFinish={async (values) => {
              await handleSubmit(values as API.LoginParams);
            }}
          >
            <Tabs activeKey={type} onChange={setType}>
              <Tabs.TabPane key="account" tab={'账户密码登录'} />
              <Tabs.TabPane key="token" tab={'Token登录'} />
            </Tabs>

            {status === 'error' && loginType === 'account' && (
              <LoginMessage content={'错误的用户名和密码'} />
            )}
            {type === 'account' && (
              <>
                <ProFormText
                  name="name"
                  fieldProps={{
                    size: 'large',
                    prefix: <UserOutlined className={styles.prefixIcon} />,
                  }}
                  placeholder={'用户名'}
                  rules={[
                    {
                      required: true,
                      message: '用户名是必填项！',
                    },
                  ]}
                />
                <ProFormText.Password
                  name="password"
                  fieldProps={{
                    size: 'large',
                    prefix: <LockOutlined className={styles.prefixIcon} />,
                  }}
                  placeholder={'密码'}
                  rules={[
                    {
                      required: true,
                      message: '密码是必填项！',
                    },
                  ]}
                />
              </>
            )}

            {status === 'error' && loginType === 'token' && <LoginMessage content="验证码错误" />}
            {type === 'token' && (
              <>
                <ProFormText.Password
                  fieldProps={{
                    size: 'large',
                    prefix: <KeyOutlined/>,
                  }}
                  name="token"
                  placeholder={'请输入密钥'}
                  rules={[
                    {
                      required: true,
                      message: 'Token缺失！',
                    },
                  ]}
                />
              </>
            )}
          </ProForm>
        </div>
      </div>
    </div>
  );
};

export default Login;
