use super::lines;

#[derive(Debug, Clone)]
struct Cypher {
    window: usize,
    records: Vec<usize>
}

impl Cypher {
    fn new(
        window: usize,
        records: Vec<usize>) -> Cypher {
        Cypher {
            window: window,
            records: records
        }
    }

    fn find_vulnerability(self) -> usize {
        let offenders = self.clone().offenders();
        let (low, high) = Cypher::find_range_that_sum_to(self.records.clone(), *offenders.first().unwrap()).unwrap();
        let range = self.records[low..high].to_vec();
        let low_val = range.clone().into_iter().min().unwrap() ;
        let high_val = range.into_iter().max().unwrap();

        low_val + high_val
    }

    fn find_range_that_sum_to(values: Vec<usize>, target: usize) -> Option<(usize, usize)> {
        (0..values.clone().len()).into_iter().find_map(|x|
            (x..values.clone().len()).into_iter().find_map(|y| {
                if(values[x..y].into_iter().sum::<usize>() == target) {
                    Some((x, y))
                } else {
                    None
                }
            } ))
    }

    fn offenders(self) -> Vec<usize> {
        (0..self.records.len()).into_iter().flat_map(|idx|
            if(idx < self.window) {
                None
            } else {
                if(Cypher::any_sum_to(self.records[idx-self.window..idx].to_vec(), self.records[idx])) {
                    None
                } else {
                    Some(self.records[idx])
                }
            }
        ).collect()
    }

    fn any_sum_to(any_of: Vec<usize>, to: usize) -> bool {
        any_of.clone().into_iter().any(|idx_1| any_of.clone().into_iter().any(|idx_2| idx_2 + idx_1 == to))
    }
}

fn create_cipher(window: usize, file: &str) -> Cypher {
    Cypher::new(
        window,
        lines(file).unwrap().into_iter().flat_map(|line| line.parse::<usize>().into_iter()).collect())
}

pub fn run() {
    println!("{:?}", create_cipher(25, "day-9-input.txt").find_vulnerability())

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_accumulate_loop() {
        let result: usize = 127;
        assert_eq!(create_cipher(5, "day-9-test.txt")
            .offenders().first(), Some(&result))
    }
    #[test]
    fn test_find_vult() {
        let result: usize = 62;
        assert_eq!(create_cipher(5, "day-9-test-2.txt").find_vulnerability(), result)
    }


}