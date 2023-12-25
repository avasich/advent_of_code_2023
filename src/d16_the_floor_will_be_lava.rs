use itertools::Itertools;
use Dir::*;
use Tile::*;

use crate::utils::{Day, Task};

enum Tile {
    Empty,
    MirrorL,
    MirrorR,
    SplitterV,
    SplitterH,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;

        match value {
            '.' => Empty,
            '/' => MirrorL,
            '\\' => MirrorR,
            '|' => SplitterV,
            '-' => SplitterH,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Dir {
    U = 1u8,
    L = 2,
    D = 4,
    R = 8,
}

#[derive(Default, Copy, Clone)]
struct Beams(u8);

impl Beams {
    fn has_beam_going(&self, direction: Dir) -> bool {
        self.0 & direction as u8 != 0
    }

    fn add_beam(&mut self, direction: Dir) {
        self.0 |= direction as u8;
    }

    fn has_beam(&self) -> bool {
        self.0 != 0
    }
}

#[allow(dead_code)]
fn print_lit(beams: &[Beams], width: usize) {
    for lit_row in &beams.iter().chunks(width) {
        lit_row.for_each(|b| match b.has_beam() {
            true => print!("#"),
            false => print!("."),
        });
        println!()
    }
}

struct Contraption {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}

impl Contraption {
    fn energize_tiles(&self, start_x: usize, start_y: usize, start_dir: Dir) -> Vec<Beams> {
        let mut beams = vec![Beams::default(); self.tiles.len()];
        let mut origins = vec![(start_x, start_y, start_dir)];

        while let Some((x, y, dir)) = origins.pop() {
            let (mut x, mut y, mut dir) = (x, y, dir);

            while !beams[x + y * self.w].has_beam_going(dir) {
                beams[x + y * self.w].add_beam(dir);
                // MirrorL  /
                // MirrorR  \
                let (new_xy, new_dir) = match (&self.tiles[x + y * self.w], dir) {
                    (Empty, _) | (SplitterV, U | D) | (SplitterH, L | R) => {
                        (self.try_step(x, y, dir), dir)
                    }
                    (MirrorL, U) | (MirrorR, D) => (self.try_step(x, y, R), R),
                    (MirrorL, D) | (MirrorR, U) => (self.try_step(x, y, L), L),
                    (MirrorL, L) | (MirrorR, R) => (self.try_step(x, y, D), D),
                    (MirrorL, R) | (MirrorR, L) => (self.try_step(x, y, U), U),
                    (SplitterV, L | R) => {
                        origins.push((x, y, D));
                        (self.try_step(x, y, U), U)
                    }
                    (SplitterH, U | D) => {
                        origins.push((x, y, L));
                        (self.try_step(x, y, R), R)
                    }
                };

                match new_xy {
                    None => break,
                    Some((x1, y1)) => {
                        (x, y) = (x1, y1);
                        dir = new_dir;
                    }
                }
            }
        }

        beams
    }

    fn try_step(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            U if y > 0 => Some((x, y - 1)),
            L if x > 0 => Some((x - 1, y)),
            D if y + 1 < self.h => Some((x, y + 1)),
            R if x + 1 < self.w => Some((x + 1, y)),
            _ => None,
        }
    }

    fn count_energized_tiles(&self, start_x: usize, start_y: usize, start_dir: Dir) -> usize {
        self.energize_tiles(start_x, start_y, start_dir)
            .iter()
            .filter(|b| b.has_beam())
            .count()
    }

    fn max_energized(&self) -> usize {
        let max_vertical = (0..self.w)
            .map(|col| {
                let top_down = self.count_energized_tiles(col, 0, D);
                let bottom_up = self.count_energized_tiles(col, self.h - 1, U);
                top_down.max(bottom_up)
            })
            .max()
            .unwrap();

        let max_horizontal = (0..self.h)
            .map(|row| {
                let left_right = self.count_energized_tiles(0, row, R);
                let right_left = self.count_energized_tiles(self.w - 1, row, L);
                left_right.max(right_left)
            })
            .max()
            .unwrap();

        max_vertical.max(max_horizontal)
    }

    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let width = lines[0].len();
        let height = lines.len();

        let tiles = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Tile::from)
            .collect_vec();

        Self {
            tiles,
            w: width,
            h: height,
        }
    }
}

fn part_1(filename: &str) -> usize {
    let contraption = Contraption::from_file(filename);

    contraption.count_energized_tiles(0, 0, R)
}

fn part_2(filename: &str) -> usize {
    let contraption = Contraption::from_file(filename);

    contraption.max_energized()
}

pub fn solution() -> Day<usize, usize> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_16/example_01.txt"],
            task: "./inputs/day_16/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_16/example_01.txt"],
            task: "./inputs/day_16/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d16_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(46, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(51, res);
    }
}
