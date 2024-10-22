import { Settings as LayoutSettings } from '@ant-design/pro-layout';

const Settings: LayoutSettings & {
  pwa?: boolean;
  logo?: string;
} = {
  navTheme: 'light',
  // 拂晓蓝
  primaryColor: '#1890ff',
  layout: 'top',
  contentWidth: 'Fluid',
  fixedHeader: false,
  fixSiderbar: false,
  splitMenus: false,
  colorWeak: false,
  title: 'Abyss',
  pwa: true,
  logo: '/assets/logo.svg',
  iconfontUrl: '',
};

export default Settings;
