use std::collections::HashMap;

pub fn solve(input: &str) -> Option<Box<u64>> {
  Some(Box::new(last_number_spoken2(input, 2020)))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  Some(Box::new(last_number_spoken2(input, 30000000)))
}

fn last_number_spoken2(input: &str, till_turn: usize) -> u64 {
  let mut memory = Memory::new(input);

  let (turn, last) = input.split(',').fold((0, 0), |(turn, _last), num| {
    let turn = turn + 1;
    let last = num.parse::<u64>().unwrap();
    (turn, last)
  });

  let mut last = last;
  for turn in turn..till_turn {
    let turn = turn + 1;

    let new_number = match memory.recall(&last) {
      (0, 0) => 0,
      (idx_recent, 0) if idx_recent != 0 => 0,
      (idx_recent, idx_pre_recent) if idx_recent != 0 && idx_pre_recent != 0 => (idx_recent - idx_pre_recent) as u64,
      impossible => panic!("Impossible recall at turn {} for {}: {:?}.", turn, last, impossible),
    };
    // dbg!((turn, last, new_number));

    memory.speak(turn, new_number);
    last = new_number;
  }

  last
}

/// Associates each number with two most recent turns when it was spoken:
///
/// `number -> (Option<recent_turn_id>, Option<pre_recent_turn_id>)`.
///
/// We also special-case zeros to speed up the algorithm
/// (0's are spoken more often than other numbers).
#[derive(Debug, Clone)]
struct Memory {
  number_to_turns: HashMap<u64, (usize, usize)>,
  zero_to_turns: [usize; 2],
}

impl Memory {
  fn new(input: &str) -> Memory {
    let mut number_to_turns = input
      .split(',')
      .enumerate()
      .map(|(idx, n)| (n.parse::<u64>().unwrap(), (idx + 1, 0)))
      .collect::<HashMap<_, _>>();

    let mut zero_to_turns = [0; 2];
    let (zero_recent, zero_pre_recent) = number_to_turns.get(&0).unwrap_or(&(0, 0));
    zero_to_turns[0] = *zero_recent;
    zero_to_turns[1] = *zero_pre_recent;
    number_to_turns.remove(&0);

    Memory {
      number_to_turns,
      zero_to_turns,
    }
  }

  fn speak(&mut self, turn: usize, number: u64) {
    if number == 0 {
      let idx_recent = self.zero_to_turns[0];
      self.zero_to_turns[1] = idx_recent;
      self.zero_to_turns[0] = turn;
    } else {
      let new_record = match self.number_to_turns.get(&number) {
        None => (turn, 0),
        Some((idx_recent, _)) if *idx_recent != 0 => (turn, *idx_recent),
        impossible => panic!("Impossible record for {} at turn {}: {:?}.", number, turn, impossible),
      };

      self.number_to_turns.insert(number, new_record);
    }
  }

  fn recall(&self, number: &u64) -> (usize, usize) {
    if *number == 0 {
      (self.zero_to_turns[0], self.zero_to_turns[1])
    } else {
      match self.number_to_turns.get(number) {
        None => (0, 0),
        Some(previous) => (previous.0, previous.1),
      }
    }
  }
}

// first version, was too slow for the second part
fn _last_number_spoken(input: &str, till_turn: usize) -> u64 {
  let mut numbers = input.split(',').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();

  let mut last = *numbers.last().unwrap();
  for _turn in numbers.len()..till_turn {
    let previous_mentions = numbers
      .iter()
      .enumerate()
      .rev()
      .filter(|(_idx, n)| **n == last)
      .take(2)
      .collect::<Vec<_>>();

    last = match &previous_mentions[..] {
      [] | [_] => 0u64,
      [(idx_recent, _), (idx_pre_recent, _)] => (idx_recent - idx_pre_recent) as u64,
      _ => panic!("Unexpected previous mentions: {:?}", previous_mentions),
    };

    numbers.push(last);
  }

  last
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_solved() {
    assert_eq!(solve("0,3,6"), Some(Box::new(436)));
    assert_eq!(solve("1,3,2"), Some(Box::new(1)));
    assert_eq!(solve("2,1,3"), Some(Box::new(10)));
    assert_eq!(solve("1,2,3"), Some(Box::new(27)));
    assert_eq!(solve("2,3,1"), Some(Box::new(78)));
    assert_eq!(solve("3,2,1"), Some(Box::new(438)));
    assert_eq!(solve("3,1,2"), Some(Box::new(1836)));

    assert_eq!(solve("1,0,18,10,19,6"), Some(Box::new(441)));
  }

  #[test]
  #[ignore]
  fn part_two_solved() {
    assert_eq!(solve2("0,3,6"), Some(Box::new(175594)));
    // assert_eq!(solve("1,3,2"), Some(Box::new(2578)));
    // assert_eq!(solve("2,1,3"), Some(Box::new(3544142)));
    // assert_eq!(solve("1,2,3"), Some(Box::new(261214)));
    // assert_eq!(solve("2,3,1"), Some(Box::new(6895259)));
    // assert_eq!(solve("3,2,1"), Some(Box::new(18)));
    // assert_eq!(solve("3,1,2"), Some(Box::new(362)));

    assert_eq!(solve("1,0,18,10,19,6"), Some(Box::new(10613991)));
  }
}
