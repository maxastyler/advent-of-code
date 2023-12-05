type Pos = (usize, usize);

fn distance_sq(p1: &Pos, p2: &Pos) -> usize {
    (p1.0.abs_diff(p2.0)).pow(2) + (p1.1.abs_diff(p2.1)).pow(2)
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Symbol {
    symbol: char,
    pos: Pos,
}

fn parse_line(line: &str, row: usize, nums: &mut Vec<Num>, symbols: &mut Vec<Symbol>) {
    let mut current_num: Option<(usize, usize, usize)> = None;
    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            let c_num = c as usize - '0' as usize;
            current_num = match current_num {
                Some((n, start, len)) => Some((n * 10 + c_num, start, len + 1)),
                None => Some((c_num, i, 1)),
            }
        } else {
            if let Some((n, start, len)) = current_num {
                nums.push(Num {
                    num: n,
                    start: (row, start),
                    len,
                });
                current_num = None;
            }
            if c != '.' {
                symbols.push(Symbol {
                    symbol: c,
                    pos: (row, i),
                })
            }
        }
    }
    if let Some((n, start, len)) = current_num {
        nums.push(Num {
            num: n,
            start: (row, start),
            len,
        });
    }
}

fn parse_input<'m>(input: &'m str) -> (Vec<Num>, Vec<Symbol>) {
    let mut nums = vec![];
    let mut symbols = vec![];
    input.lines().enumerate().for_each(|(row, line)| {
        parse_line(line, row, &mut nums, &mut symbols);
    });
    (nums, symbols)
}

pub fn part_1(input: &'static str) -> String {
    let (nums, symbols) = parse_input(input);
    format!(
        "{}",
        nums.iter()
            .filter(|n| symbols.iter().any(|s| n.adjacent(&s.pos)))
            .fold(0, |a, v| a + v.num)
    )
}

pub fn part_2(input: &'static str) -> String {
    let (nums, symbols) = parse_input(input);
    let total: usize = symbols
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
        .sum();
    format!("{}", total)
}

#[cfg(test)]
mod test {
    use crate::day_03::part_2;

    use super::part_1;

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
        assert_eq!(part_1(TEST_INPUT), String::from("4361"));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), String::from("467835"));
    }
}
