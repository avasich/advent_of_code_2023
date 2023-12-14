use crate::utils::{Day, Task};

fn parse_numbers(s: String) -> Vec<i64> {
    s.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(str::parse)
        .collect()
}

fn ways_to_win(time: i64, dist: i64) -> i64 {
    match time * time - 4 * dist {
        0 if time % 2 == 0 => 1,
        0 if time % 2 != 0 => 0,
        d if d < 0 => 0,
        d => {
            let d_sqrt = (d as f64).sqrt();

            let t1 = (time as f64 - d_sqrt) / 2.0;
            let t1 = t1.ceil() as i64;
            let t2 = (time as f64 + d_sqrt) / 2.0;
            let t2 = t2.floor() as i64;

            let is_exact = t1 * t1 - time * t1 + dist == 0;
            if is_exact {
                t2 - t1 - 1
            } else {
                t2 - t1 + 1
            }
        }
    }
}

fn parse_number(line: String) -> i64 {
    let s: String = line
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    s.parse().unwrap()
}

fn product_of_ways_to_win(filename: &str) -> i64 {
    let mut lines = crate::utils::read_lines(filename);
    let times = parse_numbers(lines.next().unwrap());
    let dists = parse_numbers(lines.next().unwrap());
    times
        .into_iter()
        .zip(dists)
        .map(|(time, dist)| ways_to_win(time, dist))
        .product()
}

fn ways_to_win_task(filename: &str) -> i64 {
    let mut numbers = crate::utils::read_lines(filename).map(parse_number);
    ways_to_win(numbers.next().unwrap(), numbers.next().unwrap())
}

pub fn solution() -> Day<i64, i64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_06/example_01.txt"],
            task: "./inputs/day_06/task.txt",
            run: product_of_ways_to_win,
        },
        part_2: Task {
            examples: vec!["./inputs/day_06/example_01.txt"],
            task: "./inputs/day_06/task.txt",
            run: ways_to_win_task,
        },
    }
}

#[cfg(test)]
mod d06_tests {
    use super::*;

    #[test]
    fn number_of_integer_inequality_solutions() {
        assert_eq!(4, ways_to_win(7, 9));
        assert_eq!(8, ways_to_win(15, 40));
        assert_eq!(9, ways_to_win(30, 200));
    }

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example(0);
        assert_eq!(res, 288);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example(0);
        assert_eq!(res, 71503);
    }
}
