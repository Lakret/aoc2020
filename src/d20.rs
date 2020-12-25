use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs;

type TilesMap = HashMap<u64, Tile>;
type Coords = (usize, usize);
type BacktrackAssignment = HashMap<Coords, (u64, Transform)>;

pub fn solve(input: &str) -> Option<Box<u64>> {
  let tiles = Tile::parse(input);
  let corners = get_corners(&tiles);

  let answer = corners.iter().product();
  Some(Box::new(answer))
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  let tiles = Tile::parse(input);

  let assignment = backtrack(&tiles);
  // dbg!(&assignment);
  // TODO: build & save image
  // TODO: find the monster

  match assignment {
    Some(assignment) => {
      let coords = [(0, 0), (0, 1)];
      for coord in coords.iter() {
        println!("{:?}: {:?}", coord, assignment.get(coord));
      }
    }
    None => (),
  }

  None
}

fn build_image(assignment: &BacktrackAssignment) {
  // TODO:
}

fn backtrack(tiles: &TilesMap) -> Option<BacktrackAssignment> {
  let size = size(tiles);
  let corner_coords = [(0, 0), (0, size - 1), (size - 1, 0), (size - 1, size - 1)]
    .iter()
    .copied()
    .collect::<HashSet<_>>();
  let corners = get_corners(tiles).into_iter().collect::<HashSet<_>>();

  let mut edge_nums_to_tile_ids = HashMap::new();
  let mut tile_ids_to_edge_nums = HashMap::new();

  for tile_id in tiles.keys().copied() {
    let edge_nums = tiles.get(&tile_id).unwrap().possible_edges_as_nums();
    tile_ids_to_edge_nums.insert(tile_id, edge_nums.clone());

    for edge_num in edge_nums {
      let edge_num_tiles = edge_nums_to_tile_ids.entry(edge_num).or_insert(HashSet::new());
      edge_num_tiles.insert(tile_id);
    }
  }

  // preceed from the first solution we've got (see `inputs/d20_solved`)
  let mut preseeded_assignment = HashMap::new();
  if tiles.contains_key(&2011) && tiles.contains_key(&3793) {
    preseeded_assignment.insert(
      (0, 0),
      (
        2011,
        Transform {
          rotation: 0,
          flip_vertical: false,
          flip_horizontal: false,
        },
      ),
    );
    preseeded_assignment.insert(
      (0, 1),
      (
        3793,
        Transform {
          rotation: 0,
          flip_vertical: true,
          flip_horizontal: true,
        },
      ),
    );
  }

  let mut unassigned_cells = vec![];
  for row_idx in 0..size {
    for col_idx in 0..size {
      if !preseeded_assignment.contains_key(&(row_idx, col_idx)) {
        unassigned_cells.push((row_idx, col_idx));
      }
    }
  }

  let mut unassigned_tile_ids = tiles.keys().copied().collect::<HashSet<_>>();
  if preseeded_assignment.len() == 2 {
    unassigned_tile_ids.remove(&2011);
    unassigned_tile_ids.remove(&3793);
  }

  let mut seen = HashSet::new();

  backtrack_inner(
    tiles,
    &corners,
    &corner_coords,
    &tile_ids_to_edge_nums,
    &edge_nums_to_tile_ids,
    preseeded_assignment,
    &unassigned_cells[..],
    unassigned_tile_ids,
    &mut seen,
  )
}

