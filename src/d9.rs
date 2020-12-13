use std::collections::HashSet;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let numbers = parse(input);
  first_non_conforming(numbers, 25).map(|res| Box::new(res))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let weakness = 248131121;
  let numbers = parse(input);
  if let Some(region) = find_contagious_set_of_nums_that_sum_to(numbers, weakness) {
    if let (Some(largest), Some(smallest)) = (region.iter().max(), region.iter().min()) {
      return Some(Box::new(largest + smallest));
    }
  }

  None
}

fn find_contagious_set_of_nums_that_sum_to(numbers: Vec<u64>, weakness: u64) -> Option<Vec<u64>> {
  for start_idx in 0..numbers.len() - 2 {
    for end_idx in (start_idx + 1)..(numbers.len() - 1) {
      let mut sum = 0;
      for x in &numbers[start_idx..end_idx + 1] {
        sum += x;
      }

      if sum == weakness {
        let region = numbers[start_idx..end_idx + 1].into_iter().cloned().collect::<Vec<_>>();
        return Some(region);
      }
    }
  }

  None
}

fn parse(input: &str) -> Vec<u64> {
  input
    .trim_end()
    .split('\n')
    .map(|line| line.parse::<u64>().unwrap())
    .collect()
}

fn first_non_conforming(input: Vec<u64>, preamble_length: usize) -> Option<u64> {
  let lookup = input.clone();

  for (idx, number) in input.into_iter().enumerate() {
    if idx >= preamble_length {
      let lookup_slice = lookup[(idx - preamble_length)..idx]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

      let mut is_sum = false;
      for x in lookup_slice.iter().filter(|x| **x <= number) {
        let y = number - x;
        if lookup_slice.contains(&y) {
          is_sum = true;
          break;
        }
      }

      if !is_sum {
        return Some(number);
      }
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let sample_input = fs::read_to_string("inputs/sample9").unwrap();
    let numbers = parse(&sample_input);
    assert_eq!(first_non_conforming(numbers, 5), Some(127));

    let input = fs::read_to_string("inputs/d9").unwrap();
    assert_eq!(solve(&input), Some(Box::new(248131121)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d9").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(31580383)));
  }

  #[test]
  fn parser_works() {
    let sample_input = fs::read_to_string("inputs/sample9").unwrap();
    let numbers = parse(&sample_input);
    assert_eq!(numbers[0], 35);
    assert_eq!(numbers.last(), Some(&576));
  }
}
