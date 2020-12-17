use std::collections::HashMap;
#[derive(Debug, Clone)]
enum Command {
    Set(Mask),
    Assign(Memory)
}
#[derive(Debug, Clone)]
struct Mask {
    mask: Vec<(usize,bool)>
}

impl Mask {
    fn new(mask: &str) -> Mask {
        let reversed = mask.chars().rev().collect::<String>();

        Mask {
            mask: (0..mask.len()).into_iter().flat_map(|idx|
                reversed.chars().nth(idx).into_iter().flat_map(move |c| match c {
                    '0' => Some((idx, false)),
                    '1' => Some((idx, true)),
                    _ => None
                })
            ).collect()
        }
    }

    fn apply(&self, number: usize) -> usize {
        let two: usize = 2;
        let thirtySixOnes: usize = (0..36).into_iter().fold(0, |s, idx| two.pow(idx));
        // let oneThen35Zeros = 2.pow(35);
        self.mask.clone().into_iter().fold(number, |partial_masked, (idx, bit_value)| {
            if bit_value {
                (1 << idx) | partial_masked
            } else {
                !(1 << idx) & partial_masked
            }
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
                    numbers.insert(m.index, mask.apply(m.value));
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
        assert_eq!(count, 165);
    }
    #[test]
    fn test_load() {
        let cmds = super::load("day-14-input.txt");
        println!("{:?}", cmds);
        assert_eq!(cmds.len(), 569)
    }

    #[test]
    fn bit_mask() {
        let mask = Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        // println!("expect {} << ")
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }
}
