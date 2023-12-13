use crate::utils::{Day, Solution, Task};

pub fn p1_calibration_total(filename: &str) -> u32 {
    crate::utils::read_lines(filename)
        .map(|line| {
            let left = line
                .chars()
                .find(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let right = line
                .chars()
                .rfind(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .unwrap();

            10 * left + right
        })
        .sum()
}

pub fn p2_calibration_total(filename: &str) -> u32 {
    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    crate::utils::read_lines(filename)
        .map(|line| {
            let (mut left_index, mut left) = line
                .char_indices()
                .find(|(_, c)| c.is_numeric())
                .map_or((line.len(), 0), |(i, c)| (i, c.to_digit(10).unwrap()));

            let (mut right_index, mut right) = line
                .char_indices()
                .rfind(|(_, c)| c.is_numeric())
                .map_or((0, 0), |(i, c)| (i + 1, c.to_digit(10).unwrap()));

            for (n, &digit_word) in digit_words.iter().enumerate() {
                if let Some(new_left_index) = line
                    .get(0..left_index)
                    .and_then(|line| line.find(digit_word))
                {
                    left_index = new_left_index + digit_word.len();
                    left = 1 + n as u32;
                }

                if let Some(new_right_index) = line
                    .get(right_index..)
                    .and_then(|line| line.rfind(digit_word))
                {
                    right_index += new_right_index;
                    right = 1 + n as u32;
                }
            }

            10 * left + right
        })
        .sum()
}

pub fn solution() -> Day<u32, u32> {
    Day {
        part_1: Task {
            example: "./inputs/day_01/example_01.txt",
            task: "./inputs/day_01/task.txt",
            run: p1_calibration_total,
        },
        part_2: Task {
            example: "./inputs/day_01/example_01.txt",
            task: "./inputs/day_01/task.txt",
            run: p2_calibration_total,
        },
    }
}

#[cfg(test)]
mod d01_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example();
        assert_eq!(res, 142);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example();
        assert_eq!(res, 281);
    }
}
