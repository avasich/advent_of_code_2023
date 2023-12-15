use std::fmt::Formatter;

use itertools::Itertools;

use crate::utils::{Day, Task};

enum Tile {
    Square,
    Round,
    Empty,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Square => '#',
            Tile::Round => 'O',
            Tile::Empty => '.',
        })
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Square,
            'O' => Tile::Round,
            _ => unreachable!(),
        }
    }
}

fn tilt_north(mut tiles: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let rows = tiles.len();
    let cols = tiles[0].len();

    for x in 0..cols {
        for y in (1..rows).rev() {
            match tiles[y][x] {
                Tile::Square => {}
                Tile::Round => {
                    let non_round = (0..y).rev().find_map(|y1| {
                        let tile = &tiles[y1][x];
                        (!matches!(tile, Tile::Round)).then_some((y1, tile))
                    });
                    match non_round {
                        None => break,
                        Some((y1, Tile::Empty)) => {
                            tiles[y1][x] = Tile::Round;
                            tiles[y][x] = Tile::Empty;
                        }
                        Some((_, Tile::Square)) => {}
                        Some((_, Tile::Round)) => unreachable!(),
                    }
                }
                Tile::Empty => {}
            }
        }
    }

    tiles
}

fn part_1(filename: &str) -> u64 {
    let tiles = crate::utils::read_lines(filename)
        .map(|line| line.chars().map(Tile::from).collect_vec())
        .collect_vec();
    let tiles = tilt_north(tiles);

    tiles
        .iter()
        .rev()
        .enumerate()
        .map(|(weight, row)| {
            let round_count = row
                .iter()
                .filter(|tile| matches!(tile, Tile::Round))
                .count();
            (round_count * (weight + 1)) as u64
        })
        .sum()
}

fn part_2(filename: &str) -> u64 {
    0
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_14/example_01.txt"],
            task: "./inputs/day_14/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_14/example_01.txt"],
            task: "./inputs/day_14/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d14_tests {
    use super::*;
    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(136, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(64, res);
    }
}
