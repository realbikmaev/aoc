use crate::utils::*;
use std::collections::HashMap;

fn verify(password: i64) -> bool {
    let num_digits = password.to_string().len();
    if num_digits != 6 {
        return false;
    }
    let mut previous: i64 = -1;
    let mut has_double = false;

    for ch in password.to_string().chars() {
        let current = ch.to_digit(10).unwrap() as i64;

        if current < previous {
            return false;
        }
        if current == previous {
            has_double = true;
        }
        previous = current;
    }

    has_double
}

fn verify_second(password: i64) -> bool {
    let num_digits = password.to_string().len();
    if num_digits != 6 {
        return false;
    }

    let mut digit_counts: HashMap<i64, i64> = HashMap::new();
    let mut previous: i64 = -1;
    let mut has_double = false;
    for ch in password.to_string().chars() {
        let current = ch.to_digit(10).unwrap() as i64;
        if current < previous {
            return false;
        }
        previous = current;

        digit_counts
            .entry(current)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    for (_, v) in digit_counts.into_iter() {
        if v == 2 {
            has_double = true;
        }
    }
    has_double
}

pub fn part_one() -> i64 {
    let range = input_range(4);
    let mut count = 0;

    for password in range.clone().into_iter() {
        if range.contains(&password) {
            if verify(password) {
                count += 1;
            }
        }
    }

    count
}

pub fn part_two() -> i64 {
    let range = input_range(4);
    let mut count = 0;

    for password in range.clone().into_iter() {
        if range.contains(&password) {
            if verify_second(password) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_part_one() {
        assert_eq!(part_one(), 460);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(part_two(), 290);
    }
}
