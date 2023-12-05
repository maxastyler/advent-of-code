use std::ops::Range;

#[derive(Debug)]
struct Map {
    range: Range<usize>,
    offset: isize,
}

fn offset(range: &Range<usize>, offset: isize) -> Range<usize> {
    range.start.checked_add_signed(offset).unwrap()..range.end.checked_add_signed(offset).unwrap()
}

fn parse_input(input: &str) -> Option<(Vec<usize>, Vec<Vec<Map>>)> {
    let (start, rest) = input.split_once("\n\n")?;
    let seeds = start[7..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let maps = rest
        .split("\n\n")
        .map(|s| {
            let mut maps = s
                .trim_start_matches(|c: char| !c.is_ascii_digit())
                .lines()
                .map(|l| {
                    let [dest, start, len] = l
                        .split_ascii_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<_>>()[..]
                    else {
                        panic!()
                    };

                    Map {
                        range: start..start + len,
                        offset: dest as isize - start as isize,
                    }
                })
                .collect::<Vec<_>>();
            maps.sort_by(|a, b| a.range.clone().partial_cmp(b.range.clone()).unwrap());
            maps
        })
        .collect();
    Some((seeds, maps))
}

fn split(range: &Range<usize>, other: &Range<usize>) -> [Option<Range<usize>>; 3] {
    if range.start < other.start {
        // we start before the other range
        if range.end <= other.start {
            [Some(range.clone()), None, None]
        } else if range.end <= other.end {
            [
                Some(range.start..other.start),
                Some(other.start..range.end),
                None,
            ]
        } else {
            [
                Some(range.start..other.start),
                Some(other.clone()),
                Some(other.end..range.end),
            ]
        }
    } else if range.start < other.end {
        if range.end <= other.end {
            [None, Some(range.clone()), None]
        } else {
            [
                None,
                Some(range.start..other.end),
                Some(other.end..range.end),
            ]
        }
    } else {
        [None, None, Some(range.clone())]
    }
}

fn convert_ingredients(mut ingredients: Vec<Range<usize>>, maps: Vec<Map>) -> Vec<Range<usize>> {
    let mut next_row = vec![];
    for map in maps.iter() {
        let mut added = vec![];
        while let Some(range) = ingredients.pop() {
            let [before, during, after] = split(&range, &map.range);
            if let Some(d) = during {
                let off = offset(&d, map.offset);
                next_row.push(off);
                if let Some(r) = before {
                    ingredients.push(r)
                }
                if let Some(r) = after {
                    ingredients.push(r)
                }
            } else {
                if let Some(r) = before {
                    added.push(r)
                }
                if let Some(r) = after {
                    added.push(r)
                }
            }
        }
        ingredients.append(&mut added)
    }
    next_row.append(&mut ingredients);
    next_row
}

pub fn part_1(input: &str) -> usize {
    let (seeds, maps) = parse_input(input).unwrap();
    let mut ings = seeds.iter().map(|v| (*v..*v + 1)).collect();
    for ms in maps {
        ings = convert_ingredients(ings, ms);
    }
    ings.iter().map(|x| x.start).min().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let (seeds, maps) = parse_input(input).unwrap();
    let mut ings = seeds.chunks(2).map(|v| (v[0]..v[0] + v[1])).collect();
    for ms in maps {
        ings = convert_ingredients(ings, ms);
    }
    ings.iter().map(|x| x.start).min().unwrap()
}

#[cfg(test)]
mod test {
    use super::offset;
    use crate::day_05::part_2;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

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
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), 46);
    }

    #[test]
    fn offset_works() {
        assert_eq!(offset(&(55..57), -3), 52..54);
    }
}
