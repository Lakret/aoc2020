use std::collections::HashMap;

// Cool reference about hexagonal coordinates:
// https://www.redblobgames.com/grids/hexagons/.
// We'll use cube coordinates here.
type Coords = (i32, i32, i32);

// HashMap instead of HashSet, since we need to track
// both presence & color. `false` is white, `true` is black.
type Tiles = HashMap<Coords, bool>;

type Path = Vec<Direction>;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let paths = input
    .trim_end()
    .split('\n')
    .map(|path| Direction::parse(path))
    .collect::<Vec<_>>();

  let mut tiles = HashMap::new();
  for path in &paths {
    flip(&mut tiles, path);
  }
  dbg!(&tiles);

  let black_tiles_count = tiles.iter().filter(|(_coords, color)| **color).count();
  Some(Box::new(black_tiles_count))
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

fn flip(tiles: &mut Tiles, path: &Path) {
  let mut coords = (0, 0, 0);

  for direction in path.iter() {
    coords = direction.step(coords);
  }

  let tile = tiles.entry(coords).or_insert(false);
  *tile = !*tile;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest,
  NorthEast,
}

use Direction::*;

impl Direction {
  fn parse(path: &str) -> Vec<Direction> {
    let mut directions = vec![];
    let mut offset = 0;

    while offset < path.len() {
      match &path[offset..offset + 1] {
        "e" => {
          directions.push(East);
          offset += 1;
        }
        "w" => {
          directions.push(West);
          offset += 1;
        }
        "s" => match &path[offset + 1..offset + 2] {
          "e" => {
            directions.push(SouthEast);
            offset += 2;
          }
          "w" => {
            directions.push(SouthWest);
            offset += 2;
          }
          _ => unreachable!(),
        },
        "n" => match &path[offset + 1..offset + 2] {
          "e" => {
            directions.push(NorthEast);
            offset += 2;
          }
          "w" => {
            directions.push(NorthWest);
            offset += 2;
          }
          _ => unreachable!(),
        },
        _ => unreachable!(),
      }
    }

    directions
  }

  fn step(&self, coords: Coords) -> Coords {
    let (x, y, z) = coords;

    match self {
      East => (x + 1, y - 1, z),
      SouthEast => (x, y - 1, z + 1),
      SouthWest => (x - 1, y, z + 1),
      West => (x - 1, y + 1, z),
      NorthWest => (x, y + 1, z - 1),
      NorthEast => (x + 1, y, z - 1),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample24").unwrap();
    assert_eq!(solve(&input), Some(Box::new(10)));

    let input = fs::read_to_string("inputs/d24").unwrap();
    assert_eq!(solve(&input), Some(Box::new(375)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d24").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
