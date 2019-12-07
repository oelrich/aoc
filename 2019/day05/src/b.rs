use loader::load_integer_row_list;

pub fn run() -> String {
  let mut comp = icc::Computer::load(&load_integer_row_list("./day/05/input.csv")[0]);
  comp.set_input(&[5]);
  let result = comp.run_to_end();
  format!("{:?}", result)
}

#[cfg(test)]
mod tests {
  use loader::load_integer_row_list;

  #[test]
  fn truth() {
    let mut comp = icc::Computer::load(&load_integer_row_list("../day/05/input.csv")[0]);
    comp.set_input(&[5]);
    let result = comp.run_to_end();
    assert_eq!(result, vec![7873292]);
  }
}
