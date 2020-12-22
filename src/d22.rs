use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Deck = VecDeque<u64>;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let (mut deck1, mut deck2) = parse(input);

  while !deck1.is_empty() && !deck2.is_empty() {
    play_round(&mut deck1, &mut deck2);
  }

  let winning_deck = if deck1.is_empty() { deck2 } else { deck1 };
  let answer = calculate_answer(&winning_deck);

  Some(Box::new(answer))
}

fn play_round(deck1: &mut Deck, deck2: &mut Deck) {
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

fn calculate_answer(winning_deck: &Deck) -> u64 {
  winning_deck
    .iter()
    .rev()
    .enumerate()
    .fold(0, |acc, (idx, elem)| acc + ((idx + 1) as u64 * elem))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let (deck1, deck2) = parse(input);

  let (_winner, winning_deck) = play_game(deck1, deck2);
  let answer = calculate_answer(&winning_deck);

  Some(Box::new(answer))
}

/// Returns `true` if the first player won.
fn play_game(mut deck1: Deck, mut deck2: Deck) -> (bool, Deck) {
  let mut round_hashes = HashSet::new();

  while !deck1.is_empty() && !deck2.is_empty() {
    let round_hash = decks_hash(&deck1, &deck2);
    if round_hashes.contains(&round_hash) {
      return (true, deck1);
    } else {
      round_hashes.insert(round_hash);
    }

    play_recursive_round(&mut deck1, &mut deck2);
  }

  if deck1.is_empty() {
    (false, deck2)
  } else {
    (true, deck1)
  }
}

fn play_recursive_round(deck1: &mut Deck, deck2: &mut Deck) {
  let top_card1 = deck1.pop_front().unwrap();
  let top_card2 = deck2.pop_front().unwrap();

  let first_won = if deck1.len() >= top_card1 as usize && deck2.len() >= top_card2 as usize {
    let recursive_deck1 = deck1.iter().take(top_card1 as usize).copied().collect::<VecDeque<_>>();
    let recursive_deck2 = deck2.iter().take(top_card2 as usize).copied().collect::<VecDeque<_>>();

    play_game(recursive_deck1, recursive_deck2).0
  } else {
    top_card1 > top_card2
  };

  if first_won {
    deck1.push_back(top_card1);
    deck1.push_back(top_card2);
  } else {
    deck2.push_back(top_card2);
    deck2.push_back(top_card1);
  }
}

fn decks_hash(deck1: &Deck, deck2: &Deck) -> u64 {
  let mut hasher1 = DefaultHasher::new();
  deck1.hash(&mut hasher1);
  let deck1_hash = hasher1.finish();

  let mut hasher2 = DefaultHasher::new();
  deck2.hash(&mut hasher2);
  let deck2_hash = hasher2.finish();

  let mut hasher = DefaultHasher::new();
  ((deck1_hash, deck2_hash)).hash(&mut hasher);
  hasher.finish()
}

fn parse(input: &str) -> (Deck, Deck) {
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
    let input = fs::read_to_string("inputs/sample22").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(291)));

    let input = fs::read_to_string("inputs/d22").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(35836)));
  }
}
