use std::collections::HashMap;
use crate::day17::State::Active;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize
}

impl Coordinate {
    fn of(x: isize, y: isize, z: isize) -> Coordinate {
        Coordinate { x: x, y: y, z: z }
    }
    fn origin() -> Coordinate {
        Coordinate { x: 0, y: 0, z: 0 }
    }

    fn neighbors(&self) -> Vec<Coordinate> {
        (self.x - 1..=self.x+1).into_iter().flat_map(move |x|
            (self.y - 1..=self.y+1).into_iter().flat_map(move |y|
                (self.z - 1..=self.z+1).into_iter().flat_map(move |z|
                    if x==self.x && y == self.y && z == self.z {
                        None
                    } else {
                        Some(Coordinate::of(x, y, z))
                    })))
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    Active,
    Inactive
}

impl State {
    fn to_char(&self) -> char {
        match self {
            State::Active => '#',
            State::Inactive => '.'
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Space {
    min: Coordinate,
    max: Coordinate,
    xyz: HashMap<Coordinate, State>
}

impl Space {
    fn from(data: &str) -> Space {
        let z = 0;
        let parts = data.split('\n').collect::<Vec<&str>>();
        let y_max = parts.len() - 1;
        let x_max = parts.clone().into_iter().map(|p| p.len()).max().unwrap_or(1) - 1;

        Space {
            min: Coordinate::of(0, 0, z),
            max: Coordinate::of(x_max as isize, y_max as isize, z),
            xyz: (0..=y_max).into_iter().flat_map(move |y| {
                let line = parts.get(y).map(|l| l.clone()).unwrap_or("");
                (0..=x_max).into_iter().map(move |x|
                    (Coordinate::of(x as isize, y as isize, z), match line.chars().nth(x as usize) {
                        Some('#') => State::Active,
                        _ => State::Inactive
                    })
                )
            }).collect()
        }


    }
    fn string_rep(&self) -> Vec<String> {
        (self.min.z..=self.max.z).into_iter()
            .map(|z| (self.min.y ..=self.max.y).into_iter()
                .map(|y| (self.min.x..=self.max.x).into_iter()
                    .map(|x| self.state_of(Coordinate::of(x, y, z)).to_char())
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"))
            .collect()
    }

    fn next(&self) -> Space {
        let coordinates: HashMap<Coordinate, State> = (self.min.x - 1..=self.max.x + 1).into_iter().flat_map(move |x|
            (self.min.y - 1..=self.max.y + 1).into_iter().flat_map(move |y|
                (self.min.z - 1..=self.max.z + 1).into_iter().map(move |z|
                    (Coordinate::of(x, y, z), self.next_state(Coordinate::of(x, y, z), self.state_of(Coordinate::of(x, y, z))))
                ))
        )
            .filter(|(_, state)| *state == State:: Active)
            .collect();

        let x_min = coordinates.clone().into_iter()
            .map(|(c, _)| c.x)
            .min().unwrap_or(0);
        let x_max = coordinates.clone().into_iter()
            .map(|(c, _)| c.x)
            .max().unwrap_or(0);

        let y_min = coordinates.clone().into_iter()
            .map(|(c, _)| c.y)
            .min().unwrap_or(0);
        let y_max = coordinates.clone().into_iter()
            .map(|(c, _)| c.y)
            .max().unwrap_or(0);

        let z_min = coordinates.clone().into_iter()
            .map(|(c, _)| c.z)
            .min().unwrap_or(0);
        let z_max = coordinates.clone().into_iter()
            .map(|(c, _)| c.z)
            .max().unwrap_or(0);


        Space {
            min: Coordinate::of(x_min, y_min, z_min),
            max: Coordinate::of(x_max, y_max, z_max),
            xyz: coordinates
        }
    }

    fn state_of(&self, coordinate: Coordinate) -> State {
        self.xyz.get(&coordinate).map(|c| c.clone()).unwrap_or(State::Inactive)
    }
    fn next_state(&self, coordinate: Coordinate, state: State) -> State {
        match state {
            State::Active => {
                let count = coordinate.neighbors()
                    .into_iter()
                    .map(|c| self.state_of(c))
                    .filter(|s| *s == State::Active)
                    .count();
                if count >= 2 && count <= 3 {
                    State::Active
                } else {
                    State::Inactive
                }

            },
            State::Inactive => {
                let count = coordinate.neighbors()
                    .into_iter()
                    .map(|c| self.state_of(c))
                    .filter(|s| *s == State::Active)
                    .count();
                if count == 3 {
                    State:: Active
                } else {
                    State::Inactive
                }
            }
        }
    }
}

pub fn run() {
    let data = super::lines("day-17-input.txt")
        .unwrap()
        .join("\n");

    let space = Space::from(data.as_str());
    let final_space = (0..6).into_iter().fold(space, |s, _| s.next());
    println!("final space has {} active spaces", final_space.xyz.len());
}

#[cfg(test)]
mod test {
    use crate::day17::{Coordinate, Space};

    #[test]
    fn test_neighbors() {
        let coordinate = Coordinate::origin();
        let neighbors = coordinate.neighbors();
        assert_eq!(neighbors.len(), 26);
        assert_eq!(neighbors,
            (-1..=1).into_iter().flat_map(move |x|
                (-1..=1).into_iter().flat_map(move |y|
                    (-1..=1).into_iter().map(move |z| Coordinate::of(x, y, z))))
                .filter(|c| *c != coordinate)
                .collect::<Vec<Coordinate>>())
    }

    #[test]
    fn test_progression() {
        let initial_state = ".#.\n\
            ..#\n\
            ###";
        let start = Space::from(initial_state);
        println!("{:?}", start);
        assert_eq!(start.string_rep(), vec!(initial_state.to_string()));

        let second = start.next();
        let expected_second = vec!(
            "#..\n\
            ..#\n\
            .#.",
            "#.#\n\
            .##\n\
            .#.",
            "#..\n\
            ..#\n\
            .#."
        ).into_iter().map(|l| l.to_string()).collect::<Vec<String>>();
        assert_eq!(second.string_rep(), expected_second);

        let third = second.next();
        let expected_third = vec!(
            ".....\n\
            .....\n\
            ..#..\n\
            .....\n\
            .....",
            "..#..\n\
            .#..#\n\
            ....#\n\
            .#...\n\
            .....",
            "##...\n\
            ##...\n\
            #....\n\
            ....#\n\
            .###.",
            "..#..\n\
            .#..#\n\
            ....#\n\
            .#...\n\
            .....",
            ".....\n\
            .....\n\
            ..#..\n\
            .....\n\
            .....");

        assert_eq!(third.string_rep(), expected_third);

        let fourth = third.next();
        let expected_fourth = vec!(
            ".......\n\
            .......\n\
            ..##...\n\
            ..###..\n\
            .......\n\
            .......\n\
            .......",
            "..#....\n\
            ...#...\n\
            #......\n\
            .....##\n\
            .#...#.\n\
            ..#.#..\n\
            ...#...",
            "...#...\n\
            .......\n\
            #......\n\
            .......\n\
            .....##\n\
            .##.#..\n\
            ...#...",
            "..#....\n\
            ...#...\n\
            #......\n\
            .....##\n\
            .#...#.\n\
            ..#.#..\n\
            ...#...",
            ".......\n\
            .......\n\
            ..##...\n\
            ..###..\n\
            .......\n\
            .......\n\
            .......");

        assert_eq!(fourth.string_rep(), expected_fourth);
    }
}