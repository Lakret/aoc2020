use std::collections::HashMap;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let program = Program::parse(input);
  let memory = program.execute().memory;
  let result = memory.into_iter().map(|(_address, value)| value).sum();

  Some(Box::new(result))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mask {
  ones: u64,
  zeros: u64,
}

impl Mask {
  fn apply(&self, value: u64) -> u64 {
    (value & self.zeros) | self.ones
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instr {
  SetMask(Mask),
  Write { address: u64, value: u64 },
}

use Instr::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program(Vec<Instr>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
  memory: HashMap<u64, u64>,
  mask: Option<Mask>,
}

impl State {
  fn new() -> State {
    State {
      memory: HashMap::new(),
      mask: None,
    }
  }
}

impl Program {
  fn parse(input: &str) -> Program {
    let instructions = input
      .trim_end()
      .split('\n')
      .map(|line| {
        let parts = &line.split(" = ").collect::<Vec<_>>();
        let (lhs, rhs) = (parts[0], parts[1]);

        if lhs == "mask" {
          let ones_mask = rhs.replace("X", "0");
          let ones = u64::from_str_radix(&ones_mask, 2)
            .expect(&format!("Ones bitmask should be a u64 in binary: {}", &ones_mask));

          let zeros_mask = rhs.replace("X", "1");
          let zeros = u64::from_str_radix(&zeros_mask, 2)
            .expect(&format!("Zeros bitmask should be a u64 in binary: {}", &zeros_mask));

          SetMask(Mask { ones, zeros })
        } else {
          let address = lhs
            .trim_start_matches("mem[")
            .trim_end_matches("]")
            .parse::<u64>()
            .expect(&format!("Memory address should be a u64: {}.", lhs));
          let value = rhs.parse::<u64>().expect(&format!("Value should be a u64: {}.", rhs));

          Write { address, value }
        }
      })
      .collect::<Vec<_>>();

    Program(instructions)
  }

  fn execute(&self) -> State {
    let mut state = State::new();

    for &instr in self.0.iter() {
      match instr {
        SetMask(mask) => {
          state.mask = Some(mask);
        }
        Write { address, value } => {
          let value = state.mask.unwrap().apply(value);
          state.memory.insert(address, value);
        }
      }
    }

    state
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/sample14").unwrap();
    let program = Program::parse(&input);

    assert_eq!(
      program.0,
      vec![
        SetMask(Mask {
          ones: 64,
          zeros: 68719476733,
        }),
        Write { address: 8, value: 11 },
        Write { address: 7, value: 101 },
        Write { address: 8, value: 0 },
      ]
    );
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample14").unwrap();
    assert_eq!(solve(&input), Some(Box::new(165)));

    let input = fs::read_to_string("inputs/d14").unwrap();
    assert_eq!(solve(&input), Some(Box::new(2346881602152)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d14").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
