use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::{Day, Task};

struct Field {
    tiles: Vec<Vec<char>>,
}

fn palindrome_table(line: &[char]) -> Vec<Vec<bool>> {
    let n = line.len();
    let mut dp = vec![vec![false; n]; n];

    for gap in 0..n {
        for i in 0..(n - gap) {
            let j = i + gap;
            dp[i][j] = match gap {
                0 => true,
                1 => line[i] == line[j],
                _ => line[i] == line[j] && dp[i + 1][j - 1],
            };
        }
    }

    dp
}

fn find_axes(dp: Vec<Vec<bool>>) -> HashSet<usize> {
    let n = dp.len();
    let mut axes = HashSet::new();

    for i in (1..n).step_by(2) {
        if dp[0][i] {
            axes.insert((i + 1) / 2);
        }
        if dp[n - i - 1][n - 1] {
            axes.insert(n - (i + 1) / 2);
        }
    }

    axes
}

fn find_axis<C: AsRef<Vec<char>>>(lines: impl Iterator<Item = C>) -> Option<usize> {
    lines
        .map(|col| palindrome_table(col.as_ref()))
        .map(find_axes)
        .reduce(|mut a, b| {
            a.retain(|x| b.contains(x));
            a
        })
        .and_then(|s| s.iter().copied().next())
}

#[derive(Debug)]
enum Axis {
    Horizontal(usize),
    Vertical(usize),
}

impl Field {
    fn from_string_iter(lines: impl Iterator<Item = String>) -> Field {
        let tiles = lines.map(|line| line.chars().collect()).collect();
        Field { tiles }
    }

    fn find_axis(&self) -> Axis {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();

        let vertical = find_axis(self.tiles.iter());
        if let Some(axis) = vertical {
            return Axis::Vertical(axis);
        }

        let horizontal = (0..cols).map(|x| (0..rows).map(|y| self.tiles[y][x]).collect_vec());
        let horizontal = find_axis(horizontal);

        Axis::Horizontal(horizontal.expect("no axis found"))
    }
}

fn part_1(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(is_empty, _)| !is_empty)
        .map(|(_, lines)| Field::from_string_iter(lines))
        .map(|field| field.find_axis())
        .map(|axis| match axis {
            Axis::Horizontal(n) => 100 * n,
            Axis::Vertical(n) => n,
        })
        .sum()
}

fn part_2(filename: &str) -> u64 {
    1
}

pub fn solution() -> Day<usize, u64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_13/example_01.txt"],
            task: "./inputs/day_13/example_01.txt",
            // task: "./inputs/day_13/task.txt",
            run: part_1,
        },

        part_2: Task {
            examples: vec!["./inputs/day_13/example_01.txt"],
            task: "./inputs/day_13/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d13_tests {
    use super::*;
    fn table(line: &str) -> Vec<Vec<bool>> {
        palindrome_table(&line.chars().collect_vec())
    }
    #[test]
    fn test_palindrome_table() {
        let dp = table("#.##..##.");
        let (t, f) = (true, false);
        assert_eq!(vec![t, f, t, f, f, f, f, f, f], dp[0]);
        assert_eq!(vec![f, t, f, f, t, f, f, f, t], dp[1]);
    }

    #[test]
    fn test_axes() {
        let dp = table("#.##..##.");
        let axes = find_axes(dp);
        assert_eq!(HashSet::from([5, 7]), axes);
    }

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example(0);
        assert_eq!(res, 405);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example(0);
        assert_eq!(res, 0);
    }
}
