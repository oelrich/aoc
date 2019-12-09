use super::error::Error;
#[derive(Debug, Clone)]
pub enum OpCode {
  Add = 1,
  Multiply = 2,
  Read = 3,
  Write = 4,
  JumpIfTrue = 5,
  JumpIfFalse = 6,
  LessThan = 7,
  Equals = 8,
  ReduceAbsoluteRelative = 9,
  Halt = 99,
}
impl OpCode {
  pub fn get_op(val: i64) -> Result<OpCode, Error> {
    match val {
      1 => Ok(OpCode::Add),
      2 => Ok(OpCode::Multiply),
      3 => Ok(OpCode::Read),
      4 => Ok(OpCode::Write),
      5 => Ok(OpCode::JumpIfTrue),
      6 => Ok(OpCode::JumpIfFalse),
      7 => Ok(OpCode::LessThan),
      8 => Ok(OpCode::Equals),
      9 => Ok(OpCode::ReduceAbsoluteRelative),
      99 => Ok(OpCode::Halt),
      _ => Err(Error::Operand(val)),
    }
  }
}
