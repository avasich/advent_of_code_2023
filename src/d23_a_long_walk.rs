use std::fmt::Formatter;

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    L,
    D,
    R,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    U,
    L,
    D,
    R,
    /// empty
    E,
    /// wall
    W,
    /// traversed
    X,
}

impl Tile {
    fn is_walkable(&self) -> bool {
        !matches!(self, Self::W | Self::X)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::U,
            '<' => Self::L,
            'v' => Self::D,
            '>' => Self::R,
            '.' => Self::E,
            '#' => Self::W,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::U => write!(f, "^"),
            Tile::L => write!(f, "<"),
            Tile::D => write!(f, "v"),
            Tile::R => write!(f, ">"),
            Tile::E => write!(f, "."),
            Tile::W => write!(f, "#"),
            Tile::X => write!(f, "x"),
        }
    }
}

struct Labyrinth {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
    start: (usize, usize),
    exit: (usize, usize),
}

impl Labyrinth {
    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let h = lines.len();
        let w = lines[0].len();
        let tiles = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Tile::from)
            .collect_vec();

        let (start_x, _) = tiles.iter().find_position(|t| t.is_walkable()).unwrap();
        let (exit_x, _) = tiles
            .iter()
            .rev()
            .find_position(|t| t.is_walkable())
            .unwrap();
        let exit_x = w - exit_x - 1;

        Self {
            tiles,
            w,
            h,
            start: (start_x, 0),
            exit: (exit_x, h - 1),
        }
    }

    fn traverse(&mut self, x: usize, y: usize, len: usize) -> Option<usize> {
        if (x, y) == self.exit {
            return Some(len);
        } else if !self.get(x, y).is_walkable() {
            return None;
        }

        let tile = self.get(x, y);

        [Dir::U, Dir::L, Dir::D, Dir::R]
            .into_iter()
            .flat_map(|dir| {
                self.try_step(x, y, dir).and_then(|(x1, y1)| {
                    self.set(x, y, Tile::X);
                    let res = self.traverse(x1, y1, len + 1);
                    self.set(x, y, tile);
                    res
                })
            })
            .max()
    }

    fn try_step(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match (dir, self.get(x, y)) {
            (Dir::U, Tile::E | Tile::U) if y > 0 => Some((x, y - 1)),
            (Dir::U, _) => None,
            (Dir::L, Tile::E | Tile::L) if x > 0 => Some((x - 1, y)),
            (Dir::L, _) => None,
            (Dir::D, Tile::E | Tile::D) if y + 1 < self.h => Some((x, y + 1)),
            (Dir::D, _) => None,
            (Dir::R, Tile::E | Tile::R) if x + 1 < self.w => Some((x + 1, y)),
            (Dir::R, _) => None,
        }
        .filter(|&(x, y)| self.get(x, y).is_walkable())
    }

    fn longest_path(&mut self) -> Option<usize> {
        let (x, y) = self.start;
        self.traverse(x, y, 0)
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.w]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[x + y * self.w] = tile;
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.tiles.chunks(self.w).for_each(|row| {
            row.iter().for_each(|t| print!("{t}"));
            println!();
        });
    }
}

fn part_1(filename: &str) -> usize {
    let mut labyrinth = Labyrinth::from_file(filename);
    let res = labyrinth.longest_path();
    res.unwrap()
}

fn part_2(filename: &str) -> u64 {
    todo!()
}

pub fn solution() -> Day<usize, u64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_23/example_01.txt"],
            task: "./inputs/day_23/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_23/example_01.txt"],
            task: "./inputs/day_23/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d20_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(94, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(0, res);
    }
}
