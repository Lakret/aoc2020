use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn solve(input: &str) -> Option<i64> {
  let contained_in = Rules::parse(input).contained_in();
  let can_contain_shiny_gold = transitive_closure(&contained_in, &"shiny gold".to_string());

  Some(can_contain_shiny_gold.len() as i64)
}

pub fn solve2(input: &str) -> Option<i64> {
  let rules = Rules::parse(input);

  let mut to_satisfy = rules
    .inner
    .get(&"shiny gold".to_string())
    .unwrap()
    .iter()
    .flat_map(|contained| to_n_strings(contained))
    .collect::<Vec<_>>();
  let mut count = 0;

  while let Some(color) = to_satisfy.pop() {
    count += 1;

    if let Some(all_contained) = rules.inner.get(&color) {
      for contained in all_contained.iter() {
        let mut items = to_n_strings(contained);
        to_satisfy.append(&mut items);
      }
    }
  }

  Some(count)
}

fn to_n_strings(contained: &Contained) -> Vec<String> {
  vec![contained.color.to_string(); contained.count]
}

fn transitive_closure(contained_in: &ContainedIn, color: &Color) -> HashSet<Color> {
  let mut checked = HashSet::new();
  transitive_closure_inner(contained_in, &mut checked, vec![color]);

  checked.remove(color);
  checked
}

fn transitive_closure_inner(contained_in: &ContainedIn, checked: &mut HashSet<String>, colors: Vec<&Color>) {
  for color in colors.iter() {
    if !checked.contains(*color) {
      checked.insert(color.to_string());

      if let Some(colors) = contained_in.get(*color) {
        transitive_closure_inner(contained_in, checked, colors.iter().collect::<Vec<_>>());
      }
    }
  }
}

type Color = String;
type ContainedIn = HashMap<Color, HashSet<Color>>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Contained {
  color: Color,
  count: usize,
}

#[derive(Debug)]
struct Rules {
  inner: HashMap<Color, Vec<Contained>>,
}

impl Rules {
  fn parse(input: &str) -> Rules {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(?P<count>\d+) (?P<color>.*) (bags|bag)\.?").unwrap();
    }

    let mut inner = HashMap::new();
    for line in input.trim_end().split('\n') {
      match &line.split(" bags contain ").collect::<Vec<_>>()[..] {
        &[color, rules] => {
          if rules == "no other bags." {
            inner.insert(color.to_string(), vec![]);
          } else {
            for rule in rules.split(", ") {
              if let Some(rule) = RE.captures_iter(rule).next() {
                if let (Some(count), Some(contained_color)) = (rule.name("count"), rule.name("color")) {
                  let contained = Contained {
                    color: contained_color.as_str().to_string(),
                    count: count.as_str().parse::<usize>().unwrap(),
                  };

                  let all_contained = inner.entry(color.to_string()).or_insert(vec![]);
                  all_contained.push(contained);
                }
              }
            }
          }
        }
        _ => panic!("cannot parse line: {}", line),
      }
    }

    Rules { inner }
  }

  fn contained_in(&self) -> ContainedIn {
    let mut contained_in = HashMap::new();

    for (container, all_contained) in self.inner.iter() {
      for contained in all_contained {
        let containers = contained_in
          .entry(contained.color.to_string())
          .or_insert(HashSet::new());
        containers.insert(container.to_string());
      }
    }

    contained_in
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let sample_input = fs::read_to_string("inputs/sample7").unwrap();
    assert_eq!(solve(&sample_input), Some(4));

    let puzzle_input = fs::read_to_string("inputs/d7").unwrap();
    assert_eq!(solve(&puzzle_input), Some(226));
  }

  #[test]
  fn part_two_solved() {
    let sample_input = fs::read_to_string("inputs/sample7").unwrap();
    assert_eq!(solve2(&sample_input), Some(32));

    let puzzle_input = fs::read_to_string("inputs/d7").unwrap();
    assert_eq!(solve2(&puzzle_input), Some(9569));
  }

  #[test]
  fn parser_works() {
    let sample_input = fs::read_to_string("inputs/sample7").unwrap();
    let rules = Rules::parse(&sample_input);

    assert_eq!(rules.inner.keys().len(), 9);

    assert_eq!(
      rules.inner.get("light red"),
      Some(&vec![
        Contained {
          color: "bright white".to_string(),
          count: 1,
        },
        Contained {
          color: "muted yellow".to_string(),
          count: 2,
        },
      ])
    );

    assert_eq!(
      rules.inner.get("bright white"),
      Some(&vec![Contained {
        color: "shiny gold".to_string(),
        count: 1,
      },])
    );

    assert_eq!(rules.inner.get("faded blue"), Some(&vec![]));
    assert_eq!(rules.inner.get("dotted black"), Some(&vec![]));
  }
}
