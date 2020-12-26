pub fn solve(input: &str) -> Option<Box<i64>> {
  let keys = input
    .trim_end()
    .split('\n')
    .map(|key| key.parse::<i64>().unwrap())
    .collect::<Vec<_>>();

  let answer = find_encryption_key_simple(keys[0], keys[1]);
  Some(Box::new(answer))
}

pub fn solve2(_input: &str) -> Option<Box<String>> {
  Some(Box::new("There's no part 2 in the last day.".to_string()))
}

fn find_encryption_key_simple(public_key1: i64, public_key2: i64) -> i64 {
  let mut loop_size2 = 0;
  let mut result = 1;

  while result != public_key2 {
    loop_size2 += 1;
    result = (result * 7) % 20201227;
  }

  let candidate1 = transform(public_key1, loop_size2);
  candidate1
}

#[allow(dead_code)]
fn find_encryption_key(public_key1: i64, public_key2: i64) -> i64 {
  let (loop_size1, loop_size2) = find_loop_sizes(public_key1, public_key2);

  let candidate1 = transform(public_key1, loop_size2);
  let candidate2 = transform(public_key2, loop_size1);

  assert_eq!(candidate1, candidate2);
  candidate1
}

#[allow(dead_code)]
fn find_loop_sizes(public_key1: i64, public_key2: i64) -> (i64, i64) {
  let mut loop_size1 = 1;
  while transform(7, loop_size1) != public_key1 {
    loop_size1 += 1;
  }

  let mut loop_size2 = 1;
  while transform(7, loop_size2) != public_key2 {
    loop_size2 += 1;
  }

  (loop_size1, loop_size2)
}

#[allow(dead_code)]
fn transform(subject: i64, loop_size: i64) -> i64 {
  let mut result = 1;

  for _ in 1..=loop_size {
    result = result * subject;
    result = result % 20201227;
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn test_match() {
    // card's public key and loop size
    assert_eq!(transform(7, 8), 5764801);
    // door's public key and loop size
    assert_eq!(transform(7, 11), 17807724);

    // door's public key and card's loop size = encryption key
    assert_eq!(transform(17807724, 8), 14897079);
    // card's public key and door's loop size = same encryption key
    assert_eq!(transform(5764801, 11), 14897079);

    let (ls1, ls2) = find_loop_sizes(5764801, 17807724);
    assert_eq!(ls1, 8);
    assert_eq!(ls2, 11);

    assert_eq!(find_encryption_key(5764801, 17807724), 14897079);
    assert_eq!(find_encryption_key_simple(5764801, 17807724), 14897079);
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample25").unwrap();
    assert_eq!(solve(&input), Some(Box::new(14897079)));

    // loop_size2 = 5497777
    let input = fs::read_to_string("inputs/d25").unwrap();
    assert_eq!(solve(&input), Some(Box::new(296776)));
  }
}
