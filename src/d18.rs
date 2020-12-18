pub fn solve(input: &str) -> Option<Box<u64>> {
  let expr = parse(input);
  dbg!(expr);
  None
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
  Value(u64),
  Add(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
}

use Expr::*;

// returns Some((number, consumed_count)) or None
fn number(input: &str) -> Option<(Expr, usize)> {
  let digits = input.chars().take_while(|ch| ch.is_ascii_digit()).collect::<String>();
  if digits.is_empty() {
    None
  } else {
    let consumed = digits.len();
    let number = digits.parse::<u64>().unwrap();
    Some((Value(number), consumed))
  }
}

fn op(input: &str) -> Option<(Expr, usize)> {
  dbg!(input);

  match expr(input) {
    Some((lhs, left_consumed)) => match &input[left_consumed..(left_consumed + 1)] {
      "+" => match expr(&input[left_consumed + 1..]) {
        Some((rhs, right_consumed)) => {
          let expr = Add(Box::new(lhs), Box::new(rhs));
          let consumed = left_consumed + right_consumed + 1;
          Some((expr, consumed))
        }
        None => None,
      },
      "*" => match expr(&input[left_consumed + 1..]) {
        Some((rhs, right_consumed)) => {
          let expr = Mul(Box::new(lhs), Box::new(rhs));
          let consumed = left_consumed + right_consumed + 1;
          Some((expr, consumed))
        }
        None => None,
      },
      _ => None,
    },
    None => None,
  }
}

fn expr(input: &str) -> Option<(Expr, usize)> {
  match number(input) {
    num @ Some(_) => num,
    None => match op(input) {
      op @ Some(_) => op,
      None => None,
    },
  }
}

fn parse(input: &str) -> Option<Expr> {
  let input = input.chars().filter(|ch| *ch != ' ').collect::<String>();

  match expr(&input) {
    Some((expr, _consumed)) => Some(expr),
    None => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let sample = "128 + 6";
    let expr = Add(Box::new(Value(128)), Box::new(Value(6)));
    assert_eq!(parse(sample), Some(expr));

    let sample = "3 * 16";
    let expr = Mul(Box::new(Value(3)), Box::new(Value(16)));
    assert_eq!(parse(sample), Some(expr));

    let sample = "3 * 16 + 5";
    let expr = Add(
      Box::new(Mul(Box::new(Value(3)), Box::new(Value(16)))),
      Box::new(Value(5)),
    );
    assert_eq!(parse(sample), Some(expr));
  }

  #[test]
  fn part_one_solved() {
    // let sample = "1 + (2 * 3) + (4 * (5 + 6))";
    // assert_eq!(solve(sample), Some(Box::new(51)));

    // let input = fs::read_to_string("inputs/d18").unwrap();
    // assert_eq!(solve(&input), None);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d18").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
