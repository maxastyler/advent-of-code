// use core::{
//     array::IntoIter,
//     iter::{repeat_with, Scan},
// };

// use core::iter::Chain;

// fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = usize> + 'a> + 'a {
//     input
//         .lines()
//         .map(|s| s.split_whitespace().map(|x| x.parse().unwrap()))
// }

// fn scan_internal(st: &mut usize, v: usize) -> Option<usize> {
//     let ret_val = v - *st;
//     *st = v;
//     Some(ret_val)
// }

// type ItType<'a> = Chain<
//     IntoIter<usize, 1>,
//     Scan<impl Iterator<Item = usize> + 'a, usize, fn(&'a mut usize, usize) -> Option<usize>>,
// >;

// /// Returns a tuple of bools
// /// (first value of sequence, first diff of sequence, lower layer of differences)
// fn diff<'a>(mut sequence: impl Iterator<Item = usize> + 'a) -> Option<(usize, usize, ItType<'a>)> {
//     if let Some(x) = sequence.next() {
//         if let Some(y) = sequence.next() {
//             return Some((
//                 x,
//                 y - x,
//                 [y - x].into_iter().chain(sequence.scan(x, scan_internal)),
//             ));
//         }
//     }
//     None
// }

fn choose(n: usize, k: usize) -> usize {
    ((n + 1 - k)..=(n)).product::<usize>() / (1..=k).product::<usize>()
}

// fn get_a_n<'a>(n: usize, sequence: impl Iterator<Item = usize> + 'a) -> usize {
//     let (first, first_diff, s) = diff(sequence).unwrap();
//     let mut sequence = s;
//     sequence = diff(sequence).unwrap().2;
//     let mut total = first;
//     let (first, first_diff, s) = diff(sequence).unwrap();
//     repeat_with(|| {
//         let (first, first_diff, s) = diff(sequence).unwrap();
//         sequence = s;
//         first
//     });
//     3
// }

// // fn calc_differences<'a>(mut sequence: impl Iterator<Item = usize> + 'a) -> (usize, usize) {
// //     let mut sequence: Box<dyn Iterator<Item = usize> + 'a> = Box::new(sequence);
// //     // (0..).for_each(|i|)
// //     // loop {
// //     // 	let (same, new) = diff(sequence);
// //     // 	sequence = new;
// //     // }
// //     // while !(*sequence).cloned().all(|x| x == 0) {
// //     // 	sequence = diff(sequence);
// //     // }
// //     (3, 3)
// // }

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    3
}

pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    3
}

#[cfg(test)]
mod test {
    use super::{choose, part_1};

    #[test]
    fn choose_works() {
        assert_eq!(choose(3, 2), 3);
        assert_eq!(choose(1000, 0), 1);
    }
    #[test]
    fn part_1_works() {
        // assert_eq!(part_1("", &mut [0; 1000]), 0);
    }
}
