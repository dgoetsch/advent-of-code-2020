use regex::internal::Input;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Field {
    name: String,
    ranges: Vec<ValidRange>
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ValidRange {
    min: usize,
    max: usize
}

impl Field {
    fn new(line: String) -> Option<Field> {
        let mut parts = line.split(':');
        let name = parts.next().map(|n| n.trim());
        let ranges = parts.next().into_iter().flat_map(|ranges| {
            ranges.split("or").flat_map(|r| {
                let mut parts = r.split('-');
                let low = parts.next().and_then(|low| low.trim().parse::<usize>().ok());
                let high = parts.next().and_then(|high| high.trim().parse::<usize>().ok());
                low.and_then(|l| high.map(|h| ValidRange { min: l, max: h }))
            })
        }).collect::<Vec<ValidRange>>();
        if ranges.is_empty() {
            None
        } else {
            name.map(|n| Field { name: n.to_string(), ranges: ranges })
        }
    }

    fn is_valid(&self, number: usize) -> bool {
        self.ranges.clone().into_iter().any(|r| r.max >= number && r.min <= number)
    }
}

fn load_fields(file: &str) -> Vec<Field> {
    super::lines(file).unwrap().into_iter().flat_map(Field::new).collect()
}

fn load_answers(file: &str) -> Vec<Vec<usize>> {
    super::lines(file).unwrap().into_iter()
        .skip_while(|l| l != "nearby tickets:")
        .map(|l| l.split(',').into_iter().flat_map(|n| n.parse::<usize>().ok()).collect::<Vec<usize>>())
        .filter(|l| !l.is_empty())
        .collect()
}

fn find_illegal_answers(fields: Vec<Field>, answers: Vec<Vec<usize>>) -> Vec<usize> {
    answers.into_iter().flat_map(|survery|
        survery.into_iter().filter(|v|
            !fields.clone().into_iter().any(|f| f.is_valid(*v))
        )
    ).collect()
}

fn only_legal_answers(fields: Vec<Field>, answers: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    answers.into_iter().filter(|survery|
        survery.into_iter().all(|v|
            fields.clone().into_iter().any(|f| f.is_valid(*v))
        )
    ).collect()
}

fn load_my_answers(file: &str) -> Vec<usize> {
    let mut lines = super::lines(file).unwrap().into_iter()
        .skip_while(|l| l != "your ticket:");
    lines.next();
    lines.next().map(|l|
        l.split(',').into_iter()
            .flat_map(|a| a.parse::<usize>().ok())
            .collect::<Vec<usize>>()
    ).unwrap_or(vec!())


}

fn part_1(file: &str) -> usize {
    let fields = load_fields(file);
    let answers = load_answers(file);

    let illegals = find_illegal_answers(fields, answers);

    illegals.into_iter().sum()
}

struct Calculator {
    fields: Vec<Field>,
    answers: Vec<Vec<usize>>
}

impl Calculator {
    fn new(file: &str) -> Calculator {
        let mut calc = Calculator {
            fields: load_fields(file),
            answers: load_answers(file)
        };

        calc.answers = only_legal_answers(calc.fields.clone(), calc.answers.clone());
        calc
    }

    fn find_fields(&self) -> HashMap<String, usize> {
        let answer_count = self.answers.clone().into_iter().map(|a| a.len()).max().unwrap_or(0);

        let possibles: HashMap<String, Vec<usize>> = self.rule_out()
            .into_iter()
            .map(|(k, v)|
                     (k, (0..answer_count).into_iter().filter(|a| !v.contains(a)).collect())
            )
            .collect();
        Calculator::answers(possibles)
    }

    fn answers(possibilities: HashMap<String, Vec<usize>>) -> HashMap<String, usize> {
        if(possibilities.is_empty()) {
            return HashMap::new();
        }
        let mut immediate_answers: HashMap<String, usize> = possibilities.clone().into_iter()
            .filter(|(k, v)| v.len() == 1)
            .flat_map(|(k, v)| v.get(0).map(|a| (k, a.clone())))
            .collect();
        let answered: HashSet<usize> = immediate_answers.clone().into_iter().map(|(_, v)| v).collect();
        if immediate_answers.is_empty() {
            return HashMap::new();
        }
        let next_possibilities: HashMap<String, Vec<usize>> = possibilities.clone().into_iter()
            .filter(|(k, v)| !immediate_answers.contains_key(k))
            .map(|(k, v)| (k, v.into_iter().filter(|v|
                !answered.contains(v)
            ).collect::<Vec<usize>>()))
            .collect();

        let remainingAnswers = Calculator::answers(next_possibilities);
        immediate_answers.extend(remainingAnswers);
        immediate_answers
    }

    fn rule_out(&self) -> HashMap<String, HashSet<usize>> {
        let mut result: HashMap<String, HashSet<usize>> = HashMap::new();
        self.fields.clone().into_iter().for_each(|f| {
            result.insert(f.name.clone(), HashSet::new());
        });

        self.answers.clone().into_iter().for_each(|answers|{
            (0..answers.len()).into_iter().for_each(|idx|{
                let answer = answers.clone()[idx];
                self.fields.clone().into_iter().for_each(|field|
                    if(!field.is_valid(answer)) {
                        let mut illegals = result.get(&field.name)
                            .map(|i| i.clone())
                            .unwrap_or(HashSet::new());
                        illegals.insert(idx);
                        result.insert(field.name, illegals.clone());
                    }
                )
            })
        });
        result
    }
}
pub fn run() {
    println!("sum of illegal answers is {}", part_1("day-16-input.txt"));
    let calculator = Calculator::new("day-16-input.txt");
    let fields = calculator.find_fields();
    println!("fields {:?}", fields);



    let my_answers = load_my_answers("day-16-input.txt");
    println!("my answers {:?}", my_answers);

    let result = fields.into_iter()
        .filter(|(name, idx)| name.starts_with("departure"))
        .flat_map(|(_, idx)| my_answers.get(idx))
        .fold(1, |prod, next| prod * next);
    println!("The product of duration fields is {}", result);
}
#[cfg(test)]
mod test {
    use crate::day16::{part_1};
    use std::collections::HashMap;

    #[test]
    fn test_sum_illegal_answers() {
        let sum = part_1("day-16-test.txt");

        assert_eq!(sum, 71)
    }

    #[test]
    fn test_only_legal() {
        let fields = super::load_fields("day-16-test.txt");
        let answers = super::load_answers("day-16-test.txt");
        let legals = super::only_legal_answers(fields, answers);
        assert_eq!(legals, vec!(vec!(7, 3, 47)))
    }

    #[test]
    fn find_survey_structure() {
        let calculator = super::Calculator::new("day-16-test-2.txt");
        let result = calculator.find_fields();
        let expected: HashMap<String, usize> = vec!(("class", 1), ("row", 0), ("seat", 2)).into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        assert_eq!(result, expected)
    }

}