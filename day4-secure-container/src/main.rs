use std::collections::HashMap;

fn main() {
    let mut result = 0;
    // Your puzzle input is 245182-790572.
    for n in 245182..=790572 {
        if is_password(n) {
            // println!("{}", n);
            result += 1;
        }
    }

    println!("part 1: total passwords: {}", result);

    result = 0;
    for n in 245182..=790572 {
        if !is_password(n) {
            continue;
        }

        let digits = number_digits(n);
        let groups = groups(digits);

        let large_group = groups.values().any(|&x| x > 2);
        let double_group = groups.values().any(|&x| x == 2);

        if large_group && !double_group {
            continue;
        }

        result += 1;
    }

    println!("part 2: total passwords: {}", result);
}

fn is_password(n: i32) -> bool {
    let digits = number_digits(n);

    // println!("n = {}, digits = {:?}", n, digits);

    let mut has_double = false;
    let mut is_increasing = true;
    let mut last_digit = 0;
    for digit in digits {
        if digit == last_digit {
            has_double = true;
        }
        if digit < last_digit {
            is_increasing = false;
        }
        last_digit = digit;
    }
    has_double && is_increasing
}

fn groups(digits: Vec<i32>) -> HashMap<i32, i32> {
    let mut groups = HashMap::new();

    for digit in digits {
        groups.entry(digit).and_modify(|e| *e += 1).or_insert(1);
    }

    groups
}

fn number_digits(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_digits() {
        assert_eq!(number_digits(12345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_is_password() {
        assert_eq!(is_password(111111), true);
        assert_eq!(is_password(223450), false);
        assert_eq!(is_password(123789), false);
    }
}
