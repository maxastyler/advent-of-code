use crate::{lru::LRU, mem::Mem};

struct Line<'a> {
    clues: &'a [u8],
    filled: u128,
    unfilled: u128,
    length: u8,
}

impl<'a> Line<'a> {
    fn from_str_line<'b: 'a>(line: &'b str, replications: usize, mem: &'b Mem<'b>) -> Self {
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
        let clues_length = clues.split(",").count();
        let clues_slice = mem
            .alloc_slice_from_iter(
                clues_length * replications,
                clues
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .cycle()
                    .take(clues_length * replications),
            )
            .unwrap();
        Self {
            filled,
            unfilled,
            clues: clues_slice,
            length: length as u8,
        }
    }

    fn congruent(num: u128, length_left: u8, filled: u128, unfilled: u128) -> bool {
        let mask = (1 << length_left) - 1;
        let filled_congruent = mask & (num & filled) == mask & filled;
        let unfilled_congruent = mask & (!num & unfilled) == mask & unfilled;
        filled_congruent & unfilled_congruent
    }

    fn cpos_inner<'b, const N: usize>(
        clues: &'b [u8],
        length_left: u8,
        filled: u128,
        unfilled: u128,
        lru: &mut LRU<(&'b [u8], u8), usize, N>,
    ) -> usize {
        lru.get(&(clues, length_left))
            .map(|x| *x)
            .unwrap_or_else(move || {
                let total = match clues {
                    [] => panic!("empty clues"),
                    [x] => (0..(length_left + 1 - x))
                        .map(|shift| ((1 << x) - 1) << shift)
                        .filter(|n| Self::congruent(*n, length_left, filled, unfilled))
                        .count(),
                    [x, rest @ ..] => {
                        let limit = length_left - &clues.iter().sum() - (clues.len() as u8 - 1) + 1;
                        (0..limit)
                            .filter_map(|shift| {
                                let bits = ((1 << x) - 1) << shift;
                                let length = shift + x + 1;
                                if Self::congruent(bits, shift + x + 1, filled, unfilled) {
                                    Some(Self::cpos_inner(
                                        rest,
                                        length_left - (shift + x + 1),
                                        filled >> length,
                                        unfilled >> length,
                                        lru,
                                    ))
                                } else {
                                    None
                                }
                            })
                            .sum()
                    }
                };
                lru.put((clues, length_left), total);
                total
            })
    }

    fn count_positions(&self) -> usize {
        let mut lru: LRU<(&[u8], u8), usize, 60> = LRU::new();

        Self::cpos_inner(
            self.clues,
            self.length,
            self.filled,
            self.unfilled,
            &mut lru,
        )
    }
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    input
        .lines()
        .map(|l| Line::from_str_line(l, 1, &mem).count_positions())
        .sum()
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    input
        .lines()
        .map(|l| Line::from_str_line(l, 5, &mem).count_positions())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::mem::Mem;

    use super::{part_1, part_2, Line};
    const TEST_INPUT_1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn count_positions() {
        let mut buffer = [0; 1000];
        let mem = Mem::new(&mut buffer);
        let l = Line::from_str_line("????? 2,2", 1, &mem);
        assert_eq!(l.count_positions(), 1);
        let l = Line::from_str_line("?????? 2,2", 1, &mem);
        assert_eq!(l.count_positions(), 3);
        let l = Line::from_str_line("????? 1,1", 1, &mem);
        assert_eq!(l.count_positions(), 6);
        let l = Line::from_str_line("????? 1,1,1", 1, &mem);
        assert_eq!(l.count_positions(), 1);
        let l = Line::from_str_line("?????? 1,1,1", 1, &mem);
        assert_eq!(l.count_positions(), 4);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT_1, &mut [0; 100]), 21);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT_1, &mut [0; 1000]), 525152);
    }
}
