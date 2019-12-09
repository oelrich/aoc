pub mod loader;
pub mod mathy;
pub struct Solvers(fn() -> String, fn() -> String);
impl Solvers {
    pub fn new(a: fn() -> String, b: fn() -> String) -> Solvers {
        Solvers(a, b)
    }
}

impl Solvers {
    pub fn a(&self) -> String {
        self.0()
    }
    pub fn b(&self) -> String {
        self.1()
    }
}
