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

fn parse_file(filename: &str) -> (Vec<Direction>, HashMap<String, Connection>) {
    let mut lines = crate::utils::read_lines(filename);
    let directions = lines.next().unwrap().chars().map(Direction::from).collect();

    let nodes = lines
        .skip(1)
        .map(|line| {
            let (name, nodes) = line.split_once(" = ").unwrap();
            (String::from(name), Connection::new(nodes))
        })
        .collect();

    (directions, nodes)
}

fn count_steps(filename: &str) -> usize {
    let (directions, nodes) = parse_file(filename);
    let mut position = "AAA";

    for (steps, &direction) in directions.iter().cycle().enumerate() {
        position = nodes[position].turn(direction);
        if position == "ZZZ" {
            return steps + 1;
        }
    }

    unreachable!()
}

#[allow(dead_code)]
fn count_ghost_steps_naive(filename: &str) -> usize {
    let (directions, nodes) = parse_file(filename);

    let mut positions = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(String::as_str)
        .collect_vec();

    for (steps, &direction) in directions.iter().cycle().enumerate() {
        for position in positions.iter_mut() {
            *position = nodes[*position].turn(direction);
        }
        if positions.iter().all(|p| p.ends_with('Z')) {
            return steps + 1;
        }
    }

    unreachable!()
}

fn count_ghost_steps_find_cycles(filename: &str) -> usize {
    let (directions, nodes) = parse_file(filename);

    let positions = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(String::as_str)
        .collect_vec();

    let mut res = 1;

    for p in positions {
        let mut position = p;

        for (current_step, &direction) in directions.iter().cycle().enumerate() {
            position = nodes[position].turn(direction);
            if position.ends_with('Z') {
                res = num::integer::lcm(res, current_step + 1);
                break;
            }
        }
    }

    res
}

pub fn solution() -> Day<usize, usize> {
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
            examples: vec!["./inputs/day_08/example_03.txt"],
            task: "./inputs/day_08/task.txt",
            run: count_ghost_steps_find_cycles,
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
        assert_eq!(6, res);
    }
}
