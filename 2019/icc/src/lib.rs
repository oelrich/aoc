use std::collections::HashMap;
pub mod adama;
mod error;
pub mod input;
mod instruction;
mod memory;
mod opcode;
mod state;
use error::*;
use instruction::*;
use memory::*;
use opcode::*;
use state::*;

pub fn run_to_end(computer: &mut Computer, input: &[i64]) -> Result<Vec<i64>, String> {
  let mut input: Vec<i64> = input.iter().cloned().rev().collect();
  let mut output = Vec::new();
  loop {
    match computer.run() {
      State::Crashed => return Err(computer.error()),
      State::Halted => break,
      State::Ready => (),
      State::Running => (),
      State::Output(_id, value) => output.push(value),
      State::Input => {
        if !computer.input_value(input.pop().expect("Asked for more input than provided.")) {
          panic!("Asked for but refused input")
        }
      }
    }
  }
  Ok(output)
}

#[derive(Debug, Clone)]
pub struct Computer {
  id: usize,
  error: Option<Error>,
  input: Option<Value>,
  memory: HashMap<i64, i64>,
  state: State,
  pc: i64,
  relative_base: i64,
}

impl Computer {
  pub fn load(id: usize, code: &[i64]) -> Computer {
    let value_pairs = code
      .iter()
      .enumerate()
      .map(|(idx, val)| (idx as i64, *val))
      .collect::<HashMap<i64, i64>>();
    Computer {
      id,
      error: None,
      state: State::Ready,
      input: None,
      memory: value_pairs,
      pc: 0,
      relative_base: 0,
    }
  }
  pub fn step(&mut self) -> State {
    let mode_op = self.get_mode_op();
    match mode_op {
      Ok(instruction) => self.execute_instruction(instruction),
      Err(err) => {
        self.error = Some(err);
        State::Crashed
      }
    }
  }

  pub fn run(&mut self) -> State {
    if self.state == State::Input {
      return State::Input;
    }
    loop {
      match self.step() {
        State::Running => (),
        State::Ready => (),
        state => return state,
      }
    }
  }

  pub fn input_value(&mut self, value: i64) -> bool {
    if let Some(dest) = self.input.clone() {
      match self.write_value(dest, value) {
        Ok(state) => self.state = state,
        Err(error) => {
          self.error = Some(error);
        }
      }
      self.input = None;
      return true;
    }
    false
  }
  pub fn error(&self) -> String {
    match self.error.clone() {
      Some(error) => format!("Error: {}", error),
      None => "I'm fully operational, and all my circuits are functioning perfectly.".to_owned(),
    }
  }

  pub fn want_input(&self) -> Option<usize> {
    self.input.clone().map(|_| self.id)
  }

  pub fn read_memory(&self, addr: i64) -> Option<i64> {
    self.memory.get(&addr).cloned()
  }
  pub fn edit_memory(&mut self, addr: i64, value: i64) {
    self.memory.insert(addr, value);
  }
}

impl Computer {
  fn set(&mut self, addr: i64, value: i64) {
    self.memory.insert(addr, value);
  }
  fn read(&self, address: i64) -> Result<i64, Error> {
    if address < 0 {
      return Err(Error::Address(address));
    }
    match self.memory.get(&address) {
      Some(value) => Ok(*value),
      None => Ok(0),
    }
  }
  fn read_value(&mut self, source: Value) -> Result<i64, Error> {
    match source {
      Value::Direct(val) => Ok(val),
      Value::Stored(addr) => self.read(addr),
      Value::Relative(base) => self.read(base + self.relative_base),
    }
  }
  fn get_mode(&mut self, mode: Mode) -> Result<Value, Error> {
    self.pc += 2;
    let value = self.read(self.pc - 1)?;
    match mode {
      Mode::Direct => Ok(Value::Direct(value)),
      Mode::Stored => Ok(Value::Stored(value)),
      Mode::Relative => Ok(Value::Relative(value)),
    }
  }

