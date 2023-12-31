use std::{fs, path::Path};

pub fn input_range(day: i64) -> std::ops::Range<i64> {
    let input = read_input(day);
    let mut input = input.split_ascii_whitespace();
    if let Some(range) = input.next() {
        let mut range = range.split("-");
        let start = range.next().unwrap().parse::<i64>().unwrap();
        let end = range.next().unwrap().parse::<i64>().unwrap();
        return start..end;
    }
    unreachable!()
}

pub fn input_paths(day: i64) -> (Vec<String>, Vec<String>) {
    let paths: Vec<Vec<String>> = read_input(day)
        .lines()
        .map(|line| line.split(",").map(|x| x.to_string()).collect())
        .collect();
    (paths[0].clone(), paths[1].clone())
}

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
