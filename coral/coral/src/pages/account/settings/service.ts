import request from 'umi-request';
import type { CurrentUser, GeographicItemType } from './data';

export async function queryCurrent(): Promise<{ data: CurrentUser }> {
  return request('/api/v1/user');
}

export async function queryProvince(): Promise<{ data: GeographicItemType[] }> {
  return request('/api/geographic/province');
}

export async function queryCity(province: string): Promise<{ data: GeographicItemType[] }> {
  return request(`/api/geographic/city/${province}`);
}

export async function query() {
  return request('/api/users');
}
