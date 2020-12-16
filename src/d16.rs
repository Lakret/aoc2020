use std::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<Box<u64>> {
  let task = Task::parse(input);

  let ticket_scanning_error_rate = task.invalid_values().into_iter().sum();
  Some(Box::new(ticket_scanning_error_rate))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let task = Task::parse(input);
  let valid_tickets = task.valid_tickets();
  // valid_tickets.push(task.your_ticket.clone());
  dbg!(valid_tickets.len());

  let assignment = task.field_to_idx(&valid_tickets).unwrap();
  dbg!(&assignment);

  let mut result = 1;
  for departure_field in assignment.keys().filter(|key| key.starts_with("departure")) {
    let departure_filed_idx = assignment[departure_field];
    result *= task.your_ticket.0[departure_filed_idx];
  }

  Some(Box::new(result))
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
    self.field_to_idx_inner(
      tickets,
      HashMap::new(),
      HashSet::new(),
      self.rules.keys().cloned().collect(),
      ticket_length,
    )
  }

  fn field_to_idx_inner(
    &self,
    tickets: &Vec<Ticket>,
    assignment: Assignment,
    assigned_idx: HashSet<usize>,
    unassigned: Vec<String>,
    ticket_length: usize,
  ) -> Option<Assignment> {
    if unassigned.is_empty() {
      Some(assignment)
    } else {
      let mut unassigned = unassigned;
      let field_name = unassigned.pop().unwrap();

      let [rule1, rule2] = self.rules.get(&field_name).unwrap();
      for idx in 0..ticket_length {
        let field_name = field_name.clone();
        if !assigned_idx.contains(&idx) {
          let rule_is_satisfied = tickets.iter().all(|ticket| {
            let value = ticket.0[idx];
            rule1.contains(&value) || rule2.contains(&value)
          });

          if rule_is_satisfied {
            let mut assignment = assignment.clone();
            assignment.insert(field_name, idx);

            let mut assigned_idx = assigned_idx.clone();
            assigned_idx.insert(idx);

            match self.field_to_idx_inner(tickets, assignment, assigned_idx, unassigned.clone(), ticket_length) {
              result @ Some(_) => return result,
              _ => continue,
            }
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
    // departure location: 49-920 or 932-950
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
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d16").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
