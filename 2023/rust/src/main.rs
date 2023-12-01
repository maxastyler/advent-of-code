mod day_01;
mod input;

const DAY_01_INPUT: &'static str = include_str!("data/day_01");

fn main() {
    println!("{}", day_01::part_1(DAY_01_INPUT));
    println!("{}", day_01::part_2(DAY_01_INPUT));
}
