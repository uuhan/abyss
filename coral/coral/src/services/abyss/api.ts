// @ts-ignore
/* eslint-disable */
import request, { extend } from 'umi-request';

const TOKEN_KEY = "TOKEN";

request.use(async (ctx, next) => {
  // 登陆地址不需要 token
  if (ctx.req.url !== '/api/v1/login/account') {
    let headers = ctx.req.options.headers;
    let token = localStorage.getItem(TOKEN_KEY);
    if (token) {
      ctx.req.options.headers = {
        'Authorization': `Bearer ${token}`,
        ...headers,
      };
    }
  }

  await next();
});

declare global {
    interface Window { abyss: any; }
}

// 调试使用
window.abyss = request;

/** 获取当前的用户 GET /api/currentUser */
export async function currentUser(options?: { [key: string]: any }) {
  return request<{
    data: API.CurrentUser;
  }>('/api/v1/user', {
    method: 'GET',
    ...(options || {}),
  });
}

/** 退出登录接口 POST /api/login/outLogin */
export async function outLogin(options?: { [key: string]: any }) {
  // 简单删除 TOKEN 以达到登出的目的
  localStorage.removeItem(TOKEN_KEY);
  return;
  // return request<Record<string, any>>('/api/v1/logout', {
  //   method: 'POST',
  //   ...(options || {}),
  // });
}

/** 登录接口 POST /api/login/account */
export async function login(body: API.LoginParams, options?: { [key: string]: any }) {
  return request<API.LoginResult>('/api/v1/login/account', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: body,
    ...(options || {}),
  }).then(response => {
    if (response.token) {
      // 保存 TOKEN
      localStorage.setItem(TOKEN_KEY, response.token);
    }
    return response;
  });
}

/** 此处后端没有提供注释 GET /api/notices */
export async function getNotices(options?: { [key: string]: any }) {
  return request<API.NoticeIconList>('/api/v1/notice', {
    method: 'GET',
    ...(options || {}),
  });
}

export async function getProducts(options?: { [key: string]: any }) {
  return request<string[]>('/api/v1/products', {
    method: 'GET',
    ...(options || {}),
  });
}

// 交易接口
export async function ctpTrade(body: API.TradeBody) {
  return request<API.TradeResult>('/api/v1/ctp/trade', {
    method: 'POST',
    data: body,
  })
}
