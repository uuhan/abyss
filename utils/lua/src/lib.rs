pub use mlua;

use mlua::prelude::*;
use std::path::{Path, PathBuf};

const LUA_RELOAD_SCRIPT: &str = include_str!("../lua_reload.lua");

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum Error {
    /// IO异常: {0}
    IO(#[from] std::io::Error),
    /// Lua执行错误: {0}
    LuaError(#[from] LuaError),
}

pub type LuaResult<T> = std::result::Result<T, Error>;

pub struct LuaBuilder {
    script_path: PathBuf,
    init_script: Option<String>,
    lua_reload_info_log: bool,
    lua_reload_debug_log: bool,
}

impl LuaBuilder {
    pub fn new(script_path: &Path) -> Self {
        LuaBuilder {
            script_path: script_path.to_path_buf(),
            init_script: None,
            lua_reload_info_log: true,
            lua_reload_debug_log: false,
        }
    }

    pub fn with_init_script(mut self, script: impl AsRef<str>) -> Self {
        self.init_script.replace(script.as_ref().to_string());
        self
    }

    pub fn with_lua_reload_info_log(mut self, enable: bool) -> Self {
        self.lua_reload_info_log = enable;
        self
    }

    pub fn with_lua_reload_debug_log(mut self, enable: bool) -> Self {
        self.lua_reload_debug_log = enable;
        self
    }

    pub fn build(&mut self) -> LuaResult<Lua> {
        let lua = unsafe { Lua::unsafe_new() };

        {
            let globals = lua.globals();
            let lua_reload: LuaTable = lua.load(LUA_RELOAD_SCRIPT).set_name("LuaReload").eval()?;
            globals.set("LuaReload", lua_reload)?;

            let path = self.script_path.canonicalize()?;
            let search_cpath = format!("{}/?.so", path.to_string_lossy());
            let search_path1 = format!("{}/?.lua", path.to_string_lossy());
            let search_path2 = format!("{}/?/init.lua", path.to_string_lossy());

            lua.load(format!(
                r#"
                    -- luajit, lua51, lua52 不支持 foo/init.lua 的 search_path
                    package.cpath = package.cpath .. ";{};";
                    package.path = package.path .. ";{};{};";
                    -- 初始化 LuaReload
                    LuaReload.SetHotReloadPath("{}")
                    LuaReload.SetPrintInfoLogs({});
                    LuaReload.SetPrintDebugLogs({});
                    LuaReload.Inject();
                "#,
                search_cpath,
                search_path1,
                search_path2,
                path.to_string_lossy(),
                self.lua_reload_info_log,
                self.lua_reload_debug_log,
            ))
            .exec()?;
        }

        if let Some(ref script) = self.init_script {
            lua.load(script).exec()?;
        }

        Ok(lua)
    }
}

pub fn lua_reload(lua: &Lua, file: impl AsRef<str>) -> LuaResult<bool> {
    lua.load(format!("LuaReload.ReloadFile(\"{}\")", file.as_ref()))
        .eval::<bool>()
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {}
