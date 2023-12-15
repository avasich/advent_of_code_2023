use std::{collections::HashMap, fmt::Formatter};

use crate::utils::{Day, Task};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Spring {
    Good,
    Bad,
    Unknown,
}

impl Spring {
    fn possibly_bad(&self) -> bool {
        !matches!(self, Spring::Good)
    }
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Good => write!(f, "."),
            Spring::Bad => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Good,
            '#' => Spring::Bad,
            '?' => Spring::Unknown,
            _ => unreachable!("wrong tile '{value}'"),
        }
    }
}

struct SpringLine {
    springs: Vec<Spring>,
    bad_spans: Vec<usize>,
    cache: HashMap<(usize, usize), usize>,
}

impl SpringLine {
    fn expand(
        springs: Vec<Spring>,
        bas_spans: Vec<usize>,
        times: usize,
    ) -> (Vec<Spring>, Vec<usize>) {
        let new_springs = std::iter::repeat(springs)
            .take(times)
            .intersperse(vec![Spring::Unknown])
            .flatten()
            .collect();

        let new_spans = std::iter::repeat(bas_spans).take(times).flatten().collect();

        (new_springs, new_spans)
    }

    fn from_line(line: &str, repeat: usize) -> Self {
        use itertools::Itertools;

        let (springs_line, bad_spans_line) = line.split_once(' ').unwrap();
        let mut springs = vec![];

        for (possibly_bad, spring_span) in &springs_line
            .chars()
            .map(Spring::from)
            .group_by(Spring::possibly_bad)
        {
            if possibly_bad {
                springs.extend(spring_span);
            } else {
                springs.push(Spring::Good)
            }
        }

        let bad_spans = bad_spans_line.split(',').flat_map(str::parse).collect();

        let (springs, bad_spans) = Self::expand(springs, bad_spans, repeat);

        Self {
            springs,
            bad_spans,
            cache: HashMap::new(),
        }
    }

    fn bad_len(&self) -> usize {
        self.bad_spans.len()
    }

    fn spring_at(&self, i: usize) -> Option<&Spring> {
        self.springs.get(i)
    }

    fn bad_span_at(&self, i: usize) -> Option<&usize> {
        self.bad_spans.get(i)
    }

    fn possibly_bad_count(&self, start: usize, cap: usize) -> usize {
        self.springs[start..]
            .iter()
            .take(cap)
            .take_while(|spring| spring.possibly_bad())
            .count()
    }

    fn next_bad(&self, start: usize, cap: usize) -> Option<usize> {
        self.springs[start..]
            .iter()
            .take(cap)
            .position(|spring| matches!(spring, Spring::Bad))
    }

    fn count_variants(&mut self, si: usize, bi: usize) -> usize {
        if let Some(res) = self.cache.get(&(si, bi)) {
            return *res;
        }

        let res = match (self.spring_at(si), self.bad_span_at(bi)) {
            (_, None) if self.springs[si..].contains(&Spring::Bad) => 0,
            (_, None) => 1,
            (None, Some(_)) => 0,
            (Some(Spring::Good), Some(_)) => self.count_variants(si + 1, bi),
            (Some(Spring::Bad), Some(&n)) => {
                let possibly_bad_count = self.possibly_bad_count(si, n);

                match self.spring_at(si + n) {
                    _ if possibly_bad_count != n => 0,
                    None if self.bad_len() - bi > 1 => 0,
                    None => 1,
                    Some(Spring::Bad) => 0,
                    Some(_) => self.count_variants(si + n + 1, bi + 1),
                }
            }
            (Some(Spring::Unknown), Some(&n)) => {
                let possibly_bad_count = self.possibly_bad_count(si, n);

                match self.spring_at(si + n) {
                    _ if possibly_bad_count != n => {
                        let continue_from = self
                            .next_bad(si, possibly_bad_count)
                            .unwrap_or(possibly_bad_count);
                        self.count_variants(si + continue_from, bi)
                    }
                    None if self.bad_len() - bi > 1 => 0,
                    None => 1,
                    Some(Spring::Bad) => self.count_variants(si + 1, bi),
                    Some(_) => {
                        let starting_here = self.count_variants(si + n + 1, bi + 1);
                        let skipping_it = self.count_variants(si + 1, bi);
                        starting_here + skipping_it
                    }
                }
            }
        };

        self.cache.insert((si, bi), res);

        res
    }
}

fn part_1(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .map(|line| SpringLine::from_line(&line, 1))
        .map(|mut line| line.count_variants(0, 0))
        .sum()
}

fn part_2(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .map(|line| SpringLine::from_line(&line, 5))
        .map(|mut line| line.count_variants(0, 0))
        .sum()
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
mod d12_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(21, res);
    }

    fn from_line(line: &str) -> usize {
        let mut spring_line = SpringLine::from_line(line, 1);
        spring_line.count_variants(0, 0)
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
