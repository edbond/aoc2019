use anyhow::{anyhow, Result};

#[derive(Debug)]
pub(crate) struct ProgramState<'a> {
    memory: &'a mut Vec<i64>,
    memory_pos: usize,
    input: Vec<i64>,

    input_pos: usize,
    output: &'a mut Vec<i64>,
    output_pos: usize,
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
// Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
#[derive(Debug, PartialEq, Eq)]
enum InstructionType {
    ADD,
    MULTIPLY,
    HALT,
    READ,
    WRITE,
}

#[derive(Debug, PartialEq, Eq)]
enum InstructionArgumentMode {
    POSITION,
    IMMEDIATE,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: InstructionType,
    arg_modes: Vec<InstructionArgumentMode>,
}

impl Instruction {
    pub fn run(&self, state: &mut ProgramState) -> Result<bool> {
        println!("execute: {:?}", self);

        match self.opcode {
            InstructionType::ADD => {
                let a: i64;
                let b: i64;

                match self.arg_modes[0] {
                    InstructionArgumentMode::IMMEDIATE => {
                        a = state.memory[state.memory_pos + 1];
                    }
                    InstructionArgumentMode::POSITION => {
                        a = state.memory[state.memory[state.memory_pos + 1] as usize];
                    }
                }

                match self.arg_modes[1] {
                    InstructionArgumentMode::IMMEDIATE => {
                        b = state.memory[state.memory_pos + 2];
                    }
                    InstructionArgumentMode::POSITION => {
                        b = state.memory[state.memory[state.memory_pos + 2] as usize];
                    }
                }

                let c = state.memory[state.memory_pos + 3];

                state.memory[c as usize] = a + b;
                state.memory_pos += 4;

                Ok(true)
            }
            InstructionType::MULTIPLY => {
                let a: i64;
                let b: i64;

                match self.arg_modes[0] {
                    InstructionArgumentMode::IMMEDIATE => {
                        a = state.memory[state.memory_pos + 1];
                    }
                    InstructionArgumentMode::POSITION => {
                        a = state.memory[state.memory[state.memory_pos + 1] as usize];
                    }
                }

                match self.arg_modes[1] {
                    InstructionArgumentMode::IMMEDIATE => {
                        b = state.memory[state.memory_pos + 2];
                    }
                    InstructionArgumentMode::POSITION => {
                        b = state.memory[state.memory[state.memory_pos + 2] as usize];
                    }
                }

                let c = state.memory[state.memory_pos + 3];

                state.memory[c as usize] = a * b;
                state.memory_pos += 4;

                Ok(true)
            }
            InstructionType::READ => {
                let pos = state.memory[state.memory_pos + 1];
                // read one input
                let val = state.input[state.input_pos];

                println!(
                    "read value {:?} from input, storing in memory at address {:?}",
                    val, pos
                );

                state.memory[pos as usize] = val;
                state.input_pos += 1;
                state.memory_pos += 2;

                Ok(true)
            }
            InstructionType::WRITE => {
                let addr = state.memory[state.memory_pos + 1];
                let val = state.memory[addr as usize];

                println!(
                    "read value from address {:?}, storing value {:?} at address {:?} in output",
                    addr, val, state.output_pos
                );
                state.output[state.output_pos] = val;
                state.output_pos += 1;
                state.memory_pos += 2;

                Ok(true)
            }
            InstructionType::HALT => Ok(false),
        }
    }
}

// ABCDE
//  1002
// DE - two-digit opcode,      02 == opcode 2
//  C - mode of 1st parameter,  0 == position mode
//  B - mode of 2nd parameter,  1 == immediate mode
//  A - mode of 3rd parameter,  0 == position mode,
//                                   omitted due to being a leading zero
fn parse_instruction(code: i64) -> Result<Instruction, anyhow::Error> {
    let opcode = code % 100;

    let a = (code / 100) % 10;
    let b = (code / 1000) % 10;
    let c = (code / 10000) % 10;

    let mut arg_modes = vec![];
    arg_modes.push(match a {
        0 => InstructionArgumentMode::POSITION,
        1 => InstructionArgumentMode::IMMEDIATE,
        _ => return Err(anyhow!("unknown argument mode {}", a)),
    });
    arg_modes.push(match b {
        0 => InstructionArgumentMode::POSITION,
        1 => InstructionArgumentMode::IMMEDIATE,
        _ => return Err(anyhow!("unknown argument mode {}", b)),
    });
    arg_modes.push(match c {
        0 => InstructionArgumentMode::POSITION,
        1 => InstructionArgumentMode::IMMEDIATE,
        _ => return Err(anyhow!("unknown argument mode {}", c)),
    });

    match opcode {
        1 => Ok(Instruction {
            opcode: InstructionType::ADD,
            arg_modes,
        }),
        2 => Ok(Instruction {
            opcode: InstructionType::MULTIPLY,
            arg_modes,
        }),
        3 => Ok(Instruction {
            opcode: InstructionType::READ,
            arg_modes,
        }),
        4 => Ok(Instruction {
            opcode: InstructionType::WRITE,
            arg_modes,
        }),
        99 => Ok(Instruction {
            opcode: InstructionType::HALT,
            arg_modes,
        }),
        _ => Err(anyhow!("unknown instruction by code {}", code)),
    }
}

impl<'a> ProgramState<'a> {
    pub fn next_instruction(&mut self) -> Result<Instruction, anyhow::Error> {
        let instruction_code = self.memory[self.memory_pos];
        parse_instruction(instruction_code)
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.next_instruction() {
                Ok(instruction) => match instruction.run(self) {
                    Ok(cont) => {
                        if !cont {
                            break;
                        }
                    }
                    Err(e) => {
                        return Err(anyhow!(
                            "error running instruction {:?}, {:?}",
                            instruction,
                            e
                        ))
                    }
                },
                Err(e) => return Err(anyhow!("error fetching next instruction {:?}", e)),
            }
        }
        Ok(())
    }

    pub(crate) fn new(memory: &'a mut Vec<i64>, input: Vec<i64>, output: &'a mut Vec<i64>) -> Self {
        Self {
            memory,
            input,
            output,
            memory_pos: 0,
            input_pos: 0,
            output_pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::computer::{
        parse_instruction, Instruction, InstructionArgumentMode, InstructionType, ProgramState,
    };

    #[test]
    fn parse_instructions() {
        assert_eq!(
            Instruction {
                opcode: InstructionType::MULTIPLY,
                arg_modes: vec![
                    InstructionArgumentMode::POSITION,
                    InstructionArgumentMode::IMMEDIATE,
                    InstructionArgumentMode::POSITION,
                ]
            },
            parse_instruction(1002).unwrap()
        )
    }

    #[test]
    fn run_program() {
        let input = vec![];
        let mut output = vec![];
        let mut memory = vec![1002, 4, 3, 4, 33];

        let mut prog = ProgramState::new(&mut memory, input, &mut output);

        match prog.run() {
            Ok(_) => {}
            Err(e) => panic!("error running program {:?}", e),
        }

        assert_eq!(memory, vec![1002, 4, 3, 4, 99])
    }
}
