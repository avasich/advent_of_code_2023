#![allow(dead_code)]

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

    fn cross_product_xy(&self, other: &Point) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Debug, Hash)]
struct Brick {
    a: Point,
    b: Point,
}

impl Brick {
    fn from_string(s: &str) -> Self {
        let (a, b) = s.split_once('~').unwrap();
        let a = a.split(',').flat_map(i32::from_str).collect_vec();
        let b = b.split(',').flat_map(i32::from_str).collect_vec();
        let a = Point::new(a[0], a[1], a[2]);
        let b = Point::new(b[0], b[1], b[2]);

        if a.z <= b.z {
            Self::new(a, b)
        } else {
            Self::new(b, a)
        }
    }

    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn set_height(&mut self, height: i32) {
        let h = self.b.z - self.a.z;
        self.a.z = height;
        self.b.z = height + h;
    }

    fn intersects_xy(&self, other: &Brick) -> bool {
        let x_overlap = (self.a.x <= other.a.x && other.a.x <= self.b.x)
            || (other.a.x <= self.a.x && self.a.x <= other.b.x);
        let y_overlap = (self.a.y <= other.a.y && other.a.y <= self.b.y)
            || (other.a.y <= self.a.y && self.a.y <= other.b.y);
        x_overlap && y_overlap
    }
}

fn support_matrix(mut bricks: Vec<Brick>) -> Vec<HashSet<usize>> {
    let mut supports = vec![HashSet::new(); bricks.len()];

    for j in 0..bricks.len() {
        let supporter = (0..j)
            .filter(|&i| bricks[i].intersects_xy(&bricks[j]))
            .max_by_key(|&i| bricks[i].b.z);

        match supporter {
            None => bricks[j].set_height(1),
            Some(i) => {
                let height = bricks[i].b.z;
                bricks[0..=i]
                    .iter()
                    .enumerate()
                    .filter(|(_, brick)| brick.b.z == height && brick.intersects_xy(&bricks[j]))
                    .for_each(|(k, _)| {
                        let _ = supports[k].insert(j);
                    });
                bricks[j].set_height(height + 1);
            }
        }
    }

    supports
}

fn parse_file(filename: &str) -> Vec<Brick> {
    crate::utils::read_lines(filename)
        .map(|line| Brick::from_string(&line))
        .sorted_by_key(|brick| (brick.a.z, brick.b.z))
        .collect_vec()
}

fn part_1(filename: &str) -> u32 {
    let bricks = parse_file(filename);

    let supports = support_matrix(bricks);

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
    let supports = support_matrix(bricks);

    let mut heights = vec![0; supports.len()];

    for i in 0..supports.len() {
        let mut affected = VecDeque::from_iter(supports[i].iter());
        let mut fallen = HashSet::from([i]);

        while let Some(j) = affected.pop_front() {
            let has_more_supports = (0..supports.len())
                .filter(|k| !fallen.contains(k))
                .any(|k| supports[k].contains(j));

            if has_more_supports {
                continue;
            }

            fallen.insert(*j);
            affected.extend(supports[*j].iter());
        }

        heights[i] = fallen.len() - 1;
    }

    heights.iter().sum::<usize>() as u32
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
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(2, -4, 0),
            b: Point::new(2, 10, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // touch
        let brick1 = Brick {
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(2, 0, 0),
            b: Point::new(2, 10, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // contain
        let brick1 = Brick {
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(2, 0, 0),
            b: Point::new(5, 0, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // partial contain
        let brick1 = Brick {
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(7, 0, 0),
            b: Point::new(15, 0, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));

        // same line, don't intersect
        let brick1 = Brick {
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(13, 0, 0),
            b: Point::new(44, 0, 0),
        };

        assert!(!brick1.intersects_xy(&brick2));
        assert!(!brick2.intersects_xy(&brick1));

        // parallel, don't intersect
        let brick1 = Brick {
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(1, 1, 0),
            b: Point::new(10, 1, 0),
        };

        assert!(!brick1.intersects_xy(&brick2));
        assert!(!brick2.intersects_xy(&brick1));

        // don't intersect
        let brick1 = Brick {
            a: Point::new(1, 0, 0),
            b: Point::new(10, 0, 0),
        };

        let brick2 = Brick {
            a: Point::new(2, 3, 0),
            b: Point::new(2, 5, 0),
        };

        assert!(!brick1.intersects_xy(&brick2));
        assert!(!brick2.intersects_xy(&brick1));

        let brick1 = Brick {
            a: Point::new(0, 1, 0),
            b: Point::new(2, 1, 0),
        };

        let brick2 = Brick {
            a: Point::new(2, 0, 0),
            b: Point::new(2, 2, 0),
        };

        assert!(brick1.intersects_xy(&brick2));
        assert!(brick2.intersects_xy(&brick1));
    }
}
