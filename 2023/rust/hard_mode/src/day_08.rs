extern crate alloc;

use crate::mem::Mem;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Map<'a> {
    name: &'a str,
    left: usize,
    right: usize,
}

fn find_position<'a>(name: &'a str, maps: &mut [Map<'a>]) -> Option<usize> {
    maps.iter()
        .enumerate()
        .find_map(|(i, map)| if map.name == name { Some(i) } else { None })
        .or_else(|| {
            maps.iter_mut().enumerate().find_map(|(i, map)| {
                if map.name == "" {
                    map.name = name;
                    Some(i)
                } else {
                    None
                }
            })
        })
}

fn parse_input<'a>(input: &'a str, mem: &'a Mem) -> (&'a [Direction], &'a [Map<'a>]) {
    let (directions, maps) = input.split_once("\n\n").unwrap();
    let dir_len = directions.chars().count();
    let dir_slice = mem.alloc_slice(dir_len, |_| Direction::Left).unwrap();
    directions
        .chars()
        .zip(dir_slice.iter_mut())
        .for_each(|(c, r)| {
            *r = match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!(),
            }
        });
    let num_lines = maps.lines().count();
    let map_slice = mem
        .alloc_slice(num_lines, |_| Map {
            name: "",
            left: 0,
            right: 0,
        })
        .unwrap();

    maps.lines().for_each(|l| {
        let left = &l[7..10];
        let right = &l[12..15];
        let index = find_position(&l[0..3], map_slice).unwrap();

        map_slice[index].left = find_position(left, map_slice).unwrap();
        map_slice[index].right = find_position(right, map_slice).unwrap();
    });

    (dir_slice, map_slice)
}

fn find_path_length<'a, F>(
    start: &str,
    end_fun: F,
    directions: &'a [Direction],
    maps: &'a [Map<'a>],
) -> usize
where
    F: Fn(&str) -> bool,
{
    let start_pos = maps
        .iter()
        .enumerate()
        .find(|(_, s)| s.name == start)
        .unwrap()
        .0;
    directions
        .iter()
        .cycle()
        .scan(start_pos, |pos, dir| {
            let element = &maps[*pos];
            *pos = match dir {
                Direction::Left => element.left,
                Direction::Right => element.right,
            };
            Some(*pos)
        })
        .enumerate()
        .find_map(|(n, i)| if end_fun(maps[i].name) { Some(n) } else { None })
        .unwrap()
        + 1
}

fn gcd(a: usize, b: usize) -> usize {
    let mut max_v = a.max(b);
    let mut min_v = a.min(b);
    while min_v != 0 {
        let temp = min_v;
        min_v = max_v.rem_euclid(min_v);
        max_v = temp;
    }
    max_v
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (directions, maps) = parse_input(input, &mem);
    find_path_length("AAA", |x| x == "ZZZ", directions, maps)
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (directions, maps) = parse_input(input, &mem);
    maps.iter()
        .filter(|m| m.name.ends_with("A"))
        .fold(1, |a, map| {
            lcm(
                find_path_length(map.name, |x| x.ends_with("Z"), directions, maps),
                a,
            )
        })
}

#[cfg(test)]
mod test {
    use super::{gcd, lcm, part_1, part_2};
    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT_1, &mut [0u8; 1000]), 2);
        assert_eq!(part_1(TEST_INPUT_2, &mut [0u8; 1000]), 6);
    }

    #[test]
    fn part_2_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part_2(input, &mut [0u8; 1000]), 6);
    }

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(6, 0), 6);
        assert_eq!(gcd(0, 6), 6);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(21, 6), 42);
    }
}
