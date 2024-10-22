import request from 'umi-request';

export async function getCommissions(
  params: {
    id?: string;
    pid?: string;
  },
  options?: { [key: string]: any }
) {
  return request<{
    day: string;
    open: number;
    close_t: number;
    close_y: number;
  }[]>('/api/v1/commissions', {
    method: 'GET',
    params: {
      ...params,
    },
    ...(options || {}),
  })
}
