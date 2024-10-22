import request from 'umi-request';

export async function getMargins(
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
  }[]>('/api/v1/margins', {
    method: 'GET',
    params: {
      ...params,
    },
    ...(options || {}),
  })
}
