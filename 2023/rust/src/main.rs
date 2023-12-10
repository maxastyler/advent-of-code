use hard_mode::day_01 as hday_01;
use hard_mode::day_02 as hday_02;
use hard_mode::day_03 as hday_03;
use hard_mode::day_04 as hday_04;
use hard_mode::day_05 as hday_05;
use hard_mode::day_06 as hday_06;
use hard_mode::day_07 as hday_07;
use hard_mode::day_08 as hday_08;
use hard_mode::day_10 as hday_10;

mod day_01;
mod day_02;
mod day_03;
mod day_05;
mod day_06;
mod day_09;
mod input;

const DAY_01_INPUT: &'static str = include_str!("data/day_01");
const DAY_02_INPUT: &'static str = include_str!("data/day_02");
const DAY_03_INPUT: &'static str = include_str!("data/day_03");
const DAY_04_INPUT: &'static str = include_str!("data/day_04");
const DAY_05_INPUT: &'static str = include_str!("data/day_05");
const DAY_06_INPUT: &'static str = include_str!("data/day_06");
const DAY_07_INPUT: &'static str = include_str!("data/day_07");
const DAY_08_INPUT: &'static str = include_str!("data/day_08");
const DAY_09_INPUT: &'static str = include_str!("data/day_09");
const DAY_10_INPUT: &'static str = include_str!("data/day_10");

fn main() {
    let mut buffer = [0u8; 1_000_000];
    println!("Day 1 part 1: {}", hday_01::part_1(DAY_01_INPUT));
    println!("Day 1 part 2: {}", hday_01::part_2(DAY_01_INPUT));
    println!("Day 2 part 1: {}", hday_02::part_1(DAY_02_INPUT));
    println!("Day 2 part 2: {}", hday_02::part_2(DAY_02_INPUT));
    println!(
        "Day 3 part 1: {}",
        hday_03::part_1(DAY_03_INPUT, &mut buffer)
    );
    println!(
        "Day 3 part 2: {}",
        hday_03::part_2(DAY_03_INPUT, &mut buffer)
    );
    println!("Day 4 part 1: {}", hday_04::part_1(DAY_04_INPUT));
    println!(
        "Day 4 part 2: {}",
        hday_04::part_2(DAY_04_INPUT, &mut buffer)
    );
    println!(
        "Day 5 part 1: {}",
        hday_05::part_1(DAY_05_INPUT, &mut buffer)
    );
    // commented out because it's slooooow
    // println!(
    //     "Day 5 part 2: {}",
    //     hday_05::part_2(DAY_05_INPUT, &mut buffer)
    // );
    println!("Day 5 part 2: {}", day_05::part_2(DAY_05_INPUT));
    println!("Day 6 part 1: {}", hday_06::part_1(DAY_06_INPUT));
    println!("Day 6 part 2: {}", hday_06::part_2(DAY_06_INPUT));
    println!(
        "Day 7 part 1: {}",
        hday_07::part_1(DAY_07_INPUT, &mut buffer)
    );
    println!(
        "Day 7 part 2: {}",
        hday_07::part_2(DAY_07_INPUT, &mut buffer)
    );
    println!(
        "Day 8 part 1: {}",
        hday_08::part_1(DAY_08_INPUT, &mut buffer)
    );
    println!(
        "Day 8 part 2: {}",
        hday_08::part_2(DAY_08_INPUT, &mut buffer)
    );
    println!("Day 9 part 1: {}", day_09::part_1(DAY_09_INPUT));
    println!("Day 9 part 2: {}", day_09::part_2(DAY_09_INPUT));
    println!("Day 10 part 1: {}", hday_10::part_1(DAY_10_INPUT));
    println!("Day 10 part 2: {}", hday_10::part_2(DAY_10_INPUT, &mut buffer));    
}
