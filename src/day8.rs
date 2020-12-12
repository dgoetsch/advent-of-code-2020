use super::lines;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize)
}

#[derive(Debug, Clone)]
enum Error {
    Parse(String)
}

impl Instruction {
    fn parse(line: String) -> Result<Instruction, Error> {
        let mut parts = line.trim().split_whitespace();

        match (parts.next(), parts.next().and_then(|m| m.parse::<isize>().ok())) {
            (Some("nop"), Some(modifier)) => Ok(Instruction::Nop(modifier)),
            (Some("acc"), Some(modifier)) => Ok(Instruction::Acc(modifier)),
            (Some("jmp"), Some(modifier)) => Ok(Instruction::Jmp(modifier)),
            _ => Err(Error::Parse(format!("Could not parse line '{}'", line)))
        }
    }
}

#[derive(Debug, Clone)]
struct Accumulator {
    instructions: Vec<Instruction>,
    processedOffsets: HashSet<usize>,
    accummulator: isize,
    offset: usize
}

impl Accumulator {
    fn new(instuctions: Vec<Instruction>) -> Accumulator {
        Accumulator {
            instructions: instuctions,
            processedOffsets: HashSet::new(),
            accummulator: 0,
            offset: 0
        }
    }
    // fn instruction(&self) -> Option<&Instruction> {
    //     self.instructions.get(self.offset)
    // }

    fn accumulate(&self, instruction: &Instruction) -> isize {
        match instruction {
            Instruction::Acc(add) => self.accummulator + add,
            _ => self.accummulator
        }
    }

    fn next_offset(&self, instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Jmp(add) =>
                if(add.is_positive()) {
                    self.offset + (*add as usize)
                } else {
                    self.offset - (-*add as usize)
                },
            _ => self.offset + 1
        }
    }

    fn should_continue(&self) -> bool {
        !self.processedOffsets.contains(&self.offset) && self.offset < self.instructions.len()
    }
    fn until_repeat(&mut self) -> bool {
        while(self.should_continue()) {
            self.processedOffsets.insert(self.offset);
            let instruction = self.instructions.get(self.offset).unwrap();
            self.accummulator = self.accumulate(instruction);
            self.offset = self.next_offset(instruction);
        }

        self.offset < self.instructions.len()
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_accumulate_loop() {
        let instructions = lines("./test.txt").unwrap().into_iter()
            .flat_map(Instruction::parse)
            .collect::<Vec<Instruction>>();

        let mut accumulator = Accumulator::new(instructions.clone());
        let loop_detected = accumulator.until_repeat();
        assert!(loop_detected);
        assert_eq!(accumulator.accummulator, 5)

    }

    #[test]
    fn test_accumulate_no_loop() {
        let instructions = lines("./test-2.txt").unwrap().into_iter()
            .flat_map(Instruction::parse)
            .collect::<Vec<Instruction>>();

        let mut accumulator = Accumulator::new(instructions.clone());
        let loop_detected = accumulator.until_repeat();
        assert!(!loop_detected);
        assert_eq!(accumulator.accummulator, 8)

    }


}

fn run() {
    let instructions = lines("./input.txt").unwrap().into_iter()
        .flat_map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    (0..instructions.len()).into_iter().for_each(|idx| {
        let mut these_instructions = instructions.clone();
        these_instructions[idx] = Instruction::Nop(0);
        let mut accumulator = Accumulator::new(these_instructions);
        let loop_found = accumulator.until_repeat();
        if(!loop_found) {
            println!("{:?}", accumulator.accummulator)
        }
    });
}