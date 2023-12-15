use itertools::Itertools;

use crate::utils::{Day, Task};

fn hash(line: &str) -> u32 {
    line.chars()
        .map(u32::from)
        .fold(0, |acc, c| (acc + c) * 17 % 256)
}

fn part_1(filename: &str) -> u32 {
    crate::utils::read_lines(filename)
        .map(|line| line.split(',').map(hash).sum::<u32>())
        .sum()
}

fn part_2(filename: &str) -> u32 {
    0
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
        assert_eq!(64, res);
    }
}
