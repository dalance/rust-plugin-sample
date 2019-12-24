use calc::Plugin;
use std::alloc::System;

#[global_allocator]
static ALLOCATOR: System = System;

#[no_mangle]
pub extern "C" fn load_plugin() -> Box<dyn Plugin> {
    Box::new(PluginAdd)
}

pub struct PluginAdd;

impl Plugin for PluginAdd {
    fn name(&self) -> String {
        String::from("Add")
    }

    fn operator(&self) -> String {
        String::from("+")
    }

    fn calc(&self, a: u32, b: u32) -> u32 {
        a + b
    }
}
