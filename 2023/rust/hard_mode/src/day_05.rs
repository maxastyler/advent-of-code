use core::ops::{Range, Sub};
use core::slice::SliceIndex;

use crate::mem::Mem;

type Map = [(Range<usize>, isize)];

/// Split one range by another
/// returns (first half of cut range, second half of cut range, if the first range overlaps)
fn split(range: Range<usize>, other: &Range<usize>) -> (Range<usize>, Option<Range<usize>>, bool) {
    if range.start < other.start {
        // we start before the other range
        if range.end <= other.start {
            (range, None, false)
        } else {
            (
                range.start..other.start,
                Some(other.start..range.end),
                false,
            )
        }
    } else if range.start < other.end {
        // we start in the other range
        if range.end <= other.end {
            (range.start..range.end, None, true)
        } else {
            (range.start..other.end, Some(other.end..range.end), true)
        }
    } else {
        // we start after the other range
        (range, None, false)
    }
}

fn offset(range: Range<usize>, offset: isize) -> Range<usize> {
    range.start.checked_add_signed(offset).unwrap()..range.end.checked_add_signed(offset).unwrap()
}

struct Stack<'mem, T> {
    buffer: &'mem mut [T],
    top: usize,
}

impl<'mem, T> Stack<'mem, T>
where
    T: Default,
{
    fn new(buffer: &'mem mut [T]) -> Self {
        Self { buffer, top: 0 }
    }

    fn push(&mut self, item: T) {
        self.buffer[self.top] = item;
        self.top += 1;
    }

    fn pop(&mut self, mut replacement: T) -> Option<T> {
        use core::mem::swap;
        if self.top == 0 {
            None
        } else {
            swap(self.buffer.get_mut(self.top)?, &mut replacement);
            Some(replacement)
        }
    }
}

/// Turn input into a pair of a str of numbers and a 7 element array of maps
fn split_input<'a>(input: &'a str, mem: &'a Mem) -> (&'a str, [&'a Map; 7]) {
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
    map.iter()
        .find_map(|(source, offset)| {
            if source.contains(&seed) {
                seed.checked_add_signed(*offset)
            } else {
                None
            }
        })
        .unwrap_or(seed)
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

fn mod_range(
    map_index: usize,
    range: Range<usize>,
    stack: &mut Stack<(usize, Range<usize>)>,
    maps: &[&Map],
) -> Range<usize> {
    let current_range = range;
    maps.iter()
        .enumerate()
        .skip(map_index)
        .for_each(|(i, &map)| {
            map.iter().filter_map(|(range, offset)| {
                let (new_range, rest, overlap) = split(0..3, range);
                Some(3)
            });
        });
    0..3
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (seeds, maps) = split_input(input, &mem);
    let mut seeds_iter = seeds
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let mut stack = Stack::new(mem.alloc_slice(200, |_| (0usize, (0usize..0))).unwrap());
    let mut lowest: Option<usize> = None;
    while let Ok([start, length]) = seeds_iter.next_chunk::<2>() {
        stack.push((0, start..start + length));
        while let Some((map_index, range)) = stack.pop((0, (0..0))) {
            // let test_lowest = mod_range(map_index, range, &mut stack).start
            lowest = lowest.map(|n| core::cmp::min(n, 3)).or(Some(3))
        }
    }
    3
}

#[cfg(test)]
mod test {
    use crate::day_05::{part_1, split};

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

    // #[test]
    // fn part_2_works()
    // {
    //     let mut buffer = [0u8; 1000];
    //     assert_eq!(part_2(TEST_DATA, &mut buffer), 35);
    // }

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
