use crate::mem::Mem;

fn num_wins_in_line(line: &str) -> usize {
    let (theirs, ours) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

    theirs
        .split_whitespace()
        .filter(|s| {
            let winner = s.parse::<usize>().unwrap();
            ours.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .any(|our_num| winner == our_num)
        })
        .count()
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| match num_wins_in_line(l) {
            0 => 0,
            i => 2usize.pow(i as u32 - 1),
        })
        .sum()
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let counts = mem.alloc_slice(input.lines().count(), |_| 1usize).unwrap();
    input.lines().enumerate().for_each(|(i, line)| {
        let n = num_wins_in_line(line);

        for index_below in (i + 1)..(i + 1 + n) {
            counts[index_below] += counts[i]
        }
    });
    counts.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::day_04::{part_1, part_2};

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut []), 13);
    }

    #[test]
    fn part_2_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_2(TEST_INPUT, &mut buffer), 30);
    }
}
