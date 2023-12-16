use crate::{growable::Growable, mem::Mem};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_position(
        &self,
        position: (usize, usize),
        rows: usize,
        cols: usize,
    ) -> Option<(usize, usize)> {
        let (row, col) = position;
        match self {
            Direction::North => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Direction::East => {
                if col == cols - 1 {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
            Direction::South => {
                if row == rows - 1 {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
            Direction::West => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
        }
    }

    #[inline]
    fn bit(&self) -> u8 {
        match self {
            Direction::North => 1 << 0,
            Direction::East => 1 << 1,
            Direction::South => 1 << 2,
            Direction::West => 1 << 3,
        }
    }
}

struct LightPather<'a> {
    light: &'a mut [u8],
    cols: usize,
    rows: usize,
}

const NorthSouth: u8 = 1 << 4;
const EastWest: u8 = 1 << 5;
const NESW: u8 = 1 << 6;
const NWSE: u8 = 1 << 7;

impl<'a> LightPather<'a> {
    pub fn new(input: &'a str, mem: &'a Mem) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();
        let vals = mem
            .alloc_slice_from_iter(
                rows * cols,
                input.lines().flat_map(|l| {
                    l.chars().map(|c| match c {
                        '|' => NorthSouth,
                        '-' => EastWest,
                        '\\' => NWSE,
                        '/' => NESW,
                        _ => 0,
                    })
                }),
            )
            .unwrap();
        Self {
            light: vals,
            cols,
            rows,
        }
    }

    fn reset(&mut self) {
        self.light.iter_mut().for_each(|l| *l &= !((1 << 4) - 1))
    }

    fn coord(&mut self, row: usize, col: usize, dir: Direction) -> (bool, u8) {
        let pos = row * self.cols + col;
        let already_seen = self.light[pos] & dir.bit() != 0;
        self.light[pos] |= dir.bit();
        (already_seen, self.light[pos])
    }

    pub fn add_beam<const N: usize>(
        &mut self,
        start: (usize, usize),
        dir: Direction,
        queue: &mut Growable<((usize, usize), Direction), N>,
    ) {
        queue.push((start, dir));

        while let Some((mut pos, mut dir)) = queue.pop() {
            loop {
                let (already_seen, b) = self.coord(pos.0, pos.1, dir);

                if already_seen {
                    break;
                } else {
                    if ((b & EastWest) != 0) & matches!(dir, Direction::North | Direction::South) {
                        if let Some(p) = Direction::East.next_position(pos, self.rows, self.cols) {
                            queue.push((p, Direction::East));
                        }
                        dir = Direction::West
                    } else if ((b & NorthSouth) != 0)
                        & matches!(dir, Direction::East | Direction::West)
                    {
                        if let Some(p) = Direction::North.next_position(pos, self.rows, self.cols) {
                            queue.push((p, Direction::North));
                        }
                        dir = Direction::South
                    } else if (b & NESW) != 0 {
                        dir = match dir {
                            Direction::North => Direction::East,
                            Direction::East => Direction::North,
                            Direction::South => Direction::West,
                            Direction::West => Direction::South,
                        }
                    } else if (b & NWSE) != 0 {
                        dir = match dir {
                            Direction::North => Direction::West,
                            Direction::East => Direction::South,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                        }
                    }
                    if let Some(np) = dir.next_position(pos, self.rows, self.cols) {
                        pos = np;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn energisation(&self) -> usize {
        self.light
            .iter()
            .filter(|x| *x & ((1 << 4) - 1) != 0)
            .count()
    }
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let mut lp = LightPather::new(input, &mem);
    let mut queue: Growable<_, 1000> = mem.alloc_growable().unwrap();
    lp.add_beam((0, 0), Direction::East, &mut queue);
    lp.energisation()
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let mut lp = LightPather::new(input, &mem);
    let mut queue: Growable<_, 1000> = mem.alloc_growable().unwrap();
    let cols = lp.cols;
    let rows = lp.rows;
    (0..rows)
        .flat_map(|r| {
            [(0, Direction::East), (cols - 1, Direction::West)]
                .into_iter()
                .map(move |(c, d)| (r, c, d))
        })
        .chain((0..cols).flat_map(|c| {
            [(0, Direction::South), (rows - 1, Direction::North)]
                .into_iter()
                .map(move |(r, d)| (r, c, d))
        }))
        .map(move |(r, c, d)| {
            lp.reset();
            lp.add_beam((r, c), d, &mut queue);
            lp.energisation()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT, &mut [0u8; 100000]), 46);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut [0u8; 1000000]), 51);
    }
}
