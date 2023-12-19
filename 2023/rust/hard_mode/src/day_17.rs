use crate::{bucket_queue::BucketQueue, mem::Mem};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn cw(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn ccw(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    row: u8,
    col: u8,
    dir: Direction,
    travel_length: u8,
}

impl State {
    pub fn neighbour(&self, map: &Map, direction: Direction) -> Option<Self> {
        let new_row = match direction {
            Direction::Left | Direction::Right => self.row,
            Direction::Up => self.row.checked_sub(1)?,
            Direction::Down => self.row + 1,
        };
        if new_row >= map.rows as u8 {
            return None;
        }
        let new_col = match direction {
            Direction::Up | Direction::Down => self.col,
            Direction::Right => self.col + 1,
            Direction::Left => self.col.checked_sub(1)?,
        };
        if new_col >= map.cols as u8 {
            return None;
        }
        if (new_row == 0) & (new_col == 0) {
            return None;
        }
        Some(Self {
            row: new_row,
            col: new_col,
            dir: direction,
            travel_length: if direction == self.dir {
                self.travel_length + 1
            } else {
                1
            },
        })
    }
}

struct Map<'a> {
    elems: &'a [u8],
    rows: usize,
    cols: usize,
}

impl<'a> Map<'a> {
    fn new<'b: 'a>(input: &'b str, mem: &'b Mem<'b>) -> Option<Self> {
        let cols = input.lines().next()?.chars().count();
        let rows = input.lines().count();
        let elems = mem
            .alloc_slice_from_iter(
                rows * cols,
                input
                    .lines()
                    .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8)),
            )
            .ok()?;
        Some(Self { elems, rows, cols })
    }

    fn score(&self, row: usize, col: usize) -> Option<&u8> {
        self.elems.get(row * self.cols + col)
    }
}

#[derive(Debug, PartialEq)]
struct Scores<'a> {
    scores: &'a mut [Option<u32>],
    row_unit: usize,
    col_unit: usize,
    dir_unit: usize,
    min_dist: u8,
}

impl<'a> Scores<'a> {
    pub fn new<'b: 'a>(map: &Map, min_distance: u8, max_distance: u8, mem: &'b Mem<'b>) -> Self {
        let dir_unit = (max_distance - min_distance + 1) as usize;
        let col_unit = dir_unit * 4;
        let row_unit = col_unit * map.cols;
        let scores = mem.alloc_slice(row_unit * map.rows, |_| None).unwrap();
        Self {
            scores,
            row_unit,
            col_unit,
            dir_unit,
            min_dist: min_distance,
        }
    }

    #[inline]
    fn index(&self, state: &State) -> Option<usize> {
        Some(
            state.row as usize * self.row_unit
                + state.col as usize * self.col_unit
                + state.dir as usize * self.dir_unit
                + (state.travel_length.checked_sub(self.min_dist))? as usize,
        )
    }

    pub fn get(&self, state: &State) -> Option<u32> {
        match self.scores.get(self.index(state)?) {
            Some(x) => *x,
            None => todo!(),
        }
    }

    pub fn insert(&mut self, state: &State, score: u32) -> Option<()> {
        if let Some(i) = self.index(state) {
            self.scores[i] = Some(score);
            Some(())
        } else {
            None
        }
    }
}

type Queue<'m> = BucketQueue<'m, 40000, 10000, (State, u32)>;

fn find_shortest_path_length<'m, const MIN_DISTANCE: u8, const MAX_DISTANCE: u8, F>(
    map: &Map,
    mem: &'m Mem<'m>,
    add_neighbours: F,
) -> usize
where
    F: Fn(State, u32, u8, u8, &Map, &mut Queue, &mut Scores),
{
    let mut queue: Queue = BucketQueue::new(mem).unwrap();
    queue
        .insert(
            (
                State {
                    row: 0,
                    col: 0,
                    dir: Direction::Right,
                    travel_length: 0,
                },
                0,
            ),
            0,
        )
        .unwrap();
    queue
        .insert(
            (
                State {
                    row: 0,
                    col: 0,
                    dir: Direction::Down,
                    travel_length: 0,
                },
                0,
            ),
            0,
        )
        .unwrap();
    let mut scores = Scores::new(map, MIN_DISTANCE, MAX_DISTANCE, mem);
    let mut end_score: Option<usize> = None;

    while let Some((current, current_score)) = queue.pop() {
        if (current.row == map.rows as u8 - 1) & (current.col == map.cols as u8 - 1) {
            end_score = Some(current_score as usize);
            break;
        }

        add_neighbours(
            current,
            current_score,
            MIN_DISTANCE,
            MAX_DISTANCE,
            map,
            &mut queue,
            &mut scores,
        );
    }
    assert_eq!(scores.scores.iter().any(|x| x.is_some()), true);
    // assert_eq!(Some(scores), None);
    end_score.unwrap()
}

fn add_neighbours(
    state: State,
    current_score: u32,
    min_distance: u8,
    max_distance: u8,
    map: &Map,
    queue: &mut Queue,
    scores: &mut Scores,
) {
    let mut and_do = |neighbour: State| -> Option<()> {
        let new_score =
            *map.score(neighbour.row as usize, neighbour.col as usize)? as u32 + current_score;
        if scores
            .get(&neighbour)
            .map(|s| s > new_score)
            .unwrap_or(true)
        {
            scores.insert(&neighbour, new_score);
            queue
                .insert((neighbour, new_score), new_score as usize)
                .unwrap();
        }
        None
    };
    if state.travel_length >= min_distance {
        if let Some(s) = state.neighbour(map, state.dir.ccw()) {
            and_do(s);
        }
        if let Some(s) = state.neighbour(map, state.dir.cw()) {
            and_do(s);
        }
    };
    if state.travel_length < max_distance {
        if let Some(s) = state.neighbour(map, state.dir) {
            and_do(s);
        }
    }
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let map = Map::new(input, &mem).unwrap();
    find_shortest_path_length::<1, 3, _>(&map, &mem, add_neighbours)
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let map = Map::new(input, &mem).unwrap();
    find_shortest_path_length::<4, 10, _>(&map, &mem, add_neighbours)
}

#[cfg(test)]
mod test {
    extern crate alloc;

    use core::iter::repeat;

    use alloc::vec::Vec;

    use super::{part_1, part_2};
    const TEST_DATA: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const TEST_DATA_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    // #[test]
    // fn part_1_works() {
    //     let mut buffer = repeat(0u8).take(1_000_000_0).collect::<Vec<_>>();
    //     assert_eq!(part_1(TEST_DATA, &mut buffer), 102);
    // }
    // #[test]
    // fn part_2_works() {
    //     let mut buffer = repeat(0u8).take(1_000_000_0).collect::<Vec<_>>();
    //     assert_eq!(part_2(TEST_DATA, &mut buffer), 94);
    //     assert_eq!(part_2(TEST_DATA_2, &mut buffer), 71);
    // }
}
