use crate::mem::{Mem, Oom};

fn count_input(input: &str) -> (usize, usize) {
    let mut in_num = false;
    let mut total_nums = 0;
    let mut total_symbols = 0;
    input.chars().for_each(|c| {
        if c.is_ascii_digit() {
            if !in_num {
                in_num = true
            }
        } else {
            if in_num {
                total_nums += 1;
                in_num = false;
            }
            if (c != '\n') & (c != '.') {
                total_symbols += 1;
            }
        }
    });
    if in_num {
        total_nums += 1;
    }
    (total_nums, total_symbols)
}

type Pos = (usize, usize);

fn distance_sq(p1: &Pos, p2: &Pos) -> usize {
    (p1.0.abs_diff(p2.0)).pow(2) + (p1.1.abs_diff(p2.1)).pow(2)
}

#[derive(Debug, Default)]
struct Num {
    num: usize,
    start: Pos,
    len: usize,
}

impl Num {
    fn adjacent(&self, pos: &Pos) -> bool {
        (0..self.len).any(|i| distance_sq(&(self.start.0, self.start.1 + i), pos) <= 2)
    }
}

#[derive(Debug, Default)]
struct Symbol {
    symbol: char,
    pos: Pos,
}

fn parse_input<'m>(input: &str, mem: &mut Mem<'m>) -> Option<(&'m [Num], &'m [Symbol])> {
    let (total_nums, total_symbols) = count_input(input);
    let mut num_index = 0;
    let mut symbol_index = 0;
    let mut current_num: Option<(usize, usize, usize)> = None;
    let nums = mem.alloc_slice_default(total_nums).ok()?;
    let symbols = mem.alloc_slice_default(total_symbols).ok()?;
    let mut current_row = 0;
    let mut current_col = 0;
    for c in input.chars() {
        current_col += 1;
        if c.is_ascii_digit() {
            let c_num = c as usize - '0' as usize;
            current_num = match current_num {
                Some((n, start, len)) => Some((n * 10 + c_num, start, len + 1)),
                None => Some((c_num, current_col, 1)),
            }
        } else {
            if let Some((n, start, len)) = current_num {
                current_num = None;
                nums[num_index] = Num {
                    num: n,
                    start: (current_row, start),
                    len,
                };
                num_index += 1;
            }
            match c {
                '\n' => {
                    current_row += 1;
                    current_col = 0;
                }
                '.' => {}
                _ => {
                    symbols[symbol_index] = Symbol {
                        symbol: c,
                        pos: (current_row, current_col),
                    };
                    symbol_index += 1;
                }
            }
        }
    }
    if let Some((n, start, len)) = current_num {
        nums[num_index] = Num {
            num: n,
            start: (current_row, start),
            len,
        };
    }
    Some((nums, symbols))
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mut mem = Mem::new(buffer);
    let (nums, symbols) = parse_input(input, &mut mem).unwrap();
    nums.iter()
        .filter(|n| symbols.iter().any(|s| n.adjacent(&s.pos)))
        .fold(0, |a, v| a + v.num)
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mut mem = Mem::new(buffer);
    let (nums, symbols) = parse_input(input, &mut mem).unwrap();
    symbols
        .iter()
        .filter_map(|s| {
            if s.symbol == '*' {
                let mut total = 0;
                let mut ratio = 1;
                for n in nums.iter() {
                    if n.adjacent(&s.pos) {
                        total += 1;
                        ratio *= n.num;
                    }
                    if total > 2 {
                        return None;
                    }
                }
                if total == 2 {
                    Some(ratio)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::{part_1, part_2};
    const TEST_INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_1(TEST_INPUT, &mut buffer), 4361);
    }

    #[test]
    fn part_2_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_2(TEST_INPUT, &mut buffer), 467835);
    }
}
