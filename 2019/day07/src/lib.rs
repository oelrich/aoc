pub mod a;
pub mod b;
use helpers::Solvers;
pub fn solve() -> Solvers {
  Solvers::new(a::run, b::run)
}
