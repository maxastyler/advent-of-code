use crate::mem::Mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    row: u8,
    col: u8,
    dir: Direction,
    travel_length: u8,
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

struct Scores<'a> {
    scores: &'a mut [Option<u32>],
    row_unit: usize,
    col_unit: usize,
    dir_unit: usize,
    min_dist: u8,
}

impl<'a> Scores<'a> {
    pub fn new<'b: 'a>(map: &Map, min_distance: u8, max_distance: u8, mem: &'b Mem<'b>) -> Self {
        let dir_unit = (min_distance - max_distance + 1) as usize;
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
    fn index(&self, state: &State) -> usize {
        state.row as usize * self.row_unit
            + state.col as usize * self.col_unit
            + state.dir as usize * self.dir_unit
            + (state.travel_length - self.min_dist) as usize
    }

    pub fn get(&self, state: &State) -> Option<u32> {
        match self.scores.get(self.index(state)) {
            Some(x) => *x,
            None => todo!(),
        }
    }

    pub fn insert(&mut self, state: &State, score: u32) {
        self.scores[self.index(state)] = Some(score)
    }
}

type Queue = ();

// fn find_shortest_path_length<'m, F, G>(
//     map: &Map,
//     mem: &'m Mem<'m>,
//     end_fun: F,
//     add_neighbours: G,
// ) -> usize
// where
//     F: Fn(&State) -> bool,
//     G: Fn(&State, &Map, &mut Queue, &mut Scores),
// {

// }

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    3
}
pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    3
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1_works() {}
    #[test]
    fn part_2_works() {}
}
