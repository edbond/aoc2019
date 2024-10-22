use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) {
    let mut total = 0;

    for line in input.lines() {
        // Fuel required to launch a given module is based on its mass.
        // Specifically, to find the fuel required for a module, take its mass, divide by three,
        // round down, and subtract 2.
        let n: i64 = line.parse::<i64>().expect("number expected");

        let fuel = n / 3 - 2;
        total += fuel;
    }

    println!("Part1: Total fuel required: {}", total);
}

fn part2(input: String) {
    let mut total = 0;

    for line in input.lines() {
        // Fuel required to launch a given module is based on its mass.
        // Specifically, to find the fuel required for a module, take its mass, divide by three,
        // round down, and subtract 2.
        let mut n: i64 = line.parse::<i64>().expect("number expected");

        loop {
            let fuel = n / 3 - 2;
            if fuel <= 0 {
                break;
            }
            
            total += fuel;
            n = fuel;
        }
    }

    println!("Part2: Total fuel required: {}", total);
}
