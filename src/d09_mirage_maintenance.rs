use itertools::Itertools;

use crate::utils::{Day, Task};

fn extrapolate(xs: &[i32]) -> i32 {
    let mut tmp = Vec::from(xs);
    let mut acc = 0;
    let len = xs.len();

    for i in (1..len).rev() {
        acc += tmp[i];
        let mut done = true;
        for j in 0..i {
            tmp[j] = tmp[j + 1] - tmp[j];
            done = done && tmp[j] == 0;
        }
        if done {
            return acc;
        }
    }
    unreachable!()
}

fn extrapolate_backwards(xs: &[i32]) -> i32 {
    let mut tmp = Vec::from(xs);
    let mut acc = 0;
    let mut sign = 1;
    let len = xs.len();

    for i in (1..len).rev() {
        acc += sign * tmp[0];
        sign *= -1;

        let mut done = true;
        for j in 0..i {
            tmp[j] = tmp[j + 1] - tmp[j];
            done = done && tmp[j] == 0;
        }
        if done {
            return acc;
        }
    }
    unreachable!()
}

fn part_1(filename: &str) -> i32 {
    crate::utils::read_lines(filename)
        .map(|line| line.split_whitespace().flat_map(str::parse).collect_vec())
        .map(|xs| extrapolate(&xs))
        .sum()
}

fn part_2(filename: &str) -> i32 {
    crate::utils::read_lines(filename)
        .map(|line| line.split_whitespace().flat_map(str::parse).collect_vec())
        .map(|xs| extrapolate_backwards(&xs))
        .sum()
}

pub fn solution() -> Day<i32, i32> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_09/example_01.txt"],
            task: "./inputs/day_09/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_09/example_01.txt"],
            task: "./inputs/day_09/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d09_tests {
    use super::*;
    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(114, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(2, res);
    }
}
