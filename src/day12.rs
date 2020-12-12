use super::lines;

pub fn run() {
    let distance = manhatten_distance("day-12-input.txt");
    println!("manhatten distance: {}", distance);
}

fn read_instructions(file: &str) -> Vec<Instruction> {
    lines(file).unwrap().into_iter().flat_map(Instruction::new).collect()
}

fn manhatten_distance(file: &str) -> isize {
    let instructions = read_instructions(file);
    let ship = instructions.into_iter().fold(Ship::new(), |ship, instruction| ship.follow(instruction));
    ship.position.x.abs() + ship.position.y.abs()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Action {
    Forward,
    Move(Direction),
    Turn(Side)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Side {
    Left,
    Right
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

use Direction::*;
use Side::*;
use Action::*;

impl Side {
    fn turn_degrees(&self, waypoint: Coordinate, degrees: isize) -> Coordinate {
        let num_turns = degrees / 90;
        (0..num_turns).into_iter().fold(waypoint, |coordinate, _| {
            self.turn(coordinate)
        })
    }
    fn turn(&self, waypoint: Coordinate) -> Coordinate {
        match self {
            Right => Coordinate { x: waypoint.y, y: -waypoint.x },
            Left => Coordinate { x: -waypoint.y, y: waypoint.x }
        }
    }
}
impl Direction {
    fn shift(&self, coordinate: Coordinate, distance: isize) -> Coordinate {
        match self {
            North => coordinate.shift_y(distance),
            South => coordinate.shift_y(-distance),
            East => coordinate.shift_x(distance),
            West => coordinate.shift_x(-distance),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Instruction {
    action: Action,
    value: isize
}

impl Instruction {
    fn new(line: String) -> Option<Instruction> {
        let mut chars = line.chars();
        let instruction = chars.next();
        let value = chars.as_str().parse::<isize>();
        match (instruction, value) {
            (Some('F'), Ok(value)) => Some(Instruction { action: Forward, value: value }),
            (Some('N'), Ok(value)) => Some(Instruction { action: Move(North), value: value }),
            (Some('S'), Ok(value)) => Some(Instruction { action: Move(South), value: value }),
            (Some('E'), Ok(value)) => Some(Instruction { action: Move(East), value: value }),
            (Some('W'), Ok(value)) => Some(Instruction { action: Move(West), value: value }),
            (Some('L'), Ok(value)) => Some(Instruction { action: Turn(Left), value: value }),
            (Some('R'), Ok(value)) => Some(Instruction { action: Turn(Right), value: value }),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {
    fn of(x: isize, y: isize) -> Coordinate {
        Coordinate { x, y }
    }
    fn shift_x(&self, x: isize) -> Coordinate {
        Coordinate { y: self.y, x: self.x + x }
    }

    fn shift_y(&self, y: isize) -> Coordinate {
        Coordinate { y: self.y + y, x: self.x }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Ship {
    position: Coordinate,
    waypoint: Coordinate
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Coordinate::of(0, 0),
            waypoint: Coordinate::of(10, 1)
        }
    }
    fn follow(&self, instruction: Instruction) -> Ship {
        match instruction.action {
            Forward => Ship { position: self.position.shift_x(self.waypoint.x * instruction.value).shift_y(self.waypoint.y * instruction.value), waypoint: self.waypoint },
            Move(direction) => Ship { position: self.position, waypoint: direction.shift(self.waypoint, instruction.value) },
            Turn(side) => Ship { position: self.position, waypoint: side.turn_degrees(self.waypoint, instruction.value) },
            _ => self.clone()
            // Forward => Ship { facing: self.facing, position: self.facing.forward(self.position, instruction.value) },
            // Move(direction) => Ship { facing: self.facing, position: direction.forward(self.position, instruction.value) },
            // Turn(side) => Ship { facing: self.facing.turn_degrees(side, instruction.value), position: self.position }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::manhatten_distance;
    use super::{ Ship, Instruction, Action, Direction, Side };
    use crate::day12::Action::{Forward, Move, Turn};
    use crate::day12::Coordinate;
    use crate::day12::Direction::North;
    use crate::day12::Side::Right;

    #[test]
    fn test() {
        let ship = Ship::new();
        let ship = ship.follow(Instruction { action: Forward, value: 10 });
        assert_eq!(ship.position, Coordinate::of(100, 10));
        let ship = ship.follow(Instruction { action: Move(North), value: 3});
        assert_eq!(ship.position, Coordinate::of(100, 10));
        assert_eq!(ship.waypoint, Coordinate::of(10, 4));
        let ship = ship.follow(Instruction { action: Forward, value: 7});
        assert_eq!(ship.position, Coordinate::of(170, 38));
        let ship = ship.follow(Instruction { action: Turn(Right), value: 90});
        assert_eq!(ship.position, Coordinate::of(170, 38));
        assert_eq!(ship.waypoint, Coordinate::of(4, -10));
        let ship = ship.follow(Instruction { action: Forward, value: 11});
        assert_eq!(ship.position, Coordinate::of(214, -72));
    }

    #[test]
    fn test_manhatten_distance() {
        assert_eq!(manhatten_distance("day-12-test.txt"), 286)
    }

}