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

#[cfg(test)]
mod d02_tests {
    use super::*;

    static EXAMPLE_01: &str = "./inputs/day_02/example_01.txt";
    static TASK: &str = "./inputs/day_02/task.txt";

    #[test]
    fn p1_example_test() {
        let res = possible_games_sum(EXAMPLE_01, (12, 13, 14));
        assert_eq!(res, 8);
    }

    #[test]
    fn p1_task_test() {
        let res = possible_games_sum(TASK, (12, 13, 14));
        println!("{res}");
    }

    #[test]
    fn p2_example_test() {
        let res = power_of_sets(EXAMPLE_01);
        assert_eq!(res, 2286);
    }

    #[test]
    fn p2_task_test() {
        let res = power_of_sets(TASK);
        println!("{res}");
    }
}
