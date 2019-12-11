pub fn run() -> String {
  let asteroids = Asteroid::from_file("./day/10/input.map");
  let (asteroid, visible) = visibility(&asteroids).pop().unwrap();
  format!("From {} we see {} asteroids.", asteroid, visible)
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
  pub fn distance(&self, p: &Point) -> f64 {
    (((self.x - p.x) as f64).powi(2) + ((self.y - p.y) as f64).powi(2)).sqrt()
  }
  pub fn sub(&self, p: &Point) -> Point {
    Point {
      x: self.x - p.x,
      y: self.y - p.y,
    }
  }
}
pub struct Line {
  x_delta: isize,
  y_delta: isize,
  o: Point,
  t: Point,
}

impl Line {
  pub fn from(a: Point, b: Point) -> Line {
    let x_delta = b.x - a.x;
    let y_delta = b.y - a.y;
    Line {
      x_delta,
      y_delta,
      o: a,
      t: b,
    }
  }
  fn len(&self) -> f64 {
    self.o.distance(&self.t)
  }

  fn closer_to_o_than_t(&self, p: &Point) -> bool {
    let dist_origin_target = self.len();
    let dist_origin_point = self.o.distance(p);
    if dist_origin_point <= dist_origin_target {
      return true;
    }
    let mod_p = p.sub(&self.t);
    let dist_origin_mod_point = self.o.distance(&mod_p);
    if dist_origin_point <= dist_origin_mod_point {
      return true;
    }
    false
  }

  fn between(&self, p: &Point) -> bool {
    let fp_error_margin: f64 = 0.01;
    let dop = self.o.distance(p);
    let dpt = p.distance(&self.t);
    let err = self.len() - dop - dpt;
    err.abs() < fp_error_margin
  }

  pub fn blocking(&self, p: &Point) -> bool {
    self.is_on_line(p) && self.between(p)
  }
  pub fn is_on_line(&self, p: &Point) -> bool {
    let px = self.y_delta * (p.x - self.o.x);
    let py = self.x_delta * (p.y - self.o.y);
    px == py
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Asteroid(isize, isize);
impl std::fmt::Display for Asteroid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "#({},{})", self.0, self.1)
  }
}
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
  if source == target {
    return false;
  }
  let line = Line::from(source.point(), target.point());
  for obstacle in obstacles
    .iter()
    .filter(|obstacle| source != *obstacle && target != *obstacle)
  {
    if line.blocking(&obstacle.point()) {
      return true;
    }
  }
  false
}

