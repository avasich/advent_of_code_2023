use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Pipe {
    NS,
    NW,
    NE,
    SW,
    SE,
    EW,
    Entrance,
    Ground,
}

impl Pipe {
    fn connected_to(&self, d: Dir) -> bool {
        use Dir::*;
        use Pipe::*;

        matches!(
            (d, self),
            (_, Entrance)
                | (N, NS | NW | NE)
                | (S, NS | SW | SE)
                | (W, NW | SW | EW)
                | (E, NE | SE | EW)
        )
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::NS,
            'J' => Pipe::NW,
            'L' => Pipe::NE,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '-' => Pipe::EW,
            'S' => Pipe::Entrance,
            _ => Pipe::Ground,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Dir {
    N,
    W,
    S,
    E,
}

impl Dir {
    fn rev(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::W => Dir::E,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
        }
    }
}

struct Maze {
    w: usize,
    h: usize,
    start_pos: (usize, usize),
    map: Vec<Vec<Pipe>>,
}

impl Maze {
    fn new(map: Vec<Vec<Pipe>>) -> Self {
        let start_pos = map
            .iter()
            .enumerate()
            .find_map(|(ri, row)| {
                row.iter()
                    .position(|p| *p == Pipe::Entrance)
                    .map(|ci| (ci, ri))
            })
            .unwrap();

        Self {
            w: map[0].len(),
            h: map.len(),
            start_pos,
            map,
        }
    }

    fn get(&self, x: usize, y: usize) -> Pipe {
        self.map[y][x]
    }

    fn adjacent_to(&self, x: usize, y: usize, d: Dir) -> Option<(usize, usize)> {
        match d {
            Dir::N if y > 0 => Some((x, y - 1)),
            Dir::W if x > 0 => Some((x - 1, y)),
            Dir::S if y + 1 < self.h => Some((x, y + 1)),
            Dir::E if x + 1 < self.w => Some((x + 1, y)),
            _ => None,
        }
    }

    fn check_connection(&self, x: usize, y: usize, d: Dir) -> Option<(usize, usize)> {
        let (x1, y1) = self.adjacent_to(x, y, d)?;

        (self.get(x, y).connected_to(d) && self.get(x1, y1).connected_to(d.rev()))
            .then_some((x1, y1))
    }

    fn loop_tiles(&self) -> Vec<(usize, usize)> {
        let dirs = [Dir::N, Dir::W, Dir::S, Dir::E];

        let (mut x, mut y) = self.start_pos;
        let mut from = *dirs.last().unwrap();
        let mut tiles = vec![];

        loop {
            for &d in dirs.iter() {
                if d == from {
                    continue;
                }
                if let Some((x1, y1)) = self.check_connection(x, y, d) {
                    (x, y) = (x1, y1);
                    from = d.rev();
                    tiles.push((x, y));
                    break;
                }
            }

            if self.get(x, y) == Pipe::Entrance {
                break tiles;
            }
        }
    }
}

fn part_1(filename: &str) -> usize {
    let map = crate::utils::read_lines(filename)
        .map(|line| line.chars().map(Pipe::from).collect_vec())
        .collect_vec();

    let maze = Maze::new(map);
    maze.loop_tiles().len() / 2
}

fn part_2(filename: &str) -> usize {
    let map = crate::utils::read_lines(filename)
        .map(|line| line.chars().map(Pipe::from).collect_vec())
        .collect_vec();

    let maze = Maze::new(map);
    let tiles = maze.loop_tiles().iter().copied().collect::<HashSet<_>>();
    let mut tiles_inside = 0;

    for y in 0..maze.h {
        use Dir::*;

        let mut is_inside = false;
        let mut from_south = false;
        let mut is_horizontal = false;

        for x in 0..maze.w {
            if tiles.contains(&(x, y)) {
                let to_east = maze.check_connection(x, y, E).is_some();

                match (is_horizontal, to_east) {
                    (false, true) => {
                        is_horizontal = true;
                        from_south = maze.check_connection(x, y, S).is_some();
                    }
                    (true, false) => {
                        is_horizontal = false;
                        let to_north = maze.check_connection(x, y, N).is_some();
                        if to_north == from_south {
                            is_inside = !is_inside;
                        }
                    }
                    (false, false) => is_inside = !is_inside,
                    _ => {}
                }
            } else if is_inside {
                tiles_inside += 1;
            }
        }
    }

    tiles_inside
}

pub fn solution() -> Day<usize, usize> {
    Day {
        part_1: Task {
            examples: vec![
                "./inputs/day_10/example_01.txt",
                "./inputs/day_10/example_02.txt",
            ],
            task: "./inputs/day_10/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec![
                "./inputs/day_10/example_03.txt",
                "./inputs/day_10/example_04.txt",
            ],
            task: "./inputs/day_10/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d10_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(4, res);
        let res = solution.part_1.run_example(1);
        assert_eq!(8, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(4, res);
        let res = solution.part_2.run_example(1);
        assert_eq!(8, res);
    }
}
