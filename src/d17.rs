use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let mut cube = Cube::parse(input, 3);

  for _turn in 0..6 {
    cube = cube.advance();
  }

  Some(Box::new(cube.active.len()))
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  let mut cube = Cube::parse(input, 4);

  for _turn in 0..6 {
    cube = cube.advance();
  }

  Some(Box::new(cube.active.len()))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
  coords: Vec<i64>,
}

impl Point {
  fn neighbours(&self) -> HashSet<Point> {
    let mut points = HashSet::new();

    points.insert(self.clone());
    for dimension_id in 0..self.coords.len() {
      for point in points.iter().cloned().collect::<Vec<_>>() {
        for delta in [-1, 0, 1].iter() {
          let mut point = point.clone();
          point.coords[dimension_id] = point.coords[dimension_id] + delta;

          points.insert(point);
        }
      }
    }

    points.remove(self);
    points
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
  // matches dimension_id with active coordinate range
  border: Vec<RangeInclusive<i64>>,
  active: HashSet<Point>,
  dimensions: usize,
}

impl Cube {
  fn empty(dimensions: usize) -> Cube {
    Cube {
      border: vec![0..=0; dimensions],
      active: HashSet::new(),
      dimensions,
    }
  }

  fn parse(input: &str, dimensions: usize) -> Cube {
    let mut cube = Cube::empty(dimensions);

    for (y, row) in input.trim_end().split('\n').enumerate() {
      for (x, ch) in row.chars().enumerate() {
        if ch == '#' {
          let mut coords = vec![0; dimensions];
          coords[0] = x as i64;
          coords[1] = y as i64;

          cube.activate(Point { coords });
        }
      }
    }

    cube
  }

  fn activate(&mut self, point: Point) {
    for (dimension_id, &coord) in point.coords.iter().enumerate() {
      self.border[dimension_id] = extend(&self.border[dimension_id], coord);
    }

    self.active.insert(point);
  }

  fn is_bordering_or_inside(&self, point: &Point) -> bool {
    (0..self.dimensions).into_iter().all(|dimension_id| {
      let dimension_border = &self.border[dimension_id];
      let dimension_coord = point.coords[dimension_id];

      dimension_border.contains(&dimension_coord)
        || (dimension_border.start() - dimension_coord).abs() <= 1
        || (dimension_border.end() - dimension_coord).abs() <= 1
    })
  }

  fn advance(self) -> Cube {
    let mut next = Cube::empty(self.dimensions);
    let mut consider = self.active.iter().cloned().collect::<Vec<_>>();
    let mut seen = HashSet::new();

    while let Some(point) = consider.pop() {
      if !seen.contains(&point)
        && (self.active.contains(&point) || self.is_bordering_or_inside(&point) || next.is_bordering_or_inside(&point))
      {
        let neighbours = point.neighbours();
        let active_neighbours = neighbours
          .iter()
          .filter(|neighbour| self.active.contains(neighbour))
          .count();

        if self.active.contains(&point) {
          if active_neighbours == 2 || active_neighbours == 3 {
            next.activate(point.clone());
          }
        } else {
          if active_neighbours == 3 {
            next.activate(point.clone());
          }
        }

        seen.insert(point);

        for neighbour in neighbours.iter() {
          if !seen.contains(neighbour) {
            consider.push(neighbour.clone());
          }
        }
      }
    }

    dbg!(next.active.len());
    next
  }
}

fn extend<T>(range: &RangeInclusive<T>, value: T) -> RangeInclusive<T>
where
  T: Copy + PartialOrd,
{
  if *range.start() > value {
    value..=*range.end()
  } else if *range.end() < value {
    *range.start()..=value
  } else {
    range.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample17").unwrap();
    let cube = Cube::parse(&input, 3);
    assert_eq!(cube.active.len(), input.chars().filter(|ch| *ch == '#').count());
    assert_eq!(solve(&input), Some(Box::new(112)));

    let input = fs::read_to_string("inputs/d17").unwrap();
    assert_eq!(solve(&input), Some(Box::new(313)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/sample17").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(848)));

    let input = fs::read_to_string("inputs/d17").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(2640)));
  }
}
