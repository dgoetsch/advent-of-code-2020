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
    read_lines("./day-6-input.txt")
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .collect::<Vec<String>>()
        )
}

#[derive(Debug, Clone)]
struct Group {
    answers: Vec<String>
}

impl Group {
    fn empty() -> Group {
        Group {
            answers: vec!()
        }
    }

    fn new(answers: String) -> Group {
       Group {
           answers: vec!(answers)
       }
    }

    fn include(&mut self, additional_answers: String) {
        let mut new_answers =  self.answers.clone();
        new_answers.push(additional_answers);
        self.answers = new_answers
    }

    fn count(&self) -> usize {
        let characterSets = self.answers.clone().into_iter().map(|answer|
            answer.clone().chars().into_iter().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();

        let distinctChars = characterSets.clone().into_iter()
            .fold(HashSet::new(), |mut chars, more_chars| {
                chars.extend(more_chars);
                chars
            });

        distinctChars
            .into_iter()
            .filter(|character| characterSets.clone().into_iter().all(|set| set.contains(&character)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::Group;
    #[test]
    fn test_group() {
        vec!(
            ("", 0),
            ("abc", 3),
            ("aaa", 1),
            ("aba", 2))
            .into_iter()
            .for_each(|(answers, expected)| assert_eq!(Group::new(answers.to_string()).count(), expected))
    }

    #[test]
    fn test_include() {
        let mut group: Group = Group::new("aba".to_string());
        group.include("bc".to_string());
        assert_eq!(group.count(), 1)
    }


}


pub fn run() {

    let res: usize = lines().unwrap()
        .into_iter()
        .fold(vec!(Group::empty()), |mut groups, line| {
            if(line.is_empty()) {
                groups.push(Group::empty());
                groups
            } else {
                groups.last_mut().map(|group| group.include(line));
                groups
            }
        })
        .into_iter()
        .map(|group| group.count())
        .sum();
    println!("{:?}", res)
}
