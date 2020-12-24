pub fn solve(input: &str) -> Option<Box<String>> {
  let mut cups = Ring::parse(input);

  for _ in 0..100 {
    cups.execute_move();
  }

  Some(Box::new(extract_answer(&cups)))
}

fn extract_answer(cups: &Ring) -> String {
  let mut answer = String::new();

  let one_idx = cups.get_index_by_label(1);
  let mut char_idx = cups.increment_index(one_idx, 1);
  while char_idx != one_idx {
    answer.push_str(&cups.data[char_idx].to_string());
    char_idx = cups.increment_index(char_idx, 1);
  }

  answer
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  let cups = input
    .trim_end()
    .chars()
    .map(|ch| ch.to_string().parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  const TOTAL_CUPS: usize = 1_000_000;

  // Ring buffer is represented as array with each idx = cup label,
  // and value under that index/label is the index/label of the next cup.
  // [3, 8, 9, 1, 2, 5, 4, 6, 7] becomes
  // [0 => (3), 1 => 2, 2 => 5, 3 => 8, 4 => 6, 5 => 4,
  //  6 => 7, 7 => 3, 8 => 9, 9 => 1]
  let mut ring = [0; TOTAL_CUPS + 1];

  // current cup label is the value of the first position
  ring[0] = cups[0];
  // add initial cups
  for two_cups in cups[..].windows(2) {
    match two_cups {
      &[prev, next] => {
        ring[prev] = next;
      }
      _ => unreachable!(),
    }
  }

  // adds remaining cup
  let mut prev = *cups.last().unwrap();
  let mut next = cups.len() + 1;
  while prev < TOTAL_CUPS {
    ring[prev] = next;

    prev = next;
    next += 1;
  }

  // adds reference from the tail to the head
  ring[TOTAL_CUPS] = ring[0];

  for _round in 0..10_000_000 {
    // pick up three cups
    let pick1 = ring[ring[0]];
    let pick2 = ring[pick1];
    let pick3 = ring[pick2];

    // pick destination
    let mut dst = ring[0] - 1;
    while dst == pick1 || dst == pick2 || dst == pick3 || dst == ring[0] || dst == 0 {
      if dst == 0 {
        dst = TOTAL_CUPS;
      } else {
        dst -= 1;
      }
    }

    // we need to go from
    // [ current => pick1 => pick2 => pick3 => after_pick3 ... //
    //   ... => dst => after_dst => ... ]
    // to
    // [ current => after_pick3 ... //
    //   ... => dst => pick1 => pick2 => pick3 => after_dst => ... ]

    // redirect current to after_pick3
    ring[ring[0]] = ring[pick3];

    // place picks between dst and after_dst
    let after_dst = ring[dst];
    ring[dst] = pick1;
    ring[pick3] = after_dst;

    // select new current cup
    ring[0] = ring[ring[0]];
  }

  let answer = ring[1] * ring[ring[1]];
  Some(Box::new(answer))
}

type Label = u32;

/// Ring buffer implementation for the crab cups game.
/// This is too slow for the part 2, but I decided to keep it
/// since it has a cool example of invariant-based API / implementation.
///
/// Implementation relies on maintaining invariant
/// `data[current_idx] == current_label`.
#[derive(Debug, Clone)]
struct Ring {
  data: Vec<Label>,
  current_idx: usize,
  current_label: Label,
  // the area where cups are moved when they are picked up
  staging: [Label; 3],
}

impl Ring {
  fn parse(input: &str) -> Ring {
    let data = input
      .trim_end()
      .chars()
      .map(|ch| ch.to_string().parse().unwrap())
      .collect::<Vec<_>>();
    let current_label = data[0];

    Ring {
      data,
      current_label,
      current_idx: 0,
      staging: [0; 3],
    }
  }

  /// Executes one move of the game:
  ///
  /// - picks up the cups
  /// - finds a destination
  /// - places cups after the destination
  /// - selects the next current cup.
  fn execute_move(&mut self) {
    let current_label = self.data[self.current_idx];

    self.pick_up();

    let mut destination_label = current_label - 1;
    while self.staging.contains(&destination_label) || destination_label == current_label || destination_label == 0 {
      if destination_label == 0 {
        destination_label = *self.data.iter().max().unwrap();
      } else {
        destination_label -= 1;
      }
    }

    let destination_idx = self.get_index_by_label(destination_label);
    self.place(destination_idx);

    self.next_current();
  }

  /// Returns the index of item with `label` in the `data` buffer.
  fn get_index_by_label(&self, label: Label) -> usize {
    let (idx, _) = self
      .data
      .iter()
      .enumerate()
      .find(|(_idx, elem)| **elem == label)
      .unwrap();

    idx
  }

  /// Find the index of an element `increment` places after the element
  /// with index `idx` in `self`, wrapping around as appropriate.
  fn increment_index(&self, idx: usize, increment: usize) -> usize {
    (idx + increment) % self.data.len()
  }

  /// Returns a vector of indices of elements, that are in range
  /// that starts `start_inc` elements after the element with index `idx`,
  /// and ends with the element `end_inc` elements after the `idx` element.
  ///
  /// Accounts for wraps.
  fn index_range(&self, idx: usize, start_inc: usize, end_inc: usize) -> Vec<usize> {
    let start_idx = self.increment_index(idx, start_inc);
    let end_idx = self.increment_index(idx, end_inc);

    if start_idx < end_idx {
      (start_idx..=end_idx).into_iter().collect()
    } else if start_idx > end_idx {
      let mut first = (start_idx..self.data.len()).into_iter().collect::<Vec<_>>();
      let mut second = (0..=end_idx).into_iter().collect::<Vec<_>>();
      first.append(&mut second);
      first
    } else {
      panic!("Unsupported arguments: start_idx {} == end_idx {}.", start_idx, end_idx);
    }
  }

  /// Picks up 3 elements after the current element from the `data` buffer
  /// and places them into the staging area.
  ///
  /// Maintains the `current_idx` points to `current_label` element invariant.
  fn pick_up(&mut self) {
    let pick_up_indices = self.index_range(self.current_idx, 1, 3);

    for (staging_idx, &pick_up_idx) in pick_up_indices.iter().enumerate() {
      self.staging[staging_idx] = self.data[pick_up_idx];
    }

    for _ in 0..3 {
      if self.data.len() > pick_up_indices[0] {
        self.data.remove(pick_up_indices[0]);
      } else {
        self.data.remove(0);
      }
    }

    self.restore_invariant();
  }

  /// Places staging elements back into the the `data` buffer
  /// directly after an element at `destination_idx`.
  ///
  /// Maintains the `current_idx` points to `current_label` element invariant.
  fn place(&mut self, destination_idx: usize) {
    let placement_indices = destination_idx + 1..=destination_idx + 3;
    for (staging_idx, placement_idx) in placement_indices.enumerate() {
      let elem = self.staging[staging_idx];

      if placement_idx > self.data.len() {
        self.data.push(elem);
      } else {
        self.data.insert(placement_idx, elem);
      }
    }

    self.restore_invariant();
  }

  /// Rotates `data` buffer in such a way that `current_idx`
  /// always points to the element with the `current_label`.
  ///
  /// This is an invariant of the algorithm, it should never change during
  /// pick ups or placements.
  fn restore_invariant(&mut self) {
    let new_current_idx = self.get_index_by_label(self.current_label);

    if new_current_idx > self.current_idx {
      self.data.rotate_left(new_current_idx - self.current_idx);
    } else if new_current_idx < self.current_idx {
      self.data.rotate_right(self.current_idx - new_current_idx);
    }
  }

  /// Selects a new current cup.
  ///
  /// Updates `current_idx` and `current_label`.
  fn next_current(&mut self) {
    self.current_idx = self.increment_index(self.current_idx, 1);
    self.current_label = self.data[self.current_idx];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ring_works() {
    let mut ring = Ring::parse("389125467");
    // (3)[891]25467
    assert_eq!(ring.index_range(0, 1, 3), vec![1, 2, 3]);
    // 3]89125(4)[67
    assert_eq!(ring.index_range(6, 1, 3), vec![7, 8, 0]);
    // [389]12546(7)
    assert_eq!(ring.index_range(8, 1, 3), vec![0, 1, 2]);

    assert_eq!(ring.current_idx, 0);
    assert_eq!(ring.current_label, 3);
    ring.execute_move();
    assert_eq!(ring.data, vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);
    assert_eq!(ring.current_idx, 1);
    assert_eq!(ring.current_label, 2);

    ring.execute_move();
    assert_eq!(ring.data, vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);
    assert_eq!(ring.current_idx, 2);
    assert_eq!(ring.current_label, 5);

    ring.execute_move();
    assert_eq!(ring.data, vec![7, 2, 5, 8, 9, 1, 3, 4, 6]);
    assert_eq!(ring.current_idx, 3);
    assert_eq!(ring.current_label, 8);

    ring.execute_move();
    assert_eq!(ring.data, vec![3, 2, 5, 8, 4, 6, 7, 9, 1]);
    assert_eq!(ring.current_idx, 4);
    assert_eq!(ring.current_label, 4);

    // TODO:
    ring.execute_move();
    assert_eq!(ring.data, vec![9, 2, 5, 8, 4, 1, 3, 6, 7]);
    assert_eq!(ring.current_idx, 5);
    assert_eq!(ring.current_label, 1);

    ring.execute_move();
    assert_eq!(ring.data, vec![7, 2, 5, 8, 4, 1, 9, 3, 6]);
    assert_eq!(ring.current_idx, 6);
    assert_eq!(ring.current_label, 9);

    ring.execute_move();
    assert_eq!(ring.data, vec![8, 3, 6, 7, 4, 1, 9, 2, 5]);
    assert_eq!(ring.current_idx, 7);
    assert_eq!(ring.current_label, 2);

    ring.execute_move();
    assert_eq!(ring.data, vec![7, 4, 1, 5, 8, 3, 9, 2, 6]);
    assert_eq!(ring.current_idx, 8);
    assert_eq!(ring.current_label, 6);

    ring.execute_move();
    assert_eq!(ring.data, vec![5, 7, 4, 1, 8, 3, 9, 2, 6]);
    assert_eq!(ring.current_idx, 0);
    assert_eq!(ring.current_label, 5);

    ring.execute_move();
    assert_eq!(ring.data, vec![5, 8, 3, 7, 4, 1, 9, 2, 6]);
    assert_eq!(ring.current_idx, 1);
    assert_eq!(ring.current_label, 8);
  }

  #[test]
  fn part_one_solved() {
    let input = "389125467";
    assert_eq!(solve(input), Some(Box::new("67384529".to_string())));

    let input = "167248359";
    assert_eq!(solve(input), Some(Box::new("38756249".to_string())));
  }

  #[test]
  fn part_two_solved() {
    assert_eq!(solve2("389125467"), Some(Box::new(149245887792)));

    assert_eq!(solve2("167248359"), Some(Box::new(21986479838)));
  }
}