fn can_see(asteroid: &Asteroid, space: &[Asteroid]) -> Vec<Asteroid> {
  let mut visible: Vec<Asteroid> = Vec::new();
  space.iter().cloned().for_each(|target| {
    if !hidden(asteroid, &target, space) {
      visible.push(target)
    }
  });

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
    assert!(l.is_on_line(&c));
  }
  #[test]
  fn line_2() {
    let a = Point::new(2, 3);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(1 + 3 * 1, 1 + 3 * 2);
    assert!(l.is_on_line(&c));
  }
  #[test]
  fn line_3() {
    let a = Point::new(2, 2);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(-2, -2);
    assert!(l.is_on_line(&c));
  }
  #[test]
  fn line_4() {
    let a = Point::new(2, 2);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(2, -2);
    assert!(!l.is_on_line(&c));
  }
  #[test]
  fn line_5() {
    let a = Point::new(1, 1);
    let b = Point::new(0, 1);
    let l = Line::from(a, b);
    let c = Point::new(2, 1);
    assert!(l.is_on_line(&c));
  }
  #[test]
  fn line_6() {
    let a = Point::new(1, 1);
    let b = Point::new(1, 0);
    let l = Line::from(a, b);
    let c = Point::new(1, 2);
    assert!(l.is_on_line(&c));
  }
  #[test]
  fn segment_1() {
    let a = Point::new(0, 0);
    let b = Point::new(7, 7);
    let l = Line::from(a, b);
    let c = Point::new(15, 15);
    assert!(l.is_on_line(&c));
    assert!(!l.blocking(&c));
  }
  #[test]
  fn segment_1b() {
    let a = Point::new(0, 0);
    let b = Point::new(134, 134);
    let l = Line::from(a, b);
    let c = Point::new(7, 7);
    assert!(l.blocking(&c));
  }
  #[test]
  fn segment_1c() {
    let a = Point::new(0, 0);
    let b = Point::new(134, 134);
    let l = Line::from(a, b);
    let c = Point::new(-7, -7);
    assert!(!l.blocking(&c));
  }
  #[test]
  fn segment_1d() {
    let a = Point::new(0, 0);
    let b = Point::new(134, 134);
    let l = Line::from(a, b);
    let c = Point::new(137, 137);
    assert!(!l.blocking(&c));
  }
  #[test]
  fn segment_1e() {
    let a = Point::new(0, 0);
    let b = Point::new(7, 7);
    let l = Line::from(a, b);
    let c = Point::new(-134, -134);
    assert!(l.is_on_line(&c));
    assert!(!l.blocking(&c));
  }
  #[test]
  fn segment_2() {
    let a = Point::new(1, 1);
    let b = Point::new(1 + 3 * 1, 1 + 3 * 2);
    let l = Line::from(a, b);
    let c = Point::new(2, 3);
    assert!(l.blocking(&c));
  }
  #[test]
  fn segment_3() {
    let a = Point::new(2, 2);
    let b = Point::new(-2, -2);
    let l = Line::from(a, b);
    let c = Point::new(1, 1);
    assert!(l.blocking(&c));
  }
  #[test]
  fn segment_4() {
    let a = Point::new(2, 2);
    let b = Point::new(1, 1);
    let l = Line::from(a, b);
    let c = Point::new(2, -2);
    assert!(!l.blocking(&c));
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
  fn can_see_1a() {
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
  fn can_see_1b() {
    let mut map = Asteroid::from_lines(vec!["...", "###", "..."]);
    map.sort();
    let asteroid = Asteroid(1, 1);
    let mut actual = can_see(&asteroid, &map);
    actual.sort();
    let mut expected = vec![Asteroid(0, 1), Asteroid(1, 1), Asteroid(2, 1)];
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
  #[test]
  fn hidden_4a() {
    let mut map = Asteroid::from_lines(vec!["...", "###", "..."]);
    map.sort();
    let asteroid = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    assert!(!hidden(&asteroid, &target, &map));
  }
  #[test]
  fn hidden_4b() {
    let source = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    let obstacles = vec![Asteroid(0, 1), Asteroid(1, 1), Asteroid(2, 1)];
    assert!(!hidden(&source, &target, &obstacles));
  }
  #[test]
  fn hidden_4b_a() {
    let source = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    let obstacles = vec![Asteroid(0, 1)];
    assert!(!hidden(&source, &target, &obstacles));
  }
  #[test]
  fn hidden_4c() {
    let source = Asteroid(1, 1);
    let target = Asteroid(3, 1);
    let obstacles = vec![
      Asteroid(0, 1),
      Asteroid(1, 1),
      Asteroid(2, 1),
      Asteroid(3, 1),
    ];
    assert!(hidden(&source, &target, &obstacles));
  }
  #[test]
  fn hidden_4d() {
    let map = Asteroid::from_lines(vec!["...", "###", "..."]);
    let obstacles = vec![Asteroid(0, 1), Asteroid(1, 1), Asteroid(2, 1)];
    assert_eq!(map, obstacles);
  }
  #[test]
  fn hidden_4e() {
    let asteroid = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    assert!(!hidden(&asteroid, &target, &[Asteroid(0, 1)]));
  }
  #[test]
  fn hidden_4f() {
    let asteroid = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    assert!(!hidden(&asteroid, &target, &[Asteroid(0, 0)]));
  }
  #[test]
  fn hidden_4g() {
    let asteroid = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    assert!(!hidden(&asteroid, &target, &[Asteroid(1, 0)]));
  }
  #[test]
  fn hidden_4h() {
    let asteroid = Asteroid(1, 1);
    let target = Asteroid(2, 1);
    assert!(!hidden(&asteroid, &target, &[Asteroid(2, 0)]));
  }
  #[test]
  fn closer_1() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(2, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(3, 1);
    assert!(!line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_2() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(2, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(0, 1);
    assert!(line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_3() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(2, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(-5, 1);
    assert!(line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_4() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(4, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(3, 1);
    assert!(line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_5() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(42, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(3, 1);
    assert!(line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_6() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(42, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(80, 1);
    assert!(!line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_7() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(42, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(-80, 1);
    assert!(line.closer_to_o_than_t(&point));
  }
  #[test]
  fn closer_8() {
    let asteroid = Point::new(1, 1);
    let target = Point::new(42, 1);
    let line = Line::from(asteroid, target);
    let point = Point::new(-41, 1);
    assert!(line.closer_to_o_than_t(&point));
  }
}
