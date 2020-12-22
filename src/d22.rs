use std::collections::VecDeque;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let (mut deck1, mut deck2) = parse(input);

  while !deck1.is_empty() && !deck2.is_empty() {
    let top_card1 = deck1.pop_front().unwrap();
    let top_card2 = deck2.pop_front().unwrap();

    if top_card1 > top_card2 {
      deck1.push_back(top_card1);
      deck1.push_back(top_card2);
    } else {
      deck2.push_back(top_card2);
      deck2.push_back(top_card1);
    }
  }

  let winning_deck = if deck1.is_empty() { deck2 } else { deck1 };

  let answer = winning_deck
    .iter()
    .rev()
    .enumerate()
    .fold(0, |acc, (idx, elem)| acc + ((idx + 1) as u64 * elem));

  Some(Box::new(answer))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  None
}

fn parse(input: &str) -> (VecDeque<u64>, VecDeque<u64>) {
  let mut decks = Vec::with_capacity(2);

  for deck in input.trim_end().split("\n\n") {
    let cards = deck
      .split('\n')
      .skip(1)
      .map(|card| card.parse::<u64>().unwrap())
      .collect::<VecDeque<_>>();
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
    let input = fs::read_to_string("inputs/sample22").unwrap();
    assert_eq!(solve(&input), Some(Box::new(306)));

    let input = fs::read_to_string("inputs/d22").unwrap();
    assert_eq!(solve(&input), Some(Box::new(32598)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d22").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
