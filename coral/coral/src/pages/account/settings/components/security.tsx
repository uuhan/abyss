import React, {
  useState,
} from 'react';
import { List, message } from 'antd';
import { history, useModel } from 'umi';
import { stringify } from 'querystring';
import request from 'umi-request';
import ProForm, {
  ProFormText,
  ModalForm,
} from '@ant-design/pro-form';
import {
  LockOutlined,
} from '@ant-design/icons';
import { outLogin } from '@/services/abyss/api';

type Unpacked<T> = T extends (infer U)[] ? U : T;

const passwordStrength = {
  strong: <span className="strong">强</span>,
  medium: <span className="medium">中</span>,
  weak: <span className="weak">弱 Weak</span>,
};

/**
 * 退出登录，并且将当前的 url 保存
 */
const loginOut = async () => {
  await outLogin();
  const { query = {}, pathname } = history.location;
  const { redirect } = query;
  // Note: There may be security issues, please note
  if (window.location.pathname !== '/user/login' && !redirect) {
    history.replace({
      pathname: '/user/login',
      search: stringify({
        redirect: pathname,
      }),
    });
  }
};


const SecurityView: React.FC = () => {
  const [showPasswordForm, setShowPasswordForm] = useState<boolean>(false);
  const { initialState, setInitialState } = useModel('@@initialState');

  const getData = () => [
    {
      title: '账户密码',
      description: (
        <p>登陆密码</p>
      ),
      actions: [<a key="Modify" onClick={() => setShowPasswordForm(true)}>修改</a>],
    },
  ];

  const data = getData();
  return (
    <>
      <List<Unpacked<typeof data>>
        itemLayout="horizontal"
        dataSource={data}
        renderItem={(item) => (
          <List.Item actions={item.actions}>
            <List.Item.Meta title={item.title} description={item.description} />
          </List.Item>
        )}
      />
      <ModalForm
        title="修改密码"
        visible={showPasswordForm}
        modalProps={{
          destroyOnClose: true,
          onCancel: () => {
            setShowPasswordForm(false)
          }
        }}
        isKeyPressSubmit={true}
        onFinish={async (form) => {
          if (form.new !== form.newDup) {
            message.error("新密码不匹配!");
            return;
          }

          if (form.old === form.new) {
            message.error("新密码不能与旧密码相同!");
            return;
          }

          delete form.newDup;

          try {
            const response = await request('/api/v1/account/change-password', {
              method: 'POST',
              data: form,
            });

            if (response.success) {
              message.info("密码更新成功, 即将退出登陆...", 3, () => {
                setInitialState((s) => ({ ...s, currentUser: undefined }));
                loginOut();
              });
            } else {
              message.error("密码更新失败!");
            }
          } catch (error: any) {
            message.error(error?.message ?? error)
          }
        }}
      >
        <ProForm.Group>
          <ProFormText.Password
            name="old"
            fieldProps={{
              size: 'large',
                prefix: <LockOutlined/>,
            }}
            placeholder={'旧密码'}
            rules={[
              {
                required: true,
                message: '旧密码是必填项！',
              },
            ]}
          />
        </ProForm.Group>
        <ProForm.Group>
          <ProFormText.Password
            name="new"
            fieldProps={{
              size: 'large',
                prefix: <LockOutlined/>,
            }}
            placeholder={'新密码'}
            rules={[
              {
                required: true,
                min: 6,
                message: '新密码长度不少于6！',
              },
            ]}
          />
          <ProFormText.Password
            name="newDup"
            fieldProps={{
              size: 'large',
                prefix: <LockOutlined/>,
            }}
            placeholder={'再次输入新密码'}
            rules={[
              {
                required: true,
                min: 6,
                message: '新密码长度不少于6！',
              },
            ]}
          />
        </ProForm.Group>
      </ModalForm >
    </>
  );
};

export default SecurityView;
