use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Position {
    EmptySeat,
    Occupied
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Delta {
    Increment,
    Decrement,
    Identity
}

impl Delta {
    fn apply(&self, value: usize) -> usize {
        match self {
            Delta::Increment => value + 1,
            Delta::Decrement => value - 1,
            Delta::Identity => value
        }
    }

}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Floor {
    x_max: usize,
    y_max: usize,
    data: HashMap<(usize, usize), Position>
}

impl Floor {
    fn new(data: Vec<String>) -> Floor {
        let x_size = data.first().map(|line| line.len()).unwrap_or(0);
        let y_size = data.len();

        let coordinates = (0..y_size).into_iter().flat_map(move |y| {
            let row = data.get(y).unwrap_or(&"".to_string()).clone();

            (0..x_size).into_iter().flat_map(move |x|{
                row.chars().nth(x).and_then(|char|
                    match char {
                        'L' => Some(((x, y), Position::EmptySeat)),
                        '#' => Some(((x, y), Position::Occupied)),
                        _ => None
                    })
            })
        }).collect::<HashMap<(usize, usize), Position>>();

        Floor {
            x_max: x_size,
            y_max: y_size,
            data: coordinates
        }
    }

    fn find_visible_seat(&self, x: usize, y:usize, delta_x: Delta, delta_y: Delta) -> Option<&Position> {
        if (delta_x == Decrement && x == 0) ||
            (delta_y == Decrement && y == 0) ||
            (delta_x == Increment && x >= self.x_max) ||
            (delta_y == Increment && y >= self.y_max) {
            None
        } else {
            let new_x = delta_x.apply(x);
            let new_y = delta_y.apply(y);
            match self.data.get(&(new_x, new_y)) {
                Some(p) => Some(p),
                None => self.find_visible_seat(new_x, new_y, delta_x, delta_y)
            }
        }
    }

    fn find_all_visible(&self, x: usize, y: usize) -> Vec<&Position> {
        vec!(
            self.find_visible_seat(x, y, Delta::Increment, Delta::Increment),
            self.find_visible_seat(x, y, Delta::Increment, Delta::Identity),
            self.find_visible_seat(x, y, Delta::Increment, Delta::Decrement),
            self.find_visible_seat(x, y, Delta::Identity, Delta::Increment),
            self.find_visible_seat(x, y, Delta::Identity, Delta::Decrement),
            self.find_visible_seat(x, y, Delta::Decrement, Delta::Increment),
            self.find_visible_seat(x, y, Delta::Decrement, Delta::Identity),
            self.find_visible_seat(x, y, Delta::Decrement, Delta::Decrement)
        ).into_iter().flatten().collect::<Vec<&Position>>()
    }
    fn calculateNext(&self) -> Floor {
        let coordinates = self.data.clone().into_iter().map(|((x, y), position)| {
            let mut adjacents: Vec<&Position> = self.find_all_visible(x, y);

            let new_position = match position {
                Position::EmptySeat =>
                    if adjacents.into_iter().all(|pos| *pos == Position::EmptySeat) {
                        Position::Occupied
                    } else {
                        Position::EmptySeat
                    },
                Position::Occupied =>
                    if adjacents.into_iter().filter(|pos| **pos == Position::Occupied).count() >= 5 {
                        Position::EmptySeat
                    } else {
                        Position::Occupied
                    }
            };
            ((x, y), new_position)
        }).collect::<HashMap<(usize, usize), Position>>();

        Floor {
            x_max: self.x_max,
            y_max: self.y_max,
            data: coordinates
        }
    }

    fn run_until_stable(&self) -> Floor {
        let next = self.calculateNext();
        if next.data == self.data {
            next
        } else {
            next.run_until_stable()
        }
    }

    fn to_string(&self) -> String {
        let x_max = self.data.clone().into_iter().map(|((x, _), _)| x).max().unwrap_or(0);
        let y_max = self.data.clone().into_iter().map(|((_, y), _)| y).max().unwrap_or(0);
        (0..y_max).into_iter().map(move |y|{
            (0..x_max).into_iter().map(move |x|
                self.data.get(&(x, y)).map(|pos| match pos {
                    Position::Occupied => '#',
                    Position::EmptySeat => 'L'
                }).unwrap_or('.')
            ) .collect::<String>()
        }).fold("".to_string(), |graph, line|
            graph + "\n" + line.as_str()
        )
    }
}

use super::lines;
use std::ops::Deref;
use crate::day11::Delta::{Decrement, Increment};

fn floor(file: &str) -> Floor {
    Floor::new(lines(file).unwrap())
}

fn count_occupied_when_stable(file: &str) -> usize {
    let stable = floor(file).run_until_stable();
    stable.data.into_iter().filter(|(_, pos)| *pos == Position::Occupied).count()
}

pub fn run() {
    let num_occupied = count_occupied_when_stable("day-11-input.txt");
    println!("number of occupied seats when stable: {}", num_occupied)
}

#[cfg(test)]
mod tests {
    use super::floor;
    use crate::day11::Position::{Occupied, EmptySeat};


    fn test(base_file: &str, transformations: Vec<&str>) {
        transformations.into_iter().fold(floor(base_file), |last_floor, file| {
            let next_floor = last_floor.calculateNext();
            assert_eq!(next_floor, floor(file), "while checking {}", file);
            next_floor
        });
    }

    #[test]
    fn pos_eq() {
        let pos = &super::Position::EmptySeat;
        assert_eq!(*pos, super::Position::EmptySeat)
    }

    #[test]
    fn test_state_transitions() {
        test("day-11-test-1a.txt", vec!(
            "day-11-test-1b.txt",
            "day-11-test-1c.txt",
            "day-11-test-1d.txt",
            "day-11-test-1e.txt",
            "day-11-test-1f.txt",
            "day-11-test-1g.txt",
        ));
    }

    #[test]
    fn test_stable_state() {
        let stable_state = floor("day-11-test-1a.txt").run_until_stable();
        assert_eq!(stable_state, floor("day-11-test-1g.txt"))
    }

    #[test]
    fn check_occupied_when_stable() {
        let count = super::count_occupied_when_stable("day-11-test-1a.txt");
        assert_eq!(count, 26)
    }

    #[test]
    fn  test_visibility() {
        let floor_1 = floor("day-11-test-2.txt");
        let visible_1 = floor_1.find_all_visible(3, 4).into_iter().filter(|p| **p == Occupied).count();
        assert_eq!(visible_1, 8);

        let floor_2 = floor("day-11-test-3.txt");
        let visible_2 = floor_2.find_all_visible(1, 1).into_iter().filter(|p| **p == EmptySeat).count();
        assert_eq!(visible_2, 1);

        let floor_3 = floor("day-11-test-4.txt");
        let visible_3 = floor_3.find_all_visible(3, 3).into_iter().count();
        assert_eq!(visible_3, 0);

    }


}