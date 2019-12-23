use anyhow::Result;
use calc::Plugin;
use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }

    fn load(&mut self, path: &Path) -> Result<()> {
        let lib = Library::new(path)?;

        let load_plugin: Symbol<extern "C" fn() -> *mut dyn Plugin> =
            unsafe { lib.get(b"load_plugin") }?;
        let plugin = unsafe { Box::from_raw(load_plugin()) };

        //let load_plugin: Symbol<extern "C" fn() -> Box<dyn Plugin>> =
        //    unsafe { lib.get(b"load_plugin") }?;
        //let plugin = load_plugin();

        self.plugins.push(plugin);
        self.libraries.push(lib);
        Ok(())
    }

    fn plugins(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        // Plugin drop must be called before Library drop.
        self.plugins.clear();
        self.libraries.clear();
    }
}

// SEGV
//pub struct PluginManager {
//    libraries: Vec<Library>,
//    plugins: Vec<Box<dyn Plugin>>,
//}

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
