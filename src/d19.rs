use regex::Regex;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let (rules, messages) = parse(input);
  let expanded = expand(&rules, 0);
  let expanded = format!("^{}$", expanded);
  let regex = Regex::new(&expanded).unwrap();

  let matching = messages.iter().filter(|message| regex.is_match(message)).count();
  Some(Box::new(matching))
}

/// New rules:
///
/// `8: 42` -(changed to)-> `8: 42 | 42 8`
/// (i.e., 8 is now r`42+`).
///
/// `11: 42 31` -(changed to)-> `11: 42 31 | 42 11 31`
/// (i.e., 11 is now r`42 (recurse 11) 31`)
///
/// 0: 8 11 is the entrypoint, and 8 and 11 are not used in any other rules.
///
/// Which means that the whole match condition can be expressed as:
///
/// 0 = r`^42+ (?<x>42 (recurse x) 31)$`
///
/// And we can test for a match with:
///
///   - match r`^.* (31)+$` (that's `starts_42_ends_31` regex);
///   - count number of 31 regex matches (via `rule31` regex)
///   - we know that the remaining string should match at least the same number of 42 matches
///     (`rule42` regex) in the beginning + at least one more time.
///     If that is satisfied, the string matches.
pub fn solve2(input: &str) -> Option<Box<usize>> {
  let (rules, messages) = parse(input);
  let rule42_non_capturing = expand(&rules, 42);
  let capturing_rule42 = format!("({})", &rule42_non_capturing);
  let rule42 = Regex::new(&capturing_rule42).unwrap();

  let rule31_non_capturing = expand(&rules, 31);
  let capturing_rule31 = format!("({})", &rule31_non_capturing);
  let rule31 = Regex::new(&capturing_rule31).unwrap();

  let starts_42_ends_31 = format!(
    r"^(?P<start_42>(?:{})+)(?P<end_31>(?:{})+)$",
    rule42_non_capturing, rule31_non_capturing
  );
  let start_and_end31 = Regex::new(&starts_42_ends_31).unwrap();

  let matching = messages
    .iter()
    .filter(|message| matches_new_rules(&start_and_end31, &rule31, &rule42, message))
    .count();
  Some(Box::new(matching))
}

fn matches_new_rules(starts_42_ends_31: &Regex, rule31: &Regex, rule42: &Regex, message: &str) -> bool {
  if starts_42_ends_31.is_match(message) {
    let capture = starts_42_ends_31.captures_iter(message).collect::<Vec<_>>();

    if capture.len() == 1 {
      let capture = capture.first().unwrap();
      let count_31 = rule31.find_iter(&capture["end_31"]).count();
      let count_42 = rule42.find_iter(&capture["start_42"]).count();

      count_42 >= count_31 + 1
    } else {
      false
    }
  } else {
    false
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
  Letter(String),
  Alternatives(Vec<Vec<usize>>),
}

use Rule::*;

/// Expands `rule` at `idx` to a non-capturing Regex string literal.
fn expand(rules: &Vec<Rule>, idx: usize) -> String {
  expand_inner(rules, &rules[idx])
}

fn expand_inner(rules: &Vec<Rule>, rule: &Rule) -> String {
  match rule {
    Letter(x) => x.to_string(),
    Alternatives(alternatives) => {
      let expanded_alternatives = alternatives
        .iter()
        .map(|alternative| {
          alternative
            .iter()
            .map(|idx| expand_inner(rules, &rules[*idx]))
            .collect::<Vec<_>>()
            .join("")
        })
        .collect::<Vec<_>>();

      if expanded_alternatives.len() > 1 {
        format!("(?:{})", expanded_alternatives.join("|"))
      } else {
        expanded_alternatives.join("")
      }
    }
  }
}

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
    let input = fs::read_to_string("inputs/sample19").unwrap();
    assert_eq!(solve(&input), Some(Box::new(2)));

    let input = fs::read_to_string("inputs/d19").unwrap();
    assert_eq!(solve(&input), Some(Box::new(224)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/sample19_2").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(12)));

    let input = fs::read_to_string("inputs/d19").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(436)));
  }
}
