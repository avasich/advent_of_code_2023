use std::{cmp::Ordering, ops::Range, str::FromStr};

use crate::utils::{Day, Task};

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .flat_map(|val| val.parse().ok())
        .collect()
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct MappingRange {
    src: u64,
    dst: u64,
    len: u64,
}

impl MappingRange {
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
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn binary_search_range(&self, val: u64) -> Result<usize, usize> {
        self.ranges.binary_search_by(|m| match val {
            _ if val < m.src => Ordering::Greater,
            _ if m.src + m.len <= val => Ordering::Less,
            _ => Ordering::Equal,
        })
    }

    fn map_value(&self, val: u64) -> u64 {
        self.binary_search_range(val)
            .map_or(val, |i| self.ranges[i].map_value_unchecked(val))
    }

    fn collapse_mappings(&self, other: &Self) -> Mapping {
        let mut ranges = vec![];

        for range in &self.ranges {
            let MappingRange {
                mut src,
                mut dst,
                mut len,
            } = *range;
            let mut left = other.binary_search_range(dst);
            let right = other.binary_search_range(dst + len - 1);

            loop {
                if left == right {
                    if left.is_ok() {
                        let containing_range = other.ranges[left.unwrap()];
                        let offset = dst - containing_range.src;
                        ranges.push(MappingRange::new(containing_range.dst + offset, src, len));
                    } else {
                        ranges.push(MappingRange::new(dst, src, len));
                    }
                    break;
                }

                match left {
                    Ok(i) => {
                        let containing_range = other.ranges[i];
                        let offset = dst - containing_range.src;

                        let new_len = containing_range.len - offset;
                        ranges.push(MappingRange::new(
                            containing_range.dst + offset,
                            src,
                            new_len,
                        ));
                        dst += new_len;
                        src += new_len;
                        len -= new_len;
                        left = if i + 1 < other.ranges.len() && other.ranges[i + 1].src == dst {
                            Ok(i + 1)
                        } else {
                            Err(i)
                        }
                    }
                    Err(i) => {
                        let next_range = other.ranges[i];
                        let new_len = next_range.src - dst;
                        ranges.push(MappingRange::new(dst, src, new_len));
                        dst += new_len;
                        src += new_len;
                        len -= new_len;
                        left = Ok(i);
                    }
                }
            }
        }

        ranges.sort();

        Mapping { ranges }
    }

    /*
       1..................................100
          10....20  30......50      80..........120
       1-9 10-19 20-29 30-49 50-79 80-99

       10..................................100
       10....20  30......50      80.............120
    */
}

impl PartialOrd for MappingRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MappingRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.src.cmp(&other.src)
    }
}

impl FromStr for MappingRange {
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

fn parse_file(filename: &str) -> (Vec<u64>, Vec<Mapping>) {
    let mut lines = crate::utils::read_lines(filename);
    let seeds = lines
        .next()
        .map(|line| parse_numbers(line.split_once(':').unwrap().1))
        .unwrap();

    lines.next();

    let mut tmp = vec![];
    let mut mappings = vec![];

    for line in lines {
        if line.is_empty() {
            tmp.sort();
            mappings.push(Mapping { ranges: tmp });
            tmp = vec![];
            continue;
        }
        if line.ends_with(':') {
            continue;
        }
        tmp.push(line.parse().unwrap());
    }

    tmp.sort();
    mappings.push(Mapping { ranges: tmp });

    (seeds, mappings)
}

pub fn lowest_location(filename: &str) -> u64 {
    let (seeds, mappings) = parse_file(filename);
    seeds
        .into_iter()
        .map(|seed| mappings.iter().fold(seed, |val, m| m.map_value(val)))
        .min()
        .unwrap()
}

pub fn lowest_location_range(filename: &str) -> u64 {
    let (seeds, mappings) = parse_file(filename);
    let mut min = u64::MAX;

    for seed_range in seeds.chunks(2) {
        let start = seed_range[0];
        let length = seed_range[1];

        let m = (start..(start + length))
            .map(|seed| mappings.iter().fold(seed, |val, m| m.map_value(val)))
            .min()
            .unwrap();
        min = min.min(m);
    }
    min
    // let (seeds, mappings) = parse_file(filename);
    // let seeds: Vec<_> = seeds
    //     .chunks(2)
    //     .map(|pair| MappingRange::new(pair[0], pair[0], pair[1]))
    //     .collect();
    // let seeds = Mapping { ranges: seeds };
    //
    // mappings
    //     .iter()
    //     .fold(seeds, |prev, next| prev.collapse_mappings(next))
    //     .ranges
    //     .iter()
    //     .map(|r| r.dst)
    //     .min()
    //     .unwrap()
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
        let (_, mappings) = parse_file(solution().part_1.example);

        let res = mappings[0].map_value(1);
        assert_eq!(res, 1);

        let res = mappings[0].map_value(79);
        assert_eq!(res, 81);

        let res = mappings[0].map_value(99);
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
