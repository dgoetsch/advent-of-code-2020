use crate::day19::Rule::Sequence;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Rule {
    Literal(char),
    Or(Box<Rule>, Box<Rule>),
    Sequence(Vec<usize>)
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum RuleError {
 DoesNotMatch(char),
    NoSuchRule(usize),
    NoData(usize),
    HasMore(usize),
    Aggregate(Vec<RuleError>)
}
impl Rule {
    fn parse(rule: &str) -> Option<Rule> {
        let rule = rule.trim();
        if rule.starts_with('"') && rule.ends_with('"') && rule.len() == 3 {
            rule.chars().nth(1).map(|c| Rule::Literal(c.clone()))
        } else if rule.contains('|') {
            let mut parts = rule.split('|');
            let head = parts.next().and_then(Rule::parse);
            let tail = parts.collect::<Vec<&str>>().join("|");
            let tail = Rule::parse(tail.as_str());

            match (head, tail) {
                (Some(r1), Some(r2)) => Some(Rule::Or(Box::new(r1), Box::new(r2))),
                (Some(r1), None) => Some(r1),
                _ => None
            }

        } else {
            let allowed = rule.split_whitespace().flat_map(|s| s.parse::<usize>().ok()).collect::<Vec<usize>>();
            if(allowed.is_empty()) {
                None
            } else {
                Some(Rule::Sequence(allowed))
            }
        }
    }


}

struct Rules {
    rules: HashMap<usize, Rule>
}

impl Rules {
    fn from(file: &str) -> Rules {
        Rules {
            rules: super::lines(file).unwrap()
                .into_iter()
                .take_while(|l| !l.is_empty())
                .flat_map(|l| {
                    let mut parts = l.split(':');
                    let idx = parts.next().and_then(|i| i.trim().parse::<usize>().ok());
                    let rule = Rule::parse(parts.collect::<Vec<&str>>().join(":").as_str());
                    match (idx, rule) {
                        (Some(idx), Some(rule)) => Some((idx, rule)),
                        _ => None
                    }
                })
                .collect::<HashMap<usize, Rule>>()
        }
    }

    fn evaluate(file: &str) -> Vec<Result<usize, RuleError>> {
        let rules = Rules::from(file);
        let data = super::lines(file)
            .unwrap()
            .into_iter()
            .skip_while(|l| !l.is_empty())
            .skip_while(|l| l.is_empty())
            .collect::<Vec<String>>();


        (0..data.len()).into_iter()
            .map(|idx| {
                data.get(idx)
                    .ok_or(RuleError::NoData(idx))
                    .and_then(|l| rules.valid(0, l.as_str()))
                    .and_then(|remaining|
                        if(remaining.into_iter().any(|r| r.is_empty())) {
                            Ok(idx)
                        } else {
                            Err(RuleError::HasMore(idx))
                        }
                    )

            })
            .collect()
    }

    fn valid(&self, idx: usize, value: &str) -> Result<Vec<String>, RuleError> {
        match self.rules.get(&idx) {
            Some(rule) => self.valid_rule(rule.clone(), value),
            None => Err(RuleError::NoSuchRule(idx))
        }
    }

    fn valid_rule(&self, rule: Rule, value: &str) -> Result<Vec<String>, RuleError> {
        match rule {
            Rule::Literal(c) =>
                value.strip_prefix(c.to_string().as_str()).map(|s| Ok(vec!(s.to_string())))
                    .unwrap_or(Err(RuleError::DoesNotMatch(c.clone()))),
            Rule::Or(r1, r2) =>
                match (
                    self.valid_rule(r1.deref().clone(), value),
                    self.valid_rule(r2.deref().borrow().clone(), value)) {
                    (Ok(r1), Ok(r2)) => {
                        let mut result = vec!();
                        result.extend(r1); result.extend(r2);
                        Ok(result)
                    },
                    (Ok(r1), Err(_)) => Ok(r1),
                    (Err(_), Ok(r2)) => Ok(r2),
                    (Err(e1), Err(e2)) => Err(RuleError::Aggregate(vec!(e1, e2)))
                },
            Rule::Sequence(rules) => rules.into_iter()
                .fold(Ok(vec!(value.to_string())), |res, rule_idx|
                    res.and_then(|strs| {
                        let results = strs.into_iter().map(|str|
                            self.valid(rule_idx.clone(), str.as_str()))
                            .collect::<Vec<Result<Vec<String>, RuleError>>>();

                        let result = results.clone().into_iter()
                            .flat_map(|r| r.ok())
                            .flatten()
                            .collect::<Vec<String>>();

                        if(result.is_empty()) {
                            Err(RuleError::Aggregate(results.into_iter()
                                .flat_map(|r| r.err())
                                .collect::<Vec<RuleError>>()))
                        } else {
                            Ok(result)
                        }
                    }))
        }
    }
}

pub fn run() {
    let count = Rules::evaluate("day-19-input.txt").into_iter().filter(|r| r.is_ok()).count();
    println!("There are {} valid rules", count)
}
#[cfg(test)]
mod test {
    #[test]
    fn test_file() {
        assert_eq!(
            super::Rules::evaluate("day-19-test.txt").into_iter().filter(|r| r.is_ok()).count(),
            2)
    }

    #[test]
    fn test_file_2() {
        assert_eq!(
            super::Rules::evaluate("day-19-test-2.txt").into_iter().filter(|r| r.is_ok()).count(),
            12)
    }

}
