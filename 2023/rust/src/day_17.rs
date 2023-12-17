use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct MinHeapWrapper<T> {
    inner: T,
    key: Reverse<usize>,
}

impl<T> MinHeapWrapper<T> {
    fn new(value: T, key: usize) -> Self {
        Self {
            inner: value,
            key: Reverse(key),
        }
    }
}

impl<T> Ord for MinHeapWrapper<T> where T: Eq
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

struct MinHeap<T> {
    inner: BinaryHeap<MinHeapWrapper<T>>,
}

impl<T> MinHeap<T>

{
    fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }
    fn insert(&mut self, value: T, key: usize) {
        self.inner.push(MinHeapWrapper::new(value, key))
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|x| x.inner)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
struct Direction {
    inner: u8,
}

impl Direction {
    fn neighbours(&self, straight_line_length: u8) -> Vec<(Self, u8)> {
        let lower = if self.inner == 0 {
            Self { inner: 3 }
        } else {
            Self {
                inner: self.inner - 1,
            }
        };
        let upper = if self.inner == 3 {
            Self { inner: 0 }
        } else {
            Self {
                inner: self.inner + 1,
            }
        };
        if straight_line_length >= 3 {
            vec![(lower, 1), (upper, 1)]
        } else {
            vec![
                (lower, 1),
                (upper, 1),
                (self.clone(), straight_line_length + 1),
            ]
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        Self { inner: value }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// A single state
struct State {
    row: i64,
    col: i64,
    dir: u8,
    straight_len: u8,
}

impl State {
    fn translate(&self, dir: u8) -> Self {
        let new_row = self.row
            + (match dir {
                0 | 2 => 0,
                1 => -1,
                _ => 1,
            });
        let new_col = self.col
            + (match dir {
                1 | 3 => 0,
                0 => 1,
                _ => -1,
            });
        Self {
            row: new_row,
            col: new_col,
            dir,
            straight_len: if dir == self.dir {
                self.straight_len + 1
            } else {
                1
            },
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        let lower = if self.dir == 0 { 3 } else { self.dir - 1 };
        let upper = if self.dir == 3 { 0 } else { self.dir + 1 };
        [
            self.translate(lower),
            self.translate(upper),
            self.translate(self.dir),
        ]
        .into_iter()
        .filter(|s| ((s.row != 0) | (s.col != 0)) & (s.straight_len <= 3))
        .collect()
    }
}

// the graph of costs needs the history of paths too

fn find_shortest_path(map: &Map, end: (i64, i64)) -> usize {
    let h = move |state: &State| (state.row.abs_diff(end.0) + state.col.abs_diff(end.1)) as usize;
    let current = State {
        row: 0,
        col: 0,
        dir: 0,
        straight_len: 0,
    };
    // hashmap indexed by (row, col, direction, length in same direction)
    let mut g_score: HashMap<State, usize> = HashMap::new();
    g_score.insert(current.clone(), 0);
    let mut prev: HashMap<State, State> = HashMap::new();

    let mut queue: MinHeap<State> = MinHeap::new();
    queue.
    let mut current = current.clone();

    while queue.len() > 0 {
        current = queue.pop()
        if (current.row == end.0) & (current.col == end.1) {
            break;
        }
        let current_g_score = g_score[&current];
        for n in current.neighbours() {
            if !((n.row < 0)
                | (n.row >= map.rows as i64)
                | (n.col < 0)
                | (n.col >= map.cols as i64))
            {
                let tentative_score =
                    current_g_score + map.coord(n.row as usize, n.col as usize).unwrap() as usize;
                if let Some(score) = g_score.get(&n) {
                    if tentative_score < *score {
                        prev.insert(n.clone(), current.clone());
                        g_score.insert(n.clone(), tentative_score);
                        if !queue.contains(&n) {
                            queue.push(n);
                        }
                    }
                } else {
                    prev.insert(n.clone(), current.clone());
                    g_score.insert(n.clone(), tentative_score);
                    if !queue.contains(&n) {
                        queue.push(n);
                    }
                }
            }
        }
    }
    g_score[&current]
}

pub fn part_1(input: &str, _buffer: &mut [u8]) -> usize {
    let map = Map::new(input);
    find_shortest_path(&map, ((map.rows - 1) as i64, (map.cols - 1) as i64))
}
pub fn part_2(input: &str, _buffer: &mut [u8]) -> usize {
    3
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

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA, &mut []), 102);
    }
    #[test]
    fn part_2_works() {}
}
