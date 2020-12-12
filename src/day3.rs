use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines() -> io::Result<Vec<String>>  {
    read_lines("./day-3-input.txt")
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .filter(|l| l.trim().len() > 0)
                .collect::<Vec<String>>()
        )
}

pub fn run() {
    let result = calculate(vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)));
    println!("{}", result)
}

fn calculate(slopes: Vec<(usize, usize)>) -> usize {
    slopes.into_iter()
        .map(|(over, down)| calculate_trees(over, down))
        .fold(1, |acc, next|
            {
                acc * next
            }
            )
}

fn calculate_trees(over: usize, down: usize) -> usize {
    lines().unwrap()
        .into_iter()
        .map(Row::new)
        .fold((0, vec!()), |(row_idx, accumulated): (usize, Vec<char>), row: Row| {
            if row_idx % down == 0 {
                let collision = row.at(over * (row_idx/down));
                let mut next = accumulated.clone();
                next.push(collision);
                (row_idx + 1, next)
            } else {
                (row_idx + 1, accumulated)
            }
        })
        .1.into_iter()
        .filter(|collision| *collision == '#')
        .count()
}

#[derive(Debug)]
struct Row {
    pattern: String,
    length: usize
}

impl Row {
    fn new(pattern: String) -> Row {
        Row { pattern: pattern.clone(), length: pattern.chars().count() }
    }

    fn at(&self, idx: usize) -> char {
        let actual_idx = idx % self.length;
        self.pattern.chars().nth(actual_idx).unwrap()
    }
}