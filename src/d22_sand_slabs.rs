use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Hash)]
struct Brick {
    p: Point,
    q: Point,
}

impl Brick {
    fn from_string(s: &str) -> Self {
        let (p, q) = s.split_once('~').unwrap();
        let p = p.split(',').flat_map(i32::from_str).collect_vec();
        let q = q.split(',').flat_map(i32::from_str).collect_vec();
        let p = Point::new(p[0], p[1], p[2]);
        let q = Point::new(q[0], q[1], q[2]);

        if p.z <= q.z {
            Self::new(p, q)
        } else {
            Self::new(q, p)
        }
    }

    fn new(a: Point, b: Point) -> Self {
        Self { p: a, q: b }
    }

    fn set_height(&mut self, height: i32) {
        let h = self.q.z - self.p.z;
        self.p.z = height;
        self.q.z = height + h;
    }

    fn intersects_xy(&self, other: &Brick) -> bool {
        let x_overlap = (self.p.x <= other.p.x && other.p.x <= self.q.x)
            || (other.p.x <= self.p.x && self.p.x <= other.q.x);
        let y_overlap = (self.p.y <= other.p.y && other.p.y <= self.q.y)
            || (other.p.y <= self.p.y && self.p.y <= other.q.y);
        x_overlap && y_overlap
    }
}

fn support_matrix(mut bricks: Vec<Brick>) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut supports = vec![HashSet::new(); bricks.len()];
    let mut supporters = vec![HashSet::new(); bricks.len()];

    for j in 0..bricks.len() {
        let supporter = (0..j)
            .filter(|&i| bricks[i].intersects_xy(&bricks[j]))
            .max_by_key(|&i| bricks[i].q.z);

        match supporter {
            None => bricks[j].set_height(1),
            Some(i) => {
                let height = bricks[i].q.z;
                bricks[0..=i]
                    .iter()
                    .enumerate()
                    .filter(|(_, brick)| brick.q.z == height && brick.intersects_xy(&bricks[j]))
                    .for_each(|(k, _)| {
                        let _ = supports[k].insert(j);
                        let _ = supporters[j].insert(k);
                    });
                bricks[j].set_height(height + 1);
            }
        }
    }

    (supports, supporters)
}

fn parse_file(filename: &str) -> Vec<Brick> {
    crate::utils::read_lines(filename)
        .map(|line| Brick::from_string(&line))
        .sorted_by_key(|brick| (brick.p.z, brick.q.z))
        .collect_vec()
}

fn part_1(filename: &str) -> u32 {
    let bricks = parse_file(filename);

    let (supports, _) = support_matrix(bricks);

    supports
        .iter()
        .filter(|supported| {
            supported
                .iter()
                .map(|j| supports.iter().filter(|row| row.contains(j)).count())
                .all(|count| count > 1)
        })
        .count() as u32
}

fn part_2(filename: &str) -> u32 {
    let bricks = parse_file(filename);
    let (supports, supporters) = support_matrix(bricks);

    let mut bricks_on_top = vec![0; supports.len()];

    for i in 0..supports.len() {
        let mut affected = VecDeque::from_iter(supports[i].iter());
        let mut fallen = HashSet::from([i]);

        while let Some(&j) = affected.pop_front() {
            let has_more_supports = supporters[j].difference(&fallen).count() > 0;

            if has_more_supports {
                continue;
            }

            fallen.insert(j);
            affected.extend(supports[j].iter());
        }

        bricks_on_top[i] = fallen.len() - 1;
    }

    bricks_on_top.iter().sum::<usize>() as u32
}

pub fn solution() -> Day<u32, u32> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_22/example_01.txt"],
            task: "./inputs/day_22/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_22/example_01.txt"],
            task: "./inputs/day_22/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d22_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example(0);
        assert_eq!(res, 5);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example(0);
        assert_eq!(res, 7);
    }

    #[test]
    fn brick_overlapping() {
        // intersect
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(2, -4, 0),
            q: Point::new(2, 10, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // touch
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(2, 0, 0),
            q: Point::new(2, 10, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // contain
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(2, 0, 0),
            q: Point::new(5, 0, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // partial contain
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(7, 0, 0),
            q: Point::new(15, 0, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // same line, don't intersect
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(13, 0, 0),
            q: Point::new(44, 0, 0),
        };

        assert!(!brick1.intersects_xy(&brick2));
        assert!(!brick2.intersects_xy(&brick1));

        // parallel, don't intersect
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(1, 1, 0),
            q: Point::new(10, 1, 0),
        };

        assert!(!brick1.intersects_xy(&brick2));
        assert!(!brick2.intersects_xy(&brick1));

        // don't intersect
        let brick1 = Brick {
            p: Point::new(1, 0, 0),
            q: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            p: Point::new(2, 3, 0),
            q: Point::new(2, 5, 0),
        };

        assert!(!brick1.intersects_xy(&brick2));
        assert!(!brick2.intersects_xy(&brick1));

        let brick1 = Brick {
            p: Point::new(0, 1, 0),
            q: Point::new(2, 1, 0),
        };

        let brick2 = Brick {
            p: Point::new(2, 0, 0),
            q: Point::new(2, 2, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));
    }
}
