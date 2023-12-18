use std::collections::HashMap;

use itertools::Itertools;
use Direction::*;

use crate::utils::{Day, Task};

struct LossMap {
    width: usize,
    height: usize,
    loss_map: Vec<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn steps(&self, n: usize) -> Steps {
        Steps {
            direction: *self,
            count: n,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

const N: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Steps {
    direction: Direction,
    count: usize,
}

impl Steps {
    fn turn_left(&self) -> Self {
        Self {
            direction: self.direction.turn_left(),
            count: 0,
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            direction: self.direction.turn_right(),
            count: 0,
        }
    }

    fn go_straight(&self) -> Self {
        Self {
            direction: self.direction,
            count: self.count + 1,
        }
    }
}

impl LossMap {
    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let width = lines[0].len();
        let height = lines.len();
        let losses = lines
            .iter()
            .flat_map(|line| line.chars())
            .flat_map(|c| c.to_digit(10))
            .collect_vec();
        Self {
            width,
            height,
            loss_map: losses,
        }
    }

    fn min_loss(&self) -> u32 {
        use Direction::*;

        let make_step = |x: usize, y: usize, dir| match dir {
            Left if x == 0 => None,
            Left => Some((x - 1, y)),
            Right if x + 1 == self.width => None,
            Right => Some((x + 1, y)),
            Up if y == 0 => None,
            Up => Some((x, y - 1)),
            Down if y + 1 == self.height => None,
            Down => Some((x, y + 1)),
        };
        let mut total_loss_map = vec![[u32::MAX; N * 4]; self.loss_map.len()];
        let mut moves = vec![
            (Some((0, 1)), Down.steps(0), 0),
            (Some((1, 0)), Right.steps(0), 0),
        ];

        while let Some((xy, steps, prev_loss)) = moves.pop() {
            if xy.is_none() {
                continue;
            }
            let (x, y) = xy.unwrap();

            let new_loss = prev_loss + self.loss_map[x + y * self.width];
            let offset = N * (steps.direction as usize);
            let total_losses_along_dir =
                &mut total_loss_map[x + y * self.width][offset..(offset + N)];

            let total_loss = total_losses_along_dir[steps.count];
            if total_loss <= new_loss {
                continue;
            }

            total_losses_along_dir[steps.count..].fill(new_loss);

            let right = steps.turn_right();
            moves.push((make_step(x, y, right.direction), right, new_loss));

            let left = steps.turn_left();
            moves.push((make_step(x, y, left.direction), left, new_loss));

            if steps.count + 1 < N {
                let straight = steps.go_straight();
                moves.push((make_step(x, y, straight.direction), straight, new_loss));
            }
        }

        // for row in total_loss_map.chunks(self.width) {
        //     row.iter()
        //         .flat_map(|losses| losses.iter().min())
        //         .for_each(|loss| print!("{loss:3} "));
        //     println!()
        // }

        *total_loss_map
            .last()
            .and_then(|losses| losses.iter().min())
            .unwrap()
    }
}

fn part_1(filename: &str) -> u32 {
    let loss_map = LossMap::from_file(filename);
    loss_map.min_loss()
}

fn part_2(filename: &str) -> usize {
    todo!()
}

pub fn solution() -> Day<u32, usize> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_17/example_01.txt"],
            task: "./inputs/day_17/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_17/example_01.txt"],
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
        assert_eq!(51, res);
    }
}

/*
123456789abcd      123456789abcd          1   2   3   4   5   6   7   8   9   a   b   c   d

2>>34^>>>1323      2413432311323         11   4   5   8  18  21  23  27  29  32  41  43  46    1
32v>>>35v5623      3215453535623          3   5   6  11  15  20  25  31  32  37  43  45  48    2
32552456v>>54      3255245654254          6   7  11  16  17  21  26  32  37  41  43  48  52    3
3446585845v52      3446585845452          9  11  15  21  22  29  31  39  41  46  47  52  54    4
4546657867v>6      4546657867536         18  16  19  25  28  33  38  46  49  53  52  55  60    5
14385987984v4      1438598798454         19  22  22  30  34  42  46  53  58  61  56  60  64    6
44578769877v6      4457876987766         24  26  27  34  42  49  55  62  66  68  66  66  70    7
36378779796v>      3637877979653         33  33  34  41  49  56  62  71  77  80  77  71  73    8
465496798688v      4654967986887         37  39  41  45  54  60  67  78  85  86  85  80  80    9
456467998645v      4564679986453         41  45  47  49  55  62  71  85  93  94  89  90  88    a
12246868655<v      1224686865563         47  48  49  53  59  67  73  85  91  96  94  99  96    b
25465488877v5      2546548887735         51  53  54  59  64  68  76  84  97 103 104 102 101    c
43226746555v>      4322674655533         55  56  56  58  64  71  79  85  90  97 110 105 104    d

>21:4 >31:1 v32:1 >42:5 >52:4 >62:5 ^61:3 >71:2 >81:3 >91:1 v92:3 v93:5 >a3:4 >b3:2
    4     5     6    11    15    20    23    25
























 */
