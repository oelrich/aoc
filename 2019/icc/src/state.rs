#[derive(Debug, Clone, PartialEq)]
pub enum State {
  Output(usize, i64),
  Input,
  Running,
  Halted,
  Ready,
  Crashed,
}
