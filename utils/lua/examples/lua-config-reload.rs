pub use abyss_lua::mlua::{DebugEvent, Function, HookTriggers, Lua, Value};
pub use abyss_lua::*;
use notify::{event::ModifyKind, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let lua = LuaBuilder::new(Path::new("examples/plugins"))
        .with_lua_reload_info_log(false)
        .build()?;

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |result: Result<Event, _>| {
            let event = result.unwrap();

            if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
                tx.send(event).unwrap();
            }
        },
        notify::Config::default(),
    )?;

    watcher.watch(Path::new("examples/plugins"), RecursiveMode::Recursive)?;

    let table = lua.create_table()?;
    let id = "hello";
    table.set(id, 100)?;
    let f: Function = lua.load("require(\"demo\").hello").eval()?;
    f.call(table)?;

    let thread = lua.create_thread(
        lua.load(
            r#"
            function (path)
                while true do
                    LuaReload.ReloadFile(path);
                    path = coroutine.yield();
                end
            end
        "#,
        )
        .eval()?,
    )?;

    for event in rx.iter() {
        for path in event.paths.iter() {
            let path = path.to_str().unwrap();
            thread.resume(path)?;
            // let f: Function = lua.load("require(\"demo\").hello").eval()?;
            //
            // f.call(("hello", 100))?;
        }
    }

    Ok(())
}
