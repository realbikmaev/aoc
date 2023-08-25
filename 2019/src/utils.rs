use std::{fs, path::Path};

pub fn input_newline_ints(day: i64) -> Vec<i64> {
    read_input(day)
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn input_comma_ints(day: i64) -> Vec<i64> {
    read_input(day)
        .split(",")
        .map(|x| x.parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

pub fn read_input(day: i64) -> String {
    let path = format!("src/inputs/day_{}.txt", day);
    let input_path = Path::new(path.as_str());
    fs::read_to_string(input_path).unwrap()
}
