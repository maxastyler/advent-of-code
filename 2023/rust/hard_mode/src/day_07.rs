use crate::mem::Mem;

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
struct Card(u8);

impl From<char> for Card {
    fn from(value: char) -> Self {
        Self(match value {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!(),
        })
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Hand(u8);

impl Hand {
    fn hand_from_counts(counts: &[u8]) -> Self {
        let mut ones = 0u8;
        let mut previous_two = false;
        let mut previous_three = false;

        for count in counts {
            match count {
                5 => return Hand(6),
                4 => return Hand(5),
                3 => {
                    if previous_two {
                        return Hand(4);
                    } else if ones >= 1 {
                        return Hand(3);
                    } else {
                        previous_three = true;
                    }
                }
                2 => {
                    if previous_three {
                        return Hand(4);
                    } else if previous_two {
                        return Hand(2);
                    } else if ones >= 2 {
                        return Hand(1);
                    } else {
                        previous_two = true;
                    }
                }
                1 => {
                    if previous_three {
                        return Hand(3);
                    } else if previous_two & (ones >= 1) {
                        return Hand(1);
                    } else if ones >= 3 {
                        return Hand(0);
                    } else {
                        ones += 1;
                    }
                }
                0 => (),
                _ => panic!(),
            }
        }
        Hand(0)
    }
    fn from_cards(cards: &mut [Card; 5]) -> Self {
        let mut counts = [0u8; 14];
        for Card(x) in cards {
            counts[*x as usize] += 1
        }
        Self::hand_from_counts(&counts)
    }
    fn from_cards_with_joker(cards: &mut [Card; 5]) -> Self {
        let mut counts = [0u8; 14];
        for Card(x) in cards.iter() {
            counts[*x as usize] += 1
        }
        let num_jokers = core::mem::replace(&mut counts[10], 0);
        *counts.iter_mut().max().unwrap() += num_jokers;
        cards.iter_mut().for_each(|c| {
            if c.0 == 10 {
                c.0 = 0
            }
        });
        Self::hand_from_counts(&counts)
    }
}

#[derive(Ord, Eq, Debug, PartialEq, PartialOrd)]
struct CardMemo(Hand, [Card; 5]);

fn parse_input<'a, F>(input: &'a str, mem: &Mem<'a>, memo_fun: F) -> &'a mut [(CardMemo, usize)]
where
    F: Fn(&mut [Card; 5]) -> Hand,
{
    let s = mem
        .alloc_slice(input.lines().count(), |_| {
            (CardMemo(Hand(0), [Card(0); 5]), 0)
        })
        .unwrap();
    s.iter_mut()
        .zip(input.lines())
        .for_each(|((card_memo, score), line)| {
            let (cards_str, num) = line.split_once(" ").unwrap();
            card_memo
                .1
                .iter_mut()
                .zip(cards_str.chars())
                .for_each(|(card, ch)| *card = ch.into());
            card_memo.0 = memo_fun(&mut card_memo.1);
            *score = num.parse().unwrap()
        });
    s
}

impl CardMemo {
    fn from_cards(mut cards: [Card; 5]) -> Self {
        Self(Hand::from_cards(&mut cards), cards)
    }
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let data = parse_input(input, &mem, &Hand::from_cards);
    data.sort_unstable();
    data.iter()
        .enumerate()
        .map(|(i, (_, score))| (i + 1) * score)
        .sum()
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let data = parse_input(input, &mem, &Hand::from_cards_with_joker);
    data.sort_unstable();
    data.iter()
        .enumerate()
        .map(|(i, (_, score))| (i + 1) * score)
        .sum()
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2, CardMemo, Hand};

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn order_of_cardmemo() {
        let cm = CardMemo::from_cards(['2'.into(); 5]);
        assert_eq!(cm.0, Hand(6));
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut [0; 1000]), 6440);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut [0; 1000]), 5905);
    }
}
