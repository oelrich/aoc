use helpers::Solvers;
pub fn solve() -> Solvers {
    Solvers::new(a::run, b::run)
}
mod a {
    pub fn run() -> String {
        "bah".into()
    }
}
mod b {
    pub fn run() -> String {
        "bah".into()
    }
}
