mod day_01;
mod day_02;
mod day_03;
mod input;

use hard_mode::day_02 as hday_02;
use hard_mode::day_03 as hday_03;
use hard_mode::day_04 as hday_04;

const DAY_01_INPUT: &'static str = include_str!("data/day_01");
const DAY_02_INPUT: &'static str = include_str!("data/day_02");
const DAY_03_INPUT: &'static str = include_str!("data/day_03");
const DAY_04_INPUT: &'static str = include_str!("data/day_04");

fn main() {
    println!("{}", day_01::part_1(DAY_01_INPUT));
    println!("{}", day_01::part_2(DAY_01_INPUT));
    println!("{}", day_02::part_1(DAY_02_INPUT));
    println!("{}", day_02::part_2(DAY_02_INPUT));
    println!("{}", hday_02::part_1(DAY_02_INPUT));
    println!("{}", hday_02::part_2(DAY_02_INPUT));
    println!("{}", day_03::part_1(DAY_03_INPUT));
    println!("{}", day_03::part_2(DAY_03_INPUT));
    println!("{}", hday_03::part_1(DAY_03_INPUT, &mut [0u8; 100000]));
    println!("{}", hday_03::part_2(DAY_03_INPUT, &mut [0u8; 100000]));
    println!("{}", hday_04::part_1(DAY_04_INPUT,));
    println!("{}", hday_04::part_2(DAY_04_INPUT, &mut [0u8; 100000]));
}
