use helpers::Solvers;
mod body;
mod system;
mod vector;
use body::Body;
use system::System;
pub fn solve() -> Solvers {
  Solvers::new(a::run, b::run)
}
mod a {
  use super::{Body, System};
  pub fn run() -> String {
    /*
    <x=4, y=1, z=1>
    <x=11, y=-18, z=-1>
    <x=-2, y=-10, z=-4>
    <x=-7, y=-2, z=14>
    */
    let system = System::new(vec![
      Body::new(4, 1, 1),
      Body::new(11, -18, -1),
      Body::new(-2, -10, -4),
      Body::new(-7, -2, 14),
    ]);
    let result = system.run(1000);
    format!("{}", result.total_energy())
  }
}
mod b {
  use super::{Body, System};
  pub fn run() -> String {
    /*
    <x=4, y=1, z=1>
    <x=11, y=-18, z=-1>
    <x=-2, y=-10, z=-4>
    <x=-7, y=-2, z=14>
    */
    let system = System::new(vec![
      Body::new(4, 1, 1),
      Body::new(11, -18, -1),
      Body::new(-2, -10, -4),
      Body::new(-7, -2, 14),
    ]);
    let result = system.run_until_repeat();
    format!("{}", result)
  }
}
