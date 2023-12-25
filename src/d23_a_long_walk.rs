use crate::utils::{Day, Task};

fn part_1(filename: &str) -> u64 {
    todo!()
}

fn part_2(filename: &str) -> u64 {
    todo!()
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_23/example_01.txt"],
            task: "./inputs/day_23/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_23/example_01.txt"],
            task: "./inputs/day_23/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d20_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(94, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();
        let res = solution.part_2.run_example(0);
        assert_eq!(0, res);
    }
}
