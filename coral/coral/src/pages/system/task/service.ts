// @ts-ignore
/* eslint-disable */
import request from 'umi-request';

/** 获取规则列表 GET /api/rule */
export async function tasks(
  params: {
    // query
    /** 当前的页码 */
    current?: number;
    /** 页面的容量 */
    pageSize?: number;
  },
  options?: { [key: string]: any },
) {
  return request<number[][]>('/api/v1/tasks', {
    method: 'GET',
  }).then(tasks => {
    const data = tasks.map(task => {
      let name = new TextDecoder().decode(Uint8Array.from(task));
      return {
        name,
      }
    });

    return {
      data,
    }
  })
}
