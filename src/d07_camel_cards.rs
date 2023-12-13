use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{Day, Task};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    weight: u8,
    cards: [u8; 5],
}

impl Hand {
    fn new(s: &str, with_j: bool) -> Self {
        let mut cards = [0; 5];
        let mut counts = HashMap::new();
        let mut j_count = 0;
        let j_weight = if with_j { 1 } else { 11 };

        s.chars().enumerate().for_each(|(i, c)| {
            let val = match c {
                c if c.is_numeric() => c.to_digit(10).unwrap(),
                'T' => 10,
                'J' => j_weight,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            } as u8;
            cards[i] = val;

            if c == 'J' && with_j {
                j_count += 1;
            } else {
                *counts.entry(val).or_insert(0) += 1;
            }
        });

        let mut counts = counts.values().copied().sorted().collect_vec();

        if with_j {
            if let Some(v) = counts.last_mut() {
                *v += j_count;
            } else {
                counts.push(j_count);
            }
        }

        let weight = match counts.as_slice() {
            [1, 1, 1, 1, 1] => 1,
            [1, 1, 1, 2] => 2,
            [1, 2, 2] => 3,
            [1, 1, 3] => 4,
            [2, 3] => 5,
            [1, 4] => 6,
            [5] => 7,
            _ => unreachable!("impossible hand type {counts:?}"),
        };

        if with_j && j_count > 0 && weight == 3 {
            println!("wtf! {s}");
        }

        Self { cards, weight }
    }
}

fn total_winnings(filename: &str, with_j: bool) -> u64 {
    crate::utils::read_lines(filename)
        .flat_map(|line| {
            line.split_once(' ')
                .map(|(cards, bid)| (Hand::new(cards, with_j), bid.parse::<u64>().unwrap()))
        })
        .sorted()
        .enumerate()
        .map(|(i, (_, bid))| ((i + 1) as u64) * bid)
        .sum()
}

pub fn solution() -> Day<u64, u64> {
    fn part_1(filename: &str) -> u64 {
        total_winnings(filename, false)
    }

    fn part_2(filename: &str) -> u64 {
        total_winnings(filename, true)
    }

    Day {
        part_1: Task {
            example: "inputs/day_07/example_01.txt",
            task: "inputs/day_07/task.txt",
            run: part_1,
        },
        part_2: Task {
            example: "inputs/day_07/example_01.txt",
            task: "inputs/day_07/task.txt",
            run: part_2,
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

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example();
        assert_eq!(5905, res);
    }

    fn weight(cards: &str) -> u8 {
        Hand::new(cards, true).weight
    }

    #[test]
    fn hand_weight_j() {
        // 11111 - 1
        // 1112  - 2
        // 122   - 3
        // 113   - 4
        // 23    - 5
        // 14    - 6
        // 5     - 7
        assert_eq!(1, weight("75T8A"));
        assert_eq!(1, weight("25Q4T"));

        assert_eq!(2, weight("38849"));
        assert_eq!(2, weight("36T35"));

        assert_eq!(3, weight("992KK"));
        assert_eq!(3, weight("27275"));

        assert_eq!(4, weight("42T22"));
        assert_eq!(4, weight("AA6A2"));

        assert_eq!(5, weight("25552"));
        assert_eq!(5, weight("77557"));

        assert_eq!(6, weight("99992"));
        assert_eq!(6, weight("QQQ3Q"));

        assert_eq!(7, weight("33333"));
        assert_eq!(7, weight("KKKKK"));

        assert_eq!(2, weight("2345J"));
        assert_eq!(2, weight("4J329"));

        assert_eq!(4, weight("J6QT6"));
        assert_eq!(4, weight("6J5J8"));

        assert_eq!(5, weight("66AAJ"));
        assert_eq!(5, weight("T77JT"));

        assert_eq!(6, weight("T55J5"));
        assert_eq!(6, weight("KTJJT"));

        assert_eq!(7, weight("555J5"));
        assert_eq!(7, weight("JJ5JJ"));
        assert_eq!(7, weight("JJJJJ"));
    }

    #[test]
    fn more_hand_weight_j() {
        // 11111 - 1
        // 1112  - 2
        // 122   - 3
        // 113   - 4
        // 23    - 5
        // 14    - 6
        // 5     - 7

        assert_eq!(6, weight("2J722"));
        assert_eq!(2, weight("92QAJ"));
        assert_eq!(2, weight("TQ67J"));
        assert_eq!(4, weight("7Q3J3"));
        assert_eq!(5, weight("533J5"));
        assert_eq!(6, weight("J88JT"));
        assert_eq!(7, weight("JJQQQ"));
        assert_eq!(5, weight("4422J"));
        assert_eq!(4, weight("K3J39"));
        assert_eq!(4, weight("J88A4"));
        assert_eq!(2, weight("J647K"));
        assert_eq!(2, weight("J8Q47"));
        assert_eq!(6, weight("7J778"));
        assert_eq!(6, weight("393J3"));
        assert_eq!(4, weight("9J4AJ"));
        assert_eq!(2, weight("T28J3"));
        assert_eq!(4, weight("2J56J"));
        assert_eq!(6, weight("7J737"));
        assert_eq!(4, weight("JK73K"));
        assert_eq!(4, weight("AJJ97"));
        assert_eq!(6, weight("777TJ"));
        assert_eq!(2, weight("J93QA"));
        assert_eq!(6, weight("AJA8A"));
        assert_eq!(2, weight("K358J"));
        assert_eq!(6, weight("8J488"));
        assert_eq!(2, weight("K7J4A"));
        assert_eq!(2, weight("873JT"));
        assert_eq!(2, weight("6374J"));
        assert_eq!(7, weight("JTJTT"));
        assert_eq!(2, weight("2J978"));
    }
}
