use std::collections::HashMap;

use hard_mode::fnv_hasher::BuildFNVHasher;

use crate::min_heap::MinHeap;

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

    fn add_neighbours(
        &self,
        min_turn_distance: u8,
        max_straight_distance: u8,
        current_score: usize,
        map: &Map,
        g_score: &mut ScoreMap,
        queue: &mut MinHeap<(Self, usize)>,
    ) {
        let mut maybe_add_state = |s: State| {
            if s.straight_len <= max_straight_distance {
                let new_score = current_score + map.coord(s.row, s.col).unwrap() as usize;
                if g_score.get(&s).map(|s| new_score < *s).unwrap_or(true) {
                    g_score.insert(s.clone(), new_score);
                    queue.insert((s, new_score), new_score);
                }
            }
        };

        if self.straight_len >= min_turn_distance {
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
    F: Fn(State, usize, &Map, &mut ScoreMap, &mut MinHeap<(State, usize)>),
    G: Fn(&State) -> bool,
{
    let mut g_score: ScoreMap = HashMap::with_hasher(BuildFNVHasher);

    let mut queue: MinHeap<(State, usize)> = MinHeap::new();

    for dir in [0, 3] {
        let current = State {
            row: 0,
            col: 0,
            dir,
            straight_len: 0,
        };
        queue.insert((current.clone(), 0), 0);
        g_score.insert(current.clone(), 0);
    }

    let mut result = None;
    while let Some((current, current_g_score)) = queue.pop() {
        if end_fun(&current) {
            result = Some(current_g_score);
            break;
        }

        neighbour_fun(current, current_g_score, map, &mut g_score, &mut queue);
    }
    let end_score = result.unwrap();
    end_score
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    find_shortest_path(
        &map,
        |s, sc, m, sm, mh| s.add_neighbours(1, 3, sc, m, sm, mh),
        |s| (s.row == map.rows - 1) & (s.col == map.cols - 1),
    )
}
pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    find_shortest_path(
        &map,
        |s, sc, m, sm, mh| s.add_neighbours(4, 10, sc, m, sm, mh),
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
