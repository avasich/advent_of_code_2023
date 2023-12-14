use crate::utils::{Day, Task};

fn parse_game_line(line: &str) -> (usize, usize, usize, usize) {
    let (game_id, attempts) = line.split_once(':').unwrap();
    let game_id = game_id.split_once(' ').unwrap().1.parse().unwrap();

    let (r, g, b) = attempts
        .split(';')
        .flat_map(|attempt| attempt.split(','))
        .flat_map(|res| res.trim().split_once(' '))
        .map(|(n, color)| (n.parse::<usize>().unwrap(), color))
        .fold((0, 0, 0), |(r, g, b), (n, color)| match color {
            "red" => (r.max(n), g, b),
            "green" => (r, g.max(n), b),
            "blue" => (r, g, b.max(n)),
            _ => unreachable!(),
        });

    (game_id, r, g, b)
}

pub fn possible_games_sum(filename: &str, contents: (usize, usize, usize)) -> usize {
    crate::utils::read_lines(filename)
        .map(|line| parse_game_line(&line))
        .filter(|&(_, r, g, b)| r <= contents.0 && g <= contents.1 && b <= contents.2)
        .map(|res| res.0)
        .sum()
}

pub fn power_of_sets(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .map(|line| parse_game_line(&line))
        .map(|(_, r, g, b)| r * g * b)
        .sum()
}

pub fn solution() -> Day<usize, usize> {
    fn part_1(filename: &str) -> usize {
        possible_games_sum(filename, (12, 13, 14))
    }

    Day {
        part_1: Task {
            examples: vec!["./inputs/day_02/example_01.txt"],
            task: "./inputs/day_02/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_02/example_01.txt"],
            task: "./inputs/day_02/task.txt",
            run: power_of_sets,
        },
    }
}

#[cfg(test)]
mod d02_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let res = solution().part_1.run_example(0);
        assert_eq!(res, 8);
    }

    #[test]
    fn p2_example_test() {
        let res = solution().part_2.run_example(0);
        assert_eq!(res, 2286);
    }
}
