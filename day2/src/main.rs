// https://adventofcode.com/2019/day/2

use std::fs;

// Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) 
// to the "1202 program alarm" state it had just before the last computer caught fire. 
// To do this, before running the program, replace position 1 with the value 12 and replace position 2 
// with the value 2. What value is left at position 0 after the program halts?
fn main() {
    let input = fs::read_to_string("input.txt").expect("input program read");

    part1(input.clone());
}

fn part1(input: String) {
    let mut numbers: Vec<i64> = str::split(&input, ",").map(|n| n.parse::<i64>().
        expect("number parsed")).collect();

    numbers[1] = 12;
    numbers[2] = 2;

    println!("original program: {:?}", numbers);

    run_program(&mut numbers);

    println!("completed program: {:?}", numbers);
    println!("first element: {:?}", numbers[0])
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