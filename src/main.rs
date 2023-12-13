use advent_of_code_2023::utils::Solution;

fn main() {
    run_day(5);
}

fn run_day(day_number: usize) {
    let d01 = advent_of_code_2023::d01_trebuchet::solution();
    let d02 = advent_of_code_2023::d02_cube_conundrum::solution();
    let d03 = advent_of_code_2023::d03_gear_ratios::solution();
    let d04 = advent_of_code_2023::d04_scratchcards::solution();
    let d05 = advent_of_code_2023::d05_fertilizer::solution();

    let days: Vec<&dyn Solution> = vec![&d01, &d02, &d03, &d04, &d05];

    let day = days[day_number - 1];
    day.run_part_1();
    day.run_part_2();
}
