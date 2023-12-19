use std::{fmt::Debug, time::Instant};

use hard_mode::{
    day_19::{Property, Rule, Rules},
    mem::Mem,
};

use crate::ranges::{Range, Ranges};

#[derive(Clone, PartialEq)]
struct PartRanges {
    x: Ranges,
    m: Ranges,
    a: Ranges,
    s: Ranges,
}

impl Debug for PartRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{\n x:{:?}\n m:{:?}\n a:{:?}\n s:{:?}\n}}",
            self.x, self.m, self.a, self.s
        ))
    }
}

impl PartRanges {
    fn new(lower: usize, upper: usize) -> Self {
        let r = Range(lower, upper);
        Self {
            x: Ranges::new(r),
            m: Ranges::new(r),
            a: Ranges::new(r),
            s: Ranges::new(r),
        }
    }
    fn new_empty() -> Self {
        Self {
            x: Ranges { ranges: vec![] },
            m: Ranges { ranges: vec![] },
            a: Ranges { ranges: vec![] },
            s: Ranges { ranges: vec![] },
        }
    }

    fn combine(&mut self, other: PartRanges) -> &mut Self {
        self.x.combine(other.x);
        self.m.combine(other.m);
        self.a.combine(other.a);
        self.s.combine(other.s);
        self
    }

    fn intersect(&mut self, property: Property, range: &Range) -> &mut Self {
        match property {
            Property::X => self.x.intersect(range),
            Property::M => self.m.intersect(range),
            Property::A => self.a.intersect(range),
            Property::S => self.s.intersect(range),
        };
        self
    }

    /// Split at the given number on the given property, returning a tuple
    /// (self lower, self higher )
    /// lower ends on property (exclusive) higher starts on property (inclusive)
    fn split_at(&mut self, property: Property, num: usize) -> (Self, &mut Self) {
        let max_range = Range(
            num,
            match property {
                Property::X => self.x.max().unwrap(),
                Property::M => self.m.max().unwrap(),
                Property::A => self.a.max().unwrap(),
                Property::S => self.s.max().unwrap(),
            },
        );
        let min_range = Range(
            match property {
                Property::X => self.x.min().unwrap(),
                Property::M => self.m.min().unwrap(),
                Property::A => self.a.min().unwrap(),
                Property::S => self.s.min().unwrap(),
            },
            num,
        );
        let mut cloned = self.clone();
        cloned.intersect(property, &max_range);
        (cloned, self.intersect(property, &min_range))
    }

    fn product(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

fn count_exits(rules: Rules) -> usize {
    let mut queue: Vec<(usize, PartRanges)> = vec![(rules.start, PartRanges::new(1, 4001))];
    let mut passed_ranges = 0;
    'next_in_queue: while let Some((mut index, mut ranges)) = queue.pop() {
        loop {
            let target = match rules.rules[index] {
                Rule::GreaterThan(prop, num, target) => {
                    let (lower, _) = ranges.split_at(prop, num as usize + 1);
                    queue.push((index + 1, lower));
                    target
                }
                Rule::LessThan(prop, num, target) => {
                    let (lower, _) = ranges.split_at(prop, num as usize);
                    queue.push((index + 1, ranges));
                    ranges = lower;
                    target
                }
                Rule::Unconditional(target) => target,
            };
            match target {
                hard_mode::day_19::Target::Accept => {
                    passed_ranges += ranges.product();
                    continue 'next_in_queue;
                }
                hard_mode::day_19::Target::Reject => continue 'next_in_queue,
                hard_mode::day_19::Target::Goto(i) => {
                    index = i;
                }
            }
        }
    }
    passed_ranges
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (rules, _) = input.split_once("\n\n").unwrap();
    let rules = Rules::parse_rules(rules, &mem);
    count_exits(rules)
}

#[cfg(test)]
mod test {

    use super::part_2;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_2_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_2(TEST_INPUT, &mut buffer), 167409079868000);
    }
}
