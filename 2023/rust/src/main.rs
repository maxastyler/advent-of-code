use hard_mode::day_02 as hday_02;
use hard_mode::day_03 as hday_03;
use hard_mode::day_04 as hday_04;
use hard_mode::day_05 as hday_05;

mod day_01;
mod day_02;
mod day_03;
mod day_05;
mod input;

const DAY_01_INPUT: &'static str = include_str!("data/day_01");
const DAY_02_INPUT: &'static str = include_str!("data/day_02");
const DAY_03_INPUT: &'static str = include_str!("data/day_03");
const DAY_04_INPUT: &'static str = include_str!("data/day_04");
const DAY_05_INPUT: &'static str = include_str!("data/day_05");

fn main() {
    println!("Day 1 part 1: {}", day_01::part_1(DAY_01_INPUT));
    println!("Day 1 part 2: {}", day_01::part_2(DAY_01_INPUT));
    println!("Day 2 part 1: {}", hday_02::part_1(DAY_02_INPUT));
    println!("Day 2 part 2: {}", hday_02::part_2(DAY_02_INPUT));
    println!(
        "Day 3 part 1: {}",
        hday_03::part_1(DAY_03_INPUT, &mut [0u8; 100000])
    );
    println!(
        "Day 3 part 2: {}",
        hday_03::part_2(DAY_03_INPUT, &mut [0u8; 100000])
    );
    println!("Day 4 part 1: {}", hday_04::part_1(DAY_04_INPUT));
    println!(
        "Day 4 part 2: {}",
        hday_04::part_2(DAY_04_INPUT, &mut [0u8; 100000])
    );
    println!("Day 5 part 1: {}", day_05::part_1(DAY_05_INPUT));
    println!("Day 5 part 2: {}", day_05::part_2(DAY_05_INPUT));
}
