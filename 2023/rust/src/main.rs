mod day_01;
mod input;
mod day_02;

const DAY_01_INPUT: &'static str = include_str!("data/day_01");
const DAY_02_INPUT: &'static str = include_str!("data/day_02");

fn main() {
    println!("{}", day_02::part_1(DAY_02_INPUT));
    println!("{}", day_02::part_2(DAY_02_INPUT));    
    // println!("{}", day_01::part_2(DAY_01_INPUT));
}
