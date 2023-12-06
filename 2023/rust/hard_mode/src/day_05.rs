use core::ops::Range;

use crate::mem::Mem;

type Map = [(Range<usize>, isize)];

/// Turn input into a pair of a str of numbers and a 7 element array of maps
fn split_input<'a>(input: &'a str, mem: &'a Mem) -> (&'a str, [&'a Map; 7]) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds = &seeds[7..];
    let mut maps = rest.split("\n\n").map(|s| {
        let s = s.trim_start_matches(|c: char| !c.is_ascii_digit());
        let mut s_iter = s.lines();
        mem.alloc_slice(s.lines().count(), |_| {
            let mut line = s_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap());
            let dest = line.next().unwrap();
            let start = line.next().unwrap();
            let length = line.next().unwrap();
            (start..(start + length), (dest as isize) - (start as isize))
        })
        .unwrap()
    });
    (seeds, [(); 7].map(|_| &*maps.next().unwrap()))
}

fn translate_layer(value: usize, layer: &Map) -> usize {
    for (range, offset) in layer {
        if range.contains(&value) {
            return value.checked_add_signed(*offset).unwrap();
        }
    }
    value
}

fn translate(seed: usize, maps: &[&Map; 7]) -> usize {
    maps.iter()
        .fold(seed, |seed, map| translate_layer(seed, map))
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (seeds, maps) = split_input(input, &mem);
    seeds
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|seed| translate(seed, &maps))
        .min()
        .unwrap()
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (seeds, maps) = split_input(input, &mem);
    seeds
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .array_chunks::<2>()
        .flat_map(|[start, length]| (start..start + length).map(|seed| translate(seed, &maps)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::day_05::{part_1, part_2};

    const TEST_DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn split_input_works() {
        // assert_eq!(split_input(TEST_DATA), ("", [""; 7]));
    }

    #[test]
    fn part_1_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_1(TEST_DATA, &mut buffer), 35);
    }

    #[test]
    fn part_2_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_2(TEST_DATA, &mut buffer), 46);
    }

    // #[test]
    // fn test_split() {
    //     let other = 5..10usize;
    //     assert_eq!(split(1..2usize, &other), (1..2usize, None));
    //     assert_eq!(split(1..5usize, &other), (1..5usize, None));
    //     assert_eq!(split(1..6usize, &other), (1..5usize, Some(5..6usize)));
    //     assert_eq!(split(1..10usize, &other), (1..5usize, Some(5..10usize)));
    //     assert_eq!(split(1..12usize, &other), (1..5usize, Some(5..12usize)));
    //     assert_eq!(split(5..8usize, &other), (5..8usize, None));
    //     assert_eq!(split(5..12usize, &other), (5..10usize, Some(10..12usize)));
    //     assert_eq!(split(10..14usize, &other), (10..14usize, None));
    // }
}
