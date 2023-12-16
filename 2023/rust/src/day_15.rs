fn HASH(input: &str) -> u8 {
    input
        .chars()
        .fold(0, |acc, v| acc.wrapping_add(v as u8).wrapping_mul(17))
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    input.split(",").map(|s| HASH(s) as usize).sum()
}
pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    let mut boxes: [Vec<(&str, u8)>; 256] = [(); 256].map(|_| vec![]);
    for inst in input.split(",") {
        if let Some((label, lens)) = inst.split_once("=") {
            let b = &mut boxes[HASH(label) as usize];
            let lens = lens.parse().unwrap();
            if let Some(r) = b.iter_mut().find(|(s, _)| *s == label) {
                r.1 = lens;
            } else {
                b.push((label, lens));
            }
        } else {
            let label = &inst[..inst.len() - 1];
            boxes[HASH(label) as usize].retain(|(l, _)| *l != label);
        }
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(j, (_, f))| (i + 1) * (j + 1) * (*f as usize))
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2, HASH};

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
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 145);
    }
}
