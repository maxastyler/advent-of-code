fn HASH(input: &str) -> u8 {
    input
        .chars()
        .fold(0, |acc, v| acc.wrapping_add(v as u8).wrapping_mul(17))
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    input.split(",").map(|s| HASH(s) as usize).sum()
}
pub fn part_2(_input: &str, _buffer: &mut [u8]) -> usize {
    3
}

#[cfg(test)]
mod test {
    use super::{part_1, HASH};

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_individual_part() {
        assert_eq!(HASH("rn=1"), 30)
    }
    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut []), 1320);
    }
    #[test]
    fn part_2_works() {}
}
