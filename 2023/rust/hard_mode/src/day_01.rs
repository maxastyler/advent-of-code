fn get_digit(line: &str) -> Option<i32> {
    let c = line.chars().next()?;
    if c.is_ascii_digit() {
        Some(c as i32 - '0' as i32)
    } else {
        None
    }
}

fn get_digit_or_str(line: &str) -> Option<i32> {
    get_digit(line).or_else(|| {
        for (s, i) in [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ] {
            if line.starts_with(s) {
                return Some(i);
            }
        }
        None
    })
}

fn first_and_last_digit(line: &str, pred: fn(&str) -> Option<i32>) -> i32 {
    let n_chars = line.chars().count();
    let first = (0..n_chars).find_map(|i| pred(&line[i..])).unwrap();
    let last = (0..n_chars).rev().find_map(|i| pred(&line[i..])).unwrap();
    first * 10 + last
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| first_and_last_digit(l, get_digit) as usize)
        .sum()
}

pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| first_and_last_digit(l, get_digit_or_str) as usize)
        .sum()
}
