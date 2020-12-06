use std::collections::HashSet;

pub fn solve(input: &str) -> Option<i64> {
  let sum_of_counts = input
    .trim_end()
    .split("\n\n")
    .map(|group| {
      group
        .chars()
        .filter(|ch| *ch != '\n')
        .collect::<HashSet<_>>()
        .len() as i64
    })
    .sum();

  Some(sum_of_counts)
}

pub fn solve2(input: &str) -> Option<i64> {
  let sum_of_counts = input
    .trim_end()
    .split("\n\n")
    .map(|group| {
      let answers_per_person = group
        .split_ascii_whitespace()
        .map(|person| person.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

      answers_per_person
        .iter()
        .fold(answers_per_person[0].clone(), |all_yes, persons_answers| {
          all_yes.intersection(persons_answers).cloned().collect()
        })
        .len() as i64
    })
    .sum();

  Some(sum_of_counts)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_works_with_sample() {
    let input = fs::read_to_string("inputs/sample6").unwrap();
    assert_eq!(solve(&input), Some(11))
  }

  #[test]
  fn part_two_works_with_sample() {
    let input = fs::read_to_string("inputs/sample6").unwrap();
    assert_eq!(solve2(&input), Some(6))
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d6").unwrap();
    assert_eq!(solve(&input), Some(6504))
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d6").unwrap();
    assert_eq!(solve2(&input), Some(3351))
  }
}
