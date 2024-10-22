#![allow(clippy::type_complexity)]
#![allow(unused_imports, non_snake_case)]
// Rust <-> CPP 胶水函数
// 这里注意, 如果 Rust 通过 void* 的方式从CPP 那边获取Fn,
// 那么这个指针在闭包执行完之后不应该被 drop 掉
// 要在 xxxSpi 类里面考虑销毁, 注册的回调生命周期和 xxxSpi 一样.
use encoding_rs::GBK;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

mod market;
mod trader;

// void* char 转 str
#[inline]
pub fn cstr(s: &[c_char]) -> Cow<str> {
    unsafe { CStr::from_ptr(s.as_ptr()).to_string_lossy() }
}

// void* char 转 str
// 注意: c的数组类型,过滤多余的0,这个函数会改变字段尺寸
// 一般用于字符串处理, 语义上不是字符串的不要使用这个函数
#[inline]
pub fn gbk(s: &[u8]) -> Cow<str> {
    let len = s.iter().take_while(|&byte| *byte != 0).count();
    let (dec, _, _) = GBK.decode(&s[0..len]);
    dec
}

#[inline]
pub fn s(s: &[u8]) -> &str {
    let len = s.iter().take_while(|&byte| *byte != 0).count();
    // NB: 确保数组是有效的utf8字符串
    unsafe { std::str::from_utf8_unchecked(&s[0..len]) }
    // unsafe {
    //     std::ffi::CStr::from_ptr(s.as_ptr() as _)
    //         .to_str()
    //         .unwrap_or_default()
    // }
}

#[inline]
pub fn ascii(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect::<String>()
}

#[inline]
pub fn a(s: &[u8]) -> &[u8] {
    let len = s.iter().take_while(|&byte| *byte != 0).count();
    &s[0..len]
}

#[cfg(test)]
mod tests {
    use super::*;

    // GBK - CTP:不合法的登录
    static SAMPLE1: &[u8] = &[
        67, 84, 80, 58, 178, 187, 186, 207, 183, 168, 181, 196, 181, 199, 194, 189,
    ];
    static SAMPLE2: &[i8] = &[
        67, 84, 80, 58, -78, -69, -70, -49, -73, -88, -75, -60, -75, -57, -62, -68, 0,
    ];

    // #[bench]
    // fn bench_ctp_s(b: &mut Bencher) {
    //     b.iter(|| {
    //         s(SAMPLE1);
    //     });
    // }
    //
    // #[bench]
    // fn bench_ctp_cstr(b: &mut Bencher) {
    //     b.iter(|| {
    //         cstr(SAMPLE2);
    //     });
    // }
    //
    // // 这个测试性能更好
    // #[bench]
    // fn bench_ctp_gbk(b: &mut Bencher) {
    //     b.iter(|| {
    //         gbk(SAMPLE1);
    //     });
    // }

    #[test]
    fn test_gbk_shrink_size() {
        let origin: [u8; 8] = [1, 2, 3, 0, 0, 0, 0, 0];
        let t = gbk(&origin);
        assert_eq!(t.as_ref().len(), 3);

        let origin: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let t = gbk(&origin);
        assert_eq!(t.as_ref().len(), 0);
    }
}
