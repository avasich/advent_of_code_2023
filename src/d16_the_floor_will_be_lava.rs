use itertools::Itertools;

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

fn part_1(filename: &str) -> usize {
    let lines = crate::utils::read_lines(filename).collect_vec();
    let width = lines[0].len();
    let height = lines.len();
    use Direction::*;
    use Tile::*;

    let tiles = lines
        .iter()
        .flat_map(|line| line.chars())
        .map(Tile::from)
        .collect_vec();

    let step = |x: usize, y: usize, dir| match dir {
        Left if x == 0 => None,
        Left => Some((x - 1, y)),
        Right if x + 1 == width => None,
        Right => Some((x + 1, y)),
        Up if y == 0 => None,
        Up => Some((x, y - 1)),
        Down if y + 1 == height => None,
        Down => Some((x, y + 1)),
    };

    let mut beams = vec![Beams::default(); tiles.len()];
    let mut starts = vec![(0, 0, Right)];

    while let Some((x, y, dir)) = starts.pop() {
        let (mut x, mut y, mut dir) = (x, y, dir);

        while !beams[x + y * width].has_beam_going(dir) {
            beams[x + y * width].add_beam(dir);
            // MirrorL  /
            // MirrorR  \
            let (xy, new_dir) = match (&tiles[x + y * width], dir) {
                (Empty, _) | (SplitterV, Up | Down) | (SplitterH, Left | Right) => {
                    (step(x, y, dir), dir)
                }
                (MirrorL, Up) | (MirrorR, Down) => (step(x, y, Right), Right),
                (MirrorL, Down) | (MirrorR, Up) => (step(x, y, Left), Left),
                (MirrorL, Left) | (MirrorR, Right) => (step(x, y, Down), Down),
                (MirrorL, Right) | (MirrorR, Left) => (step(x, y, Up), Up),
                (SplitterV, Left | Right) => {
                    starts.push((x, y, Down));
                    (step(x, y, Up), Up)
                }
                (SplitterH, Up | Down) => {
                    starts.push((x, y, Left));
                    (step(x, y, Right), Right)
                }
            };

            match xy {
                None => break,
                Some((x1, y1)) => {
                    (x, y) = (x1, y1);
                    dir = new_dir;
                }
            }
        }
    }

    beams.iter().filter(|b| b.has_beam()).count()
}

fn part_2(filename: &str) -> u64 {
    todo!()
}

pub fn solution() -> Day<usize, u64> {
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
        assert_eq!(64, res);
    }
}
