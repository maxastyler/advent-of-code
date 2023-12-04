#[derive(Default)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    pub fn update(&mut self, colour: &str, number: usize) {
        match colour {
            "red" => self.red = self.red.max(number),
            "green" => self.green = self.green.max(number),
            "blue" => self.blue = self.blue.max(number),
            _ => panic!(),
        }
    }

    pub fn satisfies(&self, other: &Hand) -> bool {
        (self.red <= other.red) & (self.blue <= other.blue) & (self.green <= other.green)
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn parse_line(input: &str) -> (usize, Hand) {
    let mut hand = Hand::default();
    let input = input.trim_start_matches(|c: char| !c.is_ascii_digit());
    let (game_num, draws) = input.split_once(": ").unwrap();
    for (num, colour) in draws
        .split("; ")
        .flat_map(|draws| draws.split(", "))
        .map(|d| d.split_once(" ").unwrap())
    {
        hand.update(colour, num.parse().unwrap())
    }
    (game_num.parse::<usize>().unwrap(), hand)
}

pub fn part_1(input: &str) -> usize {
    let limit_hand = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .lines()
        .filter_map(|l| {
            let (game_num, hand) = parse_line(l);
            if hand.satisfies(&limit_hand) {
                Some(game_num)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input.lines().map(|l| parse_line(l).1.power()).sum()
}

#[cfg(test)]
mod test {
    use crate::day_02::part_2;

    use super::part_1;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), 8);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), 2286);
    }
}