fn backtrack_inner(
  tiles: &TilesMap,
  corners: &HashSet<u64>,
  corner_coords: &HashSet<Coords>,
  tile_ids_to_edge_nums: &HashMap<u64, HashSet<u64>>,
  edge_nums_to_tile_ids: &HashMap<u64, HashSet<u64>>,
  assignment: BacktrackAssignment,
  unassigned_cells: &[Coords],
  unassigned_tile_ids: HashSet<u64>,
  seen: &mut HashSet<u64>,
) -> Option<BacktrackAssignment> {
  let all_transforms = Transform::all_transforms();

  while let Some((next_cell, rest_cells)) = unassigned_cells.split_first() {
    let possible_tile_ids = if corner_coords.contains(next_cell) {
      // corners has already been inferred, we just need to look at those that are not yet assigned
      corners
        .iter()
        .filter(|corner_tile_id| unassigned_tile_ids.contains(corner_tile_id))
        .collect::<Vec<_>>()
    } else {
      // infer all possible edges that already assigned neighbours
      // of the cell we're looking for an assignment of have. Valid assignments should match those.
      let matching_neighbour_edges = get_neighbours(*next_cell)
        .iter()
        .filter_map(|neighbour_coords| assignment.get(neighbour_coords))
        .filter_map(|(neighbour_tile_id, _transform)| tiles.get(neighbour_tile_id))
        .map(|tile| tile.possible_edges_as_nums())
        .flatten()
        .collect::<HashSet<_>>();

      if matching_neighbour_edges.is_empty() {
        return None;
      } else {
        unassigned_tile_ids
          .iter()
          .filter(|tile_id| {
            let tile = tiles.get(tile_id).unwrap();
            tile
              .possible_edges_as_nums()
              .intersection(&matching_neighbour_edges)
              .count()
              > 0
          })
          .collect::<Vec<_>>()
      }
    };

    for &tile_id in possible_tile_ids {
      let tile = tiles.get(&tile_id).unwrap();

      for transform in &all_transforms {
        if fits(tiles, &assignment, next_cell, tile, transform) {
          let mut candidate = assignment.clone();
          candidate.insert(*next_cell, (tile_id, *transform));

          let candidate_hash = hash_assignment(&candidate);
          if seen.contains(&candidate_hash) {
            continue;
          }

          let mut candidate_unassigned_tile_ids = unassigned_tile_ids.clone();
          candidate_unassigned_tile_ids.remove(&tile_id);

          match backtrack_inner(
            tiles,
            corners,
            corner_coords,
            tile_ids_to_edge_nums,
            edge_nums_to_tile_ids,
            candidate,
            rest_cells,
            candidate_unassigned_tile_ids,
            seen,
          ) {
            Some(assignment) => return Some(assignment),
            None => {
              // prevent looping when we arrive at the same position
              seen.insert(candidate_hash);
              continue;
            }
          }
        }
      }
    }
  }

  Some(assignment)
}

fn hash_assignment(assignment: &BacktrackAssignment) -> u64 {
  let mut hasher = DefaultHasher::new();

  for (k, v) in assignment.iter() {
    k.hash(&mut hasher);
    v.hash(&mut hasher);
  }

  hasher.finish()
}

fn fits(tiles: &TilesMap, assignment: &BacktrackAssignment, cell: &Coords, tile: &Tile, transform: &Transform) -> bool {
  let (row_idx, col_idx) = *cell;

  if row_idx > 0 {
    let top_tile_coords = (row_idx - 1, col_idx);

    if let Some((top_tile_id, top_tile_transform)) = assignment.get(&top_tile_coords) {
      if let Some(top_tile) = tiles.get(top_tile_id) {
        if tile.top_edge(transform) != top_tile.bottom_edge(top_tile_transform) {
          return false;
        }
      }
    }
  }

  if col_idx > 0 {
    let left_tile_coords = (row_idx, col_idx - 1);

    if let Some((left_tile_id, left_tile_transform)) = assignment.get(&left_tile_coords) {
      if let Some(left_tile) = tiles.get(left_tile_id) {
        if tile.left_edge(transform) != left_tile.right_edge(left_tile_transform) {
          return false;
        }
      }
    }
  }

  true
}

fn get_neighbours(cell: Coords) -> Vec<Coords> {
  let (row_idx, col_idx) = cell;
  let mut neighbours = Vec::with_capacity(2);

  // top neighbour
  if row_idx > 0 {
    neighbours.push((row_idx - 1, col_idx));
  }

  // left neighbour
  if col_idx > 0 {
    neighbours.push((row_idx, col_idx - 1));
  }

  neighbours
}

fn size(tiles: &TilesMap) -> usize {
  (tiles.values().len() as f64).sqrt() as usize
}

