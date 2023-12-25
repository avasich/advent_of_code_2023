use std::{
    collections::{HashMap, VecDeque},
    fmt::Formatter,
};

use itertools::Itertools;
use Dir::*;

use crate::utils::{Day, Task};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Dir {
    U,
    L,
    D,
    R,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            U => R,
            L => U,
            D => L,
            R => D,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            U => L,
            L => D,
            D => R,
            R => U,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct StepCounter<const MIN_STEPS: usize, const MAX_STEPS: usize> {
    dir: Dir,
    count: usize,
}

impl<const MIN_STEPS: usize, const MAX_STEPS: usize> StepCounter<MIN_STEPS, MAX_STEPS> {
    fn new(dir: Dir) -> Self {
        Self { dir, count: 1 }
    }

    fn can_advance(&self) -> bool {
        self.count < MAX_STEPS
    }

    fn can_stop(&self) -> bool {
        self.count >= MIN_STEPS
    }

    fn turn_left(&self) -> Self {
        Self {
            dir: self.dir.turn_left(),
            count: 1,
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            dir: self.dir.turn_right(),
            count: 1,
        }
    }

    fn advance(&self) -> Self {
        Self {
            dir: self.dir,
            count: self.count + 1,
        }
    }
}

impl<const M: usize, const N: usize> std::fmt::Display for StepCounter<M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = match self.dir {
            U => 'U',
            L => 'L',
            D => 'D',
            R => 'R',
        };
        write!(f, "{}:{}", d, self.count)
    }
}

struct LossMap {
    w: usize,
    h: usize,
    tiles: Vec<u32>,
}

impl LossMap {
    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let w = lines[0].len();
        let h = lines.len();
        let tiles = lines
            .iter()
            .flat_map(|line| line.chars())
            .flat_map(|c| c.to_digit(10))
            .collect_vec();
        Self { w, h, tiles }
    }

    fn min_loss<const MIN_STEPS: usize, const MAX_STEPS: usize>(&self) -> u32 {
        let make_step = |Point { x, y }, dir: Dir| match dir {
            U if y > 0 => Some(Point::new(x, y - 1)),
            L if x > 0 => Some(Point::new(x - 1, y)),
            D if y + 1 < self.h => Some(Point::new(x, y + 1)),
            R if x + 1 < self.w => Some(Point::new(x + 1, y)),
            _ => None,
        };

        let mut cache =
            vec![HashMap::<StepCounter<MIN_STEPS, MAX_STEPS>, u32>::new(); self.tiles.len()];
        let mut result = vec![u32::MAX; self.tiles.len()];

        let mut moves = VecDeque::from([
            (Point::new(0, 1), StepCounter::new(D), 0),
            (Point::new(1, 0), StepCounter::new(R), 0),
        ]);

        while let Some((p, steps, prev)) = moves.pop_front() {
            let curr = prev + self.tiles[p.x + p.y * self.w];
            let tile_cache = &mut cache[p.x + p.y * self.w];

            let mut make_move = |st| match tile_cache.get(&st) {
                Some(val) if *val <= curr => {}
                _ => {
                    tile_cache.insert(st, curr);
                    if let Some(point) = make_step(p, st.dir) {
                        moves.push_back((point, st, curr));
                    }
                }
            };

            if steps.can_advance() {
                make_move(steps.advance());
            }

            if steps.can_stop() {
                make_move(steps.turn_right());
                make_move(steps.turn_left());

                if result[p.x + p.y * self.w] > curr {
                    result[p.x + p.y * self.w] = curr;
                }
            }
        }

        // for row in result.chunks(self.width) {
        //     row.iter().for_each(|val| print!("{val:10}"));
        //     println!();
        // }

        *result.last().unwrap()
    }
}

fn part_1(filename: &str) -> u32 {
    let loss_map = LossMap::from_file(filename);
    loss_map.min_loss::<1, 3>()
}

fn part_2(filename: &str) -> u32 {
    let loss_map = LossMap::from_file(filename);
    loss_map.min_loss::<4, 10>()
}

pub fn solution() -> Day<u32, u32> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_17/example_01.txt"],
            task: "./inputs/day_17/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec![
                "./inputs/day_17/example_01.txt",
                "./inputs/day_17/example_02.txt",
            ],
            task: "./inputs/day_17/task.txt",
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
        assert_eq!(102, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();

        let res = solution.part_2.run_example(0);
        assert_eq!(94, res);

        let res = solution.part_2.run_example(1);
        assert_eq!(71, res);
    }
}
