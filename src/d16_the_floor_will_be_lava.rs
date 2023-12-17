use itertools::Itertools;

use crate::{
    d16_the_floor_will_be_lava::{
        Direction::{Down, Left, Right, Up},
        Tile::{Empty, MirrorL, MirrorR, SplitterH, SplitterV},
    },
    utils::{Day, Task},
};

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
enum Direction {
    Up = 1u8,
    Left = 2,
    Down = 4,
    Right = 8,
}

#[derive(Default, Copy, Clone)]
struct Beams(u8);
impl Beams {
    fn has_beam_going(&self, direction: Direction) -> bool {
        self.0 & direction as u8 != 0
    }

    fn add_beam(&mut self, direction: Direction) {
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
    width: usize,
    height: usize,
}

impl Contraption {
    fn energize_tiles(&self, start_x: usize, start_y: usize, start_dir: Direction) -> Vec<Beams> {
        let step = |x: usize, y: usize, dir| match dir {
            Left if x == 0 => None,
            Left => Some((x - 1, y)),
            Right if x + 1 == self.width => None,
            Right => Some((x + 1, y)),
            Up if y == 0 => None,
            Up => Some((x, y - 1)),
            Down if y + 1 == self.height => None,
            Down => Some((x, y + 1)),
        };

        let mut beams = vec![Beams::default(); self.tiles.len()];
        let mut origins = vec![(start_x, start_y, start_dir)];

        while let Some((x, y, dir)) = origins.pop() {
            let (mut x, mut y, mut dir) = (x, y, dir);

            while !beams[x + y * self.width].has_beam_going(dir) {
                beams[x + y * self.width].add_beam(dir);
                // MirrorL  /
                // MirrorR  \
                let (new_xy, new_dir) = match (&self.tiles[x + y * self.width], dir) {
                    (Empty, _) | (SplitterV, Up | Down) | (SplitterH, Left | Right) => {
                        (step(x, y, dir), dir)
                    }
                    (MirrorL, Up) | (MirrorR, Down) => (step(x, y, Right), Right),
                    (MirrorL, Down) | (MirrorR, Up) => (step(x, y, Left), Left),
                    (MirrorL, Left) | (MirrorR, Right) => (step(x, y, Down), Down),
                    (MirrorL, Right) | (MirrorR, Left) => (step(x, y, Up), Up),
                    (SplitterV, Left | Right) => {
                        origins.push((x, y, Down));
                        (step(x, y, Up), Up)
                    }
                    (SplitterH, Up | Down) => {
                        origins.push((x, y, Left));
                        (step(x, y, Right), Right)
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

    fn count_energized_tiles(&self, start_x: usize, start_y: usize, start_dir: Direction) -> usize {
        self.energize_tiles(start_x, start_y, start_dir)
            .iter()
            .filter(|b| b.has_beam())
            .count()
    }

    fn max_energized(&self) -> usize {
        let max_vertical = (0..self.width)
            .map(|col| {
                let top_down = self.count_energized_tiles(col, 0, Down);
                let bottom_up = self.count_energized_tiles(col, self.height - 1, Up);
                top_down.max(bottom_up)
            })
            .max()
            .unwrap();

        let max_horizontal = (0..self.height)
            .map(|row| {
                let left_right = self.count_energized_tiles(0, row, Right);
                let right_left = self.count_energized_tiles(self.width - 1, row, Left);
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
            width,
            height,
        }
    }
}

fn part_1(filename: &str) -> usize {
    let contraption = Contraption::from_file(filename);

    contraption.count_energized_tiles(0, 0, Right)
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
