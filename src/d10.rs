use std::collections::HashSet;

pub fn solve(input: &str) -> Option<i64> {
  let adapters = parse(input);
  let differences = jolt_differences(adapters);

  let one_diffs_count = differences.iter().filter(|jolt| **jolt == 1).count();
  let three_diffs_count = differences.iter().filter(|jolt| **jolt == 3).count();

  Some((one_diffs_count * three_diffs_count) as i64)
}

pub fn solve2(input: &str) -> Option<i64> {
  let adapters = parse(input);
  let differences = jolt_differences(adapters);

  // arrangement is adapters joltages together with (0) and (device), sorted
  let mut arrangement = vec![0];
  differences.iter().fold(0, |current, jolts| {
    let current = current + *jolts;
    arrangement.push(current);
    current
  });

  let total_count = split_non_overlapping(&arrangement)
    .into_iter()
    .fold(1, |acc, problem| acc * count_arrangements(&problem));

  Some(total_count as i64)
}

// Splits `arrangement` into non-overlapping subproblems
// using non-removable numbers as edges.
// E.g., input `(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)`
// has the following [non-removables]:
// `[(0)], [1], [4], 5, 6, [7], [10], 11, [12], [15], [16], [19], [(22)]`,
// thus this function will return the following subproblems for it:
// `[4], 5, 6, [7]` and `[10], 11, [12]`.
fn split_non_overlapping(arrangement: &Vec<u64>) -> Vec<Vec<u64>> {
  let mut v = vec![];
  let mut prev_idx = 0;

  for idx in non_removable(arrangement) {
    if idx - prev_idx > 1 {
      let chunk = arrangement[prev_idx..=idx].into_iter().cloned().collect::<Vec<_>>();
      v.push(chunk);
    }

    prev_idx = idx;
  }

  v
}

// Returns sorted indices of non-removable adapters.
fn non_removable(init: &Vec<u64>) -> Vec<usize> {
  let mut v = vec![0, init.len() - 1];

  for (idx, num) in init.iter().enumerate() {
    if !v.contains(&idx) {
      if num - init[idx - 1] == 3 || init[idx + 1] - num == 3 {
        v.push(idx);
      }
    }
  }

  v.sort();
  v
}

fn count_arrangements(problem: &Vec<u64>) -> usize {
  let mut counts = HashSet::new();
  count_arrangements_inner(&mut counts, problem);
  counts.len()
}

fn count_arrangements_inner(counts: &mut HashSet<Vec<u64>>, problem: &Vec<u64>) {
  if !counts.contains(problem) {
    counts.insert(problem.clone());

    for idx in 0..(problem.len() - 1) {
      if idx != 0 && idx != problem.len() - 1 {
        if problem[idx + 1] - problem[idx - 1] <= 3 {
          let mut arrangement = problem.clone();
          arrangement.remove(idx);

          count_arrangements_inner(counts, &arrangement);
        }
      }
    }
  }
}

fn parse(input: &str) -> Vec<u64> {
  input
    .trim_end()
    .split('\n')
    .map(|line| line.parse::<u64>().unwrap())
    .collect()
}

fn jolt_differences(adapters: Vec<u64>) -> Vec<u64> {
  let mut adapters = adapters.into_iter().collect::<HashSet<_>>();

  let mut result = vec![];
  let mut current = 0;
  while !adapters.is_empty() {
    if adapters.contains(&(current + 1)) {
      result.push(1);
      current += 1;
    } else if adapters.contains(&(current + 2)) {
      result.push(2);
      current += 2;
    } else if adapters.contains(&(current + 3)) {
      result.push(3);
      current += 3;
    } else {
      panic!(
        "impossible joltage difference: current = {}, adapters = {:?}",
        &current, &adapters
      );
    }

    adapters.remove(&current);
  }

  result.push(3);
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d10").unwrap();
    assert_eq!(solve(&input), Some(2812));
  }

  #[test]
  fn part_two_samples_solved() {
    let input = fs::read_to_string("inputs/sample10").unwrap();
    assert_eq!(solve2(&input), Some(8));

    let input = fs::read_to_string("inputs/sample10_2").unwrap();
    assert_eq!(solve2(&input), Some(19208));

    let input = fs::read_to_string("inputs/d10").unwrap();
    assert_eq!(solve2(&input), Some(386869246296064));
  }
}
