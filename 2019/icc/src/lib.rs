use std::collections::HashMap;
mod error;
pub mod input;
use error::Error;
enum OpCode {
  Add = 1,
  Multiply = 2,
  Read = 3,
  Write = 4,
  JumpIfTrue = 5,
  JumpIfFalse = 6,
  LessThan = 7,
  Equals = 8,
  Halt = 99,
}
impl OpCode {
  fn get_op(val: i64) -> Result<OpCode, Error> {
    match val {
      1 => Ok(OpCode::Add),
      2 => Ok(OpCode::Multiply),
      3 => Ok(OpCode::Read),
      4 => Ok(OpCode::Write),
      5 => Ok(OpCode::JumpIfTrue),
      6 => Ok(OpCode::JumpIfFalse),
      7 => Ok(OpCode::LessThan),
      8 => Ok(OpCode::Equals),
      99 => Ok(OpCode::Halt),
      _ => Err(Error::Operand(val)),
    }
  }
}
enum Mode {
  Direct,
  Stored,
}
impl Mode {
  fn dest_of_val(val: i64) -> Result<Mode, Error> {
    match val {
      0 => Ok(Mode::Direct),
      1 => Ok(Mode::Stored),
      _ => Err(Error::Mode),
    }
  }
  fn src_of_val(val: i64) -> Result<Mode, Error> {
    match val {
      0 => Ok(Mode::Stored),
      1 => Ok(Mode::Direct),
      _ => Err(Error::Mode),
    }
  }
}
enum Value {
  Direct(i64),
  Stored(i64),
}
impl std::fmt::Display for Value {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Value::Direct(val) => write!(fmt, "Direct({})", val),
      Value::Stored(addr) => write!(fmt, "Stored({})", addr),
    }
  }
}
enum Instruction {
  Add((Value, Value, Value)),
  Multiply((Value, Value, Value)),
  JumpIfTrue((Value, Value)),
  JumpIfFalse((Value, Value)),
  LessThan((Value, Value, Value)),
  Equals((Value, Value, Value)),
  Input(Value),
  Output(Value),
  Halt,
}

