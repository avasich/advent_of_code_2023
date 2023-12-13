use std::{cmp::Ordering, collections::BTreeSet, ops::Range, str::FromStr};

use crate::utils::{Day, Solution, Task};

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .flat_map(|val| val.parse().ok())
        .collect()
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Mapping {
    src: u64,
    dst: u64,
    len: u64,
}

impl Mapping {
    fn new(dst: u64, src: u64, len: u64) -> Self {
        Self { dst, src, len }
    }

    fn src_range(&self) -> Range<u64> {
        self.src..(self.src + self.len)
    }

    fn dst_range(&self) -> Range<u64> {
        self.dst..(self.dst + self.len)
    }

    fn map_value(&self, val: u64) -> Option<u64> {
        self.src_range()
            .contains(&val)
            .then(|| self.map_value_unchecked(val))
    }

    fn map_value_unchecked(&self, val: u64) -> u64 {
        self.dst + val - self.src
    }

    fn map_value_many(val: u64, mappings: &[Mapping]) -> u64 {
        mappings
            .binary_search_by(|m| match val {
                _ if val < m.src => Ordering::Greater,
                _ if m.src + m.len <= val => Ordering::Less,
                _ => Ordering::Equal,
            })
            .map_or(val, |i| mappings[i].map_value_unchecked(val))
    }

    fn chain_mappings(
        mut dst_map: BTreeSet<Mapping>,
        mut src_map: BTreeSet<Mapping>,
    ) -> BTreeSet<Mapping> {
        let res = BTreeSet::new();
        res
    }

    /*
    tth:
        1  0 69 : [ 0, 68] => [1, 69]
        0 69  1 : [69, 69] => [0,  0]

    htl:
        60 56 37 : [56, 92] => [60, 96]
        56 93  4 : [93, 96] => [56, 59]

    tth:
        [1, 69] A [56, 92] = [56, 69], rem tth [1, 55], rem htl [70, 92]
        [0, 68] = [0, 54] + [55, 68]

        [0, 54] => [1, 55]
        [55, 68] => [56, 69] => [60, 73]
        [69, 69] => [0, 0]
        [70, 92] => [74, 96]
        [93, 96] => [56, 59]

     */
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.src.cmp(&other.src)
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s
            .split_whitespace()
            .flat_map(|number| number.parse::<u64>().ok());

        let dst = numbers.next().ok_or(())?;
        let src = numbers.next().ok_or(())?;
        let len = numbers.next().ok_or(())?;

        Ok(Self::new(dst, src, len))
    }
}

fn parse_file(filename: &str) -> (Vec<u64>, Vec<Vec<Mapping>>) {
    let mut lines = crate::utils::read_lines(filename);
    let seeds = lines
        .next()
        .map(|line| parse_numbers(line.split_once(':').unwrap().1))
        .unwrap();

    lines.next();

    let mut tmp = vec![];
    let mut maps = vec![];

    for line in lines {
        if line.is_empty() {
            tmp.sort();
            maps.push(tmp);
            tmp = vec![];
            continue;
        }
        if line.ends_with(':') {
            continue;
        }
        tmp.push(line.parse().unwrap());
    }

    tmp.sort();
    maps.push(tmp);

    (seeds, maps)
}

pub fn lowest_location(filename: &str) -> u64 {
    let (seeds, maps) = parse_file(filename);
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .map(Vec::as_slice)
                .fold(seed, Mapping::map_value_many)
        })
        .min()
        .unwrap()
}

pub fn lowest_location_range(filename: &str) -> u64 {
    let (seeds, maps) = parse_file(filename);
    1
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            example: "./inputs/day_05/example_01.txt",
            task: "./inputs/day_05/task.txt",
            run: lowest_location,
        },

        part_2: Task {
            example: "./inputs/day_05/example_01.txt",
            task: "./inputs/day_05/task.txt",
            run: lowest_location_range,
        },
    }
}

#[cfg(test)]
mod d05_tests {
    use super::*;

    #[test]
    fn map_value_test() {
        let (_, maps) = parse_file(solution().part_1.example);

        let res = Mapping::map_value_many(1, &maps[0]);
        assert_eq!(res, 1);

        let res = Mapping::map_value_many(79, &maps[0]);
        assert_eq!(res, 81);

        let res = Mapping::map_value_many(99, &maps[0]);
        assert_eq!(res, 51);
    }

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example();
        assert_eq!(res, 35);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example();
        assert_eq!(res, 46);
    }
}
