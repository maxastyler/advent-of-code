use core::{ops::ControlFlow, slice::SliceIndex};

use crate::mem::Mem;

type Num = u32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Property {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Property {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone)]
pub enum Target {
    Accept,
    Reject,
    Goto(usize),
}

pub enum Rule {
    GreaterThan(Property, Num, Target),
    LessThan(Property, Num, Target),
    Unconditional(Target),
}

impl Rule {
    fn parse_comp(rule: &str, op: char) -> Option<(Property, Num, &str)> {
        rule.split_once(op).and_then(|(prop, rest)| {
            let (num, target) = rest.split_once(":")?;

            Some((prop.try_into().ok()?, num.parse().ok()?, target))
        })
    }

    fn parse_gt(rule: &str) -> Option<(Property, Num, &str)> {
        Self::parse_comp(rule, '>')
    }

    fn parse_lt(rule: &str) -> Option<(Property, Num, &str)> {
        Self::parse_comp(rule, '<')
    }

    fn parse_target(target: &str, rules: &str) -> Option<Target> {
        Some(match target {
            "A" => Target::Accept,
            "R" => Target::Reject,
            _ => Target::Goto(Self::get_rule_number(target, rules)?),
        })
    }

    pub fn parse(rule: &str, rules: &str) -> Option<Self> {
        if let Some((prop, num, target)) = Self::parse_gt(rule) {
            Some(Self::GreaterThan(
                prop,
                num,
                Self::parse_target(target, rules)?,
            ))
        } else if let Some((prop, num, target)) = Self::parse_lt(rule) {
            Some(Self::LessThan(
                prop,
                num,
                Self::parse_target(target, rules)?,
            ))
        } else {
            Some(Self::Unconditional(Self::parse_target(rule, rules)?))
        }
    }
    fn get_rule_number(rule: &str, rules: &str) -> Option<usize> {
        match rules.lines().try_fold(0usize, |pos, line| {
            let (rule_name, rest) = line.split_once("{").unwrap();
            if rule_name == rule {
                ControlFlow::Break(pos)
            } else {
                ControlFlow::Continue(pos + rest.split(",").count())
            }
        }) {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(r) => Some(r),
        }
    }
}

pub struct Rules<'a> {
    rules: &'a [Rule],
    start: usize,
}

impl<'a> Rules<'a> {
    pub fn parse_rules<'b>(rules: &'b str, mem: &'a Mem<'a>) -> Self {
        let num_rules = rules.lines().map(|l| l.split(",").count()).sum::<usize>();
        let rules_iter = rules.lines().flat_map(|l| {
            l.split_once("{")
                .unwrap()
                .1
                .trim_end_matches("}")
                .split(",")
                .map(|r| Rule::parse(r, rules).unwrap())
        });
        Self {
            rules: mem.alloc_slice_from_iter(num_rules, rules_iter).unwrap(),
            start: Rule::get_rule_number("in", rules).unwrap(),
        }
    }

    pub fn pass_part(&self, part: &Part) -> bool {
        let mut index = self.start;
        loop {
            match self.rules[index] {
                Rule::GreaterThan(prop, num, target) => {
                    if part.get(prop) > num {
                        match target {
                            Target::Accept => return true,
                            Target::Reject => return false,
                            Target::Goto(i) => index = i,
                        };
                    } else {
                        index += 1
                    }
                }
                Rule::LessThan(prop, num, target) => {
                    if part.get(prop) < num {
                        match target {
                            Target::Accept => return true,
                            Target::Reject => return false,
                            Target::Goto(i) => index = i,
                        };
                    } else {
                        index += 1
                    }
                }
                Rule::Unconditional(u) => match u {
                    Target::Accept => return true,
                    Target::Reject => return false,
                    Target::Goto(i) => index = i,
                },
            }
        }
    }
}

struct Part {
    x: Num,
    m: Num,
    a: Num,
    s: Num,
}

impl Part {
    fn parse(part_string: &str) -> Self {
        let mut part_iter = part_string
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .map(|prop_s| prop_s.split_once("=").unwrap().1.parse::<Num>().unwrap());
        Self {
            x: part_iter.next().unwrap(),
            m: part_iter.next().unwrap(),
            a: part_iter.next().unwrap(),
            s: part_iter.next().unwrap(),
        }
    }

    fn get(&self, property: Property) -> Num {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

pub fn part_1(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let rules = Rules::parse_rules(rules, &mem);
    parts
        .lines()
        .filter_map(|p| {
            let part = Part::parse(p);
            if rules.pass_part(&part) {
                Some(part.sum())
            } else {
                None
            }
        })
        .sum::<u32>() as usize
}

pub fn part_2(input: &str, buffer: &mut [u8]) -> usize {
    let mem = Mem::new(buffer);
    let (rules, _) = input.split_once("\n\n").unwrap();
    let rules = Rules::parse_rules(rules, &mem);
    let limit = 300;
    (0..limit)
        .flat_map(|x| {
            (0..limit).flat_map(move |m| {
                (0..limit).flat_map(move |a| (0..limit).map(move |s| Part { x, m, a, s }))
            })
        })
        .filter(|p| rules.pass_part(p))
        .count()
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};
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
    fn part_1_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_1(TEST_INPUT, &mut buffer), 3);
    }

    #[test]
    fn part_2_works() {
        let mut buffer = [0u8; 1000];
        assert_eq!(part_2(TEST_INPUT, &mut buffer), 3);
    }
}
