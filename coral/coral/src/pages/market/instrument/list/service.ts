// @ts-ignore
/* eslint-disable */
import request from 'umi-request';
import { TableListItem, CommissionItem, MarginItem, TickItem } from './data';
import {ctp_s, ctp_u} from '@/utils'

export async function instruments(
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
    /** 列表的内容总数 */
    total?: number;
    success?: boolean;
  }>('/api/v1/instruments', {
    method: 'GET',
    params: {
      ...params,
    },
    ...(options || {}),
  }).then((response) => {
    let data = response.data.map((raw) => {
      return {
        ...raw,
        ExchangeID: ctp_s(raw.ExchangeID as number[]),
        InstrumentID: ctp_s(raw.InstrumentID as number[]),
        InstrumentName: ctp_u(raw.InstrumentName as number[]),
        ExpireDate: ctp_s(raw.ExpireDate as number[]),
        EndDelivDate: ctp_s(raw.EndDelivDate as number[]),
      } as TableListItem
    });

    response.data = data;

    return response
  });
}

export async function get_commission(
  params: {
    id: string
  },
) {
  return request<CommissionItem>(`/api/v1/instrument/${params.id}/commission`, {
    method: 'GET',
  }).then((response) => {
    return {
      data: response,
    }
  });
}

export async function get_margin(
  params: {
    id: string
  },
) {
  return request<MarginItem>(`/api/v1/instrument/${params.id}/margin`, {
    method: 'GET',
  }).then((response) => {
    return {
      data: response,
    }
  });
}

export async function get_tick(
  params: {
    id: string
  },
) {
  return request<TickItem>(`/api/v1/tick/${params.id}`, {
    method: 'GET',
  }).then((response) => {
    return response
  });
}
