use anyhow::Result;
use calc::PluginManager;
use std::alloc::System;
use std::path::PathBuf;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() -> Result<()> {
    let mut pm = PluginManager::new();
    pm.load(&PathBuf::from("./target/debug/libplugin_add.so"))?;
    pm.load(&PathBuf::from("./target/debug/libplugin_mul.so"))?;

    for plugin in pm.plugins() {
        println!("Plugin: {}", plugin.name());
        println!("Calc: 1 {} 2 = {}", plugin.operator(), plugin.calc(1, 2));
    }

    Ok(())
}
