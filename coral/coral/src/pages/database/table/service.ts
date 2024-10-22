// @ts-ignore
/* eslint-disable */
import request from 'umi-request';
import { TableListItem } from './data';

/** 获取规则列表 GET /api/rule */
export async function table(
  params: {
    // query
    /** 当前的页码 */
    current?: number;
    /** 页面的容量 */
    pageSize?: number;
  },
  options?: { [key: string]: any },
) {
  return request<{
    data: TableListItem[];
    success: boolean;
    current: number;
    total: number;
    pageSize: number;
  }>('/api/v1/db/trees', {
    method: 'GET',
    params: {
      ...params,
    },
    ...(options || {}),
  })
}

/** 获取规则列表 GET /api/rule */
export async function table_info(
  params: Record<string, any>,
  options?: { [key: string]: any },
) {
  return request<{
    size: number;
  }>('/api/v1/db/tree', {
    method: 'GET',
    params: {
      ...params,
    },
    ...(options || {}),
  })
}

/** 新建规则 PUT /api/rule */
export async function updateTable(data: { [key: string]: any }, options?: { [key: string]: any }) {
  return request<TableListItem>('/api/rule', {
    data,
    method: 'PUT',
    ...(options || {}),
  });
}

/** 新建规则 POST /api/rule */
export async function addTable(data: { [key: string]: any }, options?: { [key: string]: any }) {
  return request<TableListItem>('/api/rule', {
    data,
    method: 'POST',
    ...(options || {}),
  });
}

/** 删除规则 DELETE /api/rule */
export async function removeTable(data: { password: string, table: number[] }, options?: { [key: string]: any }) {
  return request<Record<string, any>>('/api/v1/db/tree', {
    data,
    method: 'DELETE',
    ...(options || {}),
  });
}