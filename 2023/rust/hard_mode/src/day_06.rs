fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    let (times, distances) = input.trim().split_once("\n").unwrap();
    let times_iterator = times
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .split_whitespace();
    let distances_iterator = distances
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .split_whitespace();
    times_iterator
        .zip(distances_iterator)
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
}

fn find_limits(time: usize, distance: usize) -> Option<usize> {
    let time = time as i64;
    let distance = distance as i64;
    let first_t = (0..).find_map(|t| {
        if ((time - t) * t - distance) > 0 {
            Some(t)
        } else {
            None
        }
    })?;
    let last_t = (first_t..).find_map(|t| {
        if ((time - t) * t - distance) <= 0 {
            Some(t)
        } else {
            None
        }
    })?;
    Some((last_t - first_t) as usize)
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    parse_input(input)
        .map(|(time, distance)| find_limits(time, distance).unwrap())
        .product()
}

fn convert_line_to_num(line: &str) -> usize {
    line.trim_start_matches(|c: char| !c.is_ascii_digit())
        .split_whitespace()
        .flat_map(|x| x.chars().map(|c: char| c.to_digit(10).unwrap() as usize))
        .fold(0, |a, v| a * 10 + v)
}

pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    let (times, distances) = input.trim().split_once("\n").unwrap();
    find_limits(convert_line_to_num(times), convert_line_to_num(distances)).unwrap()
}

#[cfg(test)]
mod test {
    use crate::day_06::{part_1, part_2};

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut []), 288);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 71503);
    }
}
