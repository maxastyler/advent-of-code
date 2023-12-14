use std::collections::HashMap;

#[inline]
fn str_to_it<'a>(input: &'a str) -> impl Iterator<Item = u8> + 'a + Clone {
    input.split(",").map(|s| s.parse().unwrap())
}

struct State {
    filled: u128,
    unfilled: u128,
    length: u8,
    clues: Vec<u8>,
}

impl State {
    fn from_line(line: &str, replications: usize) -> Self {
        let mut filled = 0u128;
        let mut unfilled = 0u128;
        let (board, clues) = line.split_once(" ").unwrap();
        let length = (board.chars().count() + 1) * replications - 1;
        board
            .chars()
            .chain(['?'])
            .cycle()
            .take(length)
            .enumerate()
            .for_each(|(i, c)| match c {
                '.' => unfilled |= 1 << i,
                '#' => filled |= 1 << i,
                _ => (),
            });
        let clues_length = str_to_it(clues).count();
        Self {
            filled,
            unfilled,
            clues: str_to_it(clues)
                .cycle()
                .take(clues_length * replications)
                .collect(),
            length: length as u8,
        }
    }

    fn lines_from_clues(&self) -> Box<dyn Iterator<Item = u128> + '_> {
        fn rec_clues_iter(
            clues: &[u8],
            start_position: u8,
            total_length: u8,
            current_num: u128,
        ) -> Box<dyn Iterator<Item = u128> + '_> {
            match clues {
                [] => panic!("empty clues"),
                [x] => Box::new(
                    (start_position..(total_length + 1 - x))
                        .map(move |offset| current_num | (((1 << x) - 1) << offset)),
                ),
                [x, rest @ ..] => {
                    // println!(
                    //     "TOTAL LENGTH: {}\nAMOUNT LEFT: {}",
                    //     total_length,
                    //     rest.iter().sum::<u8>() + rest.len() as u8
                    // );
                    let limit = total_length - rest.iter().sum::<u8>() - rest.len() as u8;
                    Box::new((start_position..limit).flat_map(move |offset| {
                        rec_clues_iter(
                            rest,
                            offset + x + 1,
                            total_length,
                            current_num | (((1 << x) - 1) << offset),
                        )
                    }))
                }
            }
        }
        rec_clues_iter(&self.clues, 0, self.length, 0)
    }

    fn iter_lines<'a>(&'a self) -> impl Iterator<Item = u128> + 'a {
        self.lines_from_clues().filter(|l| self.congruent(*l))
    }

    /// test whether a number agrees with this
    fn congruent(&self, num: u128) -> bool {
        let mask = (1 << self.length) - 1;
        ((num & self.filled) == self.filled) & (mask & (!num & self.unfilled) == self.unfilled)
    }
}

fn count(clues: &[u8], total_length: u8, filled: u128, unfilled: u128) -> usize {
    // key is (clues, length_left), value is counts
    let mut mem: HashMap<(&[u8], u8), usize> = HashMap::new();
    fn rec_clues_iter(
        clues: &[u8],
	total_length: u8,
        length: u8,
        current_num: u128,
	mem: &mut HashMap<(&[u8], u8), usize>
    ) -> usize {
	*mem.entry((clues, length)).or_insert_with(|| {
        match clues {
            [] => unreachable!(),
            [x] => {
		let start_position = total_length - length;
                (start_position..(total_length + 1 - x))
                    .map(move |offset| current_num | (((1 << x) - 1) << offset)).filter(|n| s)
            }
            [x, rest @ ..] => {
                let limit = total_length - rest.iter().sum::<u8>() - rest.len() as u8;
                Box::new((0..limit).map(move |offset| {
                    rec_clues_iter(
                        rest,
                        offset + x + 1,
                        total_length,
                        current_num | (((1 << x) - 1) << offset),
			mem
                    )
                }))
            }
        }	    
	})

    }
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| State::from_line(l, 1).iter_lines().count())
        .sum::<usize>()
}
pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| State::from_line(l, 3).iter_lines().count())
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2, State};

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut []), 21);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 21);
    }
}
