use std::f64::consts::PI;
#[derive(Debug, PartialEq)]
pub struct Polar {
  length: f64,
  radians: f64,
}
enum Quadrant {
  PlusPlus,
  MinusPlus,
  MinusMinus,
  PlusMinus,
}
impl Quadrant {
  fn new(x: isize, y: isize) -> Quadrant {
    if x >= 0 {
      if y >= 0 {
        Quadrant::PlusPlus
      } else {
        Quadrant::PlusMinus
      }
    } else if y >= 0 {
      Quadrant::MinusPlus
    } else {
      Quadrant::MinusMinus
    }
  }
}

impl Polar {
  pub fn new(r: f64, a: f64) -> Polar {
    Polar {
      length: r,
      radians: a,
    }
  }
  pub fn clockwise(&self) -> f64 {
    let twelve = self.radians - (PI / 2.0);
    let reverse = (2.0 * PI - twelve) % (2.0 * PI);
    reverse / (2.0 * PI)
  }
  fn arc_tan_it(x: isize, y: isize) -> f64 {
    let result = if y == 0 {
      0.0f64.atan()
    } else if x == 0 {
      std::f64::INFINITY.atan()
    } else {
      (y as f64 / x as f64).abs().atan()
    };
    match Quadrant::new(x, y) {
      Quadrant::PlusPlus => result,
      Quadrant::MinusPlus => PI - result,
      Quadrant::MinusMinus => PI + result,
      Quadrant::PlusMinus => 2.0 * PI - result,
    }
  }
  pub fn of_cartesian(x: isize, y: isize) -> Polar {
    let radians = Polar::arc_tan_it(x, y);
    Polar {
      length: ((x.pow(2) + y.pow(2)) as f64).sqrt(),
      radians,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn polar_1() {
    assert_eq!(
      Polar::of_cartesian(3, 4),
      Polar::new(5.0, 0.9272952180016122)
    )
  }
  #[test]
  fn polar_2() {
    assert_eq!(Polar::of_cartesian(0, 0), Polar::new(0.0, 0.0))
  }
  #[test]
  fn polar_3() {
    assert_eq!(Polar::of_cartesian(1, 0), Polar::new(1.0, 0.0 * PI / 4.0))
  }
  #[test]
  fn polar_4() {
    assert_eq!(
      Polar::of_cartesian(1, 1),
      Polar::new(2.0f64.sqrt(), 1.0 * PI / 4.0)
    )
  }
  #[test]
  fn polar_5() {
    assert_eq!(Polar::of_cartesian(0, 1), Polar::new(1.0, 2.0 * PI / 4.0))
  }
  #[test]
  fn polar_6() {
    assert_eq!(
      Polar::of_cartesian(-1, 1),
      Polar::new(2.0f64.sqrt(), 3.0 * PI / 4.0)
    )
  }
  #[test]
  fn polar_7() {
    assert_eq!(Polar::of_cartesian(-1, 0), Polar::new(1.0, 4.0 * PI / 4.0))
  }
  #[test]
  fn polar_8() {
    assert_eq!(
      Polar::of_cartesian(-1, -1),
      Polar::new(2.0f64.sqrt(), 5.0 * PI / 4.0)
    )
  }
  #[test]
  fn polar_9() {
    assert_eq!(Polar::of_cartesian(0, -1), Polar::new(1.0, 6.0 * PI / 4.0))
  }
  #[test]
  fn polar_10() {
    assert_eq!(
      Polar::of_cartesian(1, -1),
      Polar::new(2.0f64.sqrt(), 7.0 * PI / 4.0)
    )
  }
  #[test]
  fn clock_12() {
    assert_eq!(Polar::of_cartesian(0, 1).clockwise(), 0.0)
  }
  #[test]
  fn clock_1330() {
    assert_eq!(Polar::of_cartesian(1, 1).clockwise(), 0.1250)
  }
  #[test]
  fn clock_15() {
    assert_eq!(Polar::of_cartesian(1, 0).clockwise(), 0.25)
  }
  #[test]
  fn clock_1630() {
    assert_eq!(Polar::of_cartesian(1, -1).clockwise(), 0.375)
  }
  #[test]
  fn clock_18() {
    assert_eq!(Polar::of_cartesian(0, -1).clockwise(), 0.5)
  }
  #[test]
  fn clock_1930() {
    assert_eq!(Polar::of_cartesian(-1, -1).clockwise(), 0.625)
  }
  #[test]
  fn clock_21() {
    assert_eq!(Polar::of_cartesian(-1, 0).clockwise(), 0.75)
  }
  #[test]
  fn clock_2230() {
    assert_eq!(Polar::of_cartesian(-1, 1).clockwise(), 0.875)
  }
  #[test]
  fn clock_21b() {
    assert_eq!(Polar::of_cartesian(-21, 0).clockwise(), 0.75)
  }
}
