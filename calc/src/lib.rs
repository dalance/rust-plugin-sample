use anyhow::Result;
use libloading::{Library, Symbol};
use std::path::Path;

pub trait Plugin {
    fn name(&self) -> String;
    fn operator(&self) -> String;
    fn calc(&self, a: u32, b: u32) -> u32;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<()> {
        let lib = Library::new(path)?;

        let load_plugin: Symbol<extern "C" fn() -> Box<dyn Plugin>> =
            unsafe { lib.get(b"load_plugin") }?;
        let plugin = load_plugin();

        self.plugins.push(plugin);
        self.libraries.push(lib);
        Ok(())
    }

    pub fn plugins(&self) -> &[Box<dyn Plugin>] {
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
