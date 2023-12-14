use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Copy, Clone)]
enum Direction {
    L,
    R,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Connection(String, String);

impl Connection {
    fn new(s: &str) -> Self {
        let (node_l, node_r) = s.split_once(", ").unwrap();
        Self(
            String::from(&node_l[1..]),
            String::from(&node_r[..(node_r.len()) - 1]),
        )
    }

    fn turn(&self, direction: Direction) -> &str {
        match direction {
            Direction::L => &self.0,
            Direction::R => &self.1,
        }
    }
}

fn count_steps(filename: &str) -> u64 {
    let mut lines = crate::utils::read_lines(filename);
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from)
        .collect_vec();

    let nodes: HashMap<_, _> = lines
        .skip(1)
        .map(|line| {
            let (name, nodes) = line.split_once(" = ").unwrap();
            (String::from(name), Connection::new(nodes))
        })
        .collect();

    let mut position = "AAA";
    let mut steps = 0;

    for &direction in directions.iter().cycle() {
        position = nodes[position].turn(direction);
        steps += 1;
        if position == "ZZZ" {
            break;
        }
    }

    steps
}

fn part_2(filename: &str) -> u64 {
    1
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            examples: vec![
                "./inputs/day_08/example_01.txt",
                "./inputs/day_08/example_02.txt",
            ],
            task: "./inputs/day_08/task.txt",
            run: count_steps,
        },
        part_2: Task {
            examples: vec!["./inputs/day_08/example_01.txt"],
            task: "./inputs/day_08/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();

        let res = solution.part_1.run_example(0);
        assert_eq!(2, res);

        let res = solution.part_1.run_example(1);
        assert_eq!(6, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(1, res);
    }
}
