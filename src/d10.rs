use std::collections::{HashMap, HashSet};

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
  let mut arrangement_without_start = reconstruct_arrangement(&differences);
  let mut arrangement = vec![0];
  arrangement.append(&mut arrangement_without_start);

  let subproblems = split_non_overlapping(&arrangement);

  let mut total = 1;
  for subproblem in subproblems.into_iter() {
    let mut counts = HashMap::new();
    count_arrangements(&mut counts, &subproblem);
    total *= counts.keys().len() as i64;
  }

  Some(total)
}

fn split_non_overlapping(init: &Vec<u64>) -> Vec<Vec<u64>> {
  let mut v = vec![];
  let mut prev_idx = 0;

  let mut non_removables = non_removable(init);
  non_removables.sort();
  for (idx, _num) in non_removables {
    if idx - prev_idx > 1 {
      let chunk = init[prev_idx..=idx].into_iter().cloned().collect::<Vec<_>>();
      v.push(chunk);
    }

    prev_idx = idx;
  }

  v
}

fn non_removable(init: &Vec<u64>) -> Vec<(usize, u64)> {
  let mut v = vec![(0, init[0]), (init.len() - 1, *init.last().unwrap())];

  for (idx, num) in init.iter().enumerate() {
    if !v.contains(&(idx, *num)) {
      if num - init[idx - 1] == 3 {
        v.push((idx, *num));
      }

      if init[idx + 1] - num == 3 {
        v.push((idx, *num));
      }
    }
  }

  v
}

// TODO: refactor to be a simple hashset to count arrangmeents, since counts are not correct
fn count_arrangements(counts: &mut HashMap<Vec<u64>, u64>, init: &Vec<u64>) -> u64 {
  match counts.get(init) {
    Some(x) => *x,
    None => {
      let mut local_counts = vec![];

      for (idx, _num) in init.iter().enumerate() {
        if idx != 0 && idx != init.len() - 1 {
          if init[idx + 1] - init[idx - 1] <= 3 {
            let mut arrangement = init.clone();
            arrangement.remove(idx);

            let derived_count = count_arrangements(counts, &arrangement);
            if !counts.contains_key(&arrangement) {
              counts.insert(arrangement, derived_count);
            }

            local_counts.push(derived_count)
          }
        }
      }

      let result = local_counts.iter().sum();
      counts.insert(init.clone(), result + 1);
      result
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

fn reconstruct_arrangement(jolt_differences: &Vec<u64>) -> Vec<u64> {
  let mut result = vec![];

  jolt_differences.iter().fold(0, |current, jolts| {
    let current = current + *jolts;
    result.push(current);
    current
  });

  result
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
