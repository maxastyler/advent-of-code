fn diff(nums: &[i64]) -> Vec<i64> {
    nums.iter()
        .zip(nums[1..].iter())
        .map(|(x, y)| y - x)
        .collect()
}

fn calculate_line(line: impl Iterator<Item = i64>) -> (i64, i64) {
    let mut stack: Vec<Vec<i64>> = vec![line.collect()];
    loop {
        let diffs = diff(stack.last().unwrap());
        if diffs.iter().all(|x| x == diffs.first().unwrap()) {
            stack.push(diffs);
            break;
        }
        stack.push(diffs);
    }
    let mut first = 0;
    let mut last = 0;
    while let Some(seq) = stack.pop() {
        first = seq.first().unwrap() - first;
        last = seq.last().unwrap() + last;
    }
    (first, last)
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| calculate_line(l.split_whitespace().map(|x| x.parse().unwrap())).1)
        .sum::<i64>() as usize
}

pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    input
        .lines()
        .map(|l| calculate_line(l.split_whitespace().map(|x| x.parse().unwrap())).0)
        .sum::<i64>() as usize
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};
    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut []), 114);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 2);
    }
}
