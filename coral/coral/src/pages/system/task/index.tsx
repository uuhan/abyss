import { PlusOutlined } from '@ant-design/icons';
import { Button, message, Drawer, Divider, Skeleton } from 'antd';
import React, { useState, useRef } from 'react';
import { PageContainer, FooterToolbar } from '@ant-design/pro-layout';
import type { ProColumns, ActionType } from '@ant-design/pro-table';
import ProTable from '@ant-design/pro-table';
import { Link } from 'umi';
import request from 'umi-request';
import { ModalForm, ProFormText, ProFormTextArea } from '@ant-design/pro-form';
import type { ProDescriptionsItemProps } from '@ant-design/pro-descriptions';
import ProDescriptions from '@ant-design/pro-descriptions';
import { tasks } from './service';
import type { TableListItem, TableListPagination } from './data';

const TableList: React.FC = () => {
  /** 新建窗口的弹窗 */
  const [deleteModalVisible, handleModalVisible] = useState<boolean>(false);
  /** 分布更新窗口的弹窗 */

  const [updateModalVisible, handleUpdateModalVisible] = useState<boolean>(false);
  const [showDetail, setShowDetail] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [tableInfo, setTableInfo] = useState<{size?: number}>({});
  const actionRef = useRef<ActionType>();
  const [currentRow, setCurrentRow] = useState<TableListItem>();
  const [selectedRowsState, setSelectedRows] = useState<TableListItem[]>([]);
  /** 国际化配置 */

  const columns: ProColumns<TableListItem>[] = [
    {
      title: '任务',
      dataIndex: 'name',
      render: (dom, entity) => {
        return dom;
      },
    },
    {
      title: '操作',
      dataIndex: 'option',
      valueType: 'option',
      width: '50px',
      fixed: true,
      render: (_, record) => {
        return [
          <Link to="#">
            <Button type="primary">查看</Button>
          </Link>,
          <a
            key="delete"
            onClick={() => {
              setCurrentRow(record);
              handleModalVisible(true);
            }}
          >
            <Button type="primary" danger>停止</Button>
          </a>,
        ];
      }
    },
  ];

  return (
    <PageContainer>
      <ProTable<TableListItem, TableListPagination>
        actionRef={actionRef}
        rowKey="name"
        search={{
          labelWidth: 120,
        }}
        request={tasks}
        columns={columns}
        rowSelection={{
          onChange: (_, selectedRows) => {
            setSelectedRows(selectedRows);
          },
        }}
      />
      {selectedRowsState?.length > 0 && (
        <FooterToolbar
          extra={
            <div>
              已选择{' '}
              <a
                style={{
                  fontWeight: 600,
                }}
              >
                {selectedRowsState.length}
              </a>{' '}
              项
            </div>
          }
        >
          <Button
            type="primary"
            onClick={async () => {
              console.log(selectedRowsState);
              setSelectedRows([]);
              actionRef.current?.reloadAndRest?.();
            }}
            danger
          >
            批量删除
          </Button>
        </FooterToolbar>
      )}
      <ModalForm
        title="请输入服务名字以确认停止!"
        width="400px"
        visible={deleteModalVisible}
        onVisibleChange={handleModalVisible}
        isKeyPressSubmit={true}
        modalProps={{
          destroyOnClose: true,
        }}
        onFinish={async (fields) => {
          if (fields.task !== currentRow?.name) {
            message.error("当前服务名错误!");
            return;
          }

          try {
            const result = await request('/api/v1/tasks', {
              method: 'DELETE',
              data: fields,
            });

            if (result.success) {
              message.info(`服务停止中: ${fields.task}`, 1, () => {
                actionRef.current?.reload();
              });
            }
          } catch (err: any) {
            message.error(err?.message ?? err);
          }

          handleModalVisible(false);

        }}
        submitter={{
          render: (props, defaultDoms) => {
            return [
              <Button
                key="cancel"
                onClick={() => {
                  handleModalVisible(false)
                }}
              >取消</Button>,
              <Button
                key="confirm"
                type="primary"
                danger
                onClick={() => {
                  props.submit();
                }}
              >删除</Button>,
            ];
          },
        }}
      >
        <ProFormText
          placeholder="服务"
          rules={[
            {
              required: true,
              message: '请输入服务名称!',
            },
          ]}
          width="md"
          name="task"
        />
      </ModalForm>

      <Drawer
        visible={showDetail}
        placement="top"
        height="min(40vh, 500px)"
        destroyOnClose={true}
        onClose={() => {
          setCurrentRow(undefined);
          setShowDetail(false);
        }}
        closable={false}
      >
        {currentRow?.name && (
          <ProDescriptions<TableListItem>
            column={2}
            title={currentRow?.name}
            request={async () => ({
              data: currentRow || {},
            })}
            columns={columns as ProDescriptionsItemProps<TableListItem>[]}
          />
        )}
        <Divider />
        { loading ? <Skeleton active /> :
          <ProDescriptions<{size?: number}>
            column={2}
            request={async () => ({ data: tableInfo })}
            columns={[
              {
                title: '条目',
                dataIndex: 'size',
                render: (dom, entity) => {
                  return dom;
                },
              },

            ]}
          />
        }
      </Drawer>
    </PageContainer>
  );
};

export default TableList;
