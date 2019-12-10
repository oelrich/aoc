pub fn run() -> String {
  let asteroids = Asteroid::from_file("../day/10/input.map");
  let _best_visibility = visibility(&asteroids);
  "Aaaaah!".to_owned()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
  x: isize,
  y: isize,
}
impl Point {
  pub fn new(x: isize, y: isize) -> Point {
    Point { x, y }
  }
}
pub struct Line {
  x_delta: isize,
  y_delta: isize,
  low: Point,
  high: Point,
  o: Point,
}

impl Line {
  pub fn from(a: Point, b: Point) -> Line {
    let x_delta = b.x - a.x;
    let y_delta = b.y - a.y;
    Line {
      x_delta,
      y_delta,
      low: Point {
        x: a.x.min(b.x),
        y: a.y.min(b.y),
      },
      high: Point {
        x: a.x.max(b.x),
        y: a.y.max(b.y),
      },
      o: a,
    }
  }
  pub fn segment_has_point(&self, p: &Point) -> bool {
    let px = self.y_delta * (p.x - self.o.x);
    let py = self.x_delta * (p.y - self.o.y);
    px == py && p.y > self.low.y && p.y < self.high.y && p.x > self.low.x && p.x < self.high.x
  }
  pub fn has_point(&self, p: &Point) -> bool {
    let px = self.y_delta * (p.x - self.o.x);
    let py = self.x_delta * (p.y - self.o.y);
    px == py
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Asteroid(isize, isize);

impl Asteroid {
  pub fn point(&self) -> Point {
    Point {
      x: self.0 as isize,
      y: self.1 as isize,
    }
  }
  fn from_char(c: char, x: isize, y: isize) -> Option<Asteroid> {
    match c {
      '.' => None,
      '#' => Some(Asteroid(x, y)),
      _ => panic!("invalid space object?!"),
    }
  }
  fn from_string(row: &str, y: isize) -> Vec<Asteroid> {
    row
      .char_indices()
      .filter_map(|(x, c)| Asteroid::from_char(c, x as isize, y))
      .collect()
  }
  #[cfg(test)]
  pub fn from_lines(data: Vec<&str>) -> Vec<Asteroid> {
    data
      .iter()
      .enumerate()
      .flat_map(|(y, row)| Asteroid::from_string(&row, y as isize))
      .collect()
  }
  pub fn from_file(file_name: &str) -> Vec<Asteroid> {
    helpers::loader::read_as_iter(file_name)
      .enumerate()
      .flat_map(|(y, row)| Asteroid::from_string(&row, y as isize))
      .collect()
  }
}

fn hidden(source: &Asteroid, target: &Asteroid, obstacles: &[Asteroid]) -> bool {
  let line = Line::from(source.point(), target.point());
  for obstacle in obstacles {
    if line.segment_has_point(&obstacle.point()) {
      return true;
    }
  }
  false
}

fn can_see(asteroid: &Asteroid, space: &[Asteroid]) -> Vec<Asteroid> {
  let mut visible: Vec<Asteroid> = Vec::new();
  for target in space.iter().clone() {
    if !hidden(asteroid, target, &space) {
      visible.push(target.clone());
    }
  }
  visible
}

fn visibility(map: &[Asteroid]) -> Vec<(Asteroid, isize)> {
  let mut done: Vec<(Asteroid, isize)> = Vec::new();
  for asteroid in map.iter().clone() {
    let visible = can_see(asteroid, map);
    done.push((*asteroid, visible.len() as isize));
  }
  done.sort_by_key(|(_a, c)| *c);
  done
    .iter()
    .map(|(a, c)| (*a, c - 1))
    .collect::<Vec<(Asteroid, isize)>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn line_1() {
    let a = Point::new(0, 0);
    let b = Point::new(7, 7);
    let l = Line::from(a, b);
    let c = Point::new(134, 134);
    assert!(l.has_point(&c));
  }
  #[test]
  fn line_2() {
    let a = Point::new(2, 3);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(1 + 3 * 1, 1 + 3 * 2);
    assert!(l.has_point(&c));
  }
  #[test]
  fn line_3() {
    let a = Point::new(2, 2);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(-2, -2);
    assert!(l.has_point(&c));
  }
  #[test]
  fn line_4() {
    let a = Point::new(2, 2);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(2, -2);
    assert!(!l.has_point(&c));
  }
  #[test]
  fn segment_1() {
    let a = Point::new(0, 0);
    let b = Point::new(7, 7);
    let l = Line::from(a, b);
    let c = Point::new(134, 134);
    assert!(!l.segment_has_point(&c));
  }
  #[test]
  fn segment_1b() {
    let a = Point::new(0, 0);
    let b = Point::new(134, 134);
    let l = Line::from(a, b);
    let c = Point::new(7, 7);
    assert!(l.segment_has_point(&c));
  }
  #[test]
  fn segment_1c() {
    let a = Point::new(0, 0);
    let b = Point::new(134, 134);
    let l = Line::from(a, b);
    let c = Point::new(-7, -7);
    assert!(!l.segment_has_point(&c));
  }
  #[test]
  fn segment_2() {
    let a = Point::new(1, 1);
    let b = Point::new(1 + 3 * 1, 1 + 3 * 2);
    let l = Line::from(a, b);
    let c = Point::new(2, 3);
    assert!(l.segment_has_point(&c));
  }
  #[test]
  fn segment_3() {
    let a = Point::new(2, 2);
    let b = Point::new(-2, -2);
    let l = Line::from(a, b);
    let c = Point::new(1, 1);
    assert!(l.segment_has_point(&c));
  }
  #[test]
  fn segment_4() {
    let a = Point::new(2, 2);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(2, -2);
    assert!(!l.segment_has_point(&c));
  }
  #[test]
  fn asteroid_line_1() {
    let actual = Asteroid::from_string(".#..#", 0);
    let expected = vec![Asteroid(1, 0), Asteroid(4, 0)];
    assert_eq!(actual, expected);
  }
  #[test]
  fn asteroid_line_2() {
    let actual = Asteroid::from_lines(vec![".#..#", ".....", "#####", "....#", "...##"]);
    let expected = vec![
      Asteroid(1, 0),
      Asteroid(4, 0),
      Asteroid(0, 2),
      Asteroid(1, 2),
      Asteroid(2, 2),
      Asteroid(3, 2),
      Asteroid(4, 2),
      Asteroid(4, 3),
      Asteroid(3, 4),
      Asteroid(4, 4),
    ];
    assert_eq!(actual, expected);
  }
  #[test]
  fn small_1() {
    let map = Asteroid::from_lines(vec![".#..#", ".....", "#####", "....#", "...##"]);
    let expected = Asteroid(3, 4);
    let actual = visibility(&map).pop().map(|(a, _u)| a).unwrap();
    assert_eq!(actual, expected);
  }
  #[test]
  fn small_2() {
    let map = Asteroid::from_lines(vec![".#..#", ".....", "#####", "....#", "...##"]);
    let mut actual = visibility(&map);
    actual.sort();
    let mut expected = vec![
      (Asteroid(1, 0), 7),
      (Asteroid(4, 0), 7),
      (Asteroid(0, 2), 6),
      (Asteroid(1, 2), 7),
      (Asteroid(2, 2), 7),
      (Asteroid(3, 2), 7),
      (Asteroid(4, 2), 5),
      (Asteroid(4, 3), 7),
      (Asteroid(3, 4), 8),
      (Asteroid(4, 4), 7),
    ];
    expected.sort();
    assert_eq!(actual, expected);
  }
  #[test]
  fn small_3() {
    let map = Asteroid::from_lines(vec![".#.", ".#.", ".#."]);
    assert_eq!(map, vec![Asteroid(1, 0), Asteroid(1, 1), Asteroid(1, 2)]);
    let mut actual = visibility(&map);
    actual.sort();
    assert_eq!(can_see(&map[0], &map), vec![Asteroid(1, 0), Asteroid(1, 1)]);
    assert_eq!(
      can_see(&map[1], &map),
      vec![Asteroid(1, 0), Asteroid(1, 1), Asteroid(1, 2)]
    );
    assert_eq!(can_see(&map[2], &map), vec![Asteroid(1, 1), Asteroid(1, 2)]);
    let mut expected = vec![
      (Asteroid(1, 0), 1),
      (Asteroid(1, 1), 2),
      (Asteroid(1, 2), 1),
    ];
    expected.sort();
    assert_eq!(actual, expected);
  }
  #[test]
  fn can_see_1() {
    let mut map = Asteroid::from_lines(vec![".#.", ".#.", ".#.", ".#."]);
    map.sort();
    let asteroid = Asteroid(1, 1);
    let mut actual = can_see(&asteroid, &map);
    actual.sort();
    assert_ne!(actual, map);
    let mut expected = vec![Asteroid(1, 0), Asteroid(1, 1), Asteroid(1, 2)];
    expected.sort();
    assert_eq!(actual, expected);
  }
  #[test]
  fn can_see_2() {
    let map = Asteroid::from_lines(vec!["..#..", "..#..", ".##..", "#.#..", "..#.."]);
    let asteroid = Asteroid(2, 1);
    let mut actual = can_see(&asteroid, &map);
    actual.sort();
    let mut expected = vec![
      Asteroid(1, 2),
      Asteroid(2, 0),
      Asteroid(2, 1),
      Asteroid(2, 2),
    ];
    expected.sort();
    assert_eq!(actual, expected);
  }
  #[test]
  fn can_see_3() {
    let map = vec![Asteroid(0, 0), Asteroid(1, 0), Asteroid(2, 0)];
    let asteroid = Asteroid(0, 0);
    let mut actual = can_see(&asteroid, &map);
    actual.sort();
    let mut expected = vec![Asteroid(0, 0), Asteroid(1, 0)];
    expected.sort();
    assert_eq!(actual, expected);
  }
  #[test]
  fn hidden_1() {
    let source = Asteroid(1, 1);
    let target = Asteroid(3, 3);
    let obstacles = vec![];
    assert!(!hidden(&source, &target, &obstacles));
  }
  #[test]
  fn hidden_2() {
    let source = Asteroid(1, 1);
    let target = Asteroid(3, 3);
    let obstacles = vec![Asteroid(2, 2)];
    assert!(hidden(&source, &target, &obstacles));
  }
  #[test]
  fn hidden_3() {
    let source = Asteroid(1, 1);
    let target = Asteroid(3, 7);
    let obstacles = vec![Asteroid(2, 2), Asteroid(2, 4)];
    assert!(hidden(&source, &target, &obstacles));
  }
}
