#[derive(PartialEq, PartialOrd)]
struct Card(u8);

impl From<char> for Card {
    fn from(value: char) -> Self {
        Self(match value {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!(),
        })
    }
}

#[derive(PartialEq, PartialOrd)]
struct Hand(u8);

impl Hand {
    fn from_hand_array(hand: [Card; 5]) -> Self {
        let mut counts = [0u8; 13];
        for Card(x) in hand {
            counts[x] += 1
        }
        let mut ones = 0u8;
        let mut previous_two = false;
        let mut previous_three = false;

        for (i, count) in counts.iter().enumerate() {
            match count {
                5 => return Hand(6),
                4 => return Hand(5),
                3 => {
                    if previous_two {
                        return Hand(4);
                    } else if previous_one {
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
                    }
                }
            }
        }
    }
}

pub fn part_1(input: &str) -> usize {
    3
}

pub fn part_2(input: &str) -> usize {
    3
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2, Card};

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), 288);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), 71503);
    }
}
