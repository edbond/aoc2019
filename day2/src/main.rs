// https://adventofcode.com/2019/day/2

use std::fs;

// Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input)
// to the "1202 program alarm" state it had just before the last computer caught fire.
// To do this, before running the program, replace position 1 with the value 12 and replace position 2
// with the value 2. What value is left at position 0 after the program halts?
fn main() {
    let input = fs::read_to_string("input.txt").expect("input program read");

    part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) {
    let mut numbers: Vec<i64> = str::split(&input, ",")
        .map(|n| n.parse::<i64>().expect("number parsed"))
        .collect();

    numbers[1] = 12;
    numbers[2] = 2;

    println!("original program: {:?}", numbers);

    run_program(&mut numbers);

    println!("completed program: {:?}", numbers);
    println!("first element: {:?}", numbers[0])
}

struct ProgramState {
    memory: Vec<i64>,
    ip: usize,
}

struct Instruction {
    opcode: InstructionType,
    params: Vec<i64>,
}

enum InstructionType {
    ADD,
    MULTIPLY,
    HALT,
}

impl ProgramState {
    fn next_instruction(&mut self) -> Option<Instruction> {
        let opcode = self.memory[self.ip];
        match opcode {
            1 => {
                let a = self.memory[self.ip + 1];
                let b = self.memory[self.ip + 2];
                let c = self.memory[self.ip + 3];

                self.ip += 4;
                Some(Instruction {
                    opcode: InstructionType::ADD,
                    params: vec![a, b, c],
                })
            }
            2 => {
                let a = self.memory[self.ip + 1];
                let b = self.memory[self.ip + 2];
                let c = self.memory[self.ip + 3];

                self.ip += 4;
                Some(Instruction {
                    opcode: InstructionType::MULTIPLY,
                    params: vec![a, b, c],
                })
            }
            99 => {
                self.ip += 1;
                Some(Instruction {
                    opcode: InstructionType::HALT,
                    params: vec![],
                })
            }
            _ => None,
        }
    }
}

fn part2(input: String) {
    let numbers: Vec<i64> = str::split(&input, ",")
        .map(|n| n.parse::<i64>().expect("number parsed"))
        .collect();
    let mut state = ProgramState {
        memory: numbers.clone(),
        ip: 0,
    };

    const TARGET: usize = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            state.memory = numbers.clone();
            state.memory[1] = noun;
            state.memory[2] = verb;
            state.ip = 0;

            loop {
                match state.next_instruction() {
                    Some(instruction) => match instruction.opcode {
                        InstructionType::ADD => {
                            let a = instruction.params[0] as usize;
                            let b = instruction.params[1] as usize;
                            let c: usize = instruction.params[2] as usize;

                            state.memory[c] = state.memory[a] + state.memory[b];
                        }
                        InstructionType::MULTIPLY => {
                            let a = instruction.params[0] as usize;
                            let b = instruction.params[1] as usize;
                            let c = instruction.params[2] as usize;

                            state.memory[c] = state.memory[a] * state.memory[b];
                        }
                        InstructionType::HALT => {
                            break;
                        }
                    },
                    None => {
                        panic!("invalid opcode");
                    }
                }
            }

            if state.memory[0] == TARGET as i64 {
                println!("noun: {}, verb: {}", noun, verb);
                println!("100 * noun + verb: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn run_program(numbers: &mut Vec<i64>) {
    let mut i = 0;

    loop {
        match numbers[i] {
            1 => {
                let a = numbers[i + 1] as usize;
                let b = numbers[i + 2] as usize;
                let c = numbers[i + 3] as usize;

                numbers[c] = numbers[a] + numbers[b];

                i += 4;
            }
            2 => {
                let a = numbers[i + 1] as usize;
                let b = numbers[i + 2] as usize;
                let c = numbers[i + 3] as usize;

                numbers[c] = numbers[a] * numbers[b];

                i += 4;
            }
            99 => {
                break;
            }
            _ => {
                panic!("invalid opcode");
            }
        }
    }
}
