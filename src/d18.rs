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
  dbg!(input);

  let digits = input.chars().take_while(|ch| ch.is_ascii_digit()).collect::<String>();
  if digits.is_empty() {
    None
  } else {
    let consumed = digits.len();
    let number = digits.parse::<u64>().unwrap();
    Some((Value(number), consumed))
  }
}

fn op(lhs: Expr, input: &str) -> Option<(Option<Expr>, usize)> {
  dbg!(input);

  if input.len() > 0 {
    if &input[..1] == ")" {
      Some((None, 0))
    } else {
      match &input[0..1] {
        "+" => match expr(&input[1..]) {
          Some((rhs, right_consumed)) => {
            println!("Got + rhs = {:?}", &rhs);
            let expr = Add(Box::new(lhs), Box::new(rhs));
            let consumed = right_consumed + 1;
            Some((Some(expr), consumed))
          }
          None => None,
        },
        "*" => match expr(&input[1..]) {
          Some((rhs, right_consumed)) => {
            println!("Got * rhs = {:?}", &rhs);
            let expr = Mul(Box::new(lhs), Box::new(rhs));
            let consumed = right_consumed + 1;
            Some((Some(expr), consumed))
          }
          None => None,
        },
        _ => None,
      }
    }
  } else {
    Some((None, 0))
  }
}

// (2 + 6) * 2 + 2 + 4
// Mul(
//   Add(Value(2), Value(6)),
//   Add(Value(2),
//     Add(Value(2), Value(4))))

fn expr(input: &str) -> Option<(Expr, usize)> {
  dbg!(input);

  if &input[0..1] == "(" {
    dbg!(&input[1..]);

    match expr(&input[1..]) {
      Some((inner, consumed)) => {
        dbg!(&inner);

        if input.len() > 1 + consumed && &input[consumed + 1..consumed + 2] == ")" {
          let consumed = consumed + 2;

          if input.len() > consumed {
            match op(inner.clone(), &input[consumed..]) {
              Some((Some(expr), right_consumed)) => Some((expr, consumed + right_consumed)),
              _ => Some((inner, consumed)),
            }
          } else {
            Some((inner, consumed))
          }
        } else {
          None
        }
      }
      None => None,
    }
  } else {
    match number(input) {
      Some((lhs, left_consumed)) => match op(lhs.clone(), &input[left_consumed..]) {
        Some((Some(expr), right_consumed)) => {
          dbg!(&expr);
          Some((expr, left_consumed + right_consumed))
        }
        Some((None, _)) => {
          dbg!("None in expr");
          Some((lhs, left_consumed))
        }
        None => None,
      },
      None => None,
    }
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
    let expr = Mul(
      Box::new(Value(3)),
      Box::new(Add(Box::new(Value(16)), Box::new(Value(5)))),
    );
    assert_eq!(parse(sample), Some(expr));

    let sample = "3 * 16 + 5 + 7";
    let expr = Mul(
      Box::new(Value(3)),
      Box::new(Add(
        Box::new(Value(16)),
        Box::new(Add(Box::new(Value(5)), Box::new(Value(7)))),
      )),
    );
    assert_eq!(parse(sample), Some(expr));

    let sample = "(2 + 6)";
    assert_eq!(parse(sample), Some(Add(Box::new(Value(2)), Box::new(Value(6)))));

    let sample = "(2 + 6) * 2";
    assert_eq!(
      parse(sample),
      Some(Mul(
        Box::new(Add(Box::new(Value(2)), Box::new(Value(6)))),
        Box::new(Value(2))
      ))
    );

    let sample = "(2 + 6) * 2 + 2 + 4";
    assert_eq!(
      parse(sample),
      Some(Mul(
        Box::new(Add(Box::new(Value(2)), Box::new(Value(6)))),
        Box::new(Add(
          Box::new(Value(2)),
          Box::new(Add(Box::new(Value(2)), Box::new(Value(4))))
        ))
      ))
    );

    let sample = "5 + (4 * (2 + 1))";
    assert_eq!(
      parse(sample),
      Some(Add(
        Box::new(Value(5)),
        Box::new(Mul(
          Box::new(Value(4)),
          Box::new(Add(Box::new(Value(2)), Box::new(Value(1))))
        ))
      ))
    )
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
