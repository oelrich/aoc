pub fn run() -> String {
  let comp = icc::Computer::load(
    0,
    &helpers::loader::load_integer_row_list("./day/02/input.csv")[0],
  );
  let result = icc::input::find_cent(&comp, 19_690_720).expect("should give some value");
  format!("{}", result)
}
#[cfg(test)]
mod tests {
  use helpers::loader::load_integer_row_list;
  use icc::input::find_cent;
  use icc::Computer;
  #[test]
  fn truth() {
    let mut comp = Computer::load(0, &load_integer_row_list("../day/02/input.csv")[0]);
    let actual = find_cent(&mut comp, 19_690_720).expect("should give some value");

    assert_eq!(actual, 4019);
  }
}
