use itertools::Itertools;

use crate::utils::{Day, Task};

fn expanding_distances(filename: &str, expansion_factor: usize) -> usize {
    let mut map = crate::utils::read_lines(filename)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Some((x, y)),
                    '.' => None,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut blank_rows = 0;
    for row in map.iter_mut() {
        let galaxies_in_row = row
            .iter_mut()
            .flatten()
            .map(|galaxy| galaxy.1 += blank_rows)
            .count();

        if galaxies_in_row == 0 {
            blank_rows += expansion_factor - 1;
        }
    }

    let mut blank_cols = 0;
    for x in 0..map[0].len() {
        let galaxies_in_col = map
            .iter_mut()
            .flat_map(|row| &mut row[x])
            .map(|galaxy| galaxy.0 += blank_cols)
            .count();

        if galaxies_in_col == 0 {
            blank_cols += expansion_factor - 1;
        }
    }

    let galaxies = map.iter().flatten().flatten().collect_vec();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|(x2, y2)| x1.abs_diff(*x2) + y1.abs_diff(*y2))
        })
        .sum()
}

pub fn solution() -> Day<usize, usize> {
    fn part_1(filename: &str) -> usize {
        expanding_distances(filename, 2)
    }

    fn part_2(filename: &str) -> usize {
        expanding_distances(filename, 1000000)
    }

    Day {
        part_1: Task {
            examples: vec!["./inputs/day_11/example_01.txt"],
            task: "./inputs/day_11/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_11/example_03.txt"],
            task: "./inputs/day_11/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d11_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(374, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let filename = solution.part_1.examples[0];

        let res = expanding_distances(filename, 10);
        assert_eq!(1030, res);
        let res = expanding_distances(filename, 100);
        assert_eq!(8410, res);
    }
}
