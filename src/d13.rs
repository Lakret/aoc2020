pub fn solve(input: &str) -> Option<Box<u32>> {
  let task = Task::parse(input);

  let mut timestamp = task.timestamp;
  loop {
    for bus_id in task.bus_ids.iter() {
      if timestamp % *bus_id == 0 {
        let answer = (timestamp - task.timestamp) * bus_id;
        return Some(Box::new(answer));
      }
    }

    timestamp += 1;
  }
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let schedule = input
    .trim_end()
    .split('\n')
    .skip(1)
    .next()
    .unwrap()
    .split(',')
    .enumerate()
    .filter(|(_idx, elem)| *elem != "x")
    .map(|(idx, bus_id)| (idx as u64, bus_id.parse::<u64>().unwrap()))
    .collect::<Vec<_>>();

  // Solution adapted from: https://bit.ly/3p1r9Tl
  let mut timestamp = 1;
  let mut wait_time = 1;
  for (offset, bus_id) in schedule.into_iter() {
    loop {
      if (timestamp + offset) % bus_id == 0 {
        wait_time *= bus_id;
        break;
      }

      timestamp += wait_time;
    }
  }

  Some(Box::new(timestamp))
}

#[derive(Debug, Clone)]
struct Task {
  timestamp: u32,
  bus_ids: Vec<u32>,
}

impl Task {
  fn parse(input: &str) -> Task {
    match &input.trim_end().split('\n').collect::<Vec<_>>()[..] {
      [timestamp, bus_ids] => {
        let timestamp = timestamp.parse::<u32>().unwrap();
        let bus_ids = bus_ids
          .split(',')
          .filter(|id| *id != "x")
          .map(|id| id.parse::<u32>().unwrap())
          .collect();

        Task { timestamp, bus_ids }
      }
      split => panic!("Unexpected input split: {:?}", split),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample13").unwrap();
    assert_eq!(solve(&input), Some(Box::new(295)));

    let input = fs::read_to_string("inputs/d13").unwrap();
    assert_eq!(solve(&input), Some(Box::new(174)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/sample13").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(1068781)));

    let input = fs::read_to_string("inputs/d13").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(780601154795940)));
  }
}
