use std::{
    collections::{HashMap, HashSet},
    fmt::Formatter,
};

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
    /// node
    N,
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
            Tile::N => write!(f, "N"),
        }
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

    #[allow(clippy::type_complexity)]
    fn traverse_nodes(
        node: (usize, usize),
        edges: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
        visited: &mut HashSet<(usize, usize)>,
        target: (usize, usize),
    ) -> Option<usize> {
        if node == target {
            return Some(0);
        }

        let mut results = vec![];
        for &(next, len) in edges.get(&node).unwrap() {
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            let next_len =
                Self::traverse_nodes(next, edges, visited, target).map(|next_len| next_len + len);
            results.push(next_len);
            visited.remove(&next);
        }

        results.into_iter().flatten().max()
    }

    fn traverse_ignore_slope_smarter(&mut self) -> usize {
        let mut edges = HashMap::<_, Vec<_>>::new();
        let mut stack = vec![(self.start, self.start, self.start, 0)];
        let mut add_edge = |t1: (usize, usize), t2: (usize, usize), len: usize| {
            edges.entry(t1).or_default().push((t2, len));
            edges.entry(t2).or_default().push((t1, len));
        };

        while let Some((curr_tile, prev_tile, prev_node, len)) = stack.pop() {
            let (x, y) = curr_tile;

            match self.get(x, y) {
                Tile::N => {
                    add_edge(curr_tile, prev_node, len);
                    continue;
                }
                _ if curr_tile == self.exit => {
                    add_edge(curr_tile, prev_node, len);
                    continue;
                }
                Tile::X => continue,
                _ => {}
            }

            let adjacent = [Dir::U, Dir::L, Dir::D, Dir::R]
                .iter()
                .flat_map(|&dir| self.try_step_ignore_slope(x, y, dir))
                .filter(|&tile| tile != prev_tile)
                .collect_vec();

            match adjacent.len() {
                0 => continue,
                // start or regular tile
                1 => {
                    self.set(x, y, Tile::X);
                    adjacent.into_iter().for_each(|tile| {
                        stack.push((tile, curr_tile, prev_node, len + 1));
                    });
                }
                // another node
                _ => {
                    self.set(x, y, Tile::N);
                    add_edge(curr_tile, prev_node, len);
                    adjacent.into_iter().for_each(|tile| {
                        stack.push((tile, curr_tile, curr_tile, 1));
                    });
                }
            }
        }

        Self::traverse_nodes(self.start, &edges, &mut HashSet::new(), self.exit).unwrap()
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
        .filter(|&(x, y)| !matches!(self.get(x, y), Tile::W))
    }

    fn longest_path(&mut self, ignore_slope: bool) -> usize {
        let (x, y) = self.start;
        if ignore_slope {
            self.traverse_ignore_slope_smarter()
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
