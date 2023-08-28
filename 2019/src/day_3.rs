use std::collections::{HashMap, HashSet};

use crate::utils::*;
type Step = i64;
type Commands = Vec<(char, i64)>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(self: Self, to: Self) -> i64 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

trait ToCommands {
    fn to_commands(self: Self) -> Commands;
}

impl ToCommands for Vec<String> {
    fn to_commands(self: Vec<String>) -> Commands {
        self.into_iter()
            .map(|x| {
                let direction = x.chars().nth(0).unwrap();
                let distance = x[1..].parse::<i64>().unwrap();
                (direction, distance)
            })
            .collect()
    }
}

trait ToLine {
    fn to_line(self: Self) -> HashSet<Point>;
    fn to_line_with_steps(self: Self) -> HashMap<Point, Step>;
}

impl ToLine for Commands {
    fn to_line(self: Commands) -> HashSet<Point> {
        let mut current_point = Point { x: 0, y: 0 };
        let mut points: HashSet<Point> = HashSet::new();
        for point in self {
            let direction = point.0;
            let distance = point.1;

            match direction {
                'U' => {
                    for _ in 0..distance {
                        current_point.y += 1;
                        points.insert(current_point);
                    }
                }
                'D' => {
                    for _ in 0..distance {
                        current_point.y -= 1;
                        points.insert(current_point);
                    }
                }
                'L' => {
                    for _ in 0..distance {
                        current_point.x -= 1;
                        points.insert(current_point);
                    }
                }
                'R' => {
                    for _ in 0..distance {
                        current_point.x += 1;
                        points.insert(current_point);
                    }
                }
                _ => panic!("invalid direction"),
            }
        }
        points
    }

    fn to_line_with_steps(self: Commands) -> HashMap<Point, Step> {
        let mut current_point = Point { x: 0, y: 0 };
        let mut current_step = 0;
        let mut points: HashMap<Point, Step> = HashMap::new();

        for point in self {
            let direction = point.0;
            let distance = point.1;

            match direction {
                'U' => {
                    for _ in 0..distance {
                        current_point.y += 1;
                        current_step += 1;
                        points.entry(current_point).or_insert(current_step);
                    }
                }
                'D' => {
                    for _ in 0..distance {
                        current_point.y -= 1;
                        current_step += 1;
                        points.entry(current_point).or_insert(current_step);
                    }
                }
                'L' => {
                    for _ in 0..distance {
                        current_point.x -= 1;
                        current_step += 1;
                        points.entry(current_point).or_insert(current_step);
                    }
                }
                'R' => {
                    for _ in 0..distance {
                        current_point.x += 1;
                        current_step += 1;
                        points.entry(current_point).or_insert(current_step);
                    }
                }

                _ => panic!("invalid direction"),
            }
        }
        points
    }
}

pub fn part_one() -> i64 {
    let origin = Point { x: 0, y: 0 };
    let (first, second) = input_paths(3);
    let first_line = first.to_commands().to_line();
    let second_line = second.to_commands().to_line();
    let mut intersections: Vec<Point> = Vec::new();
    for points in first_line {
        if second_line.contains(&points) {
            intersections.push(points);
        }
    }
    intersections
        .into_iter()
        .map(|x| x.manhattan_distance(origin))
        .min()
        .unwrap()
}

pub fn part_two() -> i64 {
    let (first, second) = input_paths(3);
    let first_line = first.to_commands().to_line_with_steps();
    let second_line = second.to_commands().to_line_with_steps();
    let mut min_steps: Option<i64> = None;
    for (point, step) in first_line {
        if second_line.contains_key(&point) {
            let second_step = second_line.get(&point).unwrap().clone();
            let total_steps = step + second_step;
            if let Some(min) = min_steps {
                if total_steps < min {
                    min_steps = Some(total_steps);
                }
            } else {
                min_steps = Some(total_steps);
            }
        }
    }

    min_steps.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_part_one() {
        assert_eq!(part_one(), 245);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(part_two(), 48262);
    }
}
