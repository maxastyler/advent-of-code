/// Start inclusive, End exclusive
#[derive(Debug, Copy, Clone, PartialEq)]
struct Range(usize, usize);

#[derive(Debug, PartialEq)]
enum RangeSplit {
    Engulfed,
    Untouched(Range),
    One(Range),
    Two(Range, Range),
}

impl Range {
    fn len(&self) -> usize {
        self.1 - self.0
    }

    fn intersect(&self, other: &Self) -> RangeSplit {
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
#[derive(Debug, Clone, PartialEq)]
struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn new(initial: Range) -> Self {
        Self {
            ranges: vec![initial],
        }
    }

    fn intersect(&mut self, other: &Range) -> &mut Self {
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
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod test {
    use crate::day_19::RangeSplit;

    use super::{Range, Ranges};

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
}
