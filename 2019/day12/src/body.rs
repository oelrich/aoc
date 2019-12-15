use super::vector::Vector;
use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Body {
  position: Vector,
  velocity: Vector,
}
impl Body {
  #[cfg(test)]
  pub fn load(position: Vector, velocity: Vector) -> Body {
    Body { position, velocity }
  }
  pub fn update_location(&self) -> Body {
    Body {
      position: self.position.add(&self.velocity),
      velocity: self.velocity.clone(),
    }
  }
  pub fn bytes(&self) -> [[[u8; 8]; 3]; 2] {
    [self.position.bytes(), self.velocity.bytes()]
  }
  pub fn energy(&self) -> i64 {
    self.position.abs_sum() * self.velocity.abs_sum()
  }
  fn gravity(&self, body: &Body) -> Vector {
    self.position.compare(&body.position)
  }
  pub fn update_velocity(&self, bodies: &HashSet<Body>) -> Body {
    let gravity_total = bodies.iter().map(|body| self.gravity(body)).sum();
    Body {
      position: self.position.clone(),
      velocity: self.velocity.add(&gravity_total),
    }
  }
  pub fn new(x: i64, y: i64, z: i64) -> Body {
    Body {
      position: Vector::new(x, y, z),
      velocity: Vector::zero(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn energy() {
    let body = Body::load(Vector::new(2, 1, -3), Vector::new(-3, -2, 1));
    assert_eq!(body.energy(), 36);
  }
  #[test]
  fn gravity() {
    let ganymede = Body::new(3, 0, 0);
    let callisto = Body::new(5, 0, 0);
    let gravity = ganymede.gravity(&callisto);
    assert_eq!(gravity, Vector::new(1, 0, 0));
  }
}
