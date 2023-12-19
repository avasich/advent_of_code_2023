use itertools::Itertools;
use Direction::*;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    U,
    L,
    D,
    R,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            U => D,
            L => R,
            D => U,
            R => L,
        }
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    len: isize,
}

impl Command {
    fn from_string_1(s: String) -> Option<Self> {
        let mut split = s.split_whitespace();

        let dir = match split.next()? {
            "U" => U,
            "L" => L,
            "D" => D,
            "R" => R,
            _ => return None,
        };
        let len = split.next()?.parse().ok()?;

        Some(Self { dir, len })
    }

    fn from_string_2(s: String) -> Option<Self> {
        let hex = s.split_whitespace().nth(2)?;

        let len = isize::from_str_radix(&hex[2..=6], 16).ok()?;
        let dir = match hex.chars().nth(7)? {
            '0' => R,
            '1' => D,
            '2' => L,
            '3' => U,
            _ => return None,
        };
        
        Some(Self { dir, len })
    }
}

fn process_commands(mut commands: Vec<Command>) -> usize {
    let is_clockwise = commands
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .map(|(c1, c2)| match (c1.dir, c2.dir) {
            (L, U) | (U, R) | (R, D) | (D, L) => 1,
            _ => -1,
        })
        .sum::<i32>()
        > 0;

    if !is_clockwise {
        commands.iter_mut().for_each(|c| c.dir = c.dir.reverse());
    }

    let horizontal_segments = commands
        .iter()
        .cycle()
        .skip_while(|c| matches!(c.dir, L | R))
        .take(1 + commands.len())
        .tuple_windows::<(_, _, _)>()
        .step_by(2);

    let (area, _) = horizontal_segments.fold((0, 0), |(area, height), (prev, curr, next)| {
        let height = match prev.dir {
            U => height - prev.len,
            D => height + prev.len,
            _ => unreachable!(),
        };
        let len = curr.len;

        let dy = match curr.dir {
            R => -height,
            L => height + 1,
            _ => unreachable!(),
        };

        let dx = match (prev.dir, curr.dir, next.dir) {
            (U, _, U) | (D, _, D) => len,
            (U, R, D) | (D, L, U) => len + 1,
            (D, R, U) | (U, L, D) => len - 1,
            _ => unreachable!(),
        };

        (area + dx * dy, height)
    });

    area.unsigned_abs()
}

fn part_1(filename: &str) -> usize {
    let commands = crate::utils::read_lines(filename)
        .flat_map(Command::from_string_1)
        .collect_vec();
    process_commands(commands)
}

fn part_2(filename: &str) -> usize {
    let commands = crate::utils::read_lines(filename)
        .flat_map(Command::from_string_2)
        .collect_vec();
    process_commands(commands)
}

pub fn solution() -> Day<usize, usize> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_18/example_01.txt"],
            task: "./inputs/day_18/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_18/example_01.txt"],
            task: "./inputs/day_18/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d17_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(62, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();

        let res = solution.part_2.run_example(0);
        assert_eq!(952408144115, res);
    }
}

/*

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######


 */
