pub fn run() -> String {
  let program = helpers::loader::load_integer_row_list("./day/07/input.csv")[0].clone();
  let settings = vec![5, 6, 7, 8, 9];
  let result = helpers::mathy::permutations_in_memory(settings)
    .iter()
    .map(|setting| {
      let mut amps = icc::adama::Amplifiers::new(&program, &setting);
      amps.run_to_end(Some(0))
    })
    .max()
    .unwrap();
  format!("{}", result)
}
