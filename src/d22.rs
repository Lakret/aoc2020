pub fn solve(input: &str) -> Option<Box<usize>> {
  let (deck1, deck2) = parse(input);
  dbg!(&deck1);
  dbg!(&deck2);

  None
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
  let mut decks = Vec::with_capacity(2);

  for deck in input.trim_end().split("\n\n") {
    let cards = deck
      .split('\n')
      .skip(1)
      .map(|card| card.parse::<u64>().unwrap())
      .collect::<Vec<_>>();
    decks.push(cards);
  }

  (decks.remove(0), decks.remove(0))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d22").unwrap();
    assert_eq!(solve(&input), None);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d22").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