fn get_corners(tiles: &TilesMap) -> Vec<u64> {
  let match_counts = count_edge_match_counts(&tiles);
  match_counts
    .into_iter()
    .take(4)
    .map(|(tile_id, _match_count)| tile_id)
    .collect()
}

fn count_edge_match_counts(tiles: &TilesMap) -> Vec<(u64, usize)> {
  let mut tile_id_to_edge_nums = HashMap::new();

  for (&tile_id, tile) in tiles.iter() {
    let edge_nums = tile.possible_edges_as_nums();
    tile_id_to_edge_nums.insert(tile_id, edge_nums);
  }

  let mut tile_id_to_match_counts = tile_id_to_edge_nums
    .iter()
    .map(|(tile_id, edge_nums)| {
      let mut matches_count = 0;

      for (another_tile_id, another_edge_nums) in tile_id_to_edge_nums.iter() {
        if another_tile_id != tile_id {
          matches_count += another_edge_nums.intersection(edge_nums).count()
        }
      }

      (*tile_id, matches_count)
    })
    .collect::<Vec<_>>();

  tile_id_to_match_counts
    .sort_by(|(_tile_id1, match_count1), (_tile_id2, match_count2)| match_count1.cmp(match_count2));

  tile_id_to_match_counts
}

/// `raw` is a vector of lines we've got as input
/// `edges` is a vector of edges clockwise (`[top, right, bottom, left]`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
  raw: Vec<String>,
  edges: Vec<String>,
}

/// Rotations are counterclockwise, i.e.:
/// 0 - no rotation, 1 - 90 left, 2 - 180, 3 - 270 left.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Transform {
  rotation: usize,
  flip_vertical: bool,
  flip_horizontal: bool,
}

impl Tile {
  fn possible_edges_as_nums(&self) -> HashSet<u64> {
    let mut possible = HashSet::new();

    for edge in self.edges.iter() {
      let (normal, reversed) = Tile::edge_as_num(edge);
      possible.insert(normal);
      possible.insert(reversed);
    }

    possible
  }

