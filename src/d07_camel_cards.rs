use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: u8,
    cards: [u8; 5],
}

impl std::str::FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [0; 5];
        let mut counts = HashMap::new();

        s.chars().enumerate().for_each(|(i, c)| {
            let val = match c {
                c if c.is_numeric() => c.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            } as u8;
            cards[i] = val;
            *counts.entry(val).or_insert(0) += 1;
        });

        let counts = counts.values().copied().sorted().collect_vec();
        let hand_type = match counts.as_slice() {
            [1, 1, 1, 1, 1] => 1,
            [1, 1, 1, 2] => 2,
            [1, 2, 2] => 3,
            [1, 1, 3] => 4,
            [2, 3] => 5,
            [1, 4] => 6,
            [5] => 7,
            _ => unreachable!(),
        };

        Ok(Hand { cards, hand_type })
    }
}

fn total_winnings(filename: &str) -> u64 {
    crate::utils::read_lines(filename)
        .flat_map(|line| {
            line.split_once(' ')
                .map(|(cards, bid)| (cards.parse::<Hand>().unwrap(), bid.parse::<u64>().unwrap()))
        })
        .sorted()
        .enumerate()
        .map(|(i, (_, bid))| ((i + 1) as u64) * bid)
        .sum()
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            example: "inputs/day_07/example_01.txt",
            task: "inputs/day_07/task.txt",
            run: total_winnings,
        },
        part_2: Task {
            example: "inputs/day_07/example_01.txt",
            task: "inputs/day_07/task.txt",
            run: total_winnings,
        },
    }
}

#[cfg(test)]
mod d07_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example();
        assert_eq!(6440, res);
    }
}
