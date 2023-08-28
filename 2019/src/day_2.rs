use crate::utils::*;

#[derive(Default, Debug, Clone)]
struct Computer {
    memory: Vec<i64>,
    cursor: usize,
    is_halted: bool,
}

impl Computer {
    fn new() -> Self {
        Computer {
            memory: Default::default(),
            cursor: 0,
            is_halted: false,
        }
    }

    fn tick(&mut self) {
        let pos = self.cursor;
        let operation = self.memory[pos];
        let dest = self.memory[pos + 3] as usize;
        let l = self.memory[pos + 1] as usize;
        let r = self.memory[pos + 2] as usize;
        match operation {
            1 => {
                self.memory[dest] = self.memory[l] + self.memory[r];
            }
            2 => self.memory[dest] = self.memory[l] * self.memory[r],
            99 => self.is_halted = true,
            _ => unreachable!(),
        };
        self.cursor = pos + 4;
    }

    fn run(&mut self) {
        while !self.is_halted {
            self.tick();
        }
    }
}

fn run(mut program: Vec<i64>, noun: i64, verb: i64) -> i64 {
    program[1] = noun;
    program[2] = verb;
    let mut computer = Computer::new();
    computer.memory = program;
    computer.run();
    return computer.memory[0];
}
pub fn part_one() -> i64 {
    let program = input_comma_ints(2);
    let memory = run(program.clone(), 12, 2);
    return memory;
}

pub fn part_two() -> i64 {
    let program = input_comma_ints(2);
    let output = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let input = program.clone();
            if run(input, noun, verb) == output {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_part_one() {
        assert_eq!(part_one(), 8017076);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(part_two(), 3146);
    }
}
