use crate::mem::Mem;

#[derive(Copy, PartialEq, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn next_dir(&self, pipe: char) -> Option<Self> {
        use Dir as D;
        match (*self, pipe) {
            (D::North, '|') => Some(D::North),
            (D::North, '7') => Some(D::West),
            (D::North, 'F') => Some(D::East),
            (D::South, '|') => Some(D::South),
            (D::South, 'L') => Some(D::East),
            (D::South, 'J') => Some(D::West),
            (D::East, '-') => Some(D::East),
            (D::East, '7') => Some(D::South),
            (D::East, 'J') => Some(D::North),
            (D::West, '-') => Some(D::West),
            (D::West, 'L') => Some(D::North),
            (D::West, 'F') => Some(D::South),
            _ => None,
        }
    }
}

struct Grid<'a> {
    data: &'a str,
    rows: usize,
    columns: usize,
}

type Coord = (usize, usize);

impl<'a> Grid<'a> {
    fn new(data: &'a str) -> Option<Self> {
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
        Some(Self {
            data,
            rows,
            columns: columns?,
        })
    }

    fn coord(&self, coord: Coord) -> Option<char> {
        let (r, c) = coord;
        if (r < self.rows) & (c < self.columns) {
            let index = r * (self.columns + 1) + c;
            self.data[index..index + 1].chars().next()
        } else {
            None
        }
    }

    fn translate(&self, coord: Coord, direction: Dir) -> Option<Coord> {
        let (r, c) = coord;
        match direction {
            Dir::North => r.checked_add_signed(-1).map(|r| (r, c)),
            Dir::South => r.checked_add_signed(1).map_or(None, |r| {
                if r >= self.rows {
                    None
                } else {
                    Some((r, c))
                }
            }),
            Dir::East => c.checked_add_signed(1).map_or(None, |c| {
                if c >= self.columns {
                    None
                } else {
                    Some((r, c))
                }
            }),
            Dir::West => c.checked_add_signed(-1).map(|c| (r, c)),
        }
    }

    fn follow_pipes<'b>(
        &'b self,
        start_pos: Coord,
        start_direction: Dir,
    ) -> impl Iterator<Item = (Coord, Dir)> + 'b {
        [(start_pos, start_direction)].into_iter().chain((0..).scan(
            (start_pos, start_direction),
            |(pos, dir), _| {
                *pos = self.translate(*pos, *dir)?;
                *dir = dir.next_dir(self.coord(*pos)?)?;
                Some((*pos, *dir))
            },
        ))
    }

    /// Follow a path, return a tuple (length, loop)
    /// where loop is a boolean that says whether the path looped before it finished
    fn path_length(&self, start_pos: Coord, start_direction: Dir) -> (usize, bool) {
        let (length, (end_pos, end_dir)) = self
            .follow_pipes(start_pos, start_direction)
            .enumerate()
            .last()
            .unwrap();
        (
            length + 1,
            if let Some(c) = self.translate(end_pos, end_dir) {
                c == start_pos
            } else {
                false
            },
        )
    }

    fn find_position<P>(&self, pred: P) -> Option<Coord>
    where
        P: Fn(char) -> bool,
    {
        self.data
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(column, c)| (row, column, c))
            })
            .find_map(|(row, column, c)| if pred(c) { Some((row, column)) } else { None })
    }
}

fn loop_area(coords: &[Coord]) -> i64 {
    (0..coords.len())
        .map(|i| {
            let index_1 = ((i as i64) - 1).rem_euclid(coords.len() as i64) as usize;
            let index_3 = (i + 1).rem_euclid(coords.len());
            coords[i].1 as i64 * (coords[index_3].0 as i64 - coords[index_1].0 as i64)
        })
        .sum::<i64>()
        / 2
}

fn interior_points(area: i64, loop_length: usize) -> usize {
    area.abs() as usize + 1 - loop_length / 2
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    let grid = Grid::new(input).unwrap();
    let s_pos = grid.find_position(|c| c == 'S').unwrap();
    [Dir::North, Dir::East, Dir::South, Dir::West]
        .into_iter()
        .filter_map(|dir| {
            let (length, loops) = grid.path_length(s_pos, dir);
            if loops {
                Some(length)
            } else {
                None
            }
        })
        .max()
        .unwrap()
        / 2
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let grid = Grid::new(input).unwrap();
    let s_pos = grid.find_position(|c| c == 'S').unwrap();
    let (direction, length) = [Dir::North, Dir::East, Dir::South, Dir::West]
        .into_iter()
        .filter_map(|dir| {
            let (length, loops) = grid.path_length(s_pos, dir);
            if loops {
                Some((dir, length))
            } else {
                None
            }
        })
        .max_by_key(|(_, l)| *l)
        .unwrap();
    let loop_slice = mem.alloc_slice(length, |_| (0, 0)).unwrap();
    grid.follow_pipes(s_pos, direction)
        .zip(loop_slice.iter_mut())
        .for_each(|((pos, _), coord)| *coord = pos);
    interior_points(loop_area(loop_slice), loop_slice.len())
}

#[cfg(test)]
mod test {
    extern crate alloc;


    use super::{part_1, part_2, Dir, Grid};
    const TEST_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_INPUT_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const TEST_INPUT_3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    const TEST_INPUT_4: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn grid_path_length_works() {
        assert_eq!(
            Grid::new(TEST_INPUT_1)
                .unwrap()
                .path_length((1, 1), Dir::East),
            (8, true)
        );
        assert_eq!(
            Grid::new(TEST_INPUT_2)
                .unwrap()
                .path_length((2, 0), Dir::East),
            (16, true)
        );

        // assert_eq!(
        //     grid.follow_pipes((1, 1), Dir::East).collect::<Vec<_>>(),
        //     Vec::new()
        // );
        // assert_eq!(part_1(TEST_INPUT_1, &mut [0u8; 1000]), 2);
        // assert_eq!(part_1(TEST_INPUT_2, &mut [0u8; 1000]), 6);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT_1, &mut [0u8; 0]), 4);
        assert_eq!(part_1(TEST_INPUT_2, &mut [0u8; 0]), 8);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT_1, &mut [0u8; 1000]), 1);
        assert_eq!(part_2(TEST_INPUT_4, &mut [0u8; 1000]), 4);
        assert_eq!(part_2(TEST_INPUT_3, &mut [0u8; 10000]), 10);
    }
}
