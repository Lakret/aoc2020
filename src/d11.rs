use std::collections::HashMap;

pub fn solve(input: &str) -> Option<i64> {
  solve_with_debug(input, false)
}

pub fn solve_debug(input: &str) -> Option<i64> {
  solve_with_debug(input, true)
}

pub fn solve2(input: &str) -> Option<i64> {
  solve2_with_debug(input, false)
}

pub fn solve2_debug(input: &str) -> Option<i64> {
  solve2_with_debug(input, true)
}

fn solve_with_debug(input: &str, debug: bool) -> Option<i64> {
  let mut layout = Layout::parse(input);
  if debug {
    layout.print();
  }

  let mut prev_layout = None;

  while prev_layout.is_none() || layout != prev_layout.unwrap() {
    prev_layout = Some(layout.clone());
    layout = layout.advance();

    if debug {
      layout.print();
    }
  }

  Some(layout.cells.values().filter(|cell| **cell == Occupied).count() as i64)
}

fn solve2_with_debug(input: &str, debug: bool) -> Option<i64> {
  let mut layout = Layout::parse(input);
  if debug {
    layout.print();
  }

  let mut prev_layout = None;

  while prev_layout.is_none() || layout != prev_layout.unwrap() {
    prev_layout = Some(layout.clone());
    layout = layout.advance2();

    if debug {
      layout.print();
    }
  }

  Some(layout.cells.values().filter(|cell| **cell == Occupied).count() as i64)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
  Floor,
  Empty,
  Occupied,
}

type Coords = (usize, usize);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// struct Coords2 {
//   row: usize,
//   col: usize,
// }

// impl Coords2 {

// }

#[derive(Debug, PartialEq, Eq, Clone)]
struct Layout {
  cells: HashMap<Coords, Cell>,
  width: usize,
  height: usize,
}

use Cell::*;

impl Layout {
  fn parse(input: &str) -> Layout {
    let mut cells = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (row_idx, line) in input.trim_end().split('\n').enumerate() {
      for (col_idx, ch) in line.chars().enumerate() {
        let cell = match ch {
          'L' => Empty,
          '.' => Floor,
          '#' => Occupied,
          _ => panic!("Unexpected input {}", ch),
        };

        cells.insert((row_idx, col_idx), cell);
        width = width.max(col_idx + 1);
      }
      height = height.max(row_idx + 1);
    }

    Layout { cells, width, height }
  }

  fn new(width: usize, height: usize) -> Layout {
    Layout {
      width,
      height,
      cells: HashMap::new(),
    }
  }

  // If a seat is empty (L) and there are no occupied seats adjacent to it,
  // the seat becomes occupied.
  // If a seat is occupied (#) and four or more seats adjacent to it are also occupied,
  // the seat becomes empty.
  // Otherwise, the seat's state does not change.
  fn advance(&self) -> Layout {
    let mut new_layout = Layout::new(self.width, self.height);

    for row_idx in 0..self.height {
      for col_idx in 0..self.width {
        let coords = (row_idx, col_idx);
        let seat = self.cells.get(&coords).unwrap();

        match seat {
          Floor => {
            new_layout.cells.insert(coords, Floor);
          }
          Empty => {
            if self.occupied_neighbors(coords) == 0 {
              new_layout.cells.insert(coords, Occupied);
            } else {
              new_layout.cells.insert(coords, Empty);
            }
          }
          Occupied => {
            if self.occupied_neighbors(coords) >= 4 {
              new_layout.cells.insert(coords, Empty);
            } else {
              new_layout.cells.insert(coords, Occupied);
            }
          }
        }
      }
    }

    new_layout
  }

  fn occupied_neighbors(&self, (row_idx, col_idx): Coords) -> usize {
    let deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    deltas
      .iter()
      .map(|(row_delta, col_delta)| (row_idx as i32 + row_delta, col_idx as i32 + col_delta))
      .filter_map(|(row_idx, col_idx)| {
        if row_idx >= 0 && col_idx >= 0 {
          self.cells.get(&(row_idx as usize, col_idx as usize))
        } else {
          None
        }
      })
      .filter(|cell| **cell == Occupied)
      .count()
  }

