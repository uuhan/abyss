import request from 'umi-request';
import type { BasicListItemDataType } from './data.d';

type ParamsType = {
  count?: number;
} & Partial<BasicListItemDataType>;

export async function queryFuseList(
  params: ParamsType,
): Promise<{ data: BasicListItemDataType[]; total: number; }> {
  return request('/api/v1/strategy/fuse', {
    params,
  });
}

export async function queryCtaList(
  params: ParamsType,
): Promise<{ data: BasicListItemDataType[]; total: number; }> {
  return request('/api/v1/strategy/cta', {
    params,
  });
}

export async function removeFuseList(
  params: ParamsType,
): Promise<{ data: BasicListItemDataType[] }> {
  return request('/api/post_fake_list', {
    method: 'POST',
    data: {
      ...params,
      method: 'delete',
    },
  });
}

export async function addFuseList(
  params: ParamsType,
): Promise<{ data: BasicListItemDataType[] }> {
  return request('/api/post_fake_list', {
    method: 'POST',
    data: {
      ...params,
      method: 'post',
    },
  });
}

export async function updateFuseList(
  params: ParamsType,
): Promise<{ data: BasicListItemDataType[] }> {
  return request('/api/post_fake_list', {
    method: 'POST',
    data: {
      ...params,
      method: 'update',
    },
  });
}
