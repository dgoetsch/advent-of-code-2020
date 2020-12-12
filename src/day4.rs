use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines() -> io::Result<Vec<String>>  {
    read_lines("./day-4-input.txt")
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .collect::<Vec<String>>()
        )
}

#[derive(Debug, Clone)]
struct Fields {
    fields: HashMap<String, String>
}

impl Fields {
    fn empty() -> Fields {
        Fields {
            fields: HashMap::new() }
    }
    fn parse(line: String) -> Fields {
        let fields = line.trim()
            .split(char::is_whitespace)
            .map(|field| field.split(|c| c == ':').collect::<Vec<&str>>())
            .filter(|parts| parts.len() == 2)
            .map(|parts| (parts[0].trim().to_string(), parts[1].trim().to_string()))
            .collect::<HashMap<String, String>>();

        Fields { fields: fields }
    }

    fn merge(&mut self, other: Fields) {
        self.fields.extend(other.fields);
    }


    fn is_valid(&self) -> bool {
        // self.fields.clone().into_iter()
        //     .map(|(key, value)| validate_arge(key, value))
        vec!("byr","iyr","eyr", "hgt","hcl","ecl","pid").into_iter()
            .map(|key| key.to_string())
            .all(|key| self
                .fields
                .get(key.as_str())
                .map(|v| Fields::is_valid_arg(key, v.clone()))
                .unwrap_or(false))
    }

    fn is_valid_arg(arg: String, value: String) -> bool {
        let hclRegex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

        match arg.as_str() {
            "byr" => value.len() == 4 && value.parse::<u32>().map(|byr| byr <= 2002 && byr >= 1920).unwrap_or(false),
            "iyr" => value.len() == 4 && value.parse::<u32>().map(|iyr| iyr <= 2020 && iyr >= 2010).unwrap_or(false),
            "eyr" => value.len() == 4 && value.parse::<u32>().map(|eyr| eyr <= 2030 && eyr >= 2020).unwrap_or(false),
            "hgt" => if(value.ends_with("cm")) {
                value.replace("cm", "").parse::<u32>().map(|hgt| hgt <=193 && hgt >=150).unwrap_or(false)
            } else if(value.ends_with("in")) {
                value.replace("in", "").parse::<u32>().map(|hgt| hgt <=76 && hgt >=59).unwrap_or(false)
            } else {
                false
            },
            "hcl" => hclRegex.is_match(value.as_str()),
            "ecl" => vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
                .into_iter()
                .filter(|pos| pos.to_string() == value)
                .count() > 0,
            "pid" => value.len() == 9 && value.parse::<u32>().is_ok(),
            _ => true
        }
    }
}
pub fn run() {
    let count = lines().unwrap()
        .into_iter()
        .fold(vec!(), |fields: Vec<Fields>, next: String| {
            if(next.trim().is_empty()) {
                let mut clone = fields.clone();
                clone.push(Fields::empty());
                clone
            } else {
                let mut clone = fields.clone();
                let new_fields = Fields::parse(next);
                match clone
                    .last_mut() {
                    Some(mut fields) => fields.merge(new_fields),
                    None => clone.push(new_fields)
                }
                clone
            }
        })
        .into_iter()
        .filter(|fields| fields.is_valid())
        .count();

    println!("{:?}", count)
}
