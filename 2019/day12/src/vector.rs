#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vector([i64; 3]);
impl std::iter::Sum for Vector {
  fn sum<I: Iterator<Item = Vector>>(iter: I) -> Self {
    iter.fold(Vector::zero(), |total, vector| total.add(&vector))
  }
}
impl Vector {
  pub fn zero() -> Vector {
    Vector([0, 0, 0])
  }
  pub fn bytes(&self) -> [[u8; 8]; 3] {
    unsafe {
      [
        std::mem::transmute::<i64, [u8; 8]>(self.0[0]),
        std::mem::transmute::<i64, [u8; 8]>(self.0[1]),
        std::mem::transmute::<i64, [u8; 8]>(self.0[2]),
      ]
    }
  }
  pub fn new(x: i64, y: i64, z: i64) -> Vector {
    Vector([x, y, z])
  }
  pub fn abs_sum(&self) -> i64 {
    self.0[0].abs() + self.0[1].abs() + self.0[2].abs()
  }
  fn cmp(a: i64, b: i64) -> i64 {
    if a == b {
      0
    } else if a > b {
      -1
    } else {
      1
    }
  }
  pub fn compare(&self, other: &Vector) -> Vector {
    Vector([
      Vector::cmp(self.0[0], other.0[0]),
      Vector::cmp(self.0[1], other.0[1]),
      Vector::cmp(self.0[2], other.0[2]),
    ])
  }
  pub fn add(&self, other: &Vector) -> Vector {
    Vector([
      self.0[0] + other.0[0],
      self.0[1] + other.0[1],
      self.0[2] + other.0[2],
    ])
  }
}
