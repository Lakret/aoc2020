use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod d1;
mod d2;

type Solver = fn(&str) -> Option<i64>;
const COMMANDS: [(&'static str, Solver); 3] =
  [("d1", d1::solve), ("d1_2", d1::solve2), ("d2", d2::solve)];

fn parse_line(line: &str) -> Option<(&str, &str)> {
  let items = line.split_ascii_whitespace().collect::<Vec<_>>();
  match &items[..] {
    // Default case for each day is to use its input file
    &[day] => day
      // ignore the task part to get the filename
      .split("_")
      .take(1)
      .next()
      .map(|input_file| (day, input_file)),
    &[day, input_file] => Some((day, input_file)),
    _ => None,
  }
}

fn main() {
  let mut rl = Editor::<()>::new();
  let commands: HashMap<&'static str, Solver> =
    COMMANDS.iter().cloned().collect();

  loop {
    let readline = rl.readline(">>> ");

    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str());

        if let Some((solver, input_file)) = parse_line(&line) {
          let input_file = Path::new("inputs").join(input_file);

          match commands.get(solver) {
            None => println!("Unrecoginzed command: {:?}.", &line),
            Some(solver) => match fs::read_to_string(&input_file) {
              Ok(input) => {
                let result = solver(&input);
                println!("{:?}", result);
              }
              Err(error) => println!(
                "Cannot read input file {:?} due to {:?}.",
                &input_file, error
              ),
            },
          }
        }
      }

      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }

      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }

      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
  }
}
