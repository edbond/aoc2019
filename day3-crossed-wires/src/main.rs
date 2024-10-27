use std::collections::HashSet;



fn main() {
    println!("Hello, world!");
}

fn parse_lines(input: &str) -> Vec<Vec<(char, i32)>> {
    input
        .lines()
        .map(|line| parse_input(line))
        .collect()
}

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .split(",")
        .map(|s| {
            let (direction, distance) = s.split_at(1);
            println!{"direction: {}, distance: {}", direction, distance};

            (
                direction.chars().next().expect("Invalid direction"),
                distance.parse::<i32>().expect("Invalid distance"),
            )
        })
        .collect()
}

fn find_closest_intersection(directions: Vec<(char, i32)>, visited_positions: &mut HashSet<(i32,i32)>) -> i32 {
    let mut current_position = (0, 0);
    let mut closest_intersection_distance = i32::MAX;

    for (direction, distance) in directions {
        for _ in 0..distance {
            current_position = match direction {
                'U' => (current_position.0, current_position.1 + 1),
                'D' => (current_position.0, current_position.1 - 1),
                'L' => (current_position.0 - 1, current_position.1),
                'R' => (current_position.0 + 1, current_position.1),
                _ => panic!("Invalid direction"),
            };
            if visited_positions.contains(&current_position) {
                let distance = i32::abs(current_position.0) + i32::abs( current_position.1);
                if distance < closest_intersection_distance {
                    closest_intersection_distance = distance;
                }
            }
            visited_positions.insert(current_position);
        }
    }
    closest_intersection_distance
}


#[cfg(test)]
mod tests {
    use std::cmp::min;

    use super::*;

    #[test]
    fn it_works() {
        let wires = parse_lines("R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83");
        let mut visited = HashSet::new();

        let mut result = i32::MAX;

        println!( "wires: {:?}", wires);

        for wire in wires {
            let r = find_closest_intersection(wire, &mut visited);
            result = min(result, r);
        }
        assert_eq!(result, 159);
    }
}