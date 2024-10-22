import request from 'umi-request';
import type { AnalysisData } from './data';

export async function fakeChartData(): Promise<{ data: AnalysisData }> {
  return request('/api/v1/fake_analysis_chart_data');
}
