import { PlusOutlined } from '@ant-design/icons';
import { Button, message, Drawer, Divider, Skeleton } from 'antd';
import React, { useState, useRef } from 'react';
import { PageContainer, FooterToolbar } from '@ant-design/pro-layout';
import type { ProColumns, ActionType } from '@ant-design/pro-table';
import ProTable from '@ant-design/pro-table';
import { useParams } from 'umi';
import { ModalForm, ProFormText, ProFormTextArea } from '@ant-design/pro-form';
import type { ProDescriptionsItemProps } from '@ant-design/pro-descriptions';
import ProDescriptions from '@ant-design/pro-descriptions';
import type { FormValueType } from './components/UpdateForm';
import UpdateForm from './components/UpdateForm';
import { table, addTable, updateTable, removeTable } from './service';
import type { TableListItem, TableListPagination } from './data';
/**
 * 添加节点
 *
 * @param fields
 */

const handleDelete = async (fields: {password: string; key: number[], table: number[]}) => {
  const hide = message.loading('正在删除');

  try {
    const data = await removeTable(fields);
    hide();
    if (data.success) {
      message.success('删除成功!');
      return true;
    }
    message.error('删除失败!');
    return false;
  } catch (error: any) {
    hide();
    message.error(`删除失败请重试！${error.message}`);
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
/**
 * 删除节点
 *
 * @param selectedRows
 */

const handleRemove = async (selectedRows: TableListItem[]) => {
  const hide = message.loading('正在删除');
  if (!selectedRows) return true;

  try {
    hide();
    message.success('删除成功，即将刷新');
    return true;
  } catch (error) {
    hide();
    message.error('删除失败，请重试');
    return false;
  }
};

type TableProps = {
  // 0,0,0,0
  search: string;
}

const Table: React.FC<TableProps> = (props) => {
  const route = useParams() as {id?: string};
  const [deleteModalVisible, handleModalVisible] = useState<boolean>(false);
  const [updateModalVisible, handleUpdateModalVisible] = useState<boolean>(false);
  const [showDetail, setShowDetail] = useState<boolean>(false);
  const [tableInfo, setTableInfo] = useState<{size?: number}>({});
  const actionRef = useRef<ActionType>();
  const [currentRow, setCurrentRow] = useState<TableListItem>();
  const [selectedRowsState, setSelectedRows] = useState<TableListItem[]>([]);

  const columns: ProColumns<TableListItem>[] = [
    {
      title: '键',
      dataIndex: 'key',
      width: '20vw',
      render: (dom, entity) => {
        return (
          <a
            onClick={() => {
              setCurrentRow(entity);
              setShowDetail(true);
            }}
          >
            <div>{entity.key.join()}</div>
            <div>{new TextDecoder().decode(Uint8Array.from(entity.key))}</div>
          </a>
        );
      },
    },
    {
      title: '值',
      dataIndex: 'value',
      ellipsis: true,
      render: (dom, entity) => {
        return dom;
      },
    },
    {
      title: '操作',
      dataIndex: 'option',
      valueType: 'option',
      width: '15vw',
      render: (_, record) => [
        <a
          key="detail"
          onClick={() => {
            handleUpdateModalVisible(true);
            setCurrentRow(record);
          }}
        >
          <Button type="primary">查看</Button>
        </a>,
        <a
          key="delete"
          onClick={() => {
            setCurrentRow(record);
            handleModalVisible(true);
          }}
        >
          <Button type="primary" danger>删除</Button>
        </a>,
      ],
    },
  ];

  // 来自搜索框或者路由
  const tableName = props.search || route.id || '';

  return ( <>
    <ProTable<TableListItem, TableListPagination>
      actionRef={actionRef}
      rowKey="key"
      search={false}
      request={async params => {
        if (params.name && params.name?.length > 0) {
          return table(params);
        }

        return {}
      }}
      params={{
        name: tableName,
      }}
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
            await handleRemove(selectedRowsState);
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
      onFinish={async (fields: any) => {
        if (currentRow) {
          const tableNameParsed = tableName.split(",").map(c => parseInt(c, 10));
          const success = await handleDelete({
            password: fields.password,
            key: currentRow?.key,
            table: tableNameParsed,
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
        render: (form, defaultDoms) => {
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
                form.submit();
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
      height="min(50vh, 600px)"
      destroyOnClose={true}
      onClose={() => {
        setCurrentRow(undefined);
        setShowDetail(false);
      }}
      closable={false}
    >
      <Divider />
    </Drawer>
  </>);
};

export default Table;
