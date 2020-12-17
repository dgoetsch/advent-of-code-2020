fn calculate_delay(bus_id: usize, after_time: usize) -> usize {
    bus_id - after_time % bus_id
}

use super::lines;
fn earliest_bus(file: &str) -> (usize, usize) {
    let lines = lines(file).unwrap();
    let arrival_time = lines.get(0).and_then(|l| l.parse::<usize>().ok()).unwrap_or(0);
    let mut buses_and_delays = lines.get(1)
        .into_iter()
        .flat_map(|l| l.split(','))
        .flat_map(|id| id.parse::<usize>().ok())
        .map(|bus_id| (calculate_delay(bus_id, arrival_time), bus_id))
        .collect::<Vec<(usize, usize)>>();
    buses_and_delays.sort();
    println!("{:?}", buses_and_delays);
    buses_and_delays.first().unwrap_or(&(0, 0)).clone()
}

fn find_earliest_timestamp_that_matches_file(file: &str) -> usize {

    let lines = lines(file).unwrap();
    let pattern = lines.get(1).into_iter().flat_map(|l| l.split(',')).collect::<Vec<&str>>();
    find_earliest_timestamp_that_matches(pattern)

}

fn find_earliest_timestamp_that_matches(pattern: Vec<&str>) -> usize {
    println!("find for {:?}", pattern.clone());
    let pattern = (0..pattern.len()).into_iter().flat_map(|idx| {
        pattern.get(idx).and_then(|p| p.parse::<usize>().ok())
            .map(|bus_id| (bus_id, idx + 1))
    }).collect::<Vec<(usize, usize)>>();

    let mut pattern_sort = pattern.clone();
    pattern_sort.sort();
    let (longest_delay, longest_pos) = pattern_sort.last().unwrap_or(&(0, 0)).clone();
    let (second_longest_delay, second_longest_pos) = pattern_sort.get(pattern_sort.len() - 2).unwrap_or(&(0, 0)).clone();
    let (first_delay, _) = pattern.first().unwrap_or(&(0, 0)).clone();
    let (second_delay, second_pos) = pattern.get(1).unwrap_or(&(0, 0)).clone();
    (0..usize::MAX).into_iter()
        .map(|idx| {
            //Optimization: Only check on times where the longest interval matches
            let offset = (idx * longest_delay);
                if(offset > longest_pos) {
                    offset - longest_pos
                } else {
                    idx
                }
        })
        .filter(|t| *t > 0)
        .filter(|time| {
            calculate_delay(second_longest_delay, *time) == second_longest_pos
        })
        .filter(|time| {
            calculate_delay(first_delay, *time) == 1
        })
        .filter(|time| {
            calculate_delay(second_delay, *time) == second_pos
        })
        .find(|time| {
            println!("calculating at {}", time);
            pattern.clone().into_iter().all(|(bus_id, delay)|
                calculate_delay(bus_id, *time)  == delay
            )
        })
        .map(|t| t + 1)
        .unwrap_or(0)

}
pub fn run() {
    let (delay, bus_id) = earliest_bus("day-13-input.txt");
    println!("The next bus is {} which will arrive in {}", bus_id, delay);

    let timestamp = find_earliest_timestamp_that_matches_file("day-13-input.txt");
    println!("Earliest timestamp matching pattern ins {}", timestamp)
}

fn compute_first_valid_schedule(first: usize, next: usize, offset: usize) -> usize {
    first * (first - (next + 1)) * offset
}
#[cfg(test)]
mod tests {
    use super::earliest_bus;
    use super::find_earliest_timestamp_that_matches;
    use crate::day13::find_earliest_timestamp_that_matches_file;

    use super::compute_first_valid_schedule;

    #[test]
    fn test_scenario() {
        assert_eq!(earliest_bus("day-13-test.txt"), (5, 59))
    }
    #[test]
    fn test_find_matching_pattern() {
        assert_eq!(find_earliest_timestamp_that_matches_file("day-13-test.txt"), 1068781);
        assert_eq!(find_earliest_timestamp_that_matches("17,x,13,19".split(',').collect()), 3417);
        assert_eq!(find_earliest_timestamp_that_matches("67,7,59,61".split(',').collect()), 754018);
        assert_eq!(find_earliest_timestamp_that_matches("67,x,7,59,61".split(',').collect()), 779210);
        assert_eq!(find_earliest_timestamp_that_matches("67,7,x,59,61".split(',').collect()), 1261476);
        assert_eq!(find_earliest_timestamp_that_matches("1789,37,47,1889".split(',').collect()), 1202161486);
    }



    #[test]
    fn test_theorem() {
        assert_eq!(find_earliest_timestamp_that_matches("17,13".split(',').collect()), compute_first_valid_schedule(17, 13, 1));
        assert_eq!(find_earliest_timestamp_that_matches("17,x,13".split(',').collect()), compute_first_valid_schedule(17, 13, 2));
        assert_eq!(find_earliest_timestamp_that_matches("17,x,x,13".split(',').collect()), compute_first_valid_schedule(17, 13, 3));
        assert_eq!(find_earliest_timestamp_that_matches("17,x,x,x,13".split(',').collect()), compute_first_valid_schedule(17, 13, 4));
    }


    #[test]
    fn test() {
        assert_eq!(6, super::calculate_delay(7, 939));
        assert_eq!(5, super::calculate_delay(59, 939));
        assert_eq!(10, super::calculate_delay(13, 939));
    }

}