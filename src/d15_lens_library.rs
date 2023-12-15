use std::collections::HashMap;

use crate::utils::{Day, Task};

enum Command {
    Remove,
    Put(u32),
}

fn parse_command(s: &str) -> (String, Command) {
    match s.split_once(['=', '-']) {
        Some((label, "")) => (String::from(label), Command::Remove),
        Some((label, focus)) => (String::from(label), Command::Put(focus.parse().unwrap())),
        None => unreachable!(),
    }
}

struct Lens {
    label: String,
    focus: u32,
}

impl Lens {
    fn new(label: String, f: u32) -> Self {
        Self { label, focus: f }
    }
}

fn hash(line: &str) -> u32 {
    line.chars()
        .map(u32::from)
        .fold(0, |acc, c| (acc + c) * 17 % 256)
}

fn part_1(filename: &str) -> u32 {
    let line = crate::utils::read_lines(filename).next().unwrap();
    line.split(',').map(hash).sum::<u32>()
}

fn part_2(filename: &str) -> u32 {
    let line = crate::utils::read_lines(filename).next().unwrap();
    line.split(',')
        .map(parse_command)
        .fold(
            HashMap::<u32, Vec<Lens>>::new(),
            |mut boxes, (label, command)| {
                use Command::*;

                let lens_box = boxes.entry(hash(&label)).or_default();
                let slot = lens_box.iter().position(|l: &Lens| l.label == label);
                match (command, slot) {
                    (Remove, None) => {}
                    (Remove, Some(slot)) => {
                        lens_box.remove(slot);
                    }
                    (Put(focus), None) => lens_box.push(Lens::new(label, focus)),
                    (Put(focus), Some(slot)) => lens_box[slot] = Lens::new(label, focus),
                }

                boxes
            },
        )
        .iter()
        .flat_map(|(n, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(slot, lens)| (1 + n) * (1 + slot as u32) * lens.focus)
        })
        .sum()
}

pub fn solution() -> Day<u32, u32> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_15/example_01.txt"],
            task: "./inputs/day_15/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_15/example_01.txt"],
            task: "./inputs/day_15/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d15_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(1320, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(145, res);
    }
}
