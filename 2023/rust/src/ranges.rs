use std::fmt::Debug;

/// Start inclusive, End exclusive
#[derive(Copy, Clone, PartialEq)]
pub struct Range(pub usize, pub usize);

#[derive(Debug, PartialEq)]
pub enum RangeSplit {
    Engulfed,
    Untouched(Range),
    One(Range),
    Two(Range, Range),
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?}-{:?})", self.0, self.1))
    }
}

impl Range {
    pub fn len(&self) -> usize {
        self.1 - self.0
    }

    pub fn intersect(&self, other: &Self) -> RangeSplit {
        if other.0 <= self.0 {
            if other.1 <= self.0 {
                RangeSplit::Untouched(*self)
            } else {
                if other.1 < self.1 {
                    RangeSplit::One(Range(other.1, self.1))
                } else {
                    RangeSplit::Engulfed
                }
            }
        } else {
            if other.0 >= self.1 {
                RangeSplit::Untouched(*self)
            } else {
                if other.1 < self.1 {
                    RangeSplit::Two(Range(self.0, other.0), Range(other.1, self.1))
                } else {
                    RangeSplit::One(Range(self.0, other.0))
                }
            }
        }
    }
}

/// A set of ranges, ordered
#[derive(Clone, PartialEq)]
pub struct Ranges {
    pub ranges: Vec<Range>,
}

impl Debug for Ranges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.ranges
            .iter()
            .map(|r| Some(r))
            .intersperse(None)
            .for_each(|r| match r {
                Some(i) => f.write_fmt(format_args!("{:?}", i)).unwrap(),
                None => f.write_str(", ").unwrap(),
            });
        f.write_str("]")
    }
}

impl Ranges {
    pub fn new(initial: Range) -> Self {
        Self {
            ranges: vec![initial],
        }
    }

    pub fn intersect(&mut self, other: &Range) -> &mut Self {
        let mut untouched_before = false;
        for index in (0..self.ranges.len()).rev() {
            match self.ranges[index].intersect(other) {
                RangeSplit::Engulfed => {
                    self.ranges.remove(index);
                }
                RangeSplit::Untouched(_) => {
                    if untouched_before {
                        break;
                    } else {
                        untouched_before = true;
                    }
                }
                RangeSplit::One(r) => {
                    self.ranges[index] = r;
                }
                RangeSplit::Two(r1, r2) => {
                    self.ranges[index] = r2;
                    self.ranges.insert(index, r1);
                    break;
                }
            }
        }
        self
    }

    pub fn combine(&mut self, mut other: Ranges) -> &mut Self {
        if self.ranges.is_empty() {
            self.ranges = other.ranges;
        } else {
            let mut this_index = self.ranges.len() - 1;
            'outer: while let Some(other_range) = other.ranges.pop() {
                // move down this_index until finding a range with range.1 > other_range.0
                while self.ranges[this_index].0 > other_range.1 {
                    if this_index == 0 {
                        self.ranges.insert(0, other_range);
                        // We've got to the bottom of self's ranges, add other range to it
                        break 'outer;
                    } else {
                        this_index -= 1;
                    }
                }
                let max_point = self.ranges[this_index].1.max(other_range.1);
                let max_index = this_index + 1;
                loop {
                    if this_index == 0 || self.ranges[this_index - 1].1 < other_range.0 {
                        // this_index is the last range that overlaps
                        let min_point = self.ranges[this_index].0.min(other_range.0);
                        let mut index = 0;
                        self.ranges[this_index] = Range(min_point, max_point);
                        let delete_range = (this_index + 1)..max_index;

                        self.ranges.retain(|_| {
                            let res = !delete_range.contains(&index);
                            index += 1;
                            res
                        });

                        continue 'outer;
                    }
                    this_index -= 1;
                }
            }
        }
        self
    }

    pub fn max(&self) -> Option<usize> {
        self.ranges.last().map(|l| l.1)
    }
    pub fn min(&self) -> Option<usize> {
        self.ranges.first().map(|f| f.0)
    }

    pub fn len(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }
}
#[cfg(test)]
mod test {
    use super::{Range, RangeSplit, Ranges};

    #[test]
    fn range_works() {
        let r = Range(5, 10);
        assert_eq!(r.intersect(&Range(0, 4)), RangeSplit::Untouched(r));
        assert_eq!(r.intersect(&Range(0, 5)), RangeSplit::Untouched(r));
        assert_eq!(r.intersect(&Range(11, 12)), RangeSplit::Untouched(r));
        assert_eq!(r.intersect(&Range(1, 11)), RangeSplit::Engulfed);
        assert_eq!(r.intersect(&Range(1, 6)), RangeSplit::One(Range(6, 10)));
        assert_eq!(r.intersect(&Range(1, 10)), RangeSplit::Engulfed);
        assert_eq!(r.intersect(&Range(1, 9)), RangeSplit::One(Range(9, 10)));
        assert_eq!(
            r.intersect(&Range(10, 13)),
            RangeSplit::Untouched(Range(5, 10))
        );
        assert_eq!(r.intersect(&Range(5, 10)), RangeSplit::Engulfed);
        assert_eq!(r.intersect(&Range(6, 10)), RangeSplit::One(Range(5, 6)));
        assert_eq!(
            r.intersect(&Range(6, 9)),
            RangeSplit::Two(Range(5, 6), Range(9, 10))
        );
    }

    #[test]
    fn ranges_works() {
        let mut r = Ranges::new(Range(4, 10));
        assert_eq!(*r.clone().intersect(&Range(0, 1)), r);
        assert_eq!(
            *r.clone().intersect(&Range(4, 9)),
            Ranges::new(Range(9, 10))
        );
        assert_eq!(
            *r.clone().intersect(&Range(5, 7)).intersect(&Range(8, 9)),
            Ranges {
                ranges: vec![Range(4, 5), Range(7, 8), Range(9, 10)]
            }
        );
    }

    #[test]
    fn test_ranges_combining() {
        let mut r = Ranges::new(Range(4, 10));
        assert_eq!(
            *r.combine(Ranges::new(Range(8, 12))),
            Ranges::new(Range(4, 12))
        );

        assert_eq!(
            *r.combine(Ranges::new(Range(1, 2))),
            Ranges {
                ranges: vec![Range(1, 2), Range(4, 12)]
            }
        );
        assert_eq!(
            *r.combine(Ranges::new(Range(2, 4))),
            Ranges {
                ranges: vec![Range(1, 12)]
            }
        );
        assert_eq!(
            *Ranges {
                ranges: vec![Range(1, 3), Range(4, 6), Range(7, 10), Range(11, 12)]
            }
            .combine(Ranges::new(Range(5, 11))),
            Ranges {
                ranges: vec![Range(1, 3), Range(4, 12)]
            }
        );
    }
}
