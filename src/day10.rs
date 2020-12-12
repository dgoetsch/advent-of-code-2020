use std::collections::HashMap;
use super::lines;

struct PathFinder {
    cache: HashMap<usize, usize>,
    count: usize
}

impl PathFinder {
    fn count_valid_combinations(&mut self, adapters: Vec<usize>) -> usize {
        self.count = self.count + 1;
        if adapters.len() == 1 {
            return 1;
        } else {
            return adapters.get(0)
                .map(|first|{
                    match self.cache.get(first) {
                        Some(count) => *count,
                        None => {
                            let result: usize = (0..3).into_iter()
                                .map(|idx| idx + 1)
                                .filter(|idx| adapters
                                    .get(*idx)
                                    .filter(|next| {
                                        let diff = *next - first;
                                        diff > 0 && diff < 4
                                    })
                                    .is_some()
                                )
                                .map(|idx| {
                                    self.count_valid_combinations(adapters[idx..adapters.len()].to_vec())
                                })
                                .sum();
                            self.cache.insert(*first, result);
                            result
                        }
                    }

                })
                .unwrap_or(0)
        }
    }
}
fn calculate_jolt_distributions(adapters: Vec<usize>) -> HashMap<usize, usize> {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    adapters.into_iter().fold(0, |prev, next| {
        let diff = next - prev;
        let count = counts.get(&diff).unwrap_or(&0);
        counts.insert(diff, count + 1);
        next
    });

    counts
}

fn is_valid(adapters: Vec<usize>) -> bool {
    !calculate_jolt_distributions(adapters)
        .into_iter()
        .map(|(diff, _)| diff)
        .any(|diff| diff > 3 || diff < 1)
}

fn count_valid_combinations(adapters: Vec<usize>) -> usize {
    if adapters.len() == 1 {
        return 1;
    } else {
        adapters.get(0)
            .map(|first|{
                (0..3).into_iter()
                    .map(|idx| idx + 1)
                    .filter(|idx| adapters
                        .get(*idx)
                        .filter(|next| {
                            let diff = *next - first;
                            diff > 0 && diff < 4
                        })
                        .is_some()
                    )
                    .map(|idx| {
                        count_valid_combinations(adapters[idx..adapters.len()].to_vec())
                    })
                    .sum()
            })
            .unwrap_or(0)
    }
}
fn sort_and_count_valid_combinations(adapters: Vec<usize>) -> usize {
    let mut sorted = adapters.clone();
    sorted.push(0);
    sorted.sort();
    let phone_jolts = sorted.last().unwrap() + 3;
    sorted.push(phone_jolts);
    let mut finder = PathFinder { cache: HashMap::new(),count: 0 };
    finder.count_valid_combinations(sorted)
}
fn sort_and_calculate_jolt_distribution(adapters: Vec<usize>) -> HashMap<usize, usize> {
    let mut sorted = adapters.clone();
    sorted.sort();
    match sorted.last() {
        Some(l) => sorted.push(l + 3),
        _ => ()
    }
    calculate_jolt_distributions(sorted)
}

fn calculate_distrubtions(file: &str) -> HashMap<usize, usize> {
    let adapters = lines(file).unwrap().into_iter()
        .flat_map(|line| line.parse::<usize>().into_iter())
        .collect::<Vec<usize>>();
    sort_and_calculate_jolt_distribution(adapters)
}

fn count_valid(file: &str) -> usize {
    let adapters = lines(file).unwrap().into_iter()
        .flat_map(|line| line.parse::<usize>().into_iter())
        .collect::<Vec<usize>>();
    sort_and_count_valid_combinations(adapters)
}
pub fn run() {
    println!("{:?}", count_valid("day-10.txt"));
}

#[cfg(test)]
mod tests {
    use super::calculate_distrubtions;
    use std::collections::HashMap;
    use crate::day10::count_valid_combinations;
    use super::count_valid;

    fn assert_distributions(file: &str, distributions: HashMap<usize, usize>) {
       let result = calculate_distrubtions(file);
        assert_eq!(distributions, result)
    }

    fn assert_count(file: &str, count: usize) {
        let result = count_valid(file);
        assert_eq!(result, count)
    }
    #[test]
    fn test_distributions() {
        assert_distributions("day-10-test-1.txt",
        vec!((1, 7), (3, 5)).into_iter().collect::<HashMap<usize, usize>>());
        assert_distributions("day-10-test-2.txt",
                             vec!((1, 22), (3, 10)).into_iter().collect::<HashMap<usize, usize>>())

    }

    #[test]
    fn count_combinations() {
        assert_count("day-10-test-1.txt", 8);
        assert_count("day-10-test-2.txt", 19208)
    }
}