impl Instruction {
  fn get_op_modes(op: i64) -> Result<(OpCode, Mode, Mode, Mode), Error> {
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

#[derive(Debug, Clone)]
pub struct Computer {
  input: Vec<i64>,
  output: Vec<i64>,
  memory: HashMap<i64, i64>,
  pc: i64,
}
enum State {
  Output,
  Input,
  Running,
  Halted,
  Crash,
}
impl Computer {
  fn set(&mut self, addr: i64, value: i64) {
    self.memory.insert(addr, value);
  }
  pub fn get(self, addr: i64) -> i64 {
    *self.memory.get(&addr).expect("msg: &str")
  }
  pub fn load(code: &[i64]) -> Computer {
    let value_pairs = code
      .iter()
      .enumerate()
      .map(|(idx, val)| (idx as i64, *val))
      .collect::<HashMap<i64, i64>>();
    Computer {
      input: Vec::new(),
      output: Vec::new(),
      memory: value_pairs,
      pc: 0,
    }
  }
  fn read(&self, address: i64) -> Result<i64, Error> {
    match self.memory.get(&address) {
      Some(value) => Ok(*value),
      None => Err(Error::Address(address)),
    }
  }
  fn get_mode(&mut self, mode: Mode) -> Result<Value, Error> {
    self.pc += 2;
    let value = self.read(self.pc - 1)?;
    match mode {
      Mode::Direct => Ok(Value::Direct(value)),
      Mode::Stored => Ok(Value::Stored(value)),
    }
  }
  fn put_direct(&mut self) -> Result<Value, Error> {
    self.pc += 2;
    Ok(Value::Direct(self.read(self.pc - 1)?))
  }
  fn get_value(&self, mode: Mode, offset: i64) -> Result<Value, Error> {
    let value = self.read(self.pc + offset)?;
    match mode {
      Mode::Direct => Ok(Value::Direct(value)),
      Mode::Stored => Ok(Value::Stored(value)),
    }
  }
  fn get_triplet(
    &mut self,
    mode0: Mode,
    mode1: Mode,
    mode_d: Mode,
  ) -> Result<(Value, Value, Value), Error> {
    self.pc += 4;
    Ok((
      self.get_value(mode0, -3)?,
      self.get_value(mode1, -2)?,
      self.get_value(mode_d, -1)?,
    ))
  }
  fn get_duplex(&mut self, mode0: Mode, mode1: Mode) -> Result<(Value, Value), Error> {
    self.pc += 3;
    Ok((self.get_value(mode0, -2)?, self.get_value(mode1, -1)?))
  }
  fn get_extended_op(&mut self, op: i64) -> Result<Instruction, Error> {
    let (code, mode0, mode1, mode_d) = Instruction::get_op_modes(op)?;
    match code {
      OpCode::Add => Ok(Instruction::Add(self.get_triplet(mode0, mode1, mode_d)?)),
      OpCode::Multiply => Ok(Instruction::Multiply(
        self.get_triplet(mode0, mode1, mode_d)?,
      )),
      OpCode::LessThan => Ok(Instruction::LessThan(
        self.get_triplet(mode0, mode1, mode_d)?,
      )),
      OpCode::Equals => Ok(Instruction::Equals(self.get_triplet(mode0, mode1, mode_d)?)),
      OpCode::JumpIfTrue => Ok(Instruction::JumpIfTrue(self.get_duplex(mode0, mode1)?)),
      OpCode::JumpIfFalse => Ok(Instruction::JumpIfFalse(self.get_duplex(mode0, mode1)?)),
      OpCode::Read => panic!("long read!"),
      OpCode::Write => Ok(Instruction::Output(self.get_mode(mode0)?)),
      _ => Err(Error::Operand(op)),
    }
  }

  fn get_mode_op(&mut self) -> Result<Instruction, Error> {
    let op_value = *self.memory.get(&self.pc).ok_or(Error::Address(self.pc))?;
    match op_value {
      1 => Ok(Instruction::Add(self.get_triplet(
        Mode::Stored,
        Mode::Stored,
        Mode::Direct,
      )?)),
      2 => Ok(Instruction::Multiply(self.get_triplet(
        Mode::Stored,
        Mode::Stored,
        Mode::Direct,
      )?)),
      3 => Ok(Instruction::Input(self.put_direct()?)),
      4 => Ok(Instruction::Output(self.get_mode(Mode::Stored)?)),
      99 => Ok(Instruction::Halt),
      _ => self.get_extended_op(op_value),
    }
  }
  fn binary_op(
    &mut self,
    (src0, src1, dest): (Value, Value, Value),
    op: fn(i64, i64) -> i64,
  ) -> Result<State, Error> {
    let value1 = self.read_value(src0)?;
    let value2 = self.read_value(src1)?;
    let result = op(value1, value2);
    self.write_value(dest, result)
  }
  fn write_value(&mut self, dest: Value, value: i64) -> Result<State, Error> {
    match dest {
      Value::Stored(addr) => {
        let addr = self.read(addr)?;
        self.memory.insert(addr, value);
      }
      Value::Direct(addr) => {
        self.memory.insert(addr, value);
      }
    }
    Ok(State::Running)
  }
  fn input_value(&mut self, dest: Value) -> Result<State, Error> {
    let value: i64;
    if self.input.is_empty() {
      println!("Enter value: ");
      let mut buffer = String::new();
      std::io::stdin()
        .read_line(&mut buffer)
        .expect("input broke");
      buffer = buffer.trim().into();
      println!("input {}", &buffer);
      value = match buffer.parse() {
        Ok(val) => val,
        Err(e) => panic!("{}", e),
      };
    } else {
      value = self.input.pop().unwrap();
    };
    self.write_value(dest, value)?;
    Ok(State::Input)
  }
  fn output_value(&mut self, src: Value) -> Result<State, Error> {
    let value = self.read_value(src)?;
    self.output.push(value);
    Ok(State::Output)
  }
  fn jump(&mut self, (value, dest): (Value, Value), cond: fn(i64) -> bool) -> Result<State, Error> {
    let value = self.read_value(value)?;
    if cond(value) {
      let dest = self.read_value(dest)?;
      self.pc = dest;
    }
    Ok(State::Running)
  }
  fn execute_instruction(&mut self, instruction: Instruction) -> Result<State, Error> {
    match instruction {
      Instruction::Add(params) => self.binary_op(params, |a, b| a + b),
      Instruction::Multiply(params) => self.binary_op(params, |a, b| a * b),
      Instruction::Equals(params) => self.binary_op(params, |a, b| if a == b { 1 } else { 0 }),
      Instruction::LessThan(params) => self.binary_op(params, |a, b| if a < b { 1 } else { 0 }),
      Instruction::JumpIfTrue(params) => self.jump(params, |v| v != 0),
      Instruction::JumpIfFalse(params) => self.jump(params, |v| v == 0),
      Instruction::Input(dest) => self.input_value(dest),
      Instruction::Output(src) => self.output_value(src),
      Instruction::Halt => Ok(State::Halted),
    }
  }
  fn read_value(&mut self, source: Value) -> Result<i64, Error> {
    match source {
      Value::Direct(val) => Ok(val),
      Value::Stored(addr) => self.read(addr),
    }
  }
  fn step(&mut self) -> Result<State, Error> {
    let mode_op = self.get_mode_op();
    match mode_op {
      Ok(instruction) => self.execute_instruction(instruction),
      Err(Error::Address(addr)) => {
        println!("Address error at {}", addr);
        Ok(State::Crash)
      }
      Err(err) => Err(err),
    }
  }
  fn is_running(&mut self) -> bool {
    match self.step() {
      Ok(State::Running) => true,
      Ok(State::Input) => true,
      Ok(State::Output) => {
        self.output.iter().for_each(|o| println!("{}", o));
        true
      }
      Ok(State::Halted) => false,
      Ok(State::Crash) => false,
      Err(err) => panic!("{}: {}", self.pc, err),
    }
  }
  pub fn set_input(&mut self, input: &[i64]) {
    input
      .iter()
      .rev()
      .cloned()
      .for_each(|entry| self.input.push(entry));
  }
  pub fn run_to_end(&mut self) -> Vec<i64> {
    while self.is_running() {}
    self.output.clone()
  }
  #[cfg(test)]
  fn core(&self) -> Vec<i64> {
    let mut mem: Vec<(i64, i64)> = self.memory.iter().map(|(k, v)| (*k, *v)).collect();
    mem.sort();
    mem.iter().map(|(_, val)| *val).collect()
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn jumpy_ext_1() {
    let input = [8];
    let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![1]);
  }
  #[test]
  fn jumpy_ext_2() {
    let input = [42];
    let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![0]);
  }
  #[test]
  fn jumpy_ext_3() {
    let input = [5];
    let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![1]);
  }
  #[test]
  fn jumpy_ext_4() {
    let input = [42];
    let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![0]);
  }
  #[test]
  fn jumpy_1() {
    let input = [8];
    let program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![1]);
  }
  #[test]
  fn jumpy_2() {
    let input = [42];
    let program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![0]);
  }
  #[test]
  fn jumpy_3() {
    let input = [5];
    let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![1]);
  }
  #[test]
  fn jumpy_4() {
    let input = [42];
    let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let result = computer.run_to_end();
    assert_eq!(result, vec![0]);
  }
  #[test]
  fn do_input_1() {
    let input = [42];
    let program = [3, 0, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    computer.run_to_end();
    assert_eq!(computer.core(), vec![42, 0, 99]);
  }
  #[test]
  fn do_input_2() {
    let input = [42, 7];
    let program = [3, 0, 3, 1, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    computer.run_to_end();
    assert_eq!(computer.core(), vec![42, 7, 3, 1, 99]);
  }
  #[test]
  fn do_output_1() {
    let program = [4, 0, 99];
    let mut computer = Computer::load(&program);
    let output = computer.run_to_end();
    assert_eq!(output, [4]);
  }
  #[test]
  fn do_output_2() {
    let program = [4, 0, 4, 4, 99];
    let mut computer = Computer::load(&program);
    let output = computer.run_to_end();
    assert_eq!(output, [4, 99]);
  }

  #[test]
  fn do_io() {
    let input = [42];
    let program = [3, 0, 4, 0, 99];
    let mut computer = Computer::load(&program);
    computer.set_input(&input);
    let output = computer.run_to_end();
    assert_eq!(output, input);
  }

  #[test]
  fn ext_compute_1() {
    let program = vec![1002, 4, 3, 4, 33];
    let expected = vec![1002, 4, 3, 4, 99];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn ext_compute_2() {
    let program = vec![1101, 100, -1, 4, 0];
    let expected = vec![1101, 100, -1, 4, 99];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }

  #[test]
  fn compute_1() {
    let program = vec![1, 0, 0, 0, 99];
    let expected = vec![2, 0, 0, 0, 99];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_2() {
    let program = vec![2, 3, 0, 3, 99];
    let expected = vec![2, 3, 0, 6, 99];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_3() {
    let program = vec![2, 4, 4, 5, 99, 0];
    let expected = vec![2, 4, 4, 5, 99, 9801];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_4() {
    let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_5() {
    let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut computer = Computer::load(&program);
    computer.run_to_end();
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn good_cent() {
    let actual = input::Input::new(12, 2).cent();
    let expected = 1202;
    assert_eq!(actual, expected);
  }

  #[test]
  fn find_cent_1() {
    let comp = Computer::load(&loader::load_integer_row_list("../day/02/input.csv")[0]);
    let actual = input::find_cent(&comp, 6_627_023).expect("should have a value");
    let expected = input::Input::new(12, 2).cent();

    assert_eq!(actual, expected);
  }
}
