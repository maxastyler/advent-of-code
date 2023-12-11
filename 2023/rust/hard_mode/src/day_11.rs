use core::iter::repeat;

use crate::mem::Mem;

struct Grid<'a> {
    empty_rows: &'a [i64],
    empty_columns: &'a [i64],
}

type Coord = (i64, i64);

impl<'a> Grid<'a> {
    fn empty_column(data: &'a str, column: usize) -> bool {
        !data.lines().any(|l| &l[column..column + 1] == "#")
    }

    fn empty_columns(data: &'a str, columns: usize) -> impl Iterator<Item = i64> + 'a {
        (0..columns).filter_map(|i| {
            if Grid::empty_column(data, i) {
                Some(i as i64)
            } else {
                None
            }
        })
    }
    fn empty_rows(data: &'a str) -> impl Iterator<Item = i64> + 'a {
        data.lines().enumerate().filter_map(|(i, l)| {
            if l.contains("#") {
                None
            } else {
                Some(i as i64)
            }
        })
    }

    fn get_galaxies(data: &'a str) -> impl Iterator<Item = Coord> + 'a {
        data.lines().enumerate().flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                if c == '#' {
                    Some((row as i64, col as i64))
                } else {
                    None
                }
            })
        })
    }

    fn new(data: &'a str, mem: &Mem<'a>) -> Option<Self> {
        let (rows, columns) = data.lines().map(|l| l.chars().count()).try_fold(
            (0usize, None),
            |(lines, last_columns), columns| {
                last_columns.map_or(Some((lines + 1, Some(columns))), |lc| {
                    if lc == columns {
                        Some((lines + 1, Some(columns)))
                    } else {
                        None
                    }
                })
            },
        )?;
        let num_empty_columns = Grid::empty_columns(data, columns?).count();
        let empty_columns = mem
            .alloc_slice_from_iter(num_empty_columns, Grid::empty_columns(data, columns?))
            .unwrap();

        let num_empty_rows = Grid::empty_rows(data).count();
        let empty_rows = mem
            .alloc_slice_from_iter(num_empty_rows, Grid::empty_rows(data))
            .unwrap();

        Some(Self {
            empty_columns,
            empty_rows,
        })
    }

    fn distance(&self, a: Coord, b: Coord, expansion: usize) -> usize {
        let (a_r, a_c) = a;
        let (b_r, b_c) = b;
        let range_row = a_r.min(b_r)..a_r.max(b_r);
        let range_col = a_c.min(b_c)..a_c.max(b_c);
        (self
            .empty_columns
            .iter()
            .filter(|i| range_col.contains(i))
            .count()
            + self
                .empty_rows
                .iter()
                .filter(|i| range_row.contains(i))
                .count())
            * expansion
            + range_row.count()
            + range_col.count()
    }
}

fn iter_pairs<A>(slice: &[A]) -> impl Iterator<Item = (&A, &A)> {
    (0..slice.len() - 1).flat_map(|i| {
        let a = &slice[i];
        slice[i + 1..].iter().map(move |b| (a, b))
    })
}

fn distances(input: &str, buffer: &mut [u8], expansion: usize) -> usize {
    let mem = Mem::new(buffer);
    let grid = Grid::new(input, &mem).unwrap();
    let galaxy_coords = mem
        .alloc_slice_from_iter(
            input.chars().filter(|c| *c == '#').count(),
            Grid::get_galaxies(input),
        )
        .unwrap();
    iter_pairs(galaxy_coords)
        .map(|(a, b)| grid.distance(*a, *b, expansion))
        .sum()
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    distances(input, buffer, 1)
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    distances(input, buffer, 1000000 - 1)
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use alloc::vec::Vec;

    use crate::day_11::distances;

    use super::{part_1, part_2, Grid};
    const TEST_INPUT_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT_1, &mut [0; 10000]), 374);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(distances(TEST_INPUT_1, &mut [0; 10000], 9), 1030);
        assert_eq!(distances(TEST_INPUT_1, &mut [0; 10000], 99), 8410);
    }
}
