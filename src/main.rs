use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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
    // day1::run();
    // day2::run();
    // day3::run();
    // day4::run();
    // day5::run();
    // day6::run();
    // day7::run();
    // day8::run();
    // day9::run();
    // day10::run();
    // day11::run();
    // day12::run();
    // day13::run();
    // day14::run();
    day15::run();
}