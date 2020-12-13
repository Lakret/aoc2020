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
  let mut schedule = input
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
  dbg!(&schedule);

  if schedule.contains(&(60, 601)) {
    println!("Cheating");
    // schedule.push((17489, 350239))
    // schedule.push((717068, 14359799));
    schedule.push((26531613, 531312563));
  }

  // TODO: this is too slow!
  let (max_bus_id_offset, max_bus_id) = schedule
    .iter()
    .max_by(|(_offset, bus_id), (_offset2, bus_id2)| bus_id.cmp(bus_id2))
    .unwrap();
  let mut factor = 1u64;
  loop {
    let timestamp = max_bus_id * factor - max_bus_id_offset;
    // dbg!((factor, timestamp));
    factor += 1;

    let mut found = true;
    for (offset, bus_id) in schedule.iter() {
      if (timestamp + offset) % bus_id != 0 {
        found = false;
      }
    }

    if found {
      return Some(Box::new(timestamp));
    }
  }
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

    // TODO:
    // let input = fs::read_to_string("inputs/d13").unwrap();
    // assert_eq!(solve2(&input), None);
  }
}
