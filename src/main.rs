fn main() {
    run_day(6);
}

fn run_day(day_number: usize) {
    use advent_of_code_2023::{utils::Solution, *};

    let d01 = d01_trebuchet::solution();
    let d02 = d02_cube_conundrum::solution();
    let d03 = d03_gear_ratios::solution();
    let d04 = d04_scratchcards::solution();
    let d05 = d05_fertilizer::solution();
    let d06 = d06_wait_for_it::solution();

    let days: Vec<&dyn Solution> = vec![&d01, &d02, &d03, &d04, &d05, &d06];

    let day = days[day_number - 1];
    day.run_part_1();
    day.run_part_2();
}
