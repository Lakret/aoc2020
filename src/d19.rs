use regex::Regex;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let (rules, messages) = parse(input);

  dbg!(&rules);
  dbg!(&messages);

  None
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

#[derive(Debug, Clone, PartialEq, Eq)]
// TODO: compile as regex?
enum Rule {
  Letter(String),
  Alternatives(Vec<Vec<usize>>),
}

use Rule::*;

fn parse(input: &str) -> (Vec<Rule>, Vec<String>) {
  match &input.trim_end().split("\n\n").collect::<Vec<_>>()[..] {
    &[rules, messages] => {
      let raw_rules = rules.split('\n').collect::<Vec<_>>();
      let mut rules = vec![Letter("".to_string()); raw_rules.len()];

      for rule in raw_rules {
        let parts = rule.split(": ").collect::<Vec<_>>();
        let idx = parts[0].parse::<usize>().unwrap();

        if parts[1].starts_with('"') {
          let letter = parts[1].strip_prefix("\"").and_then(|s| s.strip_suffix("\"")).unwrap();
          let rule = Letter(letter.to_string());

          rules[idx] = rule;
        } else {
          let alternatives = parts[1]
            .split(" | ")
            .map(|sequence| {
              sequence
                .split_ascii_whitespace()
                .map(|idx| idx.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

          rules[idx] = Alternatives(alternatives);
        }
      }

      let messages = messages.split('\n').map(|m| m.to_string()).collect::<Vec<_>>();
      (rules, messages)
    }
    unexpected => panic!("Unexpected input: {:?}.", unexpected),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/sample19").unwrap();
    let (rules, messages) = parse(&input);

    assert_eq!(
      rules,
      vec![
        Alternatives(vec![vec![4, 1, 5]]),
        Alternatives(vec![vec![2, 3], vec![3, 2]]),
        Alternatives(vec![vec![4, 4], vec![5, 5]]),
        Alternatives(vec![vec![4, 5], vec![5, 4]]),
        Letter("a".to_string()),
        Letter("b".to_string()),
      ]
    );

    assert_eq!(messages.len(), 5);
    assert_eq!(messages[0], "ababbb");
    assert_eq!(messages[4], "aaaabbb")
  }

  #[test]
  fn part_one_solved() {
    // let input = fs::read_to_string("inputs/d19").unwrap();
    // assert_eq!(solve(&input), None);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d19").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
