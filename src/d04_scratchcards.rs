use crate::utils::{Day, Task};

fn parse_card(line: &str) -> (usize, Vec<u32>, Vec<u32>) {
    let (id, all_numbers) = line.split_once(':').unwrap();
    let id = id
        .split_whitespace()
        .nth(1)
        .and_then(|val| val.parse().ok())
        .unwrap();

    let mut numbers = all_numbers
        .split(" | ")
        .map(|numbers| numbers.split_whitespace().flat_map(str::parse))
        .map(Iterator::collect);

    let winning = numbers.next().unwrap();
    let actual = numbers.next().unwrap();

    (id, winning, actual)
}

fn points(mut winning: Vec<u32>, mut actual: Vec<u32>) -> u32 {
    winning.sort();
    actual.sort();

    let mut cursor = 0;
    let mut total = 0;

    for number in winning {
        if cursor >= actual.len() {
            break;
        }

        if let Ok(found) = actual[cursor..].binary_search(&number) {
            cursor = found + 1;
            total += 1;
        }
    }

    total
}

pub fn total_points(filename: &str) -> u64 {
    crate::utils::read_lines(filename)
        .map(|line| parse_card(&line))
        .map(|(_, winning, actual)| points(winning, actual))
        .filter(|&total| total > 0)
        .map(|total| 2u64.pow(total - 1))
        .sum()
}

pub fn total_scratchcards(filename: &str) -> usize {
    let mut counts: Vec<_> = crate::utils::read_lines(filename)
        .map(|line| parse_card(&line))
        .map(|(_, winning, actual)| (1, points(winning, actual)))
        .collect();

    for i in 0..counts.len() {
        let (count, points) = counts[i];
        counts
            .iter_mut()
            .skip(i + 1)
            .take(points as usize)
            .for_each(|c| c.0 += count);
    }

    counts.into_iter().map(|(count, _)| count).sum()
}

pub fn solution() -> Day<u64, usize> {
    Day {
        part_1: Task {
            example: "./inputs/day_04/example_01.txt",
            task: "./inputs/day_04/task.txt",
            run: total_points,
        },
        part_2: Task {
            example: "./inputs/day_04/example_01.txt",
            task: "./inputs/day_04/task.txt",
            run: total_scratchcards,
        },
    }
}

#[cfg(test)]
mod d04_tests {
    use super::*;

    #[test]
    fn parse_card_test() {
        let filename = solution().part_1.example;

        let lines: Vec<_> = crate::utils::read_lines(filename).collect();

        let card1 = parse_card(&lines[0]);
        assert_eq!(card1.0, 1);
        assert_eq!(card1.1, vec![41, 48, 83, 86, 17]);
        assert_eq!(card1.2, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        let card3 = parse_card(&lines[2]);
        assert_eq!(card3.0, 3);
        assert_eq!(card3.1, vec![1, 21, 53, 59, 44]);
        assert_eq!(card3.2, vec![69, 82, 63, 72, 16, 21, 14, 1]);
    }

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example();
        assert_eq!(res, 13);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example();
        assert_eq!(res, 30);
    }
}