  fn advance2(&self) -> Layout {
    let mut new_layout = Layout::new(self.width, self.height);

    for row_idx in 0..self.height {
      for col_idx in 0..self.width {
        let coords = (row_idx, col_idx);
        let seat = self.cells.get(&coords).unwrap();

        match seat {
          Floor => {
            new_layout.cells.insert(coords, Floor);
          }
          Empty => {
            if self.visible_occupied_neighbors(coords) == 0 {
              new_layout.cells.insert(coords, Occupied);
            } else {
              new_layout.cells.insert(coords, Empty);
            }
          }
          Occupied => {
            if self.visible_occupied_neighbors(coords) >= 5 {
              new_layout.cells.insert(coords, Empty);
            } else {
              new_layout.cells.insert(coords, Occupied);
            }
          }
        }
      }
    }

    new_layout
  }

  fn visible_occupied_neighbors(&self, coords: Coords) -> usize {
    let deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut count = 0;
    for delta in deltas.iter() {
      let mut stop = false;
      let mut n = 1;

      while !stop {
        match self.delta_n(coords, delta, n) {
          None => stop = true,
          Some(visible_coords) => {
            match self.cells.get(&visible_coords) {
              None => stop = true,
              Some(Occupied) => {
                count += 1;
                stop = true;
              }
              Some(Empty) => {
                stop = true;
              }
              _ => (),
            }
            n += 1;
          }
        }
      }
    }
    count
  }

  fn delta_n(&self, (row_idx, col_idx): Coords, (row_delta, col_delta): &(i32, i32), n: i32) -> Option<Coords> {
    let (row_idx, col_idx) = (row_idx as i32 + row_delta * n, col_idx as i32 + col_delta * n);

    if row_idx >= 0 && col_idx >= 0 && row_idx < self.height as i32 && col_idx < self.width as i32 {
      Some((row_idx as usize, col_idx as usize))
    } else {
      None
    }
  }

  fn print(&self) {
    for row_idx in 0..self.height {
      for col_idx in 0..self.width {
        let ch = match self.cells.get(&(row_idx, col_idx)).unwrap() {
          Empty => 'L',
          Occupied => '#',
          Floor => '.',
        };

        print!("{}", ch);
      }
      println!();
    }

    println!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/sample11").unwrap();
    let layout = Layout::parse(&input);
    assert_eq!(layout.width, 10);
    assert_eq!(layout.height, 10);
    assert_eq!(layout.cells.keys().len(), 100);

    // corners
    assert_eq!(layout.cells.get(&(0, 0)), Some(&Empty));
    assert_eq!(layout.cells.get(&(0, 9)), Some(&Empty));
    assert_eq!(layout.cells.get(&(9, 0)), Some(&Empty));
    assert_eq!(layout.cells.get(&(9, 9)), Some(&Empty));

    assert_eq!(layout.cells.get(&(6, 0)), Some(&Floor));
    assert_eq!(layout.cells.get(&(6, 1)), Some(&Floor));
    assert_eq!(layout.cells.get(&(6, 2)), Some(&Empty));
    assert_eq!(layout.cells.get(&(6, 3)), Some(&Floor));
    assert_eq!(layout.cells.get(&(6, 4)), Some(&Empty));
    assert_eq!(layout.cells.get(&(6, 5)), Some(&Floor));
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample11").unwrap();
    assert_eq!(solve(&input), Some(37));

    let input = fs::read_to_string("inputs/d11").unwrap();
    assert_eq!(solve(&input), Some(2166));
  }

  #[test]
  fn visible_occupied_neighbors_works() {
    let sample = r#".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."#;

    let layout = Layout::parse(sample);
    assert_eq!(layout.width, 9);
    assert_eq!(layout.height, 9);
    assert_eq!(layout.cells.get(&(4, 2)), Some(&Occupied));
    assert_eq!(layout.cells.get(&(4, 3)), Some(&Empty));
    assert_eq!(layout.cells.get(&(4, 4)), Some(&Floor));

    assert_eq!(layout.visible_occupied_neighbors((4, 3)), 8);

    let sample = r#".............
.L.L.#.#.#.#.
............."#;
    let layout = Layout::parse(sample);
    assert_eq!(layout.width, 13);
    assert_eq!(layout.height, 3);

    assert_eq!(layout.visible_occupied_neighbors((1, 1)), 0);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/sample11").unwrap();
    assert_eq!(solve2(&input), Some(26));

    let input = fs::read_to_string("inputs/d11").unwrap();
    assert_eq!(solve2(&input), Some(1955));
  }
}
