import request from 'umi-request';
import type { TagType, KlineType } from './data';

export async function queryTags(): Promise<{ data: { list: TagType[] } }> {
  return request('/api/tags');
}

export async function queryKline(id: string, kind = "d"): Promise<KlineType[]> {
  return request(`/api/v1/kline/${id}?kind=${kind}&n=4000`)
}
