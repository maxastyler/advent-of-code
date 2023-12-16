#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_position(&self, position: (i64, i64)) -> (i64, i64) {
        let (row, col) = position;
        match self {
            Direction::North => (row - 1, col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
        }
    }
}

struct LightPath {
    light: Vec<u8>,
    cols: usize,
    rows: usize,
}

impl LightPath {
    fn in_bounds(&self, pos: (i64, i64)) -> bool {
        let (row, col) = pos;
        (row >= 0) & (row < self.rows as i64) & (col >= 0) & (col < self.cols as i64)
    }

    pub fn coord(&self, row: i64, col: i64, dir: Direction) -> bool {
        let pos = row * self.cols as i64 + col;
        self.light.get(pos as usize).map_or(false, |x| {
            x & (1
                << match dir {
                    Direction::North => 0,
                    Direction::East => 1,
                    Direction::South => 2,
                    Direction::West => 3,
                })
                != 0
        })
    }

    pub fn new<'a, 'b: 'a>(map: &'b Map<'a>) -> Self {
        Self {
            light: vec![0; map.rows * map.cols],
            cols: map.cols,
            rows: map.rows,
        }
    }

    pub fn add_beam(&mut self, map: &Map, start: (i64, i64), dir: Direction) {
        let mut queue: Vec<((i64, i64), Direction)> = vec![(start, dir)];
        while let Some((mut pos, mut dir)) = queue.pop() {
            let mut path = vec![];
            while self.in_bounds(pos) & !self.coord(pos.0, pos.1, dir) {
                path.push((pos, dir));
                match (map.coord(pos.0 as usize, pos.1 as usize), dir) {
                    ('-', Direction::North | Direction::South) => {
                        queue.push((Direction::East.next_position(pos), Direction::East));
                        dir = Direction::West
                    }
                    ('|', Direction::East | Direction::West) => {
                        queue.push((Direction::North.next_position(pos), Direction::North));
                        dir = Direction::South
                    }
                    ('/', _) => match dir {
                        Direction::North => dir = Direction::East,
                        Direction::East => dir = Direction::North,
                        Direction::South => dir = Direction::West,
                        Direction::West => dir = Direction::South,
                    },
                    ('\\', _) => match dir {
                        Direction::North => dir = Direction::West,
                        Direction::East => dir = Direction::South,
                        Direction::South => dir = Direction::East,
                        Direction::West => dir = Direction::North,
                    },
                    _ => {}
                }
                pos = dir.next_position(pos);
            }
            self.add_path(path);
        }
    }

    /// Add another light path to this
    pub fn add_path(&mut self, path: Vec<((i64, i64), Direction)>) {
        for ((row, col), dir) in path {
            self.light[row as usize * self.cols + col as usize] |= 1
                << match dir {
                    Direction::North => 0,
                    Direction::East => 1,
                    Direction::South => 2,
                    Direction::West => 3,
                }
        }
    }

    pub fn energisation(&self) -> usize {
        self.light.iter().filter(|x| **x != 0).count()
    }
}

struct Map<'a> {
    tiles: &'a str,
    cols: usize,
    rows: usize,
}

impl<'a> Map<'a> {
    pub fn new(input: &'a str) -> Self {
        let cols = input.lines().next().unwrap().len();
        let rows = input.lines().count();
        Self {
            tiles: input,
            cols,
            rows,
        }
    }

    pub fn coord(&self, row: usize, col: usize) -> char {
        let pos = row * (self.cols + 1) + col;
        self.tiles[pos..pos + 1].chars().next().unwrap()
    }
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    let mut lp = LightPath::new(&map);
    lp.add_beam(&map, (0, 0), Direction::East);
    lp.energisation()
}
pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    (0..map.rows)
        .flat_map(|r| {
            [(0, Direction::East), (map.cols - 1, Direction::West)]
                .into_iter()
                .map(move |(c, d)| (r, c, d))
        })
        .chain((0..map.cols).flat_map(|c| {
            [(0, Direction::South), (map.rows - 1, Direction::North)]
                .into_iter()
                .map(move |(r, d)| (r, c, d))
        }))
        .map(|(r, c, d)| {
            let mut lp = LightPath::new(&map);
            lp.add_beam(&map, (r as i64, c as i64), d);
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
        assert_eq!(part_1(TEST_INPUT, &mut []), 46);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT, &mut []), 51);
    }
}
