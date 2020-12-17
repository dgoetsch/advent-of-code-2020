use std::collections::HashMap;
#[derive(Debug, Clone)]
enum Command {
    Set(Mask),
    Assign(Memory)
}
#[derive(Debug, Clone)]
struct Mask {
    mask: usize,
    float: Vec<usize>
}

impl Mask {
    fn new(mask: &str) -> Mask {
        let reversed = mask.chars().rev().collect::<String>();
        let mut mask = Mask { mask: 0, float: vec!() };

        (0..36).into_iter().for_each(|idx|
            match reversed.chars().nth(idx) {
                Some('1') => mask.mask = mask.mask | (1 << idx),
                Some('X') => mask.float.push(idx),
                 _ => ()
            }
        );

        mask
    }

    fn apply(&self, number: usize) -> Vec<usize> {
        let base = number | self.mask;
        self.float.clone().into_iter().fold(vec!(base), |so_far, idx| {
            let one = 1 << idx;
            let zero = !one;
            so_far.into_iter()
                .flat_map(|partial_masked| {
                    vec!(one | partial_masked, zero & partial_masked).into_iter()
                })
                .collect()
        })
    }
}

#[derive(Debug, Clone)]
struct Memory {
    index: usize,
    value: usize
}

impl Command {
    fn parse(line: String) -> Result<Command, String> {
        let parts = line.split("=").collect::<Vec<&str>>();
        let left = parts.get(0).map(|l| l.trim());
        let right = parts.get(1).map(|r| r.trim());
        match left {
            Some("mask") => right.map(Mask::new).map(|m| Command::Set(m)).ok_or("No mask defined".to_string()),
            Some(maybeMemory) => {
                let r = regex::Regex::new(r"mem\[([0-9]+)\]").unwrap();
                let memory = r.captures_iter(maybeMemory)
                    .flat_map(|s|{
                        //this could panic, had a hard time with these regex captures
                        s[1].parse::<usize>().into_iter()
                    })
                    .find(|_| true);
                let value = right.and_then(|r| r.parse::<usize>().ok());
                match (memory, value) {
                    (Some(m), Some(v)) => Ok(Command::Assign(Memory { index: m, value: v})),
                    (maybeM, maybeV) => Err(format!("Could not create Command from {}", line))
                }
            },
            None => Err("empty".to_string())
        }

    }
}

fn load(file: &str) -> Vec<Command> {
    super::lines(file).unwrap()
        .into_iter()
        .flat_map(|l| Command::parse(l).into_iter())
        .collect()
}


pub fn run() {
    println!("memory totals {}", count_memory("day-14-input.txt"));
}
fn count_memory(file: &str) -> usize {
    let mut numbers: HashMap<usize, usize> = HashMap::new();
    load(file)
        .into_iter()
        .fold(Mask::new(""), |mask, command| {
            match command {
                Command::Assign(m) => {
                    mask.apply(m.index).into_iter().for_each(| addr| {
                        numbers.insert(addr, m.value);
                    });

                    mask
                },
                Command::Set(new_mask) => new_mask
            }
        });

    numbers.into_iter().map(|(_, v)| v).sum()
}


#[cfg(test)]
mod test {
    use super::Mask;

    #[test]
    fn test_count() {
        let count = super::count_memory("day-14-test.txt");
        assert_eq!(count, 208);
    }
    #[test]
    fn test_load() {
        let cmds = super::load("day-14-input.txt");
        println!("{:?}", cmds);
        assert_eq!(cmds.len(), 569)
    }
}
