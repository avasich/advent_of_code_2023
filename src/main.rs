fn main() {
    run_day(19);
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
        13 => Box::new(d13_point_of_incidence::solution()),
        14 => Box::new(d14_parabolic_reflector_dish::solution()),
        15 => Box::new(d15_lens_library::solution()),
        16 => Box::new(d16_the_floor_will_be_lava::solution()),
        17 => Box::new(d17_clumsy_crucible::solution()),
        18 => Box::new(d18_lavaduct_lagoon::solution()),
        19 => Box::new(d19_aplenty::solution()),
        _ => unreachable!(),
    };

    solution.run_part_1();
    solution.run_part_2();
}
