use core::ops::{Range, Sub};
use core::slice::SliceIndex;

use crate::mem::Mem;

type Map = [(Range<usize>, isize)];

/// Turn input into a pair of a str of numbers and a 7 element array of maps
fn split_input<'a>(
    input: &'a str,
    mem: &'a mut Mem,
) -> (&'a str, [&'a Map; 7]) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds = &seeds[7..];
    let mut maps = rest.split("\n\n").map(|s| {
        let s = s.trim_start_matches(|c: char| !c.is_ascii_digit());
        let num_elem = s.lines().count();
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

fn translate_layer(seed: usize, map: &Map) -> usize {
    map.iter().find_map(|(source, offset)| if source.contains(&seed) {
        seed.checked_add_signed(*offset)
    } else { None }).unwrap_or(seed)
}

fn translate(seed: usize, maps: &[&Map]) -> usize {
    let mut current_seed = seed;
    for map in maps {
        current_seed = translate_layer(current_seed, map);
    }
    current_seed
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mut mem = Mem::new(buffer);
    let (seeds, maps) = split_input(input, &mut mem);
    seeds
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|seed| translate(seed, &maps))
        .min()
        .unwrap()
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mut mem = Mem::new(buffer);
    let (seeds, maps) = split_input(input, &mut mem);
    let mut seeds_iter = seeds
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    while let Ok([start, length]) = seeds_iter.next_chunk::<2>() {}
    3
}


#[cfg(test)]
mod test {
    use crate::day_05::part_1;

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
    fn part_1_works()
    {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_1(TEST_DATA, &mut buffer), 35);
    }
}
