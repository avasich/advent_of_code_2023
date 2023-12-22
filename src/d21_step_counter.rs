use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use num::Integer;
use Direction::*;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    L,
    D,
    R,
}

struct Field {
    map: Vec<bool>,
    width: usize,
    height: usize,
    x0: usize,
    y0: usize,
}

impl Field {
    fn new(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let width = lines[0].len();
        let height = lines.len();
        let mut start = 0;
        let map = lines
            .iter()
            .flat_map(|line| line.chars())
            .enumerate()
            .inspect(|(i, c)| {
                if *c == 'S' {
                    start = *i;
                }
            })
            .map(|(_, c)| c != '#')
            .collect_vec();
        let (y0, x0) = start.div_rem(&width);
        Self {
            map,
            width,
            height,
            x0,
            y0,
        }
    }

    fn steps(&self, steps_left: usize) -> usize {
        let mut cache = HashSet::new();
        let mut step_queue = VecDeque::new();
        step_queue.push_back((Some((self.x0, self.y0)), steps_left));

        while let Some((xy, steps_left)) = step_queue.pop_front() {
            if xy.is_none() {
                continue;
            }

            let (x, y) = xy.unwrap();

            if !cache.insert((x, y, steps_left)) || steps_left == 0 {
                continue;
            }
            step_queue.push_back((self.try_step(x, y, L), steps_left - 1));
            step_queue.push_back((self.try_step(x, y, R), steps_left - 1));
            step_queue.push_back((self.try_step(x, y, U), steps_left - 1));
            step_queue.push_back((self.try_step(x, y, D), steps_left - 1));
        }
        cache.iter().filter(|(_, _, st)| *st == 0).count()
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.map[x + y * self.width]
    }

    fn try_step(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        match dir {
            U => (y != 0 && self.get(x, y - 1)).then(|| (x, y - 1)),
            D => (y + 1 != self.height && self.get(x, y + 1)).then(|| (x, y + 1)),
            L => (x != 0 && self.get(x - 1, y)).then(|| (x - 1, y)),
            R => (x + 1 != self.width && self.get(x + 1, y)).then(|| (x + 1, y)),
        }
    }
}

fn part_1(filename: &str, target_steps: usize) -> usize {
    let field = Field::new(filename);
    field.steps(target_steps)
}

#[allow(unused)]
fn part_2(filename: &str, target_steps: usize) -> usize {
    let field = Field::new(filename);
    field.steps(target_steps)
}

pub fn solution() -> Day<usize, usize> {
    fn part_1_task(filename: &str) -> usize {
        part_1(filename, 64)
    }

    fn part_2_task(filename: &str) -> usize {
        part_2(filename, 26501365)
    }

    Day {
        part_1: Task {
            examples: vec!["./inputs/day_21/example_01.txt"],
            task: "./inputs/day_21/task.txt",
            run: part_1_task,
        },
        part_2: Task {
            examples: vec!["./inputs/day_21/example_01.txt"],
            task: "./inputs/day_21/task.txt",
            run: part_2_task,
        },
    }
}

#[cfg(test)]
mod d21_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let filename = solution.part_1.examples[0];
        let res = part_1(filename, 6);
        assert_eq!(16, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let filename = solution.part_1.examples[0];

        let res = part_2(filename, 6);
        assert_eq!(16, res);
        let res = part_2(filename, 10);
        assert_eq!(50, res);
        let res = part_2(filename, 50);
        assert_eq!(1594, res);
        let res = part_2(filename, 100);
        assert_eq!(6536, res);
        let res = part_2(filename, 500);
        assert_eq!(167004, res);
        let res = part_2(filename, 1000);
        assert_eq!(668697, res);
        let res = part_2(filename, 5000);
        assert_eq!(16733044, res);
    }
}

/*

...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........



 */
