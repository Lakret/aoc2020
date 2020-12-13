use std::collections::HashSet;

pub fn solve(input: &str) -> Option<Box<i64>> {
  let numbers = parse(input);

  if let Some((x, y)) = find_complement(&numbers, 0) {
    return Some(Box::new(x * y));
  }

  None
}

pub fn solve2(input: &str) -> Option<Box<i64>> {
  let numbers = parse(input);

  for z in numbers.iter() {
    if let Some((x, y)) = find_complement(&numbers, *z) {
      return Some(Box::new(x * y * z));
    }
  }

  None
}

// Helpers

fn parse(input: &str) -> HashSet<i64> {
  input
    .trim_end()
    .split("\n")
    .map(|number| number.parse::<i64>().unwrap())
    .collect::<HashSet<_>>()
}

// Returns a pair of numbers `x` and `y`, that together with `z`
// sum to `2020`.
// If nothing is found, returns `None`.
fn find_complement(numbers: &HashSet<i64>, z: i64) -> Option<(i64, i64)> {
  for x in numbers.iter() {
    let y = 2020 - x - z;
    if numbers.contains(&y) {
      return Some((*x, y));
    }
  }

  None
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;

  const SAMPLE_INPUT: &'static str = "1721
979
366
299
675
1456";

  #[test]
  fn part_one_works_with_sample() {
    assert_eq!(solve(SAMPLE_INPUT), Some(Box::new(514579)));
  }

  #[test]
  fn part_two_works_with_sample() {
    assert_eq!(solve2(SAMPLE_INPUT), Some(Box::new(241861950)));
  }

  #[test]
  fn part_one_works() {
    let input = fs::read_to_string("inputs/d1").unwrap();
    assert_eq!(solve(&input), Some(Box::new(921504)));
  }

  #[test]
  fn part_two_works() {
    let input = fs::read_to_string("inputs/d1").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(195700142)));
  }
}
