use std::collections::HashMap;

fn memory_game(target_idx: usize, initial_numbers: Vec<usize>) -> usize {
    let mut numbers: HashMap<usize, usize> = HashMap::new();
    let result = (0..target_idx).into_iter().fold(None, |spoken, idx| {
        let result = initial_numbers.get(idx)
            .map(|&i| i)
            .or_else(|| spoken
                .and_then(|s| numbers.get(&s))
                .map(|prev_idx| { idx - prev_idx }))
            .or(Some(0));
        spoken.map(|s| {
            numbers.insert(s, idx)
        });
        result
    });

    result.unwrap()
}
pub fn run() {
    println!("the result is {}", memory_game(30000000, vec!(6,4,12,1,20,0,16)))
}
#[cfg(test)]
mod test {
    use crate::day15::memory_game;

    fn test_memory_game_result(input: Vec<usize>, expected: usize) {
        assert_eq!(memory_game(2020, input), expected)
    }
    #[test]
    fn test_results() {
        test_memory_game_result(vec!(1,3,2), 1);
        test_memory_game_result(vec!(2,1,3), 10);
        test_memory_game_result(vec!(1,2,3), 27);
        test_memory_game_result(vec!(2,3,1), 78);
        test_memory_game_result(vec!(3, 2, 1), 438);
        test_memory_game_result(vec!(3,1,2), 1836);
    }
}