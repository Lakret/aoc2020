pub fn solve(input: &str) -> Option<Box<usize>> {
  None
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/$day").unwrap();
    assert_eq!(solve(&input), None);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/$day").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
