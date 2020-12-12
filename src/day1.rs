use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn set_factorial(depth: usize, numbers: Vec<i32>) -> Vec<Vec<i32>> {
    if(depth == 0) {
        return vec!()
    }
    if(depth == 1) {
        return numbers.into_iter().map(|n| vec!(n)).collect()
    }
    let numbers_len = numbers.len();
    numbers.clone().into_iter().flat_map(|n|
        numbers.clone()
            .into_iter()
            .position(|p| p == n)
            .map(|idx| idx + 1)
            .filter(|idx| idx < &numbers_len.clone())
            .map(|idx|
               set_factorial(depth - 1, numbers.clone().split_at(idx).1.to_vec())
                        .into_iter()
                        .map(|result_set| {
                            let mut base = vec!(n.clone());
                            base.extend(result_set);
                            base
                        })
                   .filter(|result_set| result_set.len() == depth)
                   .collect::<Vec<Vec<i32>>>()
            )
            .unwrap_or(vec!())
    ).collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {
    use super::set_factorial;

    #[test]
    fn test_accumulate_all() {
       assert_eq!(set_factorial(3, vec!(1, 2, 3, 4, 5)), vec!(
           vec!(1, 2, 3), vec!(1, 2, 4), vec!(1, 2, 5),
           vec!(1, 3, 4), vec!(1, 3, 5), vec!(1, 4, 5),
           vec!(2, 3, 4), vec!(2, 3, 5), vec!(2, 4, 5),
           vec!(3, 4, 5)))
    }

}


pub fn run() {
    let possible_sums: HashSet<i32> = HashSet::from_iter(vec!(2020).into_iter());
    read_lines("./day-1-input.txt")
    .map(|lines| {
        lines
            .flat_map(|r| r.into_iter())
            .flat_map(|line| line.parse::<i32>().into_iter())
            .collect::<Vec<i32>>()
    })
        .map(|numbers| {

            set_factorial(3, numbers)
                .into_iter()
                .filter(|result| {
                    let sum: i32 = result.clone().into_iter().sum();
                    possible_sums.clone().contains(&sum)
                })
                .for_each(|result| {
                    println!("Found {:?}", result.clone());
                    let product = result.into_iter().fold(1, |i, n| i * n);
                    println!("Its product is {}", product);
                });
        });


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}