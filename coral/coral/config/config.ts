// https://umijs.org/config/
import { defineConfig } from 'umi';
import { join } from 'path';
import defaultSettings from './defaultSettings';
import proxy from './proxy';
import { InjectManifest } from 'workbox-webpack-plugin'
const { REACT_APP_ENV } = process.env;
export default defineConfig({
  hash: true,
  antd: {},
  dva: {
    hmr: true,
  },
  layout: {
    // https://umijs.org/zh-CN/plugins/plugin-layout
    locale: false,
    siderWidth: 160,
    ...defaultSettings,
  },
  targets: {
    ie: 11,
  },
  dynamicImport: {
    loading: '@ant-design/pro-layout/es/PageLoading',
  },
  history: {
    type: 'hash'
  },
  publicPath: "/assets/",
  // umi routes: https://umijs.org/docs/routing
  routes: [
    {
      path: '/user',
      layout: false,
      routes: [
        {
          path: '/user/login',
          layout: false,
          name: 'login',
          component: './user/Login',
        },
        {
          path: '/user',
          redirect: '/user/login',
        },
        {
          component: '404',
        },
      ],
    },
    {
      path: '/market',
      name: '市场',
      icon: 'dashboard',
      routes: [
        {
          path: '/market',
          redirect: '/market/analysis',
        },
        {
          name: '分析',
          path: '/market/analysis',
          component: './market/analysis',
        },
        {
          name: '行情',
          path: '/market/quotation',
          routes: [
            {
              name: '大盘',
              path: '/market/quotation/all',
              component: './market/quotation/all',
            },
            {
              name: '品种',
              path: '/market/quotation/product',
              component: './market/quotation/product',
            },
          ],
        },
        {
          name: '合约',
          path: '/market/instrument',
          component: './market/instrument',
          routes: [
            {
              name: '列表页',
              path: '/market/instrument/info',
              component: './market/instrument/list',
            },
            {
              name: '手续费',
              path: '/market/instrument/commission',
              component: './market/instrument/commission',
            },
            {
              name: '保证金',
              path: '/market/instrument/margin',
              component: './market/instrument/margin',
            },
          ]
        },
      ],
    },
    {
      name: '投资',
      path: '/portfolio',
      icon: 'sliders',
      access: 'canAdmin',
      routes: [
        {
          name: '项目',
          path: '/portfolio/project',
          component: './portfolio/project',
        },
        {
          name: '策略',
          path: '/portfolio/strategy',
          component: './portfolio/strategy',
        },
        {
          name: '交易',
          path: '/portfolio/trade',
          component: './portfolio/trade',
        },
        {
          path: '/portfolio/trade/:id',
          component: './portfolio/trade',
        },
        {
          name: '回测',
          path: '/portfolio/backtesting',
          component: './portfolio/project',
        },
      ],
    },
    {
      name: '风控',
      path: '/risk',
      icon: 'radar-chart',
      routes: [
        {
          name: '监控',
          path: '/risk/monitor',
          component: './risk/monitor',
        },
        {
          name: '模型',
          path: '/risk/model',
          component: './risk/model',
        },
      ],
    },
    {
      path: '/system',
      icon: 'cluster',
      name: '系统',
      access: 'canAdmin',
      routes: [
        {
          path: '/system',
          redirect: '/system/task',
        },
        {
          name: '服务',
          path: '/system/task',
          component: './system/task',
        },
      ],
    },
    {
      path: '/database',
      icon: 'database',
      name: '数据',
      access: 'canAdmin',
      routes: [
        {
          name: '表单',
          icon: 'table',
          path: '/database/table',
          component: './database/table',
        },
        {
          path: '/database/table/:id',
          component: './database/search/tables',
        },
        {
          path: '/database/search',
          name: '搜索',
          component: './database/search',
          routes: [
            {
              path: '/database/search',
              redirect: '/database/search/tables',
            },
            {
              name: '表单',
              icon: 'table',
              path: '/database/search/tables',
              component: './database/search/tables',
            },
          ],
        },
        {
          path: '/database',
          redirect: '/database/table',
        },
        {
          name: '添加',
          icon: 'smile',
          path: '/database/basic-list',
          component: './database/basic-list',
        },
      ],
    },
    {
      name: '账户',
      icon: 'user',
      path: '/account',
      routes: [
        {
          path: '/account',
          redirect: '/account/center',
        },
        {
          name: '活动',
          icon: 'smile',
          path: '/account/center',
          component: './account/center',
        },
        {
          name: '持仓',
          path: '/account/position',
          component: './account/position',
        },
        {
          name: '权益',
          path: '/account/interest',
          component: './account/interest',
        },
        {
          name: '设置',
          icon: 'smile',
          path: '/account/settings',
          component: './account/settings',
        },
        {
          name: '注册',
          icon: 'smile',
          path: '/account/register',
          component: './account/register',
          access: 'canAdmin',
        },
        {
          path: '/account/register-result',
          component: './account/register-result',
        },
      ],
    },
    {
      isHide: true,
      icon: 'warning',
      path: '/exception',
      routes: [
        {
          path: '/exception',
          redirect: '/exception/403',
        },
        {
          name: '403',
          icon: 'smile',
          path: '/exception/403',
          component: './exception/403',
        },
        {
          name: '404',
          icon: 'smile',
          path: '/exception/404',
          component: './exception/404',
        },
        {
          name: '500',
          icon: 'smile',
          path: '/exception/500',
          component: './exception/500',
        },
      ],
    },
    {
      path: '/',
      redirect: '/market/analysis',
    },
    {
      component: '404',
    },
  ],
  // Theme for antd: https://ant.design/docs/react/customize-theme-cn
  theme: {
    'primary-color': defaultSettings.primaryColor,
    'form-vertical-label-padding': '0 0px',
  },
  // esbuild is father build tools
  // https://umijs.org/plugins/plugin-esbuild
  esbuild: {},
  title: false,
  ignoreMomentLocale: true,
  proxy: proxy[REACT_APP_ENV || 'dev'],
  manifest: {
    basePath: '/',
  },
  // Fast Refresh 热更新
  fastRefresh: {},
  openAPI: [
    {
      requestLibPath: "import { request } from 'umi'",
      // 或者使用在线的版本
      // schemaPath: "https://gw.alipayobjects.com/os/antfincdn/M%24jrzTTYJN/oneapi.json"
      schemaPath: join(__dirname, 'oneapi.json'),
      mock: false,
    },
    {
      requestLibPath: "import { request } from 'umi'",
      schemaPath: 'https://gw.alipayobjects.com/os/antfincdn/CA1dOm%2631B/openapi.json',
      projectName: 'swagger',
    },
  ],
  nodeModulesTransform: {
    type: 'none',
  },
  mfsu: false,
  webpack5: {},
  chainWebpack(config) {
    // workbox 配置
    config.plugin('workbox').use(InjectManifest, [
      {
        swSrc: '@/service-worker.js',
        swDest: 'sw.js',
        exclude: [/\.map$/, /favicon\.ico$/, /^manifest.*\.js?$/],
      },
    ]);

    config.module.rule('esm')
      .test(/\.m?jsx?$/)
      .resolve.set('fullySpecified', false);

    config.module.rule('tsv')
      .test(/\.tsv$/)
      .type('asset/source' as any);
  },
  exportStatic: {},
});
