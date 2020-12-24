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
  let tiles = get_arrangement(input);

  let answer = count_black_tiles(&tiles);
  Some(Box::new(answer))
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  let tiles = get_arrangement(input);

  let tiles = advance(tiles, 100);

  let answer = count_black_tiles(&tiles);
  Some(Box::new(answer))
}

fn get_arrangement(input: &str) -> Tiles {
  let paths = input
    .trim_end()
    .split('\n')
    .map(|path| Direction::parse(path))
    .collect::<Vec<_>>();

  let mut tiles = HashMap::new();

  for path in &paths {
    flip(&mut tiles, path);
  }

  tiles
}

fn count_black_tiles(tiles: &Tiles) -> usize {
  tiles.iter().filter(|(_coords, color)| **color).count()
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

fn add(coords1: Coords, coords2: Coords) -> Coords {
  let (x1, y1, z1) = coords1;
  let (x2, y2, z2) = coords2;

  (x1 + x2, y1 + y2, z1 + z2)
}

fn advance(tiles: Tiles, moves: u32) -> Tiles {
  let mut prev = tiles;
  complement(&mut prev);

  let deltas = [(1, -1, 0), (1, 0, -1), (0, 1, -1), (-1, 1, 0), (-1, 0, 1), (0, -1, 1)];
  // neighbour of the current tile coordinates together with presence and color.
  let mut neighbours = Vec::with_capacity(6);

  for _ in 0..moves {
    let mut next = HashMap::new();

    for (&coords, &tile) in prev.iter() {
      neighbours.clear();
      for delta in deltas.iter() {
        let neighbour_coords = add(coords, *delta);
        let is_black_and_present = prev.get(&neighbour_coords).map(|tile| *tile);
        neighbours.push((neighbour_coords, is_black_and_present));
      }

      let mut black_neighbours = 0;
      for (coords, presence_and_color) in neighbours.iter() {
        match presence_and_color {
          Some(true) => black_neighbours += 1,
          Some(false) => (),
          None => {
            // if a missing neighbour is found, add it
            next.insert(*coords, false);
          }
        }
      }

      if tile && (black_neighbours == 0 || black_neighbours > 2) {
        next.insert(coords, false);
      } else if !tile && black_neighbours == 2 {
        next.insert(coords, true);
      } else {
        next.insert(coords, tile);
      }
    }

    prev = next;
  }

  prev
}

// adds missing tiles in the occupied zone borders
fn complement(tiles: &mut Tiles) {
  let max_x = tiles.keys().map(|(x, _, _)| x.abs()).max().unwrap() + 1;
  let max_y = tiles.keys().map(|(_, y, _)| y.abs()).max().unwrap() + 1;
  let max_z = tiles.keys().map(|(_, _, z)| z.abs()).max().unwrap() + 1;

  for x in -max_x..=max_x {
    for y in -max_y..=max_y {
      for z in -max_z..=max_z {
        // invariant for hexmap's cube coordinates
        if x + y + z == 0 {
          if !tiles.contains_key(&(x, y, z)) {
            tiles.insert((x, y, z), false);
          }
        }
      }
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
    let input = fs::read_to_string("inputs/sample24").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(2208)));

    let input = fs::read_to_string("inputs/d24").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(3937)));
  }
}
