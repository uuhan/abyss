import request from 'umi-request';

export async function position(
  params: { day?: string; },
  options?: { [key: string]: any }
) {
  return request(
    '/api/v1/position',
    {
      method: 'GET',
      params: {
        ...params,
      },
      ...(options || {})
    }
  )
}

export async function instrument(id: string) {
  return request(
    `/api/v1/instrument/${id}`,
    {
      method: 'GET',
    }
  )
}

export async function tick(id: string) {
  return request(
    `/api/v1/tick/${id}`,
    {
      method: 'GET',
    }
  )
}
