use abyss_lua::LuaBuilder;
use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;

fn criterion_benchmark(crt: &mut Criterion) {
    crt.bench_function("Lua Create", |b| {
        b.iter(|| {
            LuaBuilder::new(Path::new("examples/plugins"))
                .build()
                .unwrap();
        })
    });

    crt.bench_function("Lua Run", |b| {
        let lua = LuaBuilder::new(Path::new("examples/plugins"))
            .with_lua_reload_info_log(false)
            .build()
            .unwrap();
        b.iter(|| {
            lua.load("require(\"demo\").empty();").exec().unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
