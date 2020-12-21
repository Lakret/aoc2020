use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::{SliceRandom, IteratorRandom};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use rayon::prelude::*;

pub fn solve(input: &str) -> Option<Box<u64>> {
  // hyperparams
  let num_threads = 8;
  // each thread will run `max_iterations`.
  // 1st iteration will start with `max_moves_low`,
  // and each next one will increase it by a factor of `max_moves_step`,
  // but limiting it to `max_moves_high` at the max.
  let max_iterations = 50;
  let max_moves_step = 2;
  let max_moves_low = 500;
  let max_moves_high = 2_000_000;

  let tiles = parse(input);

  rayon::ThreadPoolBuilder::default()
    .num_threads(num_threads)
    .build_global()
    .unwrap_or(());

  // (0..max_iterations).into_iter().find_map(|iteration| {
  for iteration in 0..max_iterations {
    // progressively increase max_moves at each new iteration
    let max_moves = (max_moves_low * (max_moves_step as f64).powi(iteration as i32) as usize).min(max_moves_high);

    let result = (0..num_threads).into_par_iter().find_map_any(|thread_id| {
      let mut rng = thread_rng();

      println!(
        "[{}] Iteration {} with max moves {}.",
        thread_id,
        iteration + 1,
        max_moves
      );
      if let (Some(assignment), moves) = arrange(&tiles, max_moves, &mut rng) {
        let (first_row, last_row) = (assignment[0].clone(), assignment.last().unwrap());
        let (top_left, top_right) = (first_row[0], first_row.last().unwrap());
        let (bottom_left, bottom_right) = (last_row[0], last_row.last().unwrap());
        let answer = top_left.0 * top_right.0 * bottom_left.0 * bottom_right.0;

        println!(
          "[{:?}] Corners: {:?}.",
          thread_id,
          vec![&top_left, top_right, &bottom_left, bottom_right]
        );
        println!(
          "[{:?}] Solved at iteration {}, in {} moves: {}.",
          thread_id,
          iteration + 1,
          moves,
          answer
        );
        return Some(Box::new(answer));
      }

      return None;
    });

    if result.is_some() {
      return result;
    }
  }

  None
}

pub fn solve2(input: &str) -> Option<Box<u64>> {
  None
}

/// To avoid moving vectors of strings around all the time.
type TilesMap = HashMap<u64, Tile>;

/// `[[(tile_id, Transform); row_size]; col_size]`
type Assignment = Vec<Vec<(u64, Transform)>>;

type Coords = (usize, usize);

