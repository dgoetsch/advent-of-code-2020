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
    read_lines("./day-7-input.txt")
        .map(|lines|
            lines
                .flat_map(|l| l.into_iter())
                .collect::<Vec<String>>()
        )
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct AllowedBag {
    count: usize,
    color: String
}


impl AllowedBag {
    fn from(description: String) -> Option<AllowedBag> {
        let mut parts = description.split_whitespace();
        let count = parts.next().and_then(|count| count.parse::<usize>().ok());
        let color = parts.fold("".to_string(), |soFar, next| {
            if(next == "bag" || next == "bags") {
                soFar
            } else if(soFar.is_empty()) {
                next.to_string()
            } else {
                soFar + " " + next
            }
        });

        count.map(|c| {
            AllowedBag { count: c, color: color }
        })
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Rules {
    allowed_bags: HashMap<String, Vec<AllowedBag>>
}

impl Rules {

    fn empty() -> Rules {
        Rules { allowed_bags: HashMap::new() }
    }
    fn include(&mut self, line: String) {
        let (color, new_rules) = Rules::rule(line);

        match self.allowed_bags.get_mut(color.as_str()) {
            Some(rules) => {
                rules.extend(new_rules);
            },
            None => {
                self.allowed_bags.insert(color, new_rules);
            }
        };
    }
    fn rule(line: String) -> (String, Vec<AllowedBag>) {
        let mut parts = line.split("contain");
        let color = parts.next().unwrap_or("UNKNOWN_COLOR");
        let color = color.clone()
            .trim()
            .strip_suffix("bags")
            .unwrap_or(color.clone().strip_suffix("bag").unwrap_or(color))
            .trim();

        let rules = parts.next().unwrap_or("");

        let allowed_bags = rules.clone()
            .strip_prefix("s").unwrap_or(rules.clone())
            .trim().strip_suffix(".").unwrap_or(rules)
            .split(',')
            .map(|rule| {
                rule.trim().to_string()
            })
            .flat_map(|rule| AllowedBag::from(rule.trim().to_string()).into_iter())
            .collect::<Vec<AllowedBag>>();

        (color.trim().to_string(), allowed_bags)
    }

    fn colors_that_contain(&self, color: String) -> HashSet<String> {
        self.allowed_bags.clone()
            .into_iter()
            .filter(|(container_color, rules)| {
                rules.clone().into_iter().any(|rule| rule.color == color)
            })
            .map(|(c, _)| c)
            .collect()
    }

    fn colors_that_contain_rec(&self, color: String, found_so_far: HashSet<String>) -> HashSet<String> {
        let parent_containers = self.colors_that_contain(color);

        let mut new_found_so_far = parent_containers.clone();
        new_found_so_far.extend(found_so_far);

        parent_containers.clone().into_iter().fold(new_found_so_far, |found, container_color| {
            self.colors_that_contain_rec(container_color, found)
        })
    }

    fn count_contents(&self, color: String) -> usize {
        let def = vec!();
        let children = self.allowed_bags.get(color.as_str()).unwrap_or(&def);

        let children_count = children.into_iter()
            .map(|allowed| self.count_contents(allowed.color.clone()) * allowed.count);


        children_count.sum::<usize>() + 1
    }
}

// fn parseLine(line: String) -> Map<String, Map<Int ->>

pub fn run() {
    let mut rules = Rules::empty();
    lines().unwrap()
        .into_iter()
        .for_each(|line| rules.include(line));

    let possible_bags = rules.colors_that_contain_rec("shiny gold".to_string(), HashSet::new());
    println!("Hello, world! Shiny Gold can be contained by {}", possible_bags.len());
    let child_count = rules.count_contents("shiny gold".to_string());
    println!("Shiny Gold contains {}", child_count);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_allowed() {
        assert_eq!(
            AllowedBag::from("3 bright yellow bags".to_string()),
            Some(AllowedBag {
                count: 3,
                color: "bright yellow".to_string()
            })
        )
    }

    #[test]
    fn reads_allowed_one_word_color() {
        assert_eq!(
            AllowedBag::from("1 yellow bag".to_string()),
            Some(AllowedBag {
                count: 1,
                color: "yellow".to_string()
            })
        )
    }

    #[test]
    fn reads_fails_bad_number() {
        assert_eq!(
            AllowedBag::from("seven yellow bags".to_string()),
            None
        )
    }

    #[test]
    fn read_rule() {
        assert_eq!(
            Rules::rule("light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()),
            ("light red".to_string(), vec!(
                AllowedBag { count: 1, color: "bright white".to_string() },
                AllowedBag { count: 2, color: "muted yellow".to_string() }
            ))
        );

    }

    #[test]
    fn include_rules() {
        let mut rules = Rules::empty();
        rules.include("light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string());
        rules.include("olive green bags contain 1 beige bag, 2 sky blue bags.".to_string());
        rules.include("light red bags contain 1 burnt orange bag.".to_string());
        assert_eq!(
            rules,
            Rules { allowed_bags: vec!(
                ("light red".to_string(), vec!(
                    AllowedBag { count: 1, color: "bright white".to_string() },
                    AllowedBag { count: 2, color: "muted yellow".to_string() },
                    AllowedBag { count: 1, color: "burnt orange".to_string() }
                )),
                ("olive green".to_string(), vec!(
                    AllowedBag { count: 1, color: "beige".to_string() },
                    AllowedBag { count: 2, color: "sky blue".to_string() }
                ))).into_iter().collect()
            }
        );
    }

    #[test]
    fn allowed_colors() {
        let mut rules = Rules::empty();
        rules.include("light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string());
        rules.include("olive green bags contain 1 beige bag, 2 sky blue bags.".to_string());
        rules.include("light red bags contain 1 burnt orange bag.".to_string());
        rules.include("drab blue bags contain 1 burnt orange bag, 2 sky blue bags.".to_string());
        let sky_blues = rules.colors_that_contain("sky blue".to_string());
        let burnt_orange = rules.colors_that_contain("burnt orange".to_string());
        assert_eq!(
            sky_blues,
            vec!("olive green".to_string(), "drab blue".to_string()).into_iter().collect::<HashSet<String>>()
        );
        assert_eq!(
            burnt_orange,
            vec!("light red".to_string(), "drab blue".to_string()).into_iter().collect::<HashSet<String>>()
        );
    }

    #[test]
    fn colors_that_contain_rec() {
        let mut rules = Rules::empty();
        rules.include("light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string());
        rules.include("olive green bags contain 1 beige bag, 2 sky blue bags.".to_string());
        rules.include("light red bags contain 1 burnt orange bag.".to_string());
        rules.include("drab blue bags contain 1 burnt orange bag, 2 sky blue bags.".to_string());
        rules.include("hot pink bags contain 7 olive green bags".to_string());
        rules.include("light grey bags contain 4 light red bags".to_string());

        let sky_blues = rules.colors_that_contain_rec("sky blue".to_string(), HashSet::new());
        let burnt_orange = rules.colors_that_contain_rec("burnt orange".to_string(), HashSet::new());

        assert_eq!(
            sky_blues,
            vec!("olive green".to_string(), "drab blue".to_string(), "hot pink".to_string()).into_iter().collect::<HashSet<String>>()
        );
        assert_eq!(
            burnt_orange,
            vec!("light red".to_string(), "drab blue".to_string(), "light grey".to_string()).into_iter().collect::<HashSet<String>>()
        );
    }

    #[test]
    fn count_children() {
        let mut rules = Rules::empty();
        rules.include("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string());
        rules.include("faded blue bags contain 0 other bags.".to_string());
        rules.include("dotted black bags contain 0 other bags.".to_string());
        rules.include("vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.".to_string());
        rules.include("dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.".to_string());

        assert_eq!(rules.count_contents("shiny gold".to_string()), 33);
    }

    #[test]
    fn count_children_2() {
        let mut rules = Rules::empty();
        rules.include("shiny gold bags contain 2 dark red bags.".to_string());
            rules.include("dark red bags contain 2 dark orange bags.".to_string());
            rules.include("dark orange bags contain 2 dark yellow bags.".to_string());
            rules.include("dark yellow bags contain 2 dark green bags.".to_string());
            rules.include("dark green bags contain 2 dark blue bags.".to_string());
            rules.include("dark blue bags contain 2 dark violet bags.".to_string());
            rules.include("dark violet bags contain no other bags.".to_string());

        assert_eq!(rules.count_contents("shiny gold".to_string()), 127);
    }
}