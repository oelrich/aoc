use clap::{App, Arg};
use helpers::Solvers;

fn main() {
  let mut solutions: Vec<Solvers> = vec![
    day09::solve(),
    day08::solve(),
    day07::solve(),
    day06::solve(),
    day05::solve(),
    day04::solve(),
    day03::solve(),
    day02::solve(),
    day01::solve(),
  ];
  solutions.reverse();
  let matches = App::new("Advent of Code")
    .version("2019")
    .arg(Arg::with_name("all").long("all").short("a"))
    .get_matches();

  if matches.is_present("all") {
    println!("This may take a while ...");
    solutions.iter().enumerate().for_each(|(idx, s)| {
      print(s, idx + 1);
    });
  } else if let Some(latest) = solutions.pop() {
    let idx = solutions.len();
    print(&latest, idx + 1);
  }
}

fn print(solver: &Solvers, day: usize) {
  println!("[{:02}:a] {}", day, solver.a());
  println!("[{:02}:b] {}", day, solver.b());
}
