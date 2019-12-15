use helpers::Solvers;
use std::collections::HashMap;
pub fn solve() -> Solvers {
  Solvers::new(a::run, b::run)
}
mod a {
  use super::*;
  pub fn run() -> String {
    let program = helpers::loader::load_integer_row_list("./day/11/input.csv")[0].clone();
    let mut jhonny = Robot::new(&program);
    let mut panel = Panel::new();
    jhonny.run(&mut panel);

    format!("{}", panel.map.len())
  }
}
mod b {
  pub fn run() -> String {
    "bah".into()
  }
}

struct Panel {
  map: HashMap<(isize, isize), i64>,
}

impl Panel {
  pub fn new() -> Panel {
    Panel {
      map: HashMap::new(),
    }
  }
  pub fn look(&self, pos: (isize, isize)) -> i64 {
    if self.map.contains_key(&pos) {
      let value = self.map[&pos];
      return value;
    }
    0
  }
  pub fn paint_black(&mut self, pos: (isize, isize)) {
    self.map.insert(pos, 0);
  }
  pub fn paint_white(&mut self, pos: (isize, isize)) {
    self.map.insert(pos, 1);
  }
}

enum Direction {
  Up,
  Left,
  Right,
  Down,
}

impl Direction {
  pub fn turn(&self, direction: i64) -> Direction {
    match direction {
      0 => match self {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
      },
      1 => match self {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
      },
      dir => panic!("bad direction {}", dir),
    }
  }
}

struct Robot {
  position: (isize, isize),
  direction: Direction,
  brain: icc::Computer,
}
use icc::state::State;

impl Robot {
  pub fn new(program: &[i64]) -> Robot {
    Robot {
      position: (0, 0),
      direction: Direction::Up,
      brain: icc::Computer::load(5, program),
    }
  }
  fn advance(&mut self) {
    match self.direction {
      Direction::Up => self.position = (self.position.0, self.position.1 + 1),
      Direction::Right => self.position = (self.position.0 + 1, self.position.1),
      Direction::Down => self.position = (self.position.0, self.position.1 - 1),
      Direction::Left => self.position = (self.position.0 - 1, self.position.1),
    }
  }
  pub fn run(&mut self, panel: &mut Panel) {
    let mut paint = false;
    loop {
      match self.brain.run() {
        State::Crashed => panic!("{}", self.brain.error()),
        State::Halted => break,
        State::Ready => (),
        State::Running => (),
        State::Output(_id, value) => {
          if paint {
            match value {
              0 => panel.paint_black(self.position),
              1 => panel.paint_white(self.position),
              err => panic!("{} is invalid paint!", err),
            }
          } else {
            self.direction = self.direction.turn(value);
          };
          if !paint {
            self.advance();
            paint = true;
          } else {
            paint = false;
          }
        }
        State::Input => {
          let value = panel.look(self.position);
          if !self.brain.input_value(value) {
            panic!("Asked for but refused input")
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn goodish() {
    // 2008 för lågt
    assert_eq!(super::a::run(), "".to_owned());
  }
}
