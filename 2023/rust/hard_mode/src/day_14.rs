use crate::mem::{self, Mem};

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Sphere,
    Cube,
    None,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(PartialEq)]
struct Map<'a> {
    rocks: &'a mut [Tile],
    cols: usize,
    rows: usize,
}

impl<'a> Map<'a> {
    fn new<'b: 'a>(input: &str, mem: &'b Mem<'b>) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let rocks = mem
            .alloc_slice_from_iter(
                rows * cols,
                input.lines().flat_map(|l| {
                    l.chars().map(|c| match c {
                        'O' => Tile::Sphere,
                        '#' => Tile::Cube,
                        _ => Tile::None,
                    })
                }),
            )
            .unwrap();
        Self { rocks, cols, rows }
    }

    fn clone<'b: 'a>(&self, mem: &'b Mem) -> Self {
        let new_rocks = mem.clone_slice(self.rocks).unwrap();
        Self {
            rocks: new_rocks,
            cols: self.cols,
            rows: self.rows,
        }
    }

    fn coord(&self, row: usize, col: usize) -> &Tile {
        let index = row * self.cols + col;
        &self.rocks[index]
    }

    fn coord_mut(&mut self, row: usize, col: usize) -> &mut Tile {
        let index = row * self.cols + col;
        &mut self.rocks[index]
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for (row, col) in Self::iter_coords(self.rows, self.cols) {
                    if *self.coord(row, col) == Tile::Sphere {
                        // We need to move it
                        if let Some(row2) = (0..row)
                            .rev()
                            .take_while(|row2| *self.coord(*row2, col) == Tile::None)
                            .last()
                        {
                            *self.coord_mut(row2, col) = Tile::Sphere;
                            *self.coord_mut(row, col) = Tile::None
                        }
                    }
                }
            }
            Direction::West => {
                for (row, col) in Self::iter_coords(self.rows, self.cols) {
                    if *self.coord(row, col) == Tile::Sphere {
                        // We need to move it
                        if let Some(col2) = (0..col)
                            .rev()
                            .take_while(|col2| *self.coord(row, *col2) == Tile::None)
                            .last()
                        {
                            *self.coord_mut(row, col2) = Tile::Sphere;
                            *self.coord_mut(row, col) = Tile::None
                        }
                    }
                }
            }
            Direction::East => {
                for (row, col) in Self::iter_coords_rev(self.rows, self.cols) {
                    if *self.coord(row, col) == Tile::Sphere {
                        // We need to move it
                        if let Some(col2) = (col + 1..self.cols)
                            .take_while(|col2| *self.coord(row, *col2) == Tile::None)
                            .last()
                        {
                            *self.coord_mut(row, col2) = Tile::Sphere;
                            *self.coord_mut(row, col) = Tile::None
                        }
                    }
                }
            }

            Direction::South => {
                for (row, col) in Self::iter_coords_rev(self.rows, self.cols) {
                    if *self.coord(row, col) == Tile::Sphere {
                        // We need to move it
                        if let Some(row2) = (row + 1..self.rows)
                            .rev()
                            .take_while(|row2| *self.coord(*row2, col) == Tile::None)
                            .last()
                        {
                            *self.coord_mut(row2, col) = Tile::Sphere;
                            *self.coord_mut(row, col) = Tile::None
                        }
                    }
                }
            }
        }
    }

    fn iter_coords(rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..rows).flat_map(move |row| (0..cols).map(move |col| (row, col)))
    }

    fn iter_coords_rev(rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..rows)
            .rev()
            .flat_map(move |row| (0..cols).rev().map(move |col| (row, col)))
    }

    fn load(&self) -> usize {
        Self::iter_coords(self.rows, self.cols)
            .map(|(row, col)| {
                if *self.coord(row, col) == Tile::Sphere {
                    self.rows - row
                } else {
                    0
                }
            })
            .sum()
    }
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let mut map = Map::new(input, &mem);
    map.tilt(Direction::North);
    map.load()
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let mut history = mem.alloc_growable::<(Direction, Map), 1000>().unwrap();
    let mut map = Map::new(input, &mem);
    let mut start_loop = None;
    for dir in [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .iter()
    .cycle()
    {
        history.push((*dir, map.clone(&mem)));
        map.tilt(*dir);
        if let Some(i) = history.iter().enumerate().find_map(|(i, (d, m))| {
            if *m == map {
                Some(i)
            } else {
                None
            }
        }) {
            start_loop = Some(i);
            break;
        }
    }
    start_loop.unwrap()
}

#[cfg(test)]
mod test {
    use core::fmt::Debug;
    use std::println;
    extern crate std;

    use crate::mem::Mem;

    use super::{part_1, part_2, Map};

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    impl<'a> Debug for Map<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for row in 0..self.rows {
                for col in 0..self.cols {
                    let s = match self.coord(row, col) {
                        super::Tile::Sphere => "O",
                        super::Tile::Cube => "#",
                        super::Tile::None => ".",
                    };
                    f.write_str(s)?;
                }
                f.write_str("\n");
            }
            Ok(())
        }
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut [0u8; 1000]), 136);
    }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(part_2(TEST_INPUT, &mut [0u8; 1000000]), 3);
    // }
}
