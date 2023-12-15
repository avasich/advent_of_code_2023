use std::{cell::RefCell, fmt::Formatter};

use itertools::{FoldWhile, Itertools};

use crate::utils::{Day, Task};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Square,
    Round,
    Empty,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Square => '#',
            Tile::Round => 'O',
            Tile::Empty => '.',
        })
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Square,
            'O' => Tile::Round,
            _ => unreachable!(),
        }
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

struct Board {
    width: usize,
    height: usize,
    tiles: RefCell<Vec<Tile>>,
    row_spans: Vec<Vec<(usize, usize)>>,
    col_spans: Vec<Vec<(usize, usize)>>,
}

impl Board {
    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();

        let width = lines[0].len();
        let height = lines.len();

        let tiles = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Tile::from)
            .collect_vec();

        fn free_spans(tiles: impl Iterator<Item = &Tile>, length: usize) -> Vec<(usize, usize)> {
            let squares = tiles
                .positions(|tile| matches!(tile, Tile::Square))
                .map(|i| i as isize);

            let obstacles = std::iter::once(-1)
                .chain(squares)
                .chain(std::iter::once(length as isize));

            obstacles
                .tuple_windows::<(_, _)>()
                .filter(|(a, b)| b - a > 1)
                .map(|(a, b)| ((a + 1) as usize, b as usize))
                .collect_vec()
        }

        let ts = &tiles;

        let horizontal_spans = (0..height)
            .map(|y| (0..width).map(move |x| &ts[x + y * width]))
            .map(|row| free_spans(row, width))
            .collect_vec();

        let vertical_spans = (0..width)
            .map(|x| (0..height).map(move |y| &ts[x + y * width]))
            .map(|col| free_spans(col, height))
            .collect_vec();

        Self {
            width,
            height,
            tiles: RefCell::new(tiles),
            row_spans: horizontal_spans,
            col_spans: vertical_spans,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles.borrow()[x + y * self.width]
    }

    fn fill_col(&self, col: usize, y1: usize, y2: usize, tile: Tile) {
        let mut tiles = self.tiles.borrow_mut();
        (y1..y2).for_each(|y| tiles[col + y * self.width] = tile);
    }

    fn fill_row(&self, row: usize, x1: usize, x2: usize, tile: Tile) {
        let i1 = x1 + row * self.width;
        let i2 = x2 + row * self.width;
        self.tiles.borrow_mut()[i1..i2].fill(tile);
    }

    fn count_in_col(&self, col: usize, y1: usize, y2: usize, tile: Tile) -> usize {
        let tiles = self.tiles.borrow();
        (y1..y2)
            .map(|y| tiles[col + y * self.width])
            .filter(|t| *t == tile)
            .count()
    }

    fn count_in_row(&self, row: usize, x1: usize, x2: usize, tile: Tile) -> usize {
        let i1 = x1 + row * self.width;
        let i2 = x2 + row * self.width;
        self.tiles.borrow()[i1..i2]
            .iter()
            .filter(|t| **t == tile)
            .count()
    }

    fn tilt(&self, dir: Direction) -> &Self {
        match dir {
            Direction::North => {
                for col in 0..self.width {
                    let spans = &self.col_spans[col];
                    for &(y1, y2) in spans {
                        let rc = self.count_in_col(col, y1, y2, Tile::Round);
                        self.fill_col(col, y1, y1 + rc, Tile::Round);
                        self.fill_col(col, y1 + rc, y2, Tile::Empty);
                    }
                }
            }
            Direction::West => {
                for row in 0..self.height {
                    let spans = &self.row_spans[row];
                    for &(x1, x2) in spans {
                        let rc = self.count_in_row(row, x1, x2, Tile::Round);
                        self.fill_row(row, x1, x1 + rc, Tile::Round);
                        self.fill_row(row, x1 + rc, x2, Tile::Empty);
                    }
                }
            }
            Direction::South => {
                for col in 0..self.width {
                    let spans = &self.col_spans[col];
                    for &(y1, y2) in spans {
                        let rc = self.count_in_col(col, y1, y2, Tile::Round);
                        self.fill_col(col, y2 - rc, y2, Tile::Round);
                        self.fill_col(col, y1, y2 - rc, Tile::Empty);
                    }
                }
            }
            Direction::East => {
                for row in 0..self.height {
                    let spans = &self.row_spans[row];
                    for &(x1, x2) in spans {
                        let rc = self.count_in_row(row, x1, x2, Tile::Round);
                        self.fill_row(row, x2 - rc, x2, Tile::Round);
                        self.fill_row(row, x1, x2 - rc, Tile::Empty);
                    }
                }
            }
        }

        self
    }

    fn rotate(&self) {
        self.tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East);
    }

    fn rotate_n(&self, n: usize) {
        (1..=n).fold_while(vec![], |mut history, curr| {
            history.push(self.tiles.borrow().clone());
            self.rotate();

            let found = history
                .iter()
                .position(|tiles| &*self.tiles.borrow() == tiles);

            match found {
                None => FoldWhile::Continue(history),
                Some(prev) => {
                    let period = curr - prev;
                    let r = (n - prev) % period;
                    (0..r).for_each(|_| self.rotate());
                    FoldWhile::Done(history)
                }
            }
        });
    }

    fn weight(&self) -> u64 {
        (0..self.height)
            .map(|row| (row, self.count_in_row(row, 0, self.width, Tile::Round)))
            .map(|(row, count)| (self.height - row) * count)
            .sum::<usize>() as u64
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{}x{}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y));
            }
            println!();
        }
        fn print_spans(all_spans: &[Vec<(usize, usize)>]) {
            for (i, spans) in all_spans.iter().enumerate() {
                print!("{i}: ");
                for &(a, b) in spans {
                    print!("{a}:{b} ");
                }
                println!();
            }
        }
        println!("\nrows:");
        print_spans(&self.row_spans);
        println!("\ncols:");
        print_spans(&self.col_spans);
    }
}

fn part_1(filename: &str) -> u64 {
    let board = Board::from_file(filename);
    board.tilt(Direction::North);
    board.weight()
}

fn part_2(filename: &str) -> u64 {
    let board = Board::from_file(filename);
    board.rotate_n(1_000_000_000);
    board.weight()
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_14/example_01.txt"],
            task: "./inputs/day_14/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_14/example_01.txt"],
            task: "./inputs/day_14/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d14_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(136, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(64, res);
    }
}
