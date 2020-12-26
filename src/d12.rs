pub fn solve(input: &str) -> Option<Box<u32>> {
  let instructions = parse(input);
  let final_position = instructions.into_iter().fold(Position::new(), execute);

  Some(Box::new(
    (final_position.east.abs() + final_position.north.abs()) as u32,
  ))
}

pub fn solve2(input: &str) -> Option<Box<u32>> {
  let instructions = parse(input);
  let saw = instructions
    .into_iter()
    .fold(ShipAndWaypoint::new(), execute_with_waypoint);

  Some(Box::new((saw.ship.east.abs() + saw.ship.north.abs()) as u32))
}

#[derive(Debug, Clone, Copy)]
enum Action {
  North,
  South,
  East,
  West,
  Left,
  Right,
  Forward,
}

#[derive(Debug, Clone, Copy)]
struct Instruction(Action, i32);

use Action::*;

#[derive(Debug, Clone, Copy)]
struct Position {
  // only North, South, East, or West are allowed
  direction: Action,
  east: i32,
  north: i32,
}

impl Position {
  fn new() -> Position {
    Position {
      direction: East,
      east: 0,
      north: 0,
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct ShipAndWaypoint {
  ship: Position,
  waypoint: Position,
}

impl ShipAndWaypoint {
  fn new() -> ShipAndWaypoint {
    ShipAndWaypoint {
      ship: Position::new(),
      waypoint: Position {
        east: 10,
        north: 1,
        direction: East,
      },
    }
  }
}

fn execute_with_waypoint(saw: ShipAndWaypoint, inst: Instruction) -> ShipAndWaypoint {
  let Instruction(action, value) = inst;

  match action {
    North => ShipAndWaypoint {
      waypoint: Position {
        north: saw.waypoint.north + value,
        ..saw.waypoint
      },
      ..saw
    },
    South => ShipAndWaypoint {
      waypoint: Position {
        north: saw.waypoint.north - value,
        ..saw.waypoint
      },
      ..saw
    },
    East => ShipAndWaypoint {
      waypoint: Position {
        east: saw.waypoint.east + value,
        ..saw.waypoint
      },
      ..saw
    },
    West => ShipAndWaypoint {
      waypoint: Position {
        east: saw.waypoint.east - value,
        ..saw.waypoint
      },
      ..saw
    },
    Left => {
      let waypoint = turn_waypoint_left(saw.waypoint, value);
      ShipAndWaypoint { waypoint, ..saw }
    }
    Right => {
      let waypoint = turn_waypoint_right(saw.waypoint, value);
      ShipAndWaypoint { waypoint, ..saw }
    }
    Forward => {
      let ship = Position {
        east: saw.ship.east + saw.waypoint.east * value,
        north: saw.ship.north + saw.waypoint.north * value,
        ..saw.ship
      };

      ShipAndWaypoint { ship, ..saw }
    }
  }
}

fn turn_waypoint_left(waypoint: Position, value: i32) -> Position {
  match value {
    90 => Position {
      east: -waypoint.north,
      north: waypoint.east,
      ..waypoint
    },
    180 => Position {
      east: -waypoint.east,
      north: -waypoint.north,
      ..waypoint
    },
    270 => turn_waypoint_right(waypoint, 90),
    _ => panic!("Invalid value {:?}.", value),
  }
}

fn turn_waypoint_right(waypoint: Position, value: i32) -> Position {
  match value {
    90 => Position {
      east: waypoint.north,
      north: -waypoint.east,
      ..waypoint
    },
    180 => turn_waypoint_left(waypoint, value),
    270 => turn_waypoint_left(waypoint, 90),
    _ => panic!("Invalid value {:?}.", value),
  }
}

fn execute(pos: Position, inst: Instruction) -> Position {
  let Instruction(action, value) = inst;

  match action {
    North => Position {
      north: pos.north + value,
      ..pos
    },
    South => Position {
      north: pos.north - value,
      ..pos
    },
    East => Position {
      east: pos.east + value,
      ..pos
    },
    West => Position {
      east: pos.east - value,
      ..pos
    },
    Left => Position {
      direction: turn_left(pos.direction, value),
      ..pos
    },
    Right => Position {
      direction: turn_right(pos.direction, value),
      ..pos
    },
    Forward => execute(pos, Instruction(pos.direction, value)),
  }
}

fn turn_left(direction: Action, value: i32) -> Action {
  match value {
    90 => match direction {
      North => West,
      West => South,
      South => East,
      East => North,
      _ => panic!("Invalid direction {:?}.", direction),
    },
    180 => match direction {
      North => South,
      South => North,
      West => East,
      East => West,
      _ => panic!("Invalid direction {:?}.", direction),
    },
    270 => turn_right(direction, 90),
    _ => todo!(),
  }
}

fn turn_right(direction: Action, value: i32) -> Action {
  match value {
    90 => match direction {
      North => East,
      East => South,
      South => West,
      West => North,
      _ => panic!("Invalid direction {:?}.", direction),
    },
    180 => turn_left(direction, 180),
    270 => turn_left(direction, 90),
    _ => todo!(),
  }
}

fn parse(input: &str) -> Vec<Instruction> {
  input
    .trim_end()
    .split('\n')
    .map(|line| {
      let line = line.chars().collect::<Vec<_>>();

      let action = match line[0] {
        'N' => North,
        'S' => South,
        'E' => East,
        'W' => West,
        'L' => Left,
        'R' => Right,
        'F' => Forward,
        ch => panic!("unrecognized action {}", ch),
      };
      let value = line[1..].into_iter().collect::<String>().parse::<i32>().unwrap();

      Instruction(action, value)
    })
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  const SAMPLE_INPUT: &'static str = r#"F10
N3
F7
R90
F11
"#;

  #[test]
  fn part_one_solved() {
    assert_eq!(solve(SAMPLE_INPUT), Some(Box::new(25)));

    let input = fs::read_to_string("inputs/d12").unwrap();
    assert_eq!(solve(&input), Some(Box::new(508)));
  }

  #[test]
  fn part_two_solved() {
    assert_eq!(solve2(SAMPLE_INPUT), Some(Box::new(286)));

    let input = fs::read_to_string("inputs/d12").unwrap();
    assert_eq!(solve2(&input), Some(Box::new(30761)));
  }
}
