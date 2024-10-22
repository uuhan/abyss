import { PlusOutlined } from '@ant-design/icons';
import { Button, message, Drawer, Divider, Skeleton } from 'antd';
import React, { useState, useRef } from 'react';
import { PageContainer, FooterToolbar } from '@ant-design/pro-layout';
import type { ProColumns, ActionType } from '@ant-design/pro-table';
import ProTable from '@ant-design/pro-table';
import { Link } from 'umi';
import { ModalForm, ProFormText, ProFormTextArea } from '@ant-design/pro-form';
import type { ProDescriptionsItemProps } from '@ant-design/pro-descriptions';
import ProDescriptions from '@ant-design/pro-descriptions';
import type { FormValueType } from './components/UpdateForm';
import UpdateForm from './components/UpdateForm';
import { table, table_info, updateTable, removeTable } from './service';
import type { TableListItem, TableListPagination } from './data';
/**
 * 添加节点
 *
 * @param fields
 */

const handleDelete = async (fields: { password: string; table: number[]; }) => {
  const hide = message.loading('正在删除');

  try {
    const data = await removeTable({ ...fields });
    hide();
    if (data.success) {
      message.success('删除成功!');
      return true;
    }
    message.error('删除失败!');
    return false;
  } catch (error) {
    hide();
    message.error('删除失败请重试！');
    return false;
  }
};
/**
 * 更新节点
 *
 * @param fields
 */

const handleUpdate = async (fields: FormValueType, currentRow?: TableListItem) => {
  const hide = message.loading('正在配置');

  try {
    await updateTable({
      ...currentRow,
      ...fields,
    });
    hide();
    message.success('配置成功');
    return true;
  } catch (error) {
    hide();
    message.error('配置失败请重试！');
    return false;
  }
};

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
      title: '表名',
      dataIndex: 'name',
      render: (dom, entity) => {
        return (
          <a
            onClick={() => {
              setCurrentRow(entity);
              setLoading(true);
              table_info({name: entity.name.join()}).then(data => {
                setTableInfo(data);
                setLoading(false);
              }).catch(err => {
                message.error(err?.message || err);
                setLoading(false);
              });
              setShowDetail(true);
            }}
          >
            <div>{new TextDecoder().decode(Uint8Array.from(entity.name))}</div>
          </a>
        );
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
          <Link to={`/database/table/${record.name.join()}`}>
            <Button type="primary">查看</Button>
          </Link>,
          <a
            key="delete"
            onClick={() => {
              setCurrentRow(record);
              handleModalVisible(true);
            }}
          >
            <Button type="primary" danger>删除</Button>
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
        request={table}
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
        title="请输入密码以确认!"
        width="400px"
        visible={deleteModalVisible}
        onVisibleChange={handleModalVisible}
        modalProps={{
          destroyOnClose: true,
        }}
        onFinish={async (fields) => {
          if (currentRow) {
            const success = await handleDelete({
              password: fields.password,
              table: currentRow.name,
            });
            if (success) {
              handleModalVisible(false);
              if (actionRef.current) {
                actionRef.current.reload();
              }
            }
          }
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
        <ProFormText.Password
          placeholder="删除操作不可恢复!"
          rules={[
            {
              required: true,
              message: '请输入账户密码!',
            },
          ]}
          width="md"
          name="password"
        />
      </ModalForm>
      <UpdateForm
        onSubmit={async (value) => {
          const success = await handleUpdate(value, currentRow);

          if (success) {
            handleUpdateModalVisible(false);
            setCurrentRow(undefined);

            if (actionRef.current) {
              actionRef.current.reload();
            }
          }
        }}
        onCancel={() => {
          handleUpdateModalVisible(false);
          setCurrentRow(undefined);
        }}
        updateModalVisible={updateModalVisible}
        values={currentRow || {}}
      />

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
