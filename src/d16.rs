use std::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::sync::mpsc;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let task = Task::parse(input);

  let ticket_scanning_error_rate = task.invalid_values().into_iter().sum();
  Some(Box::new(ticket_scanning_error_rate))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let task = Task::parse(input);
  let valid_tickets = task.valid_tickets();

  let threads = 8;
  let (sender, receiver) = mpsc::channel();
  let _join_handles = (0..threads)
    .into_iter()
    .map(|idx| {
      let task = task.clone();
      let sender = sender.clone();
      let valid_tickets = valid_tickets.clone();

      thread::spawn(move || {
        println!("Running thread {}...", idx);
        match task.field_to_idx(&valid_tickets) {
          Some(assignment) => {
            println!("Thread {} found assignment {:?}", idx, &assignment);
            sender.send(assignment).unwrap_or(());
          }
          _ => (),
        }
      })
    })
    .collect::<Vec<_>>();

  thread::yield_now();

  match receiver.recv() {
    Ok(assignment) => {
      let mut result = 1;
      for departure_field in assignment.keys().filter(|key| key.starts_with("departure")) {
        let departure_filed_idx = assignment[departure_field];
        result *= task.your_ticket.0[departure_filed_idx];
      }

      Some(Box::new(result))
    }
    Err(err) => {
      dbg!(err);
      None
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ticket(Vec<u64>);

impl Ticket {
  fn parse(input: &str) -> Ticket {
    let ticket = input
      .trim_end()
      .split(',')
      .map(|value| value.parse::<u64>().unwrap())
      .collect::<Vec<_>>();

    Ticket(ticket)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
  rules: HashMap<String, [RangeInclusive<u64>; 2]>,
  your_ticket: Ticket,
  tickets: Vec<Ticket>,
}

type Assignment = HashMap<String, usize>;

impl Task {
  fn value_matches_some_rule(&self, value: &u64) -> bool {
    for (_field_name, [rule1, rule2]) in self.rules.iter() {
      if rule1.contains(value) || rule2.contains(value) {
        return true;
      }
    }

    false
  }

  fn is_valid_ticket(&self, ticket: &Ticket) -> bool {
    for value in ticket.0.iter() {
      if !self.value_matches_some_rule(value) {
        return false;
      }
    }

    return true;
  }

  fn valid_tickets(&self) -> Vec<Ticket> {
    self
      .tickets
      .iter()
      .filter(|ticket| self.is_valid_ticket(ticket))
      .cloned()
      .collect()
  }

  fn field_to_idx(&self, tickets: &Vec<Ticket>) -> Option<Assignment> {
    let ticket_length = tickets.first().unwrap().0.len();
    let mut tabu_assignments = HashMap::new();

    let mut rng = thread_rng();
    let mut unassigned = self.rules.keys().cloned().collect::<Vec<_>>();
    unassigned.shuffle(&mut rng);

    self.field_to_idx_inner(
      tickets,
      HashMap::new(),
      HashSet::new(),
      unassigned,
      &mut tabu_assignments,
      ticket_length,
    )
  }

  fn field_to_idx_inner(
    &self,
    tickets: &Vec<Ticket>,
    assignment: Assignment,
    assigned_idx: HashSet<usize>,
    unassigned: Vec<String>,
    tabu_assignments: &mut HashMap<String, HashSet<usize>>,
    ticket_length: usize,
  ) -> Option<Assignment> {
    if unassigned.is_empty() {
      Some(assignment)
    } else {
      let mut unassigned = unassigned;
      let field_name = unassigned.pop().unwrap();
      let excluded = tabu_assignments
        .get(&field_name)
        .map(|excluded| excluded.clone())
        .unwrap_or(HashSet::new());

      let [rule1, rule2] = self.rules.get(&field_name).unwrap();
      for idx in 0..ticket_length {
        let field_name = field_name.clone();
        if !assigned_idx.contains(&idx) && !excluded.contains(&idx) {
          let rule_is_satisfied = tickets.iter().all(|ticket| {
            let value = ticket.0[idx];
            rule1.contains(&value) || rule2.contains(&value)
          });

          if rule_is_satisfied {
            let mut assignment = assignment.clone();
            assignment.insert(field_name, idx);

            let mut assigned_idx = assigned_idx.clone();
            assigned_idx.insert(idx);

            match self.field_to_idx_inner(
              tickets,
              assignment,
              assigned_idx,
              unassigned.clone(),
              tabu_assignments,
              ticket_length,
            ) {
              result @ Some(_) => return result,
              _ => continue,
            }
          } else {
            let tabu_idx = tabu_assignments.entry(field_name).or_insert(HashSet::new());
            tabu_idx.insert(idx);
          }
        }
      }

      None
    }
  }

  fn invalid_values(&self) -> Vec<u64> {
    let mut invalid_values = vec![];

    for ticket in self.tickets.iter() {
      for value in ticket.0.iter() {
        if !self.value_matches_some_rule(value) {
          invalid_values.push(*value);
        }
      }
    }

    invalid_values
  }

  fn parse(input: &str) -> Task {
    let mut lines = input.trim_end().split('\n').rev().collect::<Vec<_>>();

    let mut line = lines.pop().unwrap();
    let mut rules = HashMap::new();
    while line != "" {
      if let [field_name, ranges] = line.split(": ").collect::<Vec<_>>()[..] {
        if let [range1, range2] = ranges.split(" or ").collect::<Vec<_>>()[..] {
          let range1 = parse_range(range1);
          let range2 = parse_range(range2);

          rules.insert(field_name.to_string(), [range1, range2]);
        }
      }

      line = lines.pop().unwrap();
    }

    assert_eq!(lines.pop().unwrap(), "your ticket:");
    let your_ticket = Ticket::parse(lines.pop().unwrap());

    assert_eq!(lines.pop().unwrap(), "");
    assert_eq!(lines.pop().unwrap(), "nearby tickets:");
    let mut tickets = vec![];
    while let Some(line) = lines.pop() {
      let ticket = Ticket::parse(line);
      tickets.push(ticket);
    }

    Task {
      rules,
      your_ticket,
      tickets,
    }
  }
}

fn parse_range(range: &str) -> RangeInclusive<u64> {
  match range.split('-').collect::<Vec<_>>()[..] {
    [start, end] => {
      let start = start.parse::<u64>().unwrap();
      let end = end.parse::<u64>().unwrap();
      start..=end
    }
    _ => panic!("Unexpected input: {}.", range),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample16").unwrap();
    assert_eq!(solve(&input), Some(Box::new(71)));

    let input = fs::read_to_string("inputs/d16").unwrap();
    assert_eq!(solve(&input), Some(Box::new(26009)));
  }

  #[test]
  fn backtracking_works() {
    let input = fs::read_to_string("inputs/sample16_2").unwrap();
    let task = Task::parse(&input);
    let assignment = task.field_to_idx(&task.tickets).unwrap();

    assert_eq!(assignment[&"row".to_string()], 0);
    assert_eq!(assignment[&"class".to_string()], 1);
    assert_eq!(assignment[&"seat".to_string()], 2);

    assert_eq!(solve2(&input), Some(Box::new(1)));
  }

  #[test]
  #[ignore]
  fn part_two_solved() {
    // Without tabu, best attempt:
    // >>> d16_2
    // [src/d16.rs:15] valid_tickets.len() = 190
    // [src/d16.rs:18] &assignment = {
    //     "departure track": 12,
    //     "class": 7,
    //     "arrival track": 2,
    //     "type": 13,
    //     "price": 16,
    //     "departure station": 5,
    //     "zone": 1,
    //     "arrival platform": 18,
    //     "departure location": 10,
    //     "wagon": 6,
    //     "seat": 17,
    //     "arrival station": 8,
    //     "arrival location": 14,
    //     "departure platform": 4,
    //     "train": 19,
    //     "duration": 11,
    //     "departure time": 0,
    //     "departure date": 3,
    //     "route": 9,
    //     "row": 15,
    // }
    // Some(589685618167)
    // Elapsed: 454.063766194s.

    // With tabu, best attempt:
    // >>> d16_2
    // [src/d16.rs:15] valid_tickets.len() = 190
    // [src/d16.rs:18] &assignment = {
    //     "departure time": 0,
    //     "arrival station": 8,
    //     "departure date": 3,
    //     "type": 13,
    //     "arrival location": 14,
    //     "class": 7,
    //     "zone": 1,
    //     "duration": 11,
    //     "route": 9,
    //     "departure station": 5,
    //     "price": 16,
    //     "train": 19,
    //     "arrival platform": 18,
    //     "arrival track": 2,
    //     "departure track": 12,
    //     "departure location": 10,
    //     "departure platform": 4,
    //     "wagon": 6,
    //     "row": 15,
    //     "seat": 17,
    // }
    // Some(589685618167)
    // Elapsed: 50.50911689s.

    let input = fs::read_to_string("inputs/d16").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(589685618167)));
  }
}
