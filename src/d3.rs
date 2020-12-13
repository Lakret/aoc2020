use std::collections::HashSet;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let tree_map = TreeMap::parse(input);
  let trees_count = tree_map.tree_count_on_slope(3, 1);
  Some(Box::new(trees_count))
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  let tree_map = TreeMap::parse(input);

  let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
  let answer = slopes.into_iter().fold(1, |answer, (right, down)| {
    answer * tree_map.tree_count_on_slope(right, down)
  });

  Some(Box::new(answer))
}

#[derive(Debug)]
struct TreeMap {
  trees: HashSet<(usize, usize)>,
  width: usize,
  height: usize,
}

impl TreeMap {
  pub fn parse(input: &str) -> TreeMap {
    let mut trees = HashSet::new();
    let mut width = None;
    let mut height = None;

    // y goes from top to bottom, starting from 0
    for (y, line) in input.trim_end().split('\n').enumerate() {
      if width == None {
        width = Some(line.len());
      }
      height = Some(y + 1);

      for (x, char) in line.chars().enumerate() {
        // x goes from left to right, starting from 0
        if char == '#' {
          trees.insert((x, y));
        }
      }
    }

    let width = width.expect("empty input is not valid");
    let height = height.expect("empty input is not valid");
    TreeMap { trees, width, height }
  }

  pub fn is_tree(&self, (x, y): (usize, usize)) -> bool {
    // emulate copying everything to the right infinite amount of times
    let x = x % self.width;

    self.trees.contains(&(x, y))
  }

  pub fn move_with_slope(&self, (x, y): (usize, usize), right: usize, down: usize) -> Option<(usize, usize)> {
    let x = x + right;
    let y = y + down;

    if y < self.height {
      Some((x, y))
    } else {
      None
    }
  }

  pub fn tree_count_on_slope(&self, right: usize, down: usize) -> usize {
    let mut trees_count = 0;
    let mut position = Some((0, 0));
    while let Some(curr_position) = position {
      if self.is_tree(curr_position) {
        trees_count += 1;
      }

      position = self.move_with_slope(curr_position, right, down);
    }

    trees_count
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/d3").expect("cannot read input for day 3");

    let parsed = TreeMap::parse(&input);
    assert_eq!(parsed.width, 31);
    assert_eq!(parsed.height, 323);

    assert!(parsed.trees.contains(&(8, 105)));
    assert!(parsed.trees.contains(&(0, 105)));
    assert!(parsed.trees.contains(&(30, 105)));
    assert!(parsed.trees.contains(&(26, 322)));

    assert!(!parsed.trees.contains(&(9, 105)));
    assert!(!parsed.trees.contains(&(7, 105)));
    assert!(!parsed.trees.contains(&(29, 105)));
    assert!(!parsed.trees.contains(&(0, 0)));
    assert!(!parsed.trees.contains(&(0, 322)));
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d3").expect("cannot read input for day 3");

    let trees_count = solve(&input);
    assert_eq!(trees_count, Some(Box::new(189)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d3").expect("cannot read input for day 3");

    let answer = solve2(&input);
    assert_eq!(answer, Some(Box::new(1718180100)));
  }
}
