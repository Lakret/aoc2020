use std::collections::HashMap;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let tiles = parse(input);
  let result = arrange(&tiles);

  None
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  None
}

fn arrange(tiles: &HashMap<u64, Tile>) -> Vec<Vec<u64>> {
  let size = (tiles.values().len() as f64).sqrt() as u64;

  // domains
  let row_range = 0..size;
  let col_range = 0..size;
  let rotation_range = 0..4;
  let flip_range = false..true;

  // TODO: initialize a random initial arrangement
  // TODO: use min-conflicts and `neighbours_constraint` to find a solution
  let positions: HashMap<u64, Position> = HashMap::new();

  todo!()
}

fn neighbours_constraint(tiles: &HashMap<u64, Tile>, tile1: u64, pos1: &Position, tile2: u64, pos2: &Position) -> bool {
  let first_tile = &tiles[&tile1];
  let second_tile = &tiles[&tile2];

  if pos1.row == pos2.row && pos1.col == pos2.col && tile1 == tile2 {
    // same cell cannot be occupied by two different tiles
    false
  } else if pos1.row == pos2.row {
    // neighbours on the same rows should have a matching edge
    if pos1.col < pos2.col {
      // ..., tile1, tile2, ... arrangement
      first_tile.right_edge(pos1) == second_tile.left_edge(pos2)
    } else {
      neighbours_constraint(tiles, tile2, pos2, tile1, pos1)
    }
  } else if pos1.col == pos2.col {
    // neighbours across rows should have a matching edge
    if pos1.row < pos2.row {
      //       ...
      // ..., tile1, ...
      // ..., tile2, ...
      //       ...
      first_tile.bottom_edge(pos1) == second_tile.top_edge(pos2)
    } else {
      neighbours_constraint(tiles, tile2, pos2, tile1, pos1)
    }
  } else {
    // no rule can be broken by tiles without a common edge
    true
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
  raw: Vec<String>,
  // 0 - top, 1 - right, 2 - bottom, 3 - left
  edges: Vec<String>,
}

/// Rotations are counterclockwise, i.e.:
/// 0 - no rotation, 1 - 90 left, 2 - 180, 3 - 270 left.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Position {
  row: usize,
  col: usize,
  rotation: usize,
  flip_vertical: bool,
  flip_horizontal: bool,
}

impl Position {
  /// Returns an array expressing edge_id and flip mappings for edges when
  /// `Position` is applied:
  ///
  /// `[ (use_edge_with_this_id_instead_of_edge_0, reverse_chars), ... ]`
  ///
  /// where edge_ids are in order `0 - top, 1 - right, 2 - bottom, 3 - left`.
  fn edge_id_and_flip_map(&self) -> ([(usize, bool); 4]) {
    let result = match self.rotation {
      0 => [(0, false), (1, false), (2, false), (3, false)],
      1 => [(1, false), (2, true), (3, false), (0, true)],
      2 => [(2, true), (3, false), (0, true), (1, true)],
      3 => [(3, true), (0, false), (1, true), (2, false)],
      _ => panic!("Impossible rotation {}.", self.rotation),
    };

    let result = if self.flip_horizontal {
      [
        (result[0].0, !result[0].1),
        result[3],
        (result[2].0, !result[2].1),
        result[1],
      ]
    } else {
      result
    };

    if self.flip_vertical {
      [
        result[2],
        (result[1].0, !result[1].1),
        result[0],
        (result[3].0, !result[3].1),
      ]
    } else {
      result
    }
  }
}

impl Tile {
  fn top_edge(&self, pos: &Position) -> String {
    self.edge_with_id_and_pos(0, pos)
  }

  fn right_edge(&self, pos: &Position) -> String {
    self.edge_with_id_and_pos(1, pos)
  }

  fn bottom_edge(&self, pos: &Position) -> String {
    self.edge_with_id_and_pos(2, pos)
  }

  fn left_edge(&self, pos: &Position) -> String {
    self.edge_with_id_and_pos(3, pos)
  }

  fn edge_with_id_and_pos(&self, edge_id: usize, pos: &Position) -> String {
    let (edge_id, reverse) = pos.edge_id_and_flip_map()[edge_id];
    if reverse {
      self.edges[edge_id].chars().rev().collect()
    } else {
      self.edges[edge_id].clone()
    }
  }
}

fn parse(input: &str) -> HashMap<u64, Tile> {
  let mut tiles = HashMap::new();

  for raw_tile in input.trim_end().split("\n\n") {
    let lines = raw_tile.split('\n').collect::<Vec<_>>();

    let id = lines[0]
      .strip_prefix("Tile ")
      .and_then(|header| header.strip_suffix(':'))
      .and_then(|id| id.parse::<u64>().ok())
      .unwrap();

    let raw = lines[1..].into_iter().map(|line| line.to_string()).collect::<Vec<_>>();

    let top = raw[0].clone();
    let right = raw.iter().map(|row| row.chars().last().unwrap().clone()).collect();
    let bottom = raw.last().unwrap().clone();
    let left = raw.iter().map(|row| row.chars().take(1).next().unwrap()).collect();
    let edges = vec![top, right, bottom, left];

    let tile = Tile { raw, edges };
    tiles.insert(id, tile);
  }

  tiles
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_and_edge_extractors_work() {
    let input = fs::read_to_string("inputs/sample20").unwrap();
    let tiles = parse(&input);

    let tile_2971 = tiles.get(&2971).unwrap();
    let top_edge = "..#.#....#";
    let right_edge = "#...##.#.#";
    let bottom_edge = "...#.#.#.#";
    let left_edge = ".###..#...";

    assert_eq!(&tile_2971.edges[0], top_edge);
    assert_eq!(&tile_2971.edges[1], right_edge);
    assert_eq!(&tile_2971.edges[2], bottom_edge);
    assert_eq!(&tile_2971.edges[3], left_edge);

    // default positions changes nothing
    let default_position = Position::default();
    assert_eq!(tile_2971.top_edge(&default_position), top_edge);
    assert_eq!(tile_2971.right_edge(&default_position), right_edge);
    assert_eq!(tile_2971.bottom_edge(&default_position), bottom_edge);
    assert_eq!(tile_2971.left_edge(&default_position), left_edge);

    // verify that edges are mapped correctly after rotations
    let rotate270 = Position {
      rotation: 3,
      ..Position::default()
    };
    assert_eq!(
      tile_2971.top_edge(&rotate270),
      left_edge.chars().rev().collect::<String>()
    );
    assert_eq!(tile_2971.right_edge(&rotate270), top_edge);
    assert_eq!(
      tile_2971.bottom_edge(&rotate270),
      right_edge.chars().rev().collect::<String>()
    );
    assert_eq!(tile_2971.left_edge(&rotate270), bottom_edge);

    // should be the same as rotate270
    let rotate90_flip_hor_and_ver = Position {
      rotation: 1,
      flip_horizontal: true,
      flip_vertical: true,
      ..Position::default()
    };
    assert_eq!(
      rotate90_flip_hor_and_ver.edge_id_and_flip_map(),
      rotate270.edge_id_and_flip_map()
    );
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample20").unwrap();
    assert_eq!(solve(&input), Some(Box::new(20899048083289)));

    // let input = fs::read_to_string("inputs/d20").unwrap();
    // assert_eq!(solve(&input), None);
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d20").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
