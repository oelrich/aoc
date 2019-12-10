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
  a: Point,
}

impl Line {
  pub fn from(a: Point, b: Point) -> Line {
    let x_delta = b.x - a.x;
    let y_delta = b.y - a.y;
    Line {
      x_delta,
      y_delta,
      a: a.clone(),
    }
  }
  pub fn has_point(&self, p: &Point) -> bool {
    let px = self.y_delta * (p.x - self.a.x);
    let py = self.x_delta * (p.y - self.a.y);
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
  pub fn dist(&self, a: &Asteroid) -> f64 {
    let x_delta = (a.0 - self.0) as f64;
    let y_delta = (a.1 - self.1) as f64;
    (x_delta.powi(2) + y_delta.powi(2)).sqrt()
  }
}
use std::collections::HashMap;

fn take_key_value(hm: &mut HashMap<Asteroid, isize>) -> Option<(Asteroid, isize)> {
  let entry = hm.iter().last().map(|(a, u)| (*a, *u));
  if let Some((a, u)) = entry {
    hm.remove(&a);
    return Some((a, u));
  }
  None
}

fn hidden(source: &Asteroid, target: &Asteroid, obstacles: &Vec<Asteroid>) -> bool {
  let dist_to_target = source.dist(&target);
  let line = Line::from(source.point(), target.point());

  obstacles
    .iter()
    .any(|o| line.has_point(&o.point()) && source.dist(o) < dist_to_target)
}

fn can_see(asteroid: &Asteroid, space: &Vec<Asteroid>) -> Vec<Asteroid> {
  let mut available: Vec<Asteroid> = space.iter().cloned().collect();
  let mut visible: Vec<Asteroid> = Vec::new();
  while let Some(target) = available.pop() {
    if !hidden(asteroid, &target, &visible) {
      visible.push(target);
    }
  }
  visible
}
fn visibility(map: &Vec<Asteroid>) -> Vec<(Asteroid, isize)> {
  let mut start: HashMap<Asteroid, isize> = map.iter().cloned().map(|a| (a, 0)).collect();
  let mut done: Vec<(Asteroid, isize)> = Vec::new();
  while let Some((asteroid, count)) = take_key_value(&mut start) {
    let visible = can_see(&asteroid, &start.keys().map(|a| *a).collect());
    let count_update = count + visible.len() as isize;
    visible.iter().for_each(|a| {
      let count = start.get_mut(a).unwrap();
      *count += 1;
    });
    done.push((asteroid, count_update));
  }
  done.sort_by_key(|(_a, c)| *c);
  done
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
  fn small_1() {
    let map = Asteroid::from_lines(vec![".#..#", ".....", "#####", "....#", "...##"]);
    let expected = Asteroid(3, 4);
    let actual = visibility(&map).pop().map(|(a, _u)| a).unwrap();
    assert_eq!(actual, expected);
  }
}