  fn edge_as_num(edge: &str) -> (u64, u64) {
    (
      u64::from_str_radix(&edge.replace('.', "0").replace('#', "1"), 2).unwrap(),
      u64::from_str_radix(
        &edge
          .chars()
          .rev()
          .collect::<String>()
          .replace('.', "0")
          .replace('#', "1"),
        2,
      )
      .unwrap(),
    )
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

  fn top_edge(&self, transform: &Transform) -> String {
    self.edge_with_id_and_pos(0, transform)
  }

  fn right_edge(&self, transform: &Transform) -> String {
    self.edge_with_id_and_pos(1, transform)
  }

  fn bottom_edge(&self, transform: &Transform) -> String {
    self.edge_with_id_and_pos(2, transform)
  }

  fn left_edge(&self, transform: &Transform) -> String {
    self.edge_with_id_and_pos(3, transform)
  }

  fn edge_with_id_and_pos(&self, edge_id: usize, transform: &Transform) -> String {
    let (edge_id, reverse) = transform.edge_id_and_flip_map()[edge_id];
    if reverse {
      self.edges[edge_id].chars().rev().collect()
    } else {
      self.edges[edge_id].clone()
    }
  }

  fn transform(&self, transform: &Transform) -> Vec<String> {
    let rows_rotated = match transform.rotation {
      0 => self.raw.clone(),
      1 => {
        let char_vecs = self
          .raw
          .iter()
          .cloned()
          .map(|row| row.chars().collect::<Vec<_>>())
          .collect::<Vec<_>>();

        let max_idx = self.raw[0].len() - 1;
        let mut rows = Vec::with_capacity(max_idx + 1);

        for col_idx in (0..=max_idx).rev() {
          let mut row = Vec::with_capacity(max_idx + 1);

          for row_idx in 0..=max_idx {
            row.push(char_vecs[row_idx][col_idx]);
          }

          rows.push(row.into_iter().collect::<String>());
        }

        rows
      }
      2 => self
        .raw
        .clone()
        .into_iter()
        .map(|row| row.chars().rev().collect::<String>())
        .rev()
        .collect(),
      3 => {
        let char_vecs = self
          .raw
          .iter()
          .cloned()
          .map(|row| row.chars().collect::<Vec<_>>())
          .collect::<Vec<_>>();

        let max_idx = self.raw[0].len() - 1;
        let mut rows = Vec::with_capacity(max_idx + 1);

        for col_idx in 0..=max_idx {
          let mut row = Vec::with_capacity(max_idx + 1);

          for row_idx in (0..=max_idx).rev() {
            row.push(char_vecs[row_idx][col_idx]);
          }

          rows.push(row.into_iter().collect::<String>());
        }

        rows
      }
      _ => unreachable!(),
    };

    let rows_flipped_horizontal = if transform.flip_horizontal {
      rows_rotated
        .into_iter()
        .map(|row| row.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
    } else {
      rows_rotated
    };

    if transform.flip_vertical {
      rows_flipped_horizontal.into_iter().rev().collect()
    } else {
      rows_flipped_horizontal
    }
  }
}

impl Transform {
  /// Returns a vector of all possible transforms.
  fn all_transforms() -> Vec<Transform> {
    let mut transforms = Vec::with_capacity(16);

    for rotation in 0..4 {
      for &flip_vertical in &[false, true] {
        for &flip_horizontal in &[false, true] {
          let transform = Transform {
            rotation,
            flip_vertical,
            flip_horizontal,
          };

          transforms.push(transform);
        }
      }
    }

    transforms
  }

  /// Returns an array expressing edge_id and flip mappings for edges when
  /// `Transform` is applied:
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_and_edge_extractors_work() {
    let input = fs::read_to_string("inputs/sample20").unwrap();
    let tiles = Tile::parse(&input);

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
    let default_position = Transform::default();
    assert_eq!(tile_2971.top_edge(&default_position), top_edge);
    assert_eq!(tile_2971.right_edge(&default_position), right_edge);
    assert_eq!(tile_2971.bottom_edge(&default_position), bottom_edge);
    assert_eq!(tile_2971.left_edge(&default_position), left_edge);

    // verify that edges are mapped correctly after rotations
    let rotate270 = Transform {
      rotation: 3,
      ..Transform::default()
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
    let rotate90_flip_hor_and_ver = Transform {
      rotation: 1,
      flip_horizontal: true,
      flip_vertical: true,
      ..Transform::default()
    };
    assert_eq!(
      rotate90_flip_hor_and_ver.edge_id_and_flip_map(),
      rotate270.edge_id_and_flip_map()
    );
  }

  #[test]
  fn transform_works() {
    let input = fs::read_to_string("inputs/d20").unwrap();
    let tiles = Tile::parse(&input);
    let tile_1579 = tiles.get(&1579).unwrap();

    let rotated_left = tile_1579.transform(&Transform {
      rotation: 1,
      flip_horizontal: false,
      flip_vertical: false,
    });

    assert_eq!(rotated_left[0], "#...##..##");
    assert_eq!(rotated_left[1], "....#..##.");
    assert_eq!(rotated_left[9], ".#.####...");

    let rotated_270_and_flipped_vertical = tile_1579.transform(&Transform {
      rotation: 3,
      flip_horizontal: false,
      flip_vertical: true,
    });

    assert_eq!(rotated_270_and_flipped_vertical[0], "##..##...#");
    assert_eq!(rotated_270_and_flipped_vertical[9], "...####.#.");

    let rotated_180_and_flipped_horizontal = tile_1579.transform(&Transform {
      rotation: 2,
      flip_horizontal: true,
      flip_vertical: false,
    });

    assert_eq!(rotated_180_and_flipped_horizontal[0], "......##.#");
    assert_eq!(rotated_180_and_flipped_horizontal[9], ".#.##.#..#");
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample20").unwrap();
    assert_eq!(solve(&input), Some(Box::new(20899048083289)));

    let input = fs::read_to_string("inputs/d20").unwrap();
    assert_eq!(solve(&input), Some(Box::new(79412832860579)));
  }

  #[test]
  fn part_two_solved() {
    // TODO:
    // let input = fs::read_to_string("inputs/d20").unwrap();
    // assert_eq!(solve2(&input), None);
  }
}
