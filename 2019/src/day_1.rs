use crate::utils::input_newline_ints;

pub fn part_one() -> i64 {
    input_newline_ints(1).into_iter().map(|x| x / 3 - 2).sum()
}

pub fn part_two() -> i64 {
    input_newline_ints(1)
        .into_iter()
        .map(|x| calculate_fuel(x))
        .sum()
}

fn op(i: i64) -> i64 {
    i / 3 - 2
}

fn calculate_fuel(mass: i64) -> i64 {
    let mut total = 0;
    let mut remainder = mass.clone();
    let mut fuel: i64;

    while remainder > 0 {
        fuel = op(remainder);
        if fuel <= 0 {
            return total;
        }
        total += fuel;
        remainder = fuel
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_part_one() {
        assert_eq!(part_one(), 3254441);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(part_two(), 4878818);
    }

    // #[test]
    // fn test_fuel() {
    //     assert_eq!(calculate_fuel(14), 2);
    // }

    // #[test]
    // fn test_fuel_2() {
    //     assert_eq!(calculate_fuel(1969), 966);
    // }

    // #[test]
    // fn test_fuel_3() {
    //     assert_eq!(calculate_fuel(100756), 50346);
    // }
}
