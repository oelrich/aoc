use super::error::Error;
#[derive(Debug, Clone)]
pub enum Mode {
  Direct,
  Stored,
  Relative,
}
impl Mode {
  pub fn dest_of_val(val: i64) -> Result<Mode, Error> {
    match val {
      0 => Ok(Mode::Direct),
      1 => Ok(Mode::Stored),
      2 => Ok(Mode::Relative),
      _ => Err(Error::Mode),
    }
  }
  pub fn src_of_val(val: i64) -> Result<Mode, Error> {
    match val {
      0 => Ok(Mode::Stored),
      1 => Ok(Mode::Direct),
      2 => Ok(Mode::Relative),
      _ => Err(Error::Mode),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Value {
  Relative(i64),
  Direct(i64),
  Stored(i64),
}
impl std::fmt::Display for Value {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Value::Relative(base) => write!(fmt, "Relative({})", base),
      Value::Direct(val) => write!(fmt, "Direct({})", val),
      Value::Stored(addr) => write!(fmt, "Stored({})", addr),
    }
  }
}
