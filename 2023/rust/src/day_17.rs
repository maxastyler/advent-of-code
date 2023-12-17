use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

use crate::min_heap::MinHeap;

struct FNVHasher {
    val: u64,
}

impl FNVHasher {
    pub fn new() -> Self {
        Self {
            val: 14695981039346656037,
        }
    }
}

impl Hasher for FNVHasher {
    fn finish(&self) -> u64 {
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|b| {
            self.val = self.val.wrapping_mul(1099511628211);
            self.val ^= *b as u64;
        })
    }
}

struct BuildFNVHasher;

impl BuildHasher for BuildFNVHasher {
    type Hasher = FNVHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::new()
    }
}

struct Map {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let data = input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();
        Self { data, rows, cols }
    }

    pub fn coord(&self, row: usize, col: usize) -> Option<u8> {
        if (row < self.rows) & (col < self.cols) {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }
}

type ScoreMap = HashMap<State, usize, BuildFNVHasher>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// A single state
struct State {
    row: usize,
    col: usize,
    dir: u8,
    straight_len: u8,
}

impl State {
    fn translate(&self, dir: u8, map: &Map) -> Option<Self> {
        let new_row = match dir {
            0 | 2 => self.row,
            1 => self.row.checked_sub(1)?,
            _ => self.row + 1,
        };
        if new_row >= map.rows {
            return None;
        }
        let new_col = match dir {
            1 | 3 => self.col,
            0 => self.col + 1,
            _ => self.col.checked_sub(1)?,
        };
        if new_col >= map.cols {
            return None;
        }
        Some(Self {
            row: new_row,
            col: new_col,
            dir,
            straight_len: if dir == self.dir {
                self.straight_len + 1
            } else {
                1
            },
        })
    }

    fn add_neighbours_p1(
        &self,
        current_score: usize,
        map: &Map,
        g_score: &mut ScoreMap,
        queue: &mut MinHeap<(Self, usize)>,
        prevs: &mut HashMap<State, State, BuildFNVHasher>,
    ) {
        let mut maybe_add_state = |s: State| {
            if s.straight_len <= 3 {
                let new_score = current_score + map.coord(s.row, s.col).unwrap() as usize;
                if g_score.get(&s).map(|s| new_score < *s).unwrap_or(true) {
                    g_score.insert(s.clone(), new_score);
                    queue.insert((s, new_score), new_score);
                }
            }
        };

        let lower = if self.dir == 0 { 3 } else { self.dir - 1 };
        self.translate(lower, map).map(&mut maybe_add_state);
        let upper = if self.dir == 3 { 0 } else { self.dir + 1 };
        self.translate(upper, map).map(&mut maybe_add_state);
        self.translate(self.dir, map).map(&mut maybe_add_state);
    }

    fn add_neighbours_p2(
        &self,
        current_score: usize,
        map: &Map,
        g_score: &mut ScoreMap,
        queue: &mut MinHeap<(Self, usize)>,
        prevs: &mut HashMap<State, State, BuildFNVHasher>,
    ) {
        let mut maybe_add_state = |s: State| {
            if s.straight_len <= 10 {
                let new_score = current_score + map.coord(s.row, s.col).unwrap() as usize;
                if g_score.get(&s).map(|s| new_score < *s).unwrap_or(true) {
                    // prevs.insert(s.clone(), self.clone());
                    g_score.insert(s.clone(), new_score);
                    queue.insert((s, new_score), new_score);
                }
            }
        };

        if self.straight_len >= 4 {
            let lower = if self.dir == 0 { 3 } else { self.dir - 1 };
            self.translate(lower, map).map(&mut maybe_add_state);
            let upper = if self.dir == 3 { 0 } else { self.dir + 1 };
            self.translate(upper, map).map(&mut maybe_add_state);
        }
        self.translate(self.dir, map).map(&mut maybe_add_state);
    }
}

fn find_shortest_path<F, G>(map: &Map, neighbour_fun: F, end_fun: G) -> usize
where
    F: Fn(
        State,
        usize,
        &Map,
        &mut ScoreMap,
        &mut MinHeap<(State, usize)>,
        &mut HashMap<State, State, BuildFNVHasher>,
    ),
    G: Fn(&State) -> bool,
{
    let current = State {
        row: 0,
        col: 0,
        dir: 0,
        straight_len: 0,
    };

    let mut prevs: HashMap<State, State, BuildFNVHasher> = HashMap::with_hasher(BuildFNVHasher);
    let mut g_score: ScoreMap = HashMap::with_hasher(BuildFNVHasher);
    g_score.insert(current.clone(), 0);
    let mut queue: MinHeap<(State, usize)> = MinHeap::new();
    queue.insert((current, 0), 0);
    let mut result = None;

    while let Some((current, current_g_score)) = queue.pop() {
        if end_fun(&current) {
            result = Some((current, current_g_score));
            break;
        }

        neighbour_fun(
            current,
            current_g_score,
            map,
            &mut g_score,
            &mut queue,
            &mut prevs,
        );
    }
    let (end, end_score) = result.unwrap();
    // println!("{:?}", reconstruct_path(end, prevs));
    end_score
}

pub fn reconstruct_path<S: BuildHasher>(
    mut current: State,
    prevs: HashMap<State, State, S>,
) -> Vec<(usize, usize)> {
    let mut path = vec![(current.row, current.col)];
    while let Some(s) = prevs.get(&current) {
        path.push((s.row, s.col));
        current = s.clone();
    }
    path.reverse();
    path
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    find_shortest_path(
        &map,
        |s, sc, m, sm, mh, p| s.add_neighbours_p1(sc, m, sm, mh, p),
        |s| (s.row == map.rows - 1) & (s.col == map.cols - 1),
    )
}
pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    find_shortest_path(
        &map,
        |s, sc, m, sm, mh, p| s.add_neighbours_p2(sc, m, sm, mh, p),
        |s| (s.row == map.rows - 1) & (s.col == map.cols - 1) & (s.straight_len >= 4),
    )
}
#[cfg(test)]
mod test {
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

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA, &mut []), 102);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_DATA, &mut []), 94);
        assert_eq!(part_2(TEST_DATA_2, &mut []), 71);
    }
}
