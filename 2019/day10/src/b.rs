use helpers::coordinates::Polar;

pub fn run() -> String {
  let map = Asteroid::from_file("./day/10/input.map");

  let mut visible = visibility(&map);
  let actual = visible.pop().map(|(a, _u)| a).unwrap();
  let sorted = sort_clockwise_from(&actual, &map);
  let nth = |p: usize| sorted.iter().nth(p - 1).unwrap();
  let count = 200;
  let (asteroid, clock) = nth(count);
  format!(
    "From our vantage point we see number {}, the magnificent {} at {} o'clock. (not 32,9)",
    count,
    asteroid,
    12.0 + 12.0 * clock
  )
}

fn find_by_sweep(count: usize, asteroids: &Vec<Asteroid>) -> (Asteroid, f64) {
  assert!(!asteroids.is_empty());

  let (asteroid, _visible) = visibility(&asteroids).pop().unwrap();

  sort_clockwise_from(&asteroid, &asteroids)[count]
}
fn clock(origin: &Asteroid, target: &Asteroid) -> f64 {
  let p = Polar::of_cartesian(target.0 - origin.0, -(target.1 - origin.1));

  p.clockwise()
}
fn sort_clockwise_from(asteroid: &Asteroid, asteroids: &Vec<Asteroid>) -> Vec<(Asteroid, f64)> {
  let mut visible_asteroids: Vec<(Asteroid, f64)> = can_see(&asteroid, &asteroids)
    .iter()
    .cloned()
    .filter(|a| a != asteroid)
    .map(|a| {
      let clock = clock(asteroid, &a);
      (a, clock)
    })
    .collect();
  visible_asteroids.sort_by(|(_, d0), (_, d1)| d1.partial_cmp(d0).unwrap());
  visible_asteroids.reverse();
  visible_asteroids
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

  #[cfg(test)]
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
  fn clock_0() {
    assert_eq!(clock(&Asteroid(0, 0), &Asteroid(0, 1)), 0.5);
  }
  #[test]
  fn clock_1() {
    assert_eq!(clock(&Asteroid(2, 0), &Asteroid(14, 0)), 0.25);
  }
  #[test]
  fn clock_3() {
    assert_eq!(clock(&Asteroid(0, 0), &Asteroid(0, -1)), 0.0);
  }
  #[test]
  fn clock_4() {
    assert_eq!(clock(&Asteroid(1, 0), &Asteroid(-1, 0)), 0.750);
  }
  #[test]
  fn clock_5a() {
    assert_eq!(clock(&Asteroid(8, 3), &Asteroid(8, 1)), 0.0);
  }
  #[test]
  fn clock_5b() {
    assert_eq!(clock(&Asteroid(8, 3), &Asteroid(7, 0)), 0.9487918088252166);
  }

  #[test]
  fn small_1() {
    let map = Asteroid::from_lines(vec![
      ".#....#####...#..",
      "##...##.#####..##",
      "##...#...#.#####.",
      "..#.....#...###..",
      "..#.#.....#....##",
    ]);

    let expected = Asteroid(8, 3);
    let mut visible = visibility(&map);
    let actual = visible.pop().map(|(a, _u)| a).unwrap();
    assert_eq!(actual, expected);

    let mut sorted = sort_clockwise_from(&actual, &map);
    sorted.reverse();
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(8, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(9, 0));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(9, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(10, 0));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(9, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(11, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(12, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(11, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(15, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(12, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(13, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(14, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(15, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(12, 3));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(16, 4));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(15, 4));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(10, 4));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(4, 4));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(2, 4));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(2, 3));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(0, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(1, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(0, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(1, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(5, 2));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(1, 0));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(5, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(6, 1));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(6, 0));
    let (a, _c) = sorted.pop().unwrap();
    assert_eq!(a, Asteroid(7, 0));
    assert!(sorted.is_empty());
  }

  #[test]
  fn small_2() {
    let map = Asteroid::from_lines(vec![
      ".#....#####...#..",
      "##...##.#####..##",
      "##...#...#.#####.",
      "..#.....#...###..",
      "..#.#.....#....##",
    ]);

    let expected = Asteroid(8, 3);
    let mut visible = visibility(&map);
    let actual = visible.pop().map(|(a, _u)| a).unwrap();
    assert_eq!(actual, expected);

    let sorted = sort_clockwise_from(&actual, &map);

    let nth = |p: usize| sorted.iter().nth(p - 1).map(|(a, _)| *a).unwrap();
    assert_eq!(nth(1), Asteroid(8, 1));
    assert_eq!(nth(30), Asteroid(7, 0));
  }
  #[test]
  fn biggish_1() {
    let map = Asteroid::from_lines(vec![
      ".#..##.###...#######",
      "##.############..##.",
      ".#.######.########.#",
      ".###.#######.####.#.",
      "#####.##.#.##.###.##",
      "..#####..#.#########",
      "####################",
      "#.####....###.#.#.##",
      "##.#################",
      "#####.##.###..####..",
      "..######..##.#######",
      "####.##.####...##..#",
      ".#####..#.######.###",
      "##...#.##########...",
      "#.##########.#######",
      ".####.#.###.###.#.##",
      "....##.##.###..#####",
      ".#.#.###########.###",
      "#.#.#.#####.####.###",
      "###.##.####.##.#..##",
    ]);
    let expected = Asteroid(11, 13);
    let mut visible = visibility(&map);
    let (actual, can_see) = visible.pop().unwrap();
    assert_eq!(actual, expected);
    assert_eq!(can_see, 210);

    let sorted = sort_clockwise_from(&actual, &map);
    let nth = |p: usize| sorted.iter().nth(p - 1).map(|(a, _)| *a).unwrap();
    assert_eq!(nth(1), Asteroid(11, 12));
    assert_eq!(nth(100), Asteroid(10, 16));
    assert_eq!(nth(200), Asteroid(8, 2));
  }
  #[test]
  fn big_1() {
    let map = Asteroid::from_file("../day/10/input.map");

    let mut visible = visibility(&map);
    let actual = visible.pop().map(|(a, _u)| a).unwrap();
    let sorted = sort_clockwise_from(&actual, &map);
    let nth = |p: usize| sorted.iter().nth(p - 1).map(|(a, _)| *a).unwrap();
    let asteroid = nth(200);
    assert_eq!(asteroid, Asteroid(16, 23))
    // 1329 not it [199]
    // 1429 not it [200]?
    //let asteroids = Asteroid::from_file("../day/10/input.map");

    //visible_asteroids.reverse();
    //  let count = 200;
    //  let (asteroid, clock) = find_by_sweep(count, &asteroids);

    // assert_eq!((asteroid, clock), (Asteroid(0, 0), 0.0));
  }
}
