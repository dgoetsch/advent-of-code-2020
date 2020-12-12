use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines() -> io::Result<Vec<String>>  {
    read_lines("./day-5-input.txt")
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .collect::<Vec<String>>()
        )
}

#[derive(Debug, Clone)]
struct Seat {
    source: String,
    row: Vec<Bound>,
    col: Vec<Bound>
}

impl Seat {
    fn new(seatNumber: String) -> Seat {
        let (row, col) = seatNumber.split_at(7);
        let row = row.chars()
            .filter(|c| c == &'F' || c == &'B')
            .map(|c| match c {
                'F' => Bound::Lower,
                _ => Bound::Upper
            })
            .collect::<Vec<Bound>>();

        let col = col.chars()
            .filter(|c| c == &'R' || c == &'L')
            .map(|c| match c {
                'L' => Bound::Lower,
                _ => Bound::Upper
            })
            .collect::<Vec<Bound>>();

        Seat {
            source: seatNumber,
            row: row,
            col: col
        }
    }

    fn id(&self) -> usize {
        self.seatNumber() * 8 + self.rowNumber()
    }

    fn seatNumber(&self) -> usize {
        self.row.clone().into_iter()
            .fold((0, 128), |(total, upperBound), bound| {
                (bound.apply(upperBound.clone()) + total, upperBound / 2)
            })
            .0
    }

    fn rowNumber(&self) -> usize {
        self.col.clone().into_iter()
            .fold((0, 8), |(total, upperBound), bound| {
                (bound.apply(upperBound.clone()) + total, upperBound / 2)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it() {
        assert_eq!(Seat::new("FBFBBFFRLR".to_string()).id(), 357)
    }
}

#[derive(Debug, Clone)]
enum Bound {
    Lower,
    Upper
}

impl Bound {
    fn apply(self, upperBound: usize) -> usize {
        match self {
            Bound::Lower => 0,
            Bound::Upper => upperBound / 2
        }
    }
}

pub fn run() {
    let ids = lines().unwrap()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(Seat::new)
        .map(|s| s.id())
        .collect::<HashSet<usize>>();
    let res = (0..901).into_iter()
        .filter(|seat| *seat > 1)
        .filter(|seat| !ids.contains(seat))
        .filter(|seat| ids.contains(&(seat - 1)))
        .filter(|seat| ids.contains(&(seat + 1)))
        .collect::<Vec<usize>>();
    println!("{:?}", res)
}
