use super::body::Body;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct System {
  iteration: usize,
  bodies: HashSet<Body>,
}

impl System {
  #[cfg(test)]
  pub fn load(iteration: usize, bodies: Vec<Body>) -> System {
    System {
      iteration,
      bodies: bodies.iter().cloned().collect(),
    }
  }
  fn state_hash(&self) -> [[[[u8; 8]; 3]; 2]; 4] {
    let mut bodies = self.bodies.iter().map(|body| body.bytes());
    [
      bodies.next().unwrap(),
      bodies.next().unwrap(),
      bodies.next().unwrap(),
      bodies.next().unwrap(),
    ]
  }
  fn update_velocities(bodies: HashSet<Body>) -> HashSet<Body> {
    bodies
      .par_iter()
      .map(|body| body.update_velocity(&bodies))
      .collect()
  }
  fn update_locations(bodies: HashSet<Body>) -> HashSet<Body> {
    bodies
      .par_iter()
      .map(|body| body.update_location())
      .collect()
  }
  fn step(state: &System) -> System {
    System {
      iteration: state.iteration + 1,
      bodies: System::update_locations(System::update_velocities(state.bodies.clone())),
    }
  }
  pub fn run_until_repeat(&self) -> usize {
    let mut values = HashSet::new();
    let mut state = self.clone();
    while values.insert(state.state_hash()) {
      state = System::step(&state);
    }
    state.iteration
  }
  pub fn run(&self, iterations: usize) -> System {
    let mut current = self.clone();
    while current.iteration < iterations {
      current = System::step(&current);
    }
    current
  }
  pub fn total_energy(&self) -> i64 {
    self.bodies.iter().map(|body| body.energy()).sum()
  }
  pub fn new(bodies: Vec<Body>) -> System {
    System {
      iteration: 0,
      bodies: bodies.iter().cloned().collect(),
    }
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::vector::Vector;
  #[test]
  fn repeat_0() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let actual = system.run_until_repeat();
    assert_eq!(actual, 2772);
  }
  #[test]
  fn repeat_1() {
    let system = System::new(vec![
      Body::new(-8, -10, 0),
      Body::new(5, 5, 10),
      Body::new(2, -7, 3),
      Body::new(9, 8, -3),
    ]);
    let actual = system.run_until_repeat();
    assert_eq!(actual, 4_686_774_924);
  }
  #[test]
  fn step_0() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      0,
      vec![
        Body::load(Vector::new(-1, 0, 2), Vector::new(0, 0, 0)),
        Body::load(Vector::new(2, -10, -7), Vector::new(0, 0, 0)),
        Body::load(Vector::new(4, -8, 8), Vector::new(0, 0, 0)),
        Body::load(Vector::new(3, 5, -1), Vector::new(0, 0, 0)),
      ],
    );
    assert_eq!(system.run(0), expected);
  }
  #[test]
  fn step_1() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      1,
      vec![
        Body::load(Vector::new(2, -1, 1), Vector::new(3, -1, -1)),
        Body::load(Vector::new(3, -7, -4), Vector::new(1, 3, 3)),
        Body::load(Vector::new(1, -7, 5), Vector::new(-3, 1, -3)),
        Body::load(Vector::new(2, 2, 0), Vector::new(-1, -3, 1)),
      ],
    );
    assert_eq!(system.run(1), expected);
  }
  #[test]
  fn step_2() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      2,
      vec![
        Body::load(Vector::new(5, -3, -1), Vector::new(3, -2, -2)),
        Body::load(Vector::new(1, -2, 2), Vector::new(-2, 5, 6)),
        Body::load(Vector::new(1, -4, -1), Vector::new(0, 3, -6)),
        Body::load(Vector::new(1, -4, 2), Vector::new(-1, -6, 2)),
      ],
    );
    assert_eq!(system.run(2), expected);
  }
  #[test]
  fn step_3() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      3,
      vec![
        Body::load(Vector::new(5, -6, -1), Vector::new(0, -3, 0)),
        Body::load(Vector::new(0, 0, 6), Vector::new(-1, 2, 4)),
        Body::load(Vector::new(2, 1, -5), Vector::new(1, 5, -4)),
        Body::load(Vector::new(1, -8, 2), Vector::new(0, -4, 0)),
      ],
    );
    assert_eq!(system.run(3), expected);
  }
  #[test]
  fn step_4() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      4,
      vec![
        Body::load(Vector::new(2, -8, 0), Vector::new(-3, -2, 1)),
        Body::load(Vector::new(2, 1, 7), Vector::new(2, 1, 1)),
        Body::load(Vector::new(2, 3, -6), Vector::new(0, 2, -1)),
        Body::load(Vector::new(2, -9, 1), Vector::new(1, -1, -1)),
      ],
    );
    assert_eq!(system.run(4), expected);
  }
  #[test]
  fn step_5() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      5,
      vec![
        Body::load(Vector::new(-1, -9, 2), Vector::new(-3, -1, 2)),
        Body::load(Vector::new(4, 1, 5), Vector::new(2, 0, -2)),
        Body::load(Vector::new(2, 2, -4), Vector::new(0, -1, 2)),
        Body::load(Vector::new(3, -7, -1), Vector::new(1, 2, -2)),
      ],
    );
    assert_eq!(system.run(5), expected);
  }
  #[test]
  fn step_6() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      6,
      vec![
        Body::load(Vector::new(-1, -7, 3), Vector::new(0, 2, 1)),
        Body::load(Vector::new(3, 0, 0), Vector::new(-1, -1, -5)),
        Body::load(Vector::new(3, -2, 1), Vector::new(1, -4, 5)),
        Body::load(Vector::new(3, -4, -2), Vector::new(0, 3, -1)),
      ],
    );
    assert_eq!(system.run(6), expected);
  }
  #[test]
  fn step_7() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      7,
      vec![
        Body::load(Vector::new(2, -2, 1), Vector::new(3, 5, -2)),
        Body::load(Vector::new(1, -4, -4), Vector::new(-2, -4, -4)),
        Body::load(Vector::new(3, -7, 5), Vector::new(0, -5, 4)),
        Body::load(Vector::new(2, 0, 0), Vector::new(-1, 4, 2)),
      ],
    );
    assert_eq!(system.run(7), expected);
  }
  #[test]
  fn step_8() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      8,
      vec![
        Body::load(Vector::new(5, 2, -2), Vector::new(3, 4, -3)),
        Body::load(Vector::new(2, -7, -5), Vector::new(1, -3, -1)),
        Body::load(Vector::new(0, -9, 6), Vector::new(-3, -2, 1)),
        Body::load(Vector::new(1, 1, 3), Vector::new(-1, 1, 3)),
      ],
    );
    assert_eq!(system.run(8), expected);
  }
  #[test]
  fn step_9() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      9,
      vec![
        Body::load(Vector::new(5, 3, -4), Vector::new(0, 1, -2)),
        Body::load(Vector::new(2, -9, -3), Vector::new(0, -2, 2)),
        Body::load(Vector::new(0, -8, 4), Vector::new(0, 1, -2)),
        Body::load(Vector::new(1, 1, 5), Vector::new(0, 0, 2)),
      ],
    );
    assert_eq!(system.run(9), expected);
  }
  #[test]
  fn step_10() {
    let system = System::new(vec![
      Body::new(-1, 0, 2),
      Body::new(2, -10, -7),
      Body::new(4, -8, 8),
      Body::new(3, 5, -1),
    ]);
    let expected = System::load(
      10,
      vec![
        Body::load(Vector::new(2, 1, -3), Vector::new(-3, -2, 1)),
        Body::load(Vector::new(1, -8, 0), Vector::new(-1, 1, 3)),
        Body::load(Vector::new(3, -6, 1), Vector::new(3, 2, -3)),
        Body::load(Vector::new(2, 0, 4), Vector::new(1, -1, -1)),
      ],
    );
    let actual = system.run(10);
    assert_eq!(actual, expected);
    assert_eq!(actual.total_energy(), 179);
  }
}
