use std::{cell::RefCell, collections::HashSet, fmt::Formatter, rc::Rc};

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    L,
    D,
    R,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    U,
    L,
    D,
    R,
    /// empty
    E,
    /// wall
    W,
    /// traversed
    X,
}

impl Tile {
    fn is_walkable(&self) -> bool {
        !matches!(self, Self::W | Self::X)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::U,
            '<' => Self::L,
            'v' => Self::D,
            '>' => Self::R,
            '.' => Self::E,
            '#' => Self::W,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::U => write!(f, "^"),
            Tile::L => write!(f, "<"),
            Tile::D => write!(f, "v"),
            Tile::R => write!(f, ">"),
            Tile::E => write!(f, "."),
            Tile::W => write!(f, "#"),
            Tile::X => write!(f, "x"),
        }
    }
}

struct Node {
    xy: (usize, usize),
    prev: Option<Rc<RefCell<Node>>>,
    children: usize,
}

impl Node {
    fn new(xy: (usize, usize)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            xy,
            prev: None,
            children: 0,
        }))
    }

    fn attach(xy: (usize, usize), other: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let node = Self::new(xy);
        node.borrow_mut().prev = Some(Rc::clone(other));
        node
    }

    fn remove_child(&mut self) {
        self.children -= 1;
    }
}

struct Labyrinth {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
    start: (usize, usize),
    exit: (usize, usize),
}

impl Labyrinth {
    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();
        let h = lines.len();
        let w = lines[0].len();
        let tiles = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Tile::from)
            .collect_vec();

        let (start_x, _) = tiles.iter().find_position(|t| t.is_walkable()).unwrap();
        let (exit_x, _) = tiles
            .iter()
            .rev()
            .find_position(|t| t.is_walkable())
            .unwrap();
        let exit_x = w - exit_x - 1;

        Self {
            tiles,
            w,
            h,
            start: (start_x, 0),
            exit: (exit_x, h - 1),
        }
    }

    fn traverse(&mut self, x: usize, y: usize, len: usize) -> Option<usize> {
        if (x, y) == self.exit {
            return Some(len);
        } else if !self.get(x, y).is_walkable() {
            return None;
        }

        let tile = self.get(x, y);

        [Dir::U, Dir::L, Dir::D, Dir::R]
            .into_iter()
            .flat_map(|dir| {
                self.try_step(x, y, dir).and_then(|(x1, y1)| {
                    self.set(x, y, Tile::X);
                    let res = self.traverse(x1, y1, len + 1);
                    self.set(x, y, tile);
                    res
                })
            })
            .max()
    }

    fn traverse_ignore_slope(&mut self) -> usize {
        let mut res = 0;

        let mut stack = vec![(Node::new(self.start), 0)];
        let mut visited = HashSet::new();

        fn unwind(mut node: Rc<RefCell<Node>>, visited: &mut HashSet<(usize, usize)>) {
            while node.borrow().children == 0 {
                visited.remove(&node.borrow().xy);
                match &node.clone().borrow().prev {
                    None => break,
                    Some(prev) => {
                        prev.borrow_mut().remove_child();
                        node = prev.clone();
                    }
                }
            }
        }

        while let Some((node, len)) = stack.pop() {
            let (x, y) = node.borrow().xy;
            if (x, y) == self.exit {
                if len > res {
                    res = len;
                }
                unwind(node, &mut visited);
                continue;
            }

            let to_check = [Dir::U, Dir::L, Dir::D, Dir::R]
                .into_iter()
                .flat_map(|dir| self.try_step_ignore_slope(x, y, dir))
                .filter(|next| !visited.contains(next))
                .collect_vec();

            if to_check.is_empty() {
                unwind(node, &mut visited);
            } else {
                visited.insert((x, y));
                node.borrow_mut().children += to_check.len();
                stack.extend(
                    to_check
                        .into_iter()
                        .map(|next| Node::attach(next, &node))
                        .map(|next| (next, len + 1)),
                )
            }
        }

        res
    }

    fn try_step(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match (dir, self.get(x, y)) {
            (Dir::U, Tile::U | Tile::E) if y > 0 => Some((x, y - 1)),
            (Dir::L, Tile::L | Tile::E) if x > 0 => Some((x - 1, y)),
            (Dir::D, Tile::D | Tile::E) if y + 1 < self.h => Some((x, y + 1)),
            (Dir::R, Tile::R | Tile::E) if x + 1 < self.w => Some((x + 1, y)),
            _ => None,
        }
        .filter(|&(x, y)| self.get(x, y).is_walkable())
    }

    fn try_step_ignore_slope(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::U if y > 0 => Some((x, y - 1)),
            Dir::L if x > 0 => Some((x - 1, y)),
            Dir::D if y + 1 < self.h => Some((x, y + 1)),
            Dir::R if x + 1 < self.w => Some((x + 1, y)),
            _ => None,
        }
        .filter(|&(x, y)| self.get(x, y).is_walkable())
    }

    fn longest_path(&mut self, ignore_slope: bool) -> usize {
        let (x, y) = self.start;
        if ignore_slope {
            self.traverse_ignore_slope()
        } else {
            self.traverse(x, y, 0).unwrap()
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.w]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[x + y * self.w] = tile;
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.tiles.chunks(self.w).for_each(|row| {
            row.iter().for_each(|t| print!("{t}"));
            println!();
        });
    }
}

fn part_1(filename: &str) -> usize {
    Labyrinth::from_file(filename).longest_path(false)
}

fn part_2(filename: &str) -> usize {
    Labyrinth::from_file(filename).longest_path(true)
}

pub fn solution() -> Day<usize, usize> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_23/example_01.txt"],
            task: "./inputs/day_23/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_23/example_01.txt"],
            task: "./inputs/day_23/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d23_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(94, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(154, res);
    }
}
