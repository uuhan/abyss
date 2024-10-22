// TODO: 使用 cmake 的方式构建绑定
#[allow(unused_imports)]
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

#[cfg(feature = "v6_3_13")]
static VERSION: &'static str = "v6_3_13";

#[cfg(feature = "v6_3_15")]
static VERSION: &'static str = "v6_3_15";

#[cfg(feature = "v6_3_19")]
static VERSION: &str = "v6_3_19";

#[cfg(feature = "v6_7_0")]
static VERSION: &str = "v6_7_0";

pub fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    #[allow(unused)]
    let out_dir = std::path::PathBuf::from(&out);

    // 不用每次重复生成
    // 如果之后有更新, 可以使用这段逻辑或者直接使用bindgen命令
    cfg_if::cfg_if! {
        if #[cfg(feature = "ctpbind")] {
            let _bindings = bindgen::Builder::default()
                .clang_args(&["-I", format!("./ctp/{}/linux/include", VERSION).as_str()])
                .clang_args(&["-x", "c++", "-std=c++11"])
                .clang_args(&[format!("-D{}", VERSION)])
                .rust_target(bindgen::RustTarget::Nightly)
                .layout_tests(false)
                .generate_inline_functions(false)
                .derive_debug(true)
                .derive_default(true)
                .header("./ctp/ctp.h")
                .generate()
                .expect("Unable to generate ctp bindings")
                .write_to_file(out_dir.join("ctp.rs"))
                .expect("Can not write ctp.rs");
        }
    }

    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .flag("-w")
        .flag("-std=c++11")
        .flag("-I./ctp")
        .flag(format!("-I{}/include", &out).as_str())
        .flag(format!("-D{}", VERSION).as_str())
        .file("./ctp/ctp.cc")
        .file("./ctp/mdspi.cc")
        .file("./ctp/mdapi.cc")
        .file("./ctp/traderapi.cc")
        .file("./ctp/traderspi.cc");

    #[cfg(debug_assertions)]
    builder.flag("-DDEBUG");

    let mut options = CopyOptions::new();
    options.skip_exist = true;

    match &target_os[..] {
        "linux" => {
            match &target_arch[..] {
                "x86_64" => {
                    copy(format!("ctp/{}/linux/lib", VERSION), out_dir, &options).unwrap();
                    // linux flags
                    builder
                        .flag("-DTARGET=1")
                        .flag(format!("-I./ctp/{}/linux/include", VERSION).as_str());
                }
                _ => {
                    panic!("unsupported arch");
                }
            }
        }
        "macos" => {
            match &target_arch[..] {
                "aarch64" => {
                    copy(
                        format!("ctp/{}/macos/aarch64/lib", VERSION),
                        out_dir,
                        &options,
                    )
                    .unwrap();
                    // mac-arm flags
                    builder
                        .flag("-DTARGET=1")
                        .flag(format!("-I./ctp/{}/macos/include", VERSION).as_str());
                }
                "x86_64" => {
                    copy(
                        format!("ctp/{}/macos/x86_64/lib", VERSION),
                        out_dir,
                        &options,
                    )
                    .unwrap();
                    // mac-intel flags
                    builder
                        .flag("-DTARGET=1")
                        .flag(format!("-I./ctp/{}/macos/include", VERSION).as_str());
                }
                _ => {
                    panic!("unsupported arch");
                }
            }
        }
        _ => {
            panic!("unsupported os")
        }
    }

    builder.compile("ctp");

    println!("cargo:rerun-if-changed=ctp/ctp.h");
    println!("cargo:rerun-if-changed=ctp/ctp.cc");
    println!("cargo:rerun-if-changed=ctp/mdspi.cc");
    println!("cargo:rerun-if-changed=ctp/mdapi.cc");
    println!("cargo:rerun-if-changed=ctp/traderapi.cc");
    println!("cargo:rerun-if-changed=ctp/traderspi.cc");

    println!("cargo:rustc-link-search=native={}/lib", &out);

    #[cfg(not(feature = "no-linkage"))]
    link_library();
}

#[allow(unused)]
fn link_library() {
    let target = std::env::var("TARGET").unwrap();

    if target == "aarch64-apple-ios" {
        // aarch64-apple-ios
        println!("cargo:rustc-link-lib=static=ctpapi.arm64");
        println!("cargo:rustc-link-lib=static=crypto.arm64");
        println!("cargo:rustc-link-lib=static=ssl.arm64");
        println!("cargo:rustc-link-lib=static=comunicationkeylib.arm64");
    }

    if target == "x86_64-unknown-linux-gnu" {
        // x86_64-unknown-linux-gnu
        println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
        println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");
    }

    if target == "x86_64-apple-darwin" {
        println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
        println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");
    }

    if target == "aarch64-apple-darwin" {
        println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
        println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");
    }
}
