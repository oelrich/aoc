use helpers::coordinates::Polar;

pub fn run() -> String {
  let asteroids = Asteroid::from_file("./day/10/input.map");
  let (asteroid, visible) = visibility(&asteroids).pop().unwrap();

  let mut visible_asteroids: Vec<(Asteroid, f64)> = can_see(&asteroid, &asteroids)
    .iter()
    .cloned()
    .filter(|a| *a != asteroid)
    .map(|a| {
      let p = Polar::new(a.0 as f64, a.1 as f64);
      (a, p.clockwise())
    })
    .collect();
  visible_asteroids.sort_by(|(_, d0), (_, d1)| d0.partial_cmp(d1).unwrap());
  //visible_asteroids.reverse();
  let count = 200;
  let (asteroid, clock) = visible_asteroids[count];
  format!(
    "From our vantage point we see number {}, the magnificient {} at {} o'clock. (not 32,9)",
    count,
    asteroid,
    12.0 + 12.0 * clock
  )
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
