use super::error::Error;
use super::memory::{Mode, Value};
use super::opcode::OpCode;

pub enum Instruction {
  Add((Value, Value, Value)),
  Multiply((Value, Value, Value)),
  JumpIfTrue((Value, Value)),
  JumpIfFalse((Value, Value)),
  LessThan((Value, Value, Value)),
  Equals((Value, Value, Value)),
  ReduceAbsoluteRelative(Value),
  Input(Value),
  Output(Value),
  Halt,
}

impl Instruction {
  pub fn get_op_modes(op: i64) -> Result<(OpCode, Mode, Mode, Mode), Error> {
    let op_val = op % 100;
    let mode0 = (op % 1000) / 100;
    let mode1 = (op % 10000) / 1000;
    let mode2 = (op % 100_000) / 10_000;
    Ok((
      OpCode::get_op(op_val)?,
      Mode::src_of_val(mode0)?,
      Mode::src_of_val(mode1)?,
      Mode::dest_of_val(mode2)?,
    ))
  }
}
