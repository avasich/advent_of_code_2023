use std::fmt::Formatter;

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Eq, PartialEq)]
enum Tile {
    Good,
    Bad,
    Unknown,
}

impl Tile {
    fn possibly_bad(&self) -> bool {
        !matches!(self, Tile::Good)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Good => write!(f, "."),
            Tile::Bad => write!(f, "#"),
            Tile::Unknown => write!(f, "?"),
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Good,
            '#' => Tile::Bad,
            '?' => Tile::Unknown,
            _ => unreachable!("wrong tile '{value}'"),
        }
    }
}

fn parse_line(line: &str) -> (Vec<Tile>, Vec<usize>) {
    let (spans_line, consecutives) = line.split_once(' ').unwrap();
    let mut spans = vec![];

    for (possibly_bad, span) in &spans_line
        .chars()
        .map(Tile::from)
        .group_by(Tile::possibly_bad)
    {
        if possibly_bad {
            spans.extend(span);
        } else {
            spans.push(Tile::Good)
        }
    }

    let consecutives = consecutives.split(',').flat_map(str::parse).collect();

    (spans, consecutives)
}

fn count_variants(spans: &[Tile], consecutives: &[usize]) -> usize {
    fn bad_count(spans: &[Tile], cap: usize) -> usize {
        spans
            .iter()
            .take(cap)
            .take_while(|tile| tile.possibly_bad())
            .count()
    }

    match (spans.first(), consecutives.first()) {
        (_, None) if spans.contains(&Tile::Bad) => 0,
        (_, None) => 1,
        (None, Some(_)) => 0,
        (Some(Tile::Good), Some(_)) => count_variants(&spans[1..], consecutives),
        (Some(Tile::Bad), Some(&n)) => {
            let matching = bad_count(spans, n);

            match spans.get(n) {
                _ if matching != n => 0,
                None if consecutives.len() > 1 => 0,
                None => 1,
                Some(Tile::Bad) => 0,
                Some(_) => count_variants(&spans[(n + 1)..], &consecutives[1..]),
            }
        }
        (Some(Tile::Unknown), Some(&n)) => {
            let matching = bad_count(spans, n);

            match spans.get(n) {
                _ if matching != n => {
                    let continue_from = spans
                        .iter()
                        .take(matching)
                        .position(|tile| matches!(tile, Tile::Bad))
                        .unwrap_or(matching);
                    count_variants(&spans[continue_from..], consecutives)
                }
                None if consecutives.len() > 1 => 0,
                None => 1,
                Some(Tile::Bad) => count_variants(&spans[1..], consecutives),
                Some(_) => {
                    let starting_here = count_variants(&spans[(n + 1)..], &consecutives[1..]);
                    let skipping_it = count_variants(&spans[1..], consecutives);
                    starting_here + skipping_it
                }
            }
        }
    }
}

fn part_1(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .map(|line| parse_line(&line))
        .map(|(spans, consecutives)| count_variants(&spans, &consecutives))
        .sum()
}

fn part_2(filename: &str) -> usize {
    println!("part 2 not solved");
    0
    // todo!()
}

pub fn solution() -> Day<usize, usize> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_12/example_01.txt"],
            task: "./inputs/day_12/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_12/example_03.txt"],
            task: "./inputs/day_12/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d11_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(21, res);
    }

    fn from_line(line: &str) -> usize {
        let (spans, cons) = parse_line(line);
        count_variants(&spans, &cons)
    }

    #[test]
    fn p1_test_lines() {
        assert_eq!(4, from_line("???????????.??? 10,2"));
        assert_eq!(6, from_line("#..?????#???#???#? 1,3,6,2"));
        assert_eq!(4, from_line("?..?.??.#?#????#???. 1,2,8,1"));
        assert_eq!(2, from_line("?##.???#?#?#?###? 2,11"));
        assert_eq!(6, from_line("?#???.?#???#?#????#? 2,1,3,3,5"));
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(374, res);
    }
}
