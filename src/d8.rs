use std::collections::HashSet;
use std::hash::Hash;

use Op::*;

pub fn solve(input: &str) -> Option<i64> {
  let mut machine = Machine::parse(&input);
  machine.run_till_repetition();

  Some(machine.acc)
}

pub fn solve2(input: &str) -> Option<i64> {
  let machine = Machine::parse(&input);

  for (idx, Instr(op, arg)) in machine.program.iter().enumerate() {
    match op {
      Nop => {
        if let Some(acc) = try_to_run_with_replaced_instr(&machine, idx, Instr(Jmp, *arg)) {
          return Some(acc);
        }
      }
      Jmp => {
        if let Some(acc) = try_to_run_with_replaced_instr(&machine, idx, Instr(Nop, *arg)) {
          return Some(acc);
        }
      }
      _ => (),
    }
  }

  None
}

fn try_to_run_with_replaced_instr(machine: &Machine, idx: usize, instr: Instr) -> Option<i64> {
  let mut new_machine = machine.clone();
  new_machine.program[idx] = instr;

  new_machine.run_till_repetition();
  if new_machine.terminated() {
    return Some(new_machine.acc);
  }

  None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
  acc: i64,
  program: Vec<Instr>,
  ip: i64,
  trace: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Instr(Op, i64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Op {
  Acc,
  Jmp,
  Nop,
}

impl Op {
  fn parse(input: &str) -> Op {
    match input {
      "acc" => Acc,
      "jmp" => Jmp,
      "nop" => Nop,
      _ => panic!("unexpected op: {}", input),
    }
  }
}

impl Machine {
  fn parse(input: &str) -> Machine {
    let mut program = vec![];

    for instr in input.trim_end().split("\n") {
      match &instr.split_ascii_whitespace().collect::<Vec<_>>()[..] {
        &[op, arg] => {
          let op = Op::parse(op);
          let arg = arg
            .trim_start_matches("+")
            .parse::<i64>()
            .expect(&format!("expected arg, got {:?}", arg));
          program.push(Instr(op, arg));
        }
        unexpected => panic!("unexpected line: {:?}", unexpected),
      }
    }

    Machine {
      acc: 0,
      ip: 0,
      trace: vec![],
      program,
    }
  }

  fn advance(&mut self, trace: bool) {
    if trace {
      self.trace.push(self.ip);
    }

    if !self.terminated() {
      let Instr(op, arg) = self.program[self.ip as usize];
      match op {
        Nop => self.ip += 1,
        Jmp => self.ip += arg,
        Acc => {
          self.acc += arg;
          self.ip += 1;
        }
      }
    }
  }

  fn run_till_repetition(&mut self) {
    let mut seen = HashSet::new();

    while !seen.contains(&self.ip) {
      seen.insert(self.ip);
      self.advance(false);
    }
  }

  /// Returns `true` if the program has terminated.
  fn terminated(&self) -> bool {
    if self.ip < 0 || self.ip as usize >= self.program.len() {
      true
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample8").unwrap();
    assert_eq!(solve(&input), Some(5));

    let input = fs::read_to_string("inputs/d8").unwrap();
    assert_eq!(solve(&input), Some(1614));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/sample8").unwrap();
    assert_eq!(solve2(&input), Some(8));

    let input = fs::read_to_string("inputs/d8").unwrap();
    assert_eq!(solve2(&input), Some(1260));
  }

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/sample8").unwrap();
    let machine = Machine::parse(&input);

    assert_eq!(
      machine,
      Machine {
        acc: 0,
        program: vec![
          Instr(Nop, 0,),
          Instr(Acc, 1,),
          Instr(Jmp, 4,),
          Instr(Acc, 3,),
          Instr(Jmp, -3,),
          Instr(Acc, -99,),
          Instr(Acc, 1,),
          Instr(Jmp, -4,),
          Instr(Acc, 6,),
        ],
        ip: 0,
        trace: vec![],
      }
    );
  }
}
