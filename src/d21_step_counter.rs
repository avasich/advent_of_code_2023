use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use itertools::Itertools;
use num::Integer;
use Dir::*;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    L,
    D,
    R,
}

struct Field {
    map: Vec<bool>,
    w: usize,
    h: usize,
    start: (usize, usize),
}

impl Field {
    fn new(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let w = lines[0].len();
        let h = lines.len();
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
        let start = start.div_rem(&w);
        Self { map, w, h, start }
    }

    fn steps(&self, target_steps: usize, starts: &[(usize, usize)]) -> usize {
        let mut visited = HashMap::new();
        let mut step_queue = VecDeque::new();
        step_queue.extend(starts.iter().map(|start| (*start, target_steps)));

        while let Some((xy, steps_left)) = step_queue.pop_front() {
            let can_reach = steps_left % 2 == 0;

            if visited.insert(xy, can_reach).is_some() || steps_left == 0 {
                continue;
            }

            step_queue.extend(
                [U, L, D, R]
                    .into_iter()
                    .flat_map(|dir| self.try_step(xy, dir))
                    .map(|xy| (xy, steps_left - 1)),
            );
        }
        visited.values().filter(|v| **v).count()
    }

    #[allow(dead_code, clippy::type_complexity)]
    fn step_wrap_dumb(
        &self,
        target_steps: usize,
        px_r: RangeInclusive<isize>,
        py_r: RangeInclusive<isize>,
    ) -> HashMap<((usize, usize), (isize, isize)), usize> {
        let mut visited = HashMap::<((usize, usize), (isize, isize)), usize>::new();
        let mut step_queue = VecDeque::new();
        let patch = (0, 0);
        step_queue.push_back((Some((self.start, patch)), target_steps));

        while let Some((point, steps_left)) = step_queue.pop_front() {
            if point.is_none() {
                continue;
            }
            let (xy, patch) = point.unwrap();
            let (px, py) = patch;
            if !px_r.contains(&px) || !py_r.contains(&py) {
                continue;
            }

            match visited.get(&(xy, patch)) {
                Some(steps) if *steps >= steps_left => {
                    continue;
                }
                _ => {}
            }

            visited.insert((xy, patch), steps_left);

            if steps_left == 0 {
                continue;
            }

            step_queue.push_back((self.try_step_wrap(xy, patch, U), steps_left - 1));
            step_queue.push_back((self.try_step_wrap(xy, patch, D), steps_left - 1));
            step_queue.push_back((self.try_step_wrap(xy, patch, L), steps_left - 1));
            step_queue.push_back((self.try_step_wrap(xy, patch, R), steps_left - 1));
        }
        visited
    }

    fn step_wrap(&self, target_steps: usize) -> Vec<isize> {
        let w = self.w as isize;
        let h = self.h as isize;

        let mut visited = HashMap::<(isize, isize), usize>::new();
        let mut border = HashSet::<(isize, isize)>::new();

        border.insert((self.start.0 as isize, self.start.1 as isize));
        let r = target_steps % self.w;

        let mut xs = vec![];

        for step in 1..=target_steps {
            let mut new_border = HashSet::new();

            for &(x, y) in &border {
                visited.insert((x, y), step % 2);
                for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                    let (x1, y1) = (x + dx, y + dy);
                    let xm = x1.rem_euclid(w) as usize;
                    let ym = y1.rem_euclid(h) as usize;
                    if self.map[xm + ym * self.w] && !visited.contains_key(&(x1, y1)) {
                        new_border.insert((x1, y1));
                    }
                }
            }
            if (step + r) % self.w == 0 {
                let already_visited = visited
                    .iter()
                    .filter(|(_, parity)| **parity == step % 2)
                    .count();
                xs.push(already_visited as isize);

                if xs.len() >= 3 {
                    break;
                }
            }

            border = new_border;
        }

        xs
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.map[x + y * self.w]
    }

    fn try_step(&self, (x, y): (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        match dir {
            U if y > 0 => Some((x, y - 1)),
            L if x > 0 => Some((x - 1, y)),
            D if y + 1 < self.h => Some((x, y + 1)),
            R if x + 1 < self.w => Some((x + 1, y)),
            _ => None,
        }
        .filter(|&(x, y)| self.get(x, y))
    }

    fn try_step_wrap(
        &self,
        (x, y): (usize, usize),
        (px, py): (isize, isize),
        dir: Dir,
    ) -> Option<((usize, usize), (isize, isize))> {
        let (xy, pxy) = match dir {
            U => (y > 0)
                .then(|| ((x, y - 1), (px, py)))
                .unwrap_or_else(|| ((x, self.h - 1), (px, py - 1))),
            L => (x > 0)
                .then(|| ((x - 1, y), (px, py)))
                .unwrap_or_else(|| ((self.w - 1, y), (px - 1, py))),
            D => (y + 1 < self.h)
                .then(|| ((x, y + 1), (px, py)))
                .unwrap_or_else(|| ((x, 0), (px, py + 1))),
            R => (x + 1 < self.w)
                .then(|| ((x + 1, y), (px, py)))
                .unwrap_or_else(|| ((0, y), (px + 1, py))),
        };

        self.get(xy.0, xy.1).then_some((xy, pxy))
    }
}

fn part_1(filename: &str, target_steps: usize) -> usize {
    let field = Field::new(filename);
    field.steps(target_steps, &[field.start])
}

fn part_2(filename: &str, target_steps: usize) -> usize {
    let field = Field::new(filename);
    let vs = field.step_wrap(target_steps);

    let a = (vs[2] - 2 * vs[1] + vs[0]) / 2;
    let b = vs[1] - vs[0] - 3 * a;
    let c = vs[0] - b - a;
    let n = 1 + (target_steps / field.w) as isize;

    (a * n * n + b * n + c) as usize
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
    fn p2_example_test() {}

    #[test]
    fn bar() {
        let solution = solution();
        let field = Field::new(solution.part_2.task);
        let target_steps: usize = 65 + 131 * 4;
        let px_range = -7..=7;
        let py_range = -7..=7;

        let vs = field.step_wrap_dumb(target_steps, px_range.clone(), py_range.clone());
        let res = vs.iter().filter(|(_, s)| **s % 2 == 0).count();
        println!("{res}");

        let res = part_2(solution.part_2.task, target_steps);
        println!("{res}");
    }
}
