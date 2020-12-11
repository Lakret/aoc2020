#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use std::time::Instant;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod d1;
mod d10;
mod d11;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

type Solver = fn(&str) -> Option<i64>;
const COMMANDS: [(&'static str, Solver); 24] = [
  ("d1", d1::solve),
  ("d1_2", d1::solve2),
  ("d2", d2::solve),
  ("d2_2", d2::solve2),
  ("d3", d3::solve),
  ("d3_2", d3::solve2),
  ("d4", d4::solve),
  ("d4_2", d4::solve2),
  ("d5", d5::solve),
  ("d5_2", d5::solve2),
  ("d6", d6::solve),
  ("d6_2", d6::solve2),
  ("d7", d7::solve),
  ("d7_2", d7::solve2),
  ("d8", d8::solve),
  ("d8_2", d8::solve2),
  ("d9", d9::solve),
  ("d9_2", d9::solve2),
  ("d10", d10::solve),
  ("d10_2", d10::solve2),
  ("d11", d11::solve),
  ("d11_2", d11::solve2),
  ("d11_debug", d11::solve_debug),
  ("d11_2_debug", d11::solve2_debug),
];

fn parse_line(line: &str) -> Option<(&str, PathBuf)> {
  let items = line.split_ascii_whitespace().collect::<Vec<_>>();
  match &items[..] {
    // Default case for each day is to use its input file
    &[day] => task_name_to_default_input_path(day).map(|input_file| (day, input_file)),
    &[day, input_file] => Some((day, Path::new("inputs").join(input_file))),
    _ => None,
  }
}

fn solver_name_to_default_input_path(solver_name: &str) -> Option<PathBuf> {
  COMMANDS
    .iter()
    .find(|(name, _solver)| *name == solver_name)
    .and_then(|(name, _solver)| task_name_to_default_input_path(name))
}

fn task_name_to_default_input_path(task_name: &str) -> Option<PathBuf> {
  task_name
    // ignore the task part to get the filename
    .split("_")
    .take(1)
    .next()
    .map(|path| Path::new("inputs").join(path))
}

fn run_command<P>(solver: &Solver, input_file: P)
where
  P: AsRef<Path> + std::fmt::Debug,
{
  match fs::read_to_string(&input_file) {
    Ok(input) => {
      let now = Instant::now();

      let result = solver(&input);
      println!("{:?}", result);

      println!("Elapsed: {:?}.", now.elapsed());
    }
    Err(error) => println!("Cannot read input file {:?} due to {:?}.", &input_file, error),
  }
}

fn main() {
  let mut rl = Editor::<()>::new();
  let commands: HashMap<&'static str, Solver> = COMMANDS.iter().cloned().collect();

  loop {
    let readline = rl.readline(">>> ");

    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str());

        if &line == "all" {
          for (name, solver) in COMMANDS.iter() {
            if !name.contains("debug") {
              if let Some(input_file) = solver_name_to_default_input_path(name) {
                println!("Running {}", name);
                run_command(solver, input_file);
                println!();
              }
            }
          }
        } else if let Some((solver, input_file)) = parse_line(&line) {
          match commands.get(solver) {
            None => println!("Unrecoginzed command: {:?}.", &line),
            Some(solver) => run_command(solver, input_file),
          }
        }
      }

      Err(error) => {
        match error {
          ReadlineError::Interrupted => println!("CTRL-C"),
          ReadlineError::Eof => println!("CTRL-D"),
          _ => println!("Error: {:?}", error),
        }

        break;
      }
    }
  }
}
