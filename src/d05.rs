use std::collections::HashSet;

pub fn solve(input: &str) -> Option<Box<u32>> {
  input
    .trim_end()
    .split_ascii_whitespace()
    .map(|pass| seat_id(pass))
    .max()
    .map(|max_seat_id| Box::new(max_seat_id))
}

pub fn solve2(input: &str) -> Option<Box<u32>> {
  let all_seat_ids = input
    .trim_end()
    .split_ascii_whitespace()
    .map(|pass| seat_id(pass))
    .collect::<HashSet<_>>();

  let mut candidates = all_seat_ids
    .iter()
    .filter_map(|seat_id| {
      if all_seat_ids.contains(&(seat_id + 2)) {
        Some(seat_id + 1)
      } else if all_seat_ids.contains(&(seat_id - 2)) {
        Some(seat_id - 1)
      } else {
        None
      }
    })
    .filter(|candidate| !all_seat_ids.contains(candidate))
    .collect::<HashSet<_>>();

  if candidates.len() == 1 {
    candidates.drain().next().map(|candidate| Box::new(candidate))
  } else {
    panic!("more than one candidate: {:?}", candidates);
  }
}

fn seat_id(pass: &str) -> u32 {
  let (row, col) = decode_boarding_pass(pass);
  row * 8 + col
}

fn decode_boarding_pass(pass: &str) -> (u32, u32) {
  if pass.len() != 10 {
    panic!("Invalid length = {}", pass.len());
  }

  let row = extract_from_range(&pass[..7], 0, 127).unwrap();
  let col = extract_from_range(&pass[7..], 0, 7).unwrap();
  (row, col)
}

fn extract_from_range(divisions: &str, min: u32, max: u32) -> Result<u32, String> {
  match divide_range(&divisions.chars().collect::<Vec<_>>(), min, max) {
    (min, max) if min == max => Ok(min),
    (min, max) => Err(format!("didn't arrive at one value: min = {}, max = {}", min, max)),
  }
}

fn divide_range(divisions: &[char], min: u32, max: u32) -> (u32, u32) {
  match divisions {
    &[division, ref rest @ ..] => {
      let half_of_elems_length = (max - min) / 2;

      match division {
        'F' | 'L' => divide_range(rest, min, min + half_of_elems_length),
        'B' | 'R' => divide_range(rest, min + half_of_elems_length + 1, max),
        _ => panic!("unexpected division {}", division),
      }
    }
    &[] => (min, max),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn boarding_pass_decodes() {
    assert_eq!(decode_boarding_pass("FBFBBFFRLR"), (44, 5))
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d05").unwrap();
    assert_eq!(solve(&input), Some(Box::new(822)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d05").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(705)));
  }
}
