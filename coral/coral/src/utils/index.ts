import _ from 'lodash'

function str(raw: number[]): number[] {
  return _.takeWhile(raw, (c) => c!=0)
}

export function ctp_s(raw: number[]): String {
  return String.fromCharCode.apply(null, str(raw))
}

// CTP 中文数据用 GB18030 编码
export function ctp_u(raw: number[]): String {
  let arr = Uint8Array.from(str(raw));
  return new TextDecoder('GB18030').decode(arr)
}
