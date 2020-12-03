use regex::Regex;

pub fn solve(input: &str) -> Option<i64> {
  let passwords = parse(input);
  let valid_count = passwords
    .into_iter()
    .filter(|password| password.is_valid())
    .count();

  Some(valid_count as i64)
}

pub fn solve2(input: &str) -> Option<i64> {
  let passwords = parse(input);
  let valid_count = passwords
    .into_iter()
    .filter(|password| password.is_valid2())
    .count();

  Some(valid_count as i64)
}

#[derive(Debug, PartialEq, Eq)]
struct Password {
  policy: Policy,
  password: String,
}

impl Password {
  pub fn is_valid(&self) -> bool {
    let policy_letter_count = self
      .password
      .chars()
      .filter(|letter| *letter == self.policy.letter)
      .count();

    policy_letter_count >= self.policy.min
      && policy_letter_count <= self.policy.max
  }

  pub fn is_valid2(&self) -> bool {
    let password_chars = self.password.chars().collect::<Vec<_>>();
    if let (Some(first), Some(second)) = (
      password_chars.get(self.policy.min - 1),
      password_chars.get(self.policy.max - 1),
    ) {
      let first_matches = *first == self.policy.letter;
      let second_matches = *second == self.policy.letter;

      // a xor b = (a and not b) or (not a and b)
      return (first_matches && !second_matches)
        || (!first_matches && second_matches);
    }

    false
  }
}

#[derive(Debug, PartialEq, Eq)]
struct Policy {
  letter: char,
  min: usize,
  max: usize,
}

fn parse(input: &str) -> Vec<Password> {
  let re = Regex::new(
    r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)\n?$",
  )
  .unwrap();

  input
    .trim_end()
    .split('\n')
    .map(|line| match re.captures_iter(line).next() {
      Some(cap) => {
        match (
          cap.name("min"),
          cap.name("max"),
          cap.name("letter"),
          cap.name("password"),
        ) {
          (Some(min), Some(max), Some(letter), Some(password)) => {
            let min = min.as_str().parse::<usize>().unwrap();
            let max = max.as_str().parse::<usize>().unwrap();
            let letter = letter.as_str().parse::<char>().unwrap();
            let password = password.as_str().to_string();

            Password {
              policy: Policy { min, max, letter },
              password,
            }
          }
          _ => panic!("Invalid password format: {:?}", cap),
        }
      }
      None => panic!("non-matching line in input: {}", line),
    })
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/d2").expect("can read day 2 input");
    let passwords = parse(&input);

    assert_eq!(
      passwords[0],
      Password {
        policy: Policy {
          min: 8,
          max: 9,
          letter: 'n'
        },
        password: "nnnnnnnnn".to_string()
      }
    );

    assert_eq!(
      passwords[999],
      Password {
        policy: Policy {
          min: 4,
          max: 15,
          letter: 'b'
        },
        password: "fctbwzqnwbnvqbqlb".to_string()
      }
    );
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d2").expect("can read day 2 input");
    assert_eq!(solve(&input), Some(538));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d2").expect("can read day 2 input");
    assert_eq!(solve2(&input), Some(489));
  }
}
