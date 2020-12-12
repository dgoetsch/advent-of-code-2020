use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

mod day9;
mod day10;
mod day11;
mod day12;
mod day8;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines(file: &str) -> io::Result<Vec<String>>  {
    read_lines(file)
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .collect::<Vec<String>>()
        )
}



fn main() {
    // day8::run()
    // day9::run()
    // day10::run()
    // day11::run();
    day12::run();
}