use crate::vec2::Vec2;

pub fn calculate_area<'a>(points: impl Iterator<Item = (&'a str, i64)>) -> usize {
    let (area, perimeter, _) =
        points.fold((0, 0, Vec2(0, 0)), |(area, perimeter, i1), (dir, dist)| {
            let i2 = i1.add(dir, dist).unwrap();
            (
                area + (i1.0 * i2.1 - i2.0 * i1.1),
                (i2.0 - i1.0).abs() + (i2.1 - i1.1).abs() + perimeter,
                i2,
            )
        });

    ((area.abs() / 2) + perimeter / 2 + 1) as usize
}

fn calculate_area_from_input<F>(input: &str, line_parser: F) -> usize
where
    F: Fn(&str) -> (&str, i64) + Copy,
{
    calculate_area(
        input
            .lines()
            .cycle()
            .map(line_parser)
            .take(input.lines().count()),
    )
}

fn p1_parser(line: &str) -> (&str, i64) {
    (
        &line[0..1],
        line[2..].split_once(" ").unwrap().0.parse().unwrap(),
    )
}

fn p2_parser(line: &str) -> (&str, i64) {
    let num = line.split_once("(#").unwrap().1;
    (&num[5..6], i64::from_str_radix(&num[..5], 16).unwrap())
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    calculate_area_from_input(input, p1_parser)
}

pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    calculate_area_from_input(input, p2_parser)
}
#[cfg(test)]
mod test {
    use super::{part_1, part_2};

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut []), 62);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 952408144115);
    }
}
