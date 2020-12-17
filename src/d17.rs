use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let mut cube = Cube::parse(input);

  for _turn in 0..6 {
    cube = cube.advance();
  }

  Some(Box::new(cube.active.len()))
}

pub fn solve2(input: &str) -> Option<Box<usize>> {
  None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}

impl Point {
  fn neighbours(&self) -> Vec<Point> {
    let mut points = Vec::with_capacity(26);

    for delta_x in [-1, 0, 1].iter() {
      for delta_y in [-1, 0, 1].iter() {
        for delta_z in [-1, 0, 1].iter() {
          if !(*delta_x == 0 && *delta_y == 0 && *delta_z == 0) {
            let point = Point {
              x: self.x + delta_x,
              y: self.y + delta_y,
              z: self.z + delta_z,
            };
            points.push(point);
          }
        }
      }
    }

    points
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
  xs: RangeInclusive<i64>,
  ys: RangeInclusive<i64>,
  zs: RangeInclusive<i64>,
  active: HashSet<Point>,
}

impl std::default::Default for Cube {
  fn default() -> Cube {
    Cube {
      xs: 0..=0,
      ys: 0..=0,
      zs: 0..=0,
      active: HashSet::new(),
    }
  }
}

impl Cube {
  fn parse(input: &str) -> Cube {
    let mut cube = Cube::default();

    for (y, row) in input.trim_end().split('\n').enumerate() {
      for (x, ch) in row.chars().enumerate() {
        if ch == '#' {
          let point = Point {
            x: x as i64,
            y: y as i64,
            z: 0,
          };
          cube.activate(point);
        }
      }
    }

    cube
  }

  fn activate(&mut self, point: Point) {
    self.xs = extend(&self.xs, point.x);
    self.ys = extend(&self.ys, point.y);
    self.zs = extend(&self.zs, point.z);

    self.active.insert(point);
  }

  fn is_bodering_or_inside(&self, point: &Point) -> bool {
    (self.xs.contains(&point.x) || (self.xs.start() - point.x).abs() <= 1 || (self.xs.end() - point.x).abs() <= 1)
      && (self.ys.contains(&point.y) || (self.ys.start() - point.y).abs() <= 1 || (self.ys.end() - point.y).abs() <= 1)
      && (self.zs.contains(&point.z) || (self.zs.start() - point.z).abs() <= 1 || (self.zs.end() - point.z).abs() <= 1)
  }

  fn advance(self) -> Cube {
    let mut next = Cube::default();
    let mut consider = self.active.iter().cloned().collect::<Vec<_>>();
    let mut seen = HashSet::new();

    while let Some(point) = consider.pop() {
      if !seen.contains(&point)
        && (self.active.contains(&point) || self.is_bodering_or_inside(&point) || next.is_bodering_or_inside(&point))
      {
        let neighbours = point.neighbours();
        let active_neighbours = neighbours
          .iter()
          .filter(|neighbour| self.active.contains(neighbour))
          .count();

        if self.active.contains(&point) {
          if active_neighbours == 2 || active_neighbours == 3 {
            next.activate(point);
          }
        } else {
          if active_neighbours == 3 {
            next.activate(point);
          }
        }

        seen.insert(point);

        for &neighbour in neighbours.iter() {
          if !seen.contains(&neighbour) {
            consider.push(neighbour);
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

    let cube = Cube::parse(&input);
    assert_eq!(cube.active.len(), input.chars().filter(|ch| *ch == '#').count());

    assert_eq!(solve(&input), Some(Box::new(112)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d17").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
