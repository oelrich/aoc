pub fn run() -> String {
  let program = helpers::loader::load_integer_row_list("./day/09/input.csv")[0].clone();
  let mut computer = icc::Computer::load(0, &program);
  let result = icc::run_to_end(&mut computer, &[1]);
  match result {
    Ok(data) => format! {"{:?}", data},
    Err(error) => error,
  }
}
#[cfg(test)]
mod test {
  #[test]
  fn name() {
    // 3638931938
  }
}
