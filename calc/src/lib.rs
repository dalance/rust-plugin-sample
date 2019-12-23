pub trait Plugin {
    fn name(&self) -> String;
    fn operator(&self) -> String;
    fn calc(&self, a: u32, b: u32) -> u32;
}
