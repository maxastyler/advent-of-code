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

pub fn num_valid_times(time_distance: (usize, usize)) -> usize {
    let time = time_distance.0 as f64;
    let distance = time_distance.1 as f64;
    let diff = (time.powi(2) - distance * 4.0).sqrt();
    let offset = if diff == diff.round() { 1.0 } else { 0.0 };
    (((((time + diff) / 2.0).ceil() - ((time - diff) / 2.0)).floor()) - offset).round() as usize
}

pub fn part_1(input: &str) -> usize {
    parse_input(input).map(num_valid_times).product()
}

fn convert_line_to_num(line: &str) -> usize {
    line.trim_start_matches(|c: char| !c.is_ascii_digit())
        .split_whitespace()
        .flat_map(|x| x.chars().map(|c: char| c.to_digit(10).unwrap() as usize))
        .fold(0, |a, v| a * 10 + v)
}

pub fn part_2(input: &str) -> usize {
    let (times, distances) = input.trim().split_once("\n").unwrap();
    num_valid_times((convert_line_to_num(times), convert_line_to_num(distances)))
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};
    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), 288);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), 71503);
    }
}
