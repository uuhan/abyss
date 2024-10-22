pub use abyss_lua::mlua::{DebugEvent, HookTriggers, Lua, Value};
pub use abyss_lua::*;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let lua = LuaBuilder::new(Path::new("examples/plugins"))
        .with_lua_reload_info_log(false)
        .build()?;

    lua.load("require(\"loop\").run();").exec()?;

    Ok(())
}
