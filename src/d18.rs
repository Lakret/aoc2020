pub fn solve(input: &str) -> Option<Box<u64>> {
  let result: u64 = input
    .trim_end()
    .split('\n')
    .map(|line| {
      let tokens = Token::parse(line);
      eval_tokens(&tokens)
    })
    .sum();

  Some(Box::new(result))
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

fn eval_tokens(tokens: &Vec<Token>) -> u64 {
  let mut ops_stack: Vec<Token> = vec![];
  let mut nums_stack: Vec<u64> = vec![];

  for token in tokens.iter() {
    match token {
      Num(x) => try_perform_op(&mut ops_stack, *x, &mut nums_stack),
      Parens(y) => {
        let x = eval_tokens(y);
        try_perform_op(&mut ops_stack, x, &mut nums_stack);
      }
      operation => ops_stack.push(operation.clone()),
    }
  }

  nums_stack.pop().unwrap()
}

fn try_perform_op(ops_stack: &mut Vec<Token>, value: u64, nums_stack: &mut Vec<u64>) {
  match ops_stack.pop() {
    None => nums_stack.push(value),
    Some(Plus) => {
      let prev = nums_stack.pop().unwrap();
      nums_stack.push(prev + value);
    }
    Some(Star) => {
      let prev = nums_stack.pop().unwrap();
      nums_stack.push(prev * value);
    }
    unexpected => panic!("ops_stack should only contain Plus & Star, got: {:?}", unexpected),
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  Num(u64),
  Plus,
  Star,
  Parens(Vec<Token>),
}

use Token::*;

impl Token {
  fn parse(input: &str) -> Vec<Token> {
    let mut lexems = input
      .trim_end()
      .split(' ')
      .flat_map(|lexem| {
        if lexem.starts_with("(") {
          let parens_count = lexem.chars().filter(|ch| *ch == '(').count();
          let mut num_and_parens = vec!["("; parens_count];

          let mut num = vec![lexem.trim_start_matches('(')];

          num_and_parens.append(&mut num);
          num_and_parens
        } else if lexem.ends_with(")") {
          let mut num_and_parens = vec![lexem.trim_end_matches(')')];

          let parens_count = lexem.chars().filter(|ch| *ch == ')').count();
          let mut parens = vec![")"; parens_count];

          num_and_parens.append(&mut parens);
          num_and_parens
        } else {
          vec![lexem]
        }
      })
      .rev()
      .collect::<Vec<_>>();

    parse_inner(&mut lexems)
  }
}

fn parse_inner(lexems: &mut Vec<&str>) -> Vec<Token> {
  let mut result = vec![];

  while let Some(lexem) = lexems.pop() {
    match lexem {
      "+" => result.push(Plus),
      "*" => result.push(Star),
      "(" => {
        let tokens_in_parens = parse_inner(lexems);
        result.push(Parens(tokens_in_parens))
      }
      ")" => return result,
      _ => {
        let num = parse_num(lexem);
        result.push(num)
      }
    }
  }

  result
}

fn parse_num(num: &str) -> Token {
  Num(num.parse::<u64>().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let tokens = Token::parse("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(
      tokens,
      vec![
        Num(1),
        Plus,
        Parens(vec![Num(2), Star, Num(3)]),
        Plus,
        Parens(vec![Num(4), Star, Parens(vec![Num(5), Plus, Num(6)])])
      ]
    );

    let tokens = Token::parse("1 + (((2 * 3) + 4) * 5)");
    assert_eq!(
      tokens,
      vec![
        Num(1),
        Plus,
        Parens(vec![
          Parens(vec![Parens(vec![Num(2), Star, Num(3)]), Plus, Num(4)]),
          Star,
          Num(5)
        ]),
      ]
    );
  }

  #[test]
  fn part_one_solved() {
    let sample = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(solve(sample), Some(Box::new(51)));

    let sample = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(solve(sample), Some(Box::new(71)));

    let sample = "2 * 3 + (4 * 5)";
    assert_eq!(solve(sample), Some(Box::new(26)));

    let sample = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(solve(sample), Some(Box::new(437)));

    let sample = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(solve(sample), Some(Box::new(12240)));

    let sample = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(solve(sample), Some(Box::new(13632)));

    let input = fs::read_to_string("inputs/d18").unwrap();
    assert_eq!(solve(&input), None);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d18").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