  fn put_mode(&mut self, mode: Mode) -> Result<Value, Error> {
    self.pc += 2;
    let value = self.read(self.pc - 1)?;
    match mode {
      Mode::Direct => Ok(Value::Direct(value)),
      Mode::Stored => Ok(Value::Stored(value)),
      Mode::Relative => Ok(Value::Relative(value)),
    }
  }

  fn get_value(&self, mode: Mode, offset: i64) -> Result<Value, Error> {
    let value = self.read(self.pc + offset)?;
    match mode {
      Mode::Direct => Ok(Value::Direct(value)),
      Mode::Stored => Ok(Value::Stored(value)),
      Mode::Relative => Ok(Value::Relative(value)),
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
      OpCode::Read => Ok(Instruction::Input(self.put_mode(mode0)?)),
      OpCode::Write => Ok(Instruction::Output(self.get_mode(mode0)?)),
      OpCode::ReduceAbsoluteRelative => {
        Ok(Instruction::ReduceAbsoluteRelative(self.get_mode(mode0)?))
      }
      OpCode::Halt => Ok(Instruction::Halt), // Weird address mode for halt?
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
      3 => Ok(Instruction::Input(self.put_mode(Mode::Direct)?)),
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
      Value::Relative(addr) => {
        let addr = addr + self.relative_base;
        self.memory.insert(addr, value);
      }
    }
    Ok(State::Running)
  }
  fn output_value(&mut self, src: Value) -> Result<State, Error> {
    let value = self.read_value(src)?;
    Ok(State::Output(self.id, value))
  }
  fn jump(
    &mut self,
    (value, dest): (Value, Value),
    has_property: fn(i64) -> bool,
  ) -> Result<State, Error> {
    let value = self.read_value(value)?;
    if has_property(value) {
      let dest = self.read_value(dest)?;
      self.pc = dest;
    }
    Ok(State::Running)
  }
  fn update_relative(&mut self, value: Value) -> Result<State, Error> {
    let value = self.read_value(value)?;
    self.relative_base += value;
    Ok(State::Running)
  }
  fn execute_instruction(&mut self, instruction: Instruction) -> State {
    let state = match instruction {
      Instruction::Add(params) => self.binary_op(params, |a, b| a + b),
      Instruction::Multiply(params) => self.binary_op(params, |a, b| a * b),
      Instruction::Equals(params) => self.binary_op(params, |a, b| if a == b { 1 } else { 0 }),
      Instruction::LessThan(params) => self.binary_op(params, |a, b| if a < b { 1 } else { 0 }),
      Instruction::JumpIfTrue(params) => self.jump(params, |v| v != 0),
      Instruction::JumpIfFalse(params) => self.jump(params, |v| v == 0),
      Instruction::ReduceAbsoluteRelative(value) => self.update_relative(value),
      Instruction::Input(dest) => {
        self.input = Some(dest);
        Ok(State::Input)
      }
      Instruction::Output(src) => self.output_value(src),
      Instruction::Halt => Ok(State::Halted),
    };
    self.state = match state {
      Ok(state) => state,
      Err(err) => {
        self.error = Some(err);
        State::Crashed
      }
    };
    self.state.clone()
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
  const ID: usize = 0;

  #[test]
  fn relative_1() {
    let program = vec![
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(program));
    assert_eq!(computer.relative_base, 16);
    assert_eq!(computer.pc, 15);
  }
  #[test]
  fn relative_2() {
    let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let expected = vec![34_915_192 * 34_915_192];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(expected));
  }
  #[test]
  fn relative_3() {
    let program = vec![104, 1125899906842624, 99];
    let expected = vec![1_125_899_906_842_624];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(expected));
  }

  #[test]
  fn relative_4() {
    let program = vec![
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(program));
  }
  #[test]
  fn relative_5() {
    let program = vec![109, 19, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(vec![]));
    assert_eq!(computer.relative_base, 19);
    assert_eq!(computer.pc, 2);
  }
  #[test]
  fn relative_6() {
    let program = vec![109, 1, 209, 0, 109, 18, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(vec![]));
    assert_eq!(computer.relative_base, 20);
    assert_eq!(computer.pc, 6);
  }
  #[test]
  fn jumpy_ext_1() {
    let input = [8];
    let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![1]));
  }
  #[test]
  fn jumpy_ext_2() {
    let input = [42];
    let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![0]));
  }
  #[test]
  fn jumpy_ext_3() {
    let input = [5];
    let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![1]));
  }
  #[test]
  fn jumpy_ext_4() {
    let input = [42];
    let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![0]));
  }
  #[test]
  fn jumpy_1() {
    let input = [8];
    let program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![1]));
  }
  #[test]
  fn jumpy_2() {
    let input = [42];
    let program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![0]));
  }
  #[test]
  fn jumpy_3() {
    let input = [5];
    let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![1]));
  }
  #[test]
  fn jumpy_4() {
    let input = [42];
    let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(vec![0]));
  }
  #[test]
  fn do_input_1() {
    let input = [42];
    let program = [3, 0, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), vec![42, 0, 99]);
  }
  #[test]
  fn do_input_2() {
    let input = [42, 7];
    let program = [3, 0, 3, 1, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), vec![42, 7, 3, 1, 99]);
  }
  #[test]
  fn do_input_3() {
    let input = [42];
    let program = [109, 2, 203, 1, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(computer.relative_base, 2);
    assert_eq!(computer.pc, 4);
    assert_eq!(computer.core(), vec![109, 2, 203, 42, 99]);
    assert!(result.is_ok());
  }
  #[test]
  fn do_input_4() {
    let input = [42];
    let program = [103, 1, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), vec![103, 42, 99]);
  }
  #[test]
  fn do_input_5() {
    let input = [42, 23];
    let program = [103, 1, 103, 0, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), vec![23, 42, 103, 0, 99]);
  }
  #[test]
  fn do_output_1() {
    let program = [4, 0, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(vec![4]));
  }
  #[test]
  fn do_output_2() {
    let program = [4, 0, 4, 4, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &[]);
    assert_eq!(result, Ok(vec![4, 99]));
  }

  #[test]
  fn do_io() {
    let input = vec![42];
    let program = [3, 0, 4, 0, 99];
    let mut computer = Computer::load(ID, &program);
    let result = run_to_end(&mut computer, &input);
    assert_eq!(result, Ok(input));
  }

  #[test]
  fn ext_compute_1() {
    let program = vec![1002, 4, 3, 4, 33];
    let input = vec![];
    let expected = vec![1002, 4, 3, 4, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn ext_compute_2() {
    let program = vec![1101, 100, -1, 4, 0];
    let input = vec![];
    let expected = vec![1101, 100, -1, 4, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), expected);
  }

  #[test]
  fn compute_1() {
    let program = vec![1, 0, 0, 0, 99];
    let input = vec![];
    let expected = vec![2, 0, 0, 0, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_2() {
    let program = vec![2, 3, 0, 3, 99];
    let input = vec![];
    let expected = vec![2, 3, 0, 6, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_3() {
    let program = vec![2, 4, 4, 5, 99, 0];
    let input = vec![];
    let expected = vec![2, 4, 4, 5, 99, 9801];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_4() {
    let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let input = vec![];
    let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
    assert_eq!(computer.core(), expected);
  }
  #[test]
  fn compute_5() {
    let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let input = vec![];
    let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut computer = Computer::load(ID, &program);
    assert!(run_to_end(&mut computer, &input).is_ok());
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
    let comp = Computer::load(ID, &loader::load_integer_row_list("../day/02/input.csv")[0]);
    let actual = input::find_cent(&comp, 6_627_023).expect("should have a value");
    let expected = input::Input::new(12, 2).cent();

    assert_eq!(actual, expected);
  }
}
