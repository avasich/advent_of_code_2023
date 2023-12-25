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

enum Dir {
    N,
    W,
    S,
    E,
}

struct Board {
    w: usize,
    h: usize,
    tiles: RefCell<Vec<Tile>>,
    row_spans: Vec<Vec<(usize, usize)>>,
    col_spans: Vec<Vec<(usize, usize)>>,
}

impl Board {
    fn from_file(filename: &str) -> Self {
        let lines = crate::utils::read_lines(filename).collect_vec();

        let w = lines[0].len();
        let h = lines.len();

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

        let row_spans = tiles
            .chunks(w)
            .map(|row| free_spans(row.iter(), w))
            .collect_vec();

        let col_spans = (0..w)
            .map(|x| tiles.iter().skip(x).step_by(w))
            .map(|col| free_spans(col, h))
            .collect_vec();

        Self {
            w,
            h,
            tiles: RefCell::new(tiles),
            row_spans,
            col_spans,
        }
    }

    fn fill_col(&self, col: usize, y1: usize, y2: usize, tile: Tile) {
        let mut tiles = self.tiles.borrow_mut();
        let i1 = col + y1 * self.w;
        let i2 = col + y2 * self.w;
        (i1..i2).step_by(self.w).for_each(|i| tiles[i] = tile);
    }

    fn fill_row(&self, row: usize, x1: usize, x2: usize, tile: Tile) {
        let i1 = x1 + row * self.w;
        let i2 = x2 + row * self.w;
        self.tiles.borrow_mut()[i1..i2].fill(tile);
    }

    fn count_in_col(&self, col: usize, y1: usize, y2: usize, tile: Tile) -> usize {
        let tiles = self.tiles.borrow();
        let i1 = col + y1 * self.w;
        let i2 = col + y2 * self.w;
        (i1..i2)
            .step_by(self.w)
            .filter(|i| tiles[*i] == tile)
            .count()
    }

    fn count_in_row(&self, row: usize, x1: usize, x2: usize, tile: Tile) -> usize {
        let i1 = x1 + row * self.w;
        let i2 = x2 + row * self.w;
        self.tiles.borrow()[i1..i2]
            .iter()
            .filter(|t| **t == tile)
            .count()
    }

    fn tilt(&self, dir: Dir) -> &Self {
        match dir {
            Dir::N => {
                for col in 0..self.w {
                    let spans = &self.col_spans[col];
                    for &(y1, y2) in spans {
                        let rc = self.count_in_col(col, y1, y2, Tile::Round);
                        self.fill_col(col, y1, y1 + rc, Tile::Round);
                        self.fill_col(col, y1 + rc, y2, Tile::Empty);
                    }
                }
            }
            Dir::W => {
                for row in 0..self.h {
                    let spans = &self.row_spans[row];
                    for &(x1, x2) in spans {
                        let rc = self.count_in_row(row, x1, x2, Tile::Round);
                        self.fill_row(row, x1, x1 + rc, Tile::Round);
                        self.fill_row(row, x1 + rc, x2, Tile::Empty);
                    }
                }
            }
            Dir::S => {
                for col in 0..self.w {
                    let spans = &self.col_spans[col];
                    for &(y1, y2) in spans {
                        let rc = self.count_in_col(col, y1, y2, Tile::Round);
                        self.fill_col(col, y2 - rc, y2, Tile::Round);
                        self.fill_col(col, y1, y2 - rc, Tile::Empty);
                    }
                }
            }
            Dir::E => {
                for row in 0..self.h {
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
        self.tilt(Dir::N).tilt(Dir::W).tilt(Dir::S).tilt(Dir::E);
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
        (0..self.h)
            .map(|row| (row, self.count_in_row(row, 0, self.w, Tile::Round)))
            .map(|(row, count)| (self.h - row) * count)
            .sum::<usize>() as u64
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{}x{}", self.w, self.h);
        for row in self.tiles.borrow().chunks(self.w) {
            row.iter().for_each(|tile| print!("{tile}"));
            println!()
        }
        fn print_spans(all_spans: &[Vec<(usize, usize)>]) {
            for (i, spans) in all_spans.iter().enumerate() {
                print!("{i}: ");
                spans.iter().for_each(|(a, b)| print!("{a}:{b} "));
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
    board.tilt(Dir::N);
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
