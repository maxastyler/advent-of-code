use std::time::Instant;

use hard_mode::day_01 as hday_01;
use hard_mode::day_02 as hday_02;
use hard_mode::day_03 as hday_03;
use hard_mode::day_04 as hday_04;
use hard_mode::day_05 as hday_05;
use hard_mode::day_06 as hday_06;
use hard_mode::day_07 as hday_07;
use hard_mode::day_08 as hday_08;
use hard_mode::day_10 as hday_10;
use hard_mode::day_11 as hday_11;
use hard_mode::day_12 as hday_12;

mod day_01;
mod day_02;
mod day_03;
mod day_05;
mod day_06;
mod day_09;
// mod day_12;
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
const DAY_11_INPUT: &'static str = include_str!("data/day_11");
const DAY_12_INPUT: &'static str = include_str!("data/day_12");

fn run_day<F>(fun: F, input: &str, buffer: &mut [u8], day: &str, part: &str)
where
    F: Fn(&str, &mut [u8]) -> usize,
{
    let start_time = Instant::now();
    let result = fun(input, buffer);
    let total_time = start_time.elapsed();
    println!("Day {} part {}: {} ({:?})", day, part, result, total_time);
}

fn main() {
    let mut buffer = [0u8; 1_000_000];
    run_day(hday_01::part_1, DAY_01_INPUT, &mut buffer, "1", "1");
    run_day(hday_01::part_2, DAY_01_INPUT, &mut buffer, "1", "2");
    run_day(hday_02::part_1, DAY_02_INPUT, &mut buffer, "2", "1");
    run_day(hday_02::part_2, DAY_02_INPUT, &mut buffer, "2", "2");
    run_day(hday_03::part_1, DAY_03_INPUT, &mut buffer, "3", "1");
    run_day(hday_03::part_2, DAY_03_INPUT, &mut buffer, "3", "2");
    run_day(hday_04::part_1, DAY_04_INPUT, &mut buffer, "4", "1");
    run_day(hday_04::part_2, DAY_04_INPUT, &mut buffer, "4", "2");
    run_day(hday_05::part_1, DAY_05_INPUT, &mut buffer, "5", "1");
    // commented out because it's slooooow
    // run_day(hday_05::part_2, DAY_05_INPUT, &mut buffer, "5", "2");
    run_day(day_05::part_2, DAY_05_INPUT, &mut buffer, "5", "2");
    run_day(hday_06::part_1, DAY_06_INPUT, &mut buffer, "6", "1");
    run_day(hday_06::part_2, DAY_06_INPUT, &mut buffer, "6", "2");
    run_day(hday_07::part_1, DAY_07_INPUT, &mut buffer, "7", "1");
    run_day(hday_07::part_2, DAY_07_INPUT, &mut buffer, "7", "2");
    run_day(hday_08::part_1, DAY_08_INPUT, &mut buffer, "8", "1");
    run_day(hday_08::part_2, DAY_08_INPUT, &mut buffer, "8", "2");
    run_day(day_09::part_1, DAY_09_INPUT, &mut buffer, "9", "1");
    run_day(day_09::part_2, DAY_09_INPUT, &mut buffer, "9", "2");

    run_day(hday_10::part_1, DAY_10_INPUT, &mut buffer, "10", "1");
    run_day(hday_10::part_2, DAY_10_INPUT, &mut buffer, "10", "2");

    run_day(hday_11::part_1, DAY_11_INPUT, &mut buffer, "11", "1");
    run_day(hday_11::part_2, DAY_11_INPUT, &mut buffer, "11", "2");
    run_day(hday_12::part_1, DAY_12_INPUT, &mut buffer, "12", "1");
    run_day(hday_12::part_2, DAY_12_INPUT, &mut buffer, "12", "2");
}
