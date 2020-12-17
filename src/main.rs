#[macro_use]
extern crate lazy_static;

use std::time::Instant;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use rustyline::error::ReadlineError;
use rustyline::Editor;

type Solver = fn(&str) -> Option<Box<dyn std::fmt::Debug>>;

/// This macro counts a number of repetitions of some token.
macro_rules! count {
  () => (0usize);
  ($x:tt $($xs:tt)*) => (1usize + count!($($xs)*));
}

/// This macro defines passed modules, and corresponding
/// `module_name` and `module_name_2` commands for the REPL.
///
/// You can also add custom commands in the `commands` map
/// in the `main()` function.
macro_rules! commands {
  ($($module:ident),*) => {
     $(mod $module;)*

    const COMMANDS: [(&'static str, Solver); count!($($module,)*)] = [
      $(
        (
          stringify!($module),
          // Note the cast to  `Box<dyn std::fmt::Debug>` here and below.
          //
          // This is needed to allow `solve` functions in the day modules
          // to return any `Option<Box<T>>`, as long as this `T` implements `Debug`;
          // This prevents errors when calling `assert_eq!` on results
          // of those functions.
          |input: &str| $module::solve(input).map(|x| (x as Box<dyn std::fmt::Debug>))
        ),
      )*

      $(
        (
          concat!(stringify!($module), "_2"),
          |input: &str| $module::solve2(input).map(|x| (x as Box<dyn std::fmt::Debug>))
        ),
      )*
    ];
  };
}

commands!(d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15, d16, d17);

fn main() {
  let mut rl = Editor::<()>::new();

  let mut commands: HashMap<&'static str, Solver> = COMMANDS.iter().cloned().collect();
  // special commands should go here
  commands.insert("d11_debug", |input: &str| {
    d11::solve_debug(input).map(|x| (x as Box<dyn std::fmt::Debug>))
  });
  commands.insert("d11_2_debug", |input: &str| {
    d11::solve2_debug(input).map(|x| (x as Box<dyn std::fmt::Debug>))
  });

  loop {
    let readline = rl.readline(">>> ");

    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str());

        if &line == "all" {
          for (name, solver) in commands.iter() {
            if !name.contains("debug") {
              if let Some(input_file) = solver_name_to_default_input_path(&commands, name) {
                println!("Running {}", name);
                run_command(solver, input_file);
                println!();
              }
            }
          }
        } else if &line == "list" {
          let mut command_names = commands.keys().map(|x| *x).collect::<Vec<_>>();
          command_names.sort();

          println!(
            "The following commands are defined:\n{}",
            textwrap::fill(&command_names.join(", "), 80)
          );
        } else if &line == "next" {
          gen_next_day();
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

fn parse_line(line: &str) -> Option<(&str, PathBuf)> {
  let items = line.split_ascii_whitespace().collect::<Vec<_>>();
  match &items[..] {
    // Default case for each day is to use its input file
    &[day] => task_name_to_default_input_path(day).map(|input_file| (day, input_file)),
    &[day, input_file] => Some((day, Path::new("inputs").join(input_file))),
    _ => None,
  }
}

fn solver_name_to_default_input_path(commands: &HashMap<&'static str, Solver>, solver_name: &str) -> Option<PathBuf> {
  commands
    .iter()
    .find(|(name, _solver)| **name == solver_name)
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

use regex::Regex;

fn gen_next_day() {
  lazy_static! {
    static ref DAY_FILE_NAME: Regex = Regex::new(r"d(?P<number>\d+)\.rs").unwrap();
    static ref COMMANDS_DAY: Regex = Regex::new(r"(?P<day>d\d+)").unwrap();
  }

  let prev_day = fs::read_dir("src")
    .unwrap()
    .filter_map(|item| match item {
      Ok(item) => {
        if item.file_type().unwrap().is_file() {
          let filename = item.file_name().into_string().unwrap();

          DAY_FILE_NAME.captures_iter(&filename).next().and_then(|capture| {
            capture
              .name("number")
              .and_then(|number| number.as_str().parse::<u32>().ok())
          })
        } else {
          None
        }
      }
      Err(_err) => None,
    })
    .max();

  let next_day = match prev_day {
    Some(prev_day) => prev_day + 1,
    None => 1,
  };

  let module_name = format!("d{:02}", next_day);

  println!(
    "Last time we worked on d{}, generating day {}...",
    prev_day.unwrap_or(0),
    &module_name
  );

  let next_filename = format!("{}.rs", &module_name);
  let next_path = Path::new("src").join(&next_filename);
  let next_input_path = Path::new("inputs").join(&module_name);

  let contents = fs::read_to_string("day.template.rs").unwrap();
  let contents = contents.replace("$day", &module_name);

  match fs::write(&next_path, contents).and_then(|_| fs::write(&next_input_path, "")) {
    Ok(_) => {
      let this_path = "src/main.rs";
      let this_contents = fs::read_to_string(this_path).unwrap();

      match this_contents.split('\n').find(|line| line.starts_with("commands!")) {
        None => println!("Cannot find commands! invocation, please, adjust manually."),
        Some(commands_invocation) => {
          let mut modules = COMMANDS_DAY
            .captures_iter(commands_invocation)
            .filter_map(|capture| capture.name("day").map(|c| c.as_str()))
            .collect::<Vec<_>>();
          modules.push(&module_name);

          let new_commands_invocation = format!("commands!({});", modules.join(", "));
          println!("New commands:\n\t{}", &new_commands_invocation);
          let new_this_content = this_contents.replace(commands_invocation, &new_commands_invocation);
          fs::write(this_path, new_this_content).unwrap();
          println!("done.")
        }
      }
    }
    Err(error) => println!("Failed with {:?}", error),
  }
}
