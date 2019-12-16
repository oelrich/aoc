#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
  x: isize,
  y: isize,
}

impl Point {
  pub fn zero() -> Point {
    Point { x: 0, y: 0 }
  }
  pub fn y(y: isize) -> Point {
    Point { x: 0, y }
  }
  pub fn x(x: isize) -> Point {
    Point { x, y: 0 }
  }
  pub fn add(&self, point: Point) -> Point {
    Point {
      x: self.x + point.x,
      y: self.y + point.y,
    }
  }
}
