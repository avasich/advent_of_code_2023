fn main() {
    run_day(12);
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
        8 => Box::new(d08_haunted_wasteland::solution()),
        9 => Box::new(d09_mirage_maintenance::solution()),
        10 => Box::new(d10_pipe_maze::solution()),
        11 => Box::new(d11_cosmic_expansion::solution()),
        12 => Box::new(d12_hot_springs::solution()),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
