use std::fmt::Debug;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Tile {
    Sphere,
    Cube,
    None,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(PartialEq, Clone)]
struct Map {
    pub rocks: Vec<Tile>,
    cols: usize,
    rows: usize,
}

impl Debug for Map {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let s = match self.coord(row, col) {
                    Tile::Sphere => "O",
                    Tile::Cube => "#",
                    Tile::None => ".",
                };
                f.write_str(s)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let rocks = input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    'O' => Tile::Sphere,
                    '#' => Tile::Cube,
                    _ => Tile::None,
                })
            })
            .collect();

        Self { rocks, cols, rows }
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
                        if let Some(row2) = (row + 1..self.rows)
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

    fn cycle(&mut self) {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            self.tilt(dir)
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

/// returns (the map, the first iteration the map occurred, the next iteration it occurred,
/// the number of tilts from the initial map)
pub fn find_start_of_loop(mut map: Map) -> (usize, Vec<Map>) {
    let mut history = vec![map.clone()];
    let start_point = loop {
        map.cycle();

        if let Some((i, _)) = history
            .iter()
            .enumerate()
            .find(|(_, old_map)| *old_map == &map)
        {
            break i;
        } else {
            history.push(map.clone());
        }
    };
    (start_point, history)
}

pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    let total_cycles = 1000000000;
    let map = Map::new(input);
    let (start, history) = find_start_of_loop(map);
    let point_in_cycle = (total_cycles - start).rem_euclid(history[start..].len());
    history[start + point_in_cycle].load()
}

#[cfg(test)]
mod test {
    extern crate std;

    use super::part_2;

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

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 64);
    }
}
