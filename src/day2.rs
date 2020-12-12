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
    read_lines("./day-2-input.txt")
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .collect::<Vec<String>>()
        )
}
pub fn run() {
    let count = lines()
        .into_iter()
        .flat_map(|results| results.into_iter())
        .map(parse)
        .flat_map(|r| r.into_iter())
        .filter(is_valid)
        .collect::<Vec<Entry>>()
        .len();
    println!("Found {} entries", count)
}

fn parse(line: String) -> Result<Entry, ParseErr> {
    let parts = line.split(":").collect::<Vec<&str>>();

    if(parts.len() != 2) {
        return Err(ParseErr{source: line, message: "line has too many parts".to_string()})
    }

    let rule = parts[0].trim();
    let pass = parts[1].trim().to_string();

    let rule_parts = rule.split(char::is_whitespace).collect::<Vec<&str>>();

    if(rule_parts.len() != 2) {
        return Err(ParseErr{source: line, message: "rule has too many parts".to_string()})
    }

    let range = rule_parts[0].to_string();
    let char = rule_parts[1].to_string();

    if(char.len() != 1) {
        return Err(ParseErr{source: line, message: "too many char in rule".to_string()})
    }
    let rangeParts = range.split("-")
        .flat_map(|bound| bound.parse::<usize>().into_iter())
        .collect::<Vec<usize>>();

    if(rangeParts.len() != 2) {
        return Err(ParseErr{source: line, message: "invalid range".to_string()})
    }



    Ok(Entry {
        min: rangeParts[0],
        max: rangeParts[1],
        char: char.chars().nth(0).unwrap(),
        password: pass.to_string()
    })
}

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    char: char,
    password: String
}

fn is_valid(entry: &Entry) -> bool {
    if entry.min > entry.password.len() || entry.max > entry.password.len() {
        return false
    }
    let adjusted_min = entry.min - 1;
    let adjusted_max = entry.max - 1;

    let low: char = entry.password.chars().nth(adjusted_min).unwrap();
    let high: char = entry.password.chars().nth(adjusted_max).unwrap();

    (low == entry.char && high != entry.char) || (high == entry.char && low != entry.char)
    // let count = entry.password
    //     .chars()
    //     .into_iter()
    //     .filter(|c| c == &entry.char).collect::<Vec<char>>().len();
    //
    // count >= entry.min && count <= entry.max
}
#[derive(Debug)]
struct ParseErr {
    source: String,
    message: String
}
