import request from 'umi-request';

export interface StateType {
  status?: 'ok' | 'error';
  message?: string;
}

export interface UserRegisterParams {
  name: string,
  role: string[],
  password: string;
}

export async function doRegister(params: UserRegisterParams) {
  return request('/api/v1/user/register', {
    method: 'POST',
    data: params,
  });
}
