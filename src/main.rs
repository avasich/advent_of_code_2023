fn main() {
    run_day(7);
}

fn run_day(day_number: usize) {
    use advent_of_code_2023::{utils::Solution, *};

    let solution: Box<dyn Solution> = match day_number {
        1 => Box::new(d01_trebuchet::solution()),
        2 => Box::new(d02_cube_conundrum::solution()),
        3 => Box::new(d03_gear_ratios::solution()),
        4 => Box::new(d04_scratchcards::solution()),
        5 => Box::new(d05_fertilizer::solution()),
        6 => Box::new(d06_wait_for_it::solution()),
        7 => Box::new(d07_camel_cards::solution()),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