fn arrange(tiles: &HashMap<u64, Tile>, max_moves: usize, rng: &mut ThreadRng) -> (Option<Assignment>, usize) {
  // cache this to avoid re-generating it all the time
  let transforms = Transform::all_transforms();
  let size = size(tiles);
  let mut all_coords = (0..size)
    .flat_map(|row_id| (0..size).map(|col_id| (row_id, col_id)).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  // initial random assignment
  let mut assignment = random_assignment(tiles, rng);

  let max_tabu = size / 3;
  let mut tabu: VecDeque<(Coords, (u64, Transform))> = VecDeque::new();

  // try to improve while there are conflicts / until max moves reached.
  let mut moves = 0;
  let mut conflicts = get_all_conflicts(tiles, &assignment);
  while !conflicts.is_empty() {
    // don't get stuck in a pathological case
    moves += 1;
    if moves > max_moves {
      return (None, moves);
    }

    // select random conflicted variable
    let (coords, conflicted_coords) = conflicts.into_iter().choose(rng).unwrap();
    let (row_id, col_id) = coords;
    let (tile_id, curr_transform) = assignment[row_id][col_id];

    let mut improved = false;
    let local_conflicts_count = conflicted_coords.len();

    let local_tabu = tabu
      .iter()
      .copied()
      .filter_map(|((tabu_row_id, tabu_col_id), tile_id_and_transform)| {
        if tabu_row_id == row_id && tabu_col_id == tabu_col_id {
          Some(tile_id_and_transform)
        } else {
          None
        }
      })
      .collect::<HashSet<_>>();

    // try to apply all possible transforms != current transform to the current tile
    for &transform in &transforms {
      if transform != curr_transform && !local_tabu.contains(&(tile_id, transform)) {
        assignment[row_id][col_id] = (tile_id, transform);

        // if some of the improves the conflicts count, we can move on to a next tile
        let new_conflicts = get_conflicts(tiles, &assignment, coords);
        if new_conflicts.len() < local_conflicts_count {
          // TODO: extract tabu add logic
          tabu.push_front(((row_id, col_id), assignment[row_id][col_id]));

          improved = true;
          break;
        }
      }
    }

    // if no improvement happen via transforms, try to swap with some other cell
    if !improved {
      // add some tasty randomness to avoid pathological cases
      all_coords.shuffle(rng);

      for &another_coords in all_coords.iter() {
        let another_tile_id_and_transform = assignment[another_coords.0][another_coords.1];
        if another_coords != coords && !local_tabu.contains(&another_tile_id_and_transform) {
          // TODO: pre-process by figuring out the tiles with min. number of matches for other tiles edges
          // those are likely corners
          // TODO: shall we also count number of conflicts in `another_coords`
          // and only swap when both this and another cell's conflicts count improve?
          // TODO: global conflict counts to check improvement instead of local conflict counts?
          // TODO: should we select the most optimal transform / position, instead of breaking on the first
          // that improves things slightly?

          swap(&mut assignment, (row_id, col_id), another_coords);

          // same, break as soon as we improve
          let new_conflicts = get_conflicts(tiles, &assignment, coords);
          if new_conflicts.len() < local_conflicts_count {
            // TODO: extract tabu add logic
            tabu.push_front(((row_id, col_id), assignment[row_id][col_id]));

            improved = true;
            break;
          } else {
            // we need to undo if no improvement was made
            swap(&mut assignment, (row_id, col_id), another_coords);
          }
        }
      }
    }

    // if still no improvements, do a random swap to break ties.
    // we don't check if it improves or makes the situation worse,
    // to escape local minimums.
    if !improved {
      let swap_row_id = rng.gen_range(0, size);
      let swap_col_id = rng.gen_range(0, size);

      swap(&mut assignment, (row_id, col_id), (swap_row_id, swap_col_id));

      // TODO: extract tabu add logic
      tabu.push_front(((row_id, col_id), assignment[row_id][col_id]));
    }

    if tabu.len() > max_tabu {
      tabu.truncate(max_tabu);
    }

    conflicts = get_all_conflicts(tiles, &assignment);
  }

  (Some(assignment), moves)
}

fn swap(assignment: &mut Assignment, this: Coords, another: Coords) {
  let (row_id, col_id) = this;
  let (swap_row_id, swap_col_id) = another;

  let (this_tile_id, this_transform) = assignment[row_id][col_id];
  assignment[row_id][col_id] = assignment[swap_row_id][swap_col_id];
  assignment[swap_row_id][swap_col_id] = (this_tile_id, this_transform);
}

fn size(tiles: &HashMap<u64, Tile>) -> usize {
  (tiles.values().len() as f64).sqrt() as usize
}

fn random_assignment(tiles: &HashMap<u64, Tile>, rng: &mut ThreadRng) -> Assignment {
  let non_default_transform = Transform {
    rotation: rng.gen_range(0, 3),
    flip_horizontal: rng.gen(),
    flip_vertical: rng.gen(),
  };

  let mut tile_ids = tiles
    .keys()
    .copied()
    // .map(|tile_id| (tile_id, Transform::default()))
    .map(|tile_id| (tile_id, non_default_transform))
    .collect::<Vec<_>>();
  tile_ids.shuffle(rng);

  let size = size(tiles);
  let mut assignment = Vec::with_capacity(size);
  for row_id in 0..size {
    let row = tile_ids[row_id * size..(row_id + 1) * size].to_vec();
    assignment.push(row);
  }

  assignment
}

/// Returns a map of conflicts, mapping coordinates of a cell with conflicts in the current `assignment`
/// to a vector of coords of its left, top, or both left & top neighbours with which it conflicts.
fn get_all_conflicts(tiles: &TilesMap, assignment: &Assignment) -> HashMap<Coords, Vec<Coords>> {
  let mut conflicts = HashMap::new();
  let size = assignment.len();

  for row_id in 0..size {
    for col_id in 0..size {
      let coords = (row_id, col_id);
      let local_conflicts = get_conflicts(tiles, assignment, coords);
      if !local_conflicts.is_empty() {
        conflicts.insert(coords, local_conflicts);
      }
    }
  }

  conflicts
}

/// Gets a vector (possibly empty) of conflicts for a tile with `coords` in `assignment`.
fn get_conflicts(tiles: &TilesMap, assignment: &Assignment, coords: Coords) -> Vec<Coords> {
  let (row_id, col_id) = coords;
  let (curr_tile_id, curr_tile_transform) = &assignment[row_id][col_id];
  let curr_tile = tiles.get(curr_tile_id).unwrap();
  let mut conflicts = vec![];

  // check for conflict with top tile, if it exists
  if row_id > 0 {
    let (top_tile_id, top_tile_transform) = &assignment[row_id - 1][col_id];
    let top_tile = tiles.get(top_tile_id).unwrap();

    if curr_tile.top_edge(curr_tile_transform) != top_tile.bottom_edge(top_tile_transform) {
      conflicts.push((row_id - 1, col_id));
    }
  }

  // check for conflict with left tile, if it exists
  if col_id > 0 {
    let (left_tile_id, left_tile_transform) = &assignment[row_id][col_id - 1];
    let left_tile = tiles.get(left_tile_id).unwrap();

    if curr_tile.left_edge(curr_tile_transform) != left_tile.right_edge(left_tile_transform) {
      conflicts.push((row_id, col_id - 1));
    }
  }

  conflicts
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

impl Tile {
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
