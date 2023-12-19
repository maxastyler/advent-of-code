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

#[derive(Copy, Clone, Debug)]
pub enum Target {
    Accept,
    Reject,
    Goto(usize),
}

#[derive(Debug)]
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

    fn parse_target(target: &str, rule_nums: &[(usize, &str)]) -> Option<Target> {
        Some(match target {
            "A" => Target::Accept,
            "R" => Target::Reject,
            _ => Target::Goto(Self::get_rule_number(target, rule_nums)?),
        })
    }

    pub fn parse(rule: &str, rule_nums: &[(usize, &str)]) -> Option<Self> {
        if let Some((prop, num, target)) = Self::parse_gt(rule) {
            Some(Self::GreaterThan(
                prop,
                num,
                Self::parse_target(target, rule_nums)?,
            ))
        } else if let Some((prop, num, target)) = Self::parse_lt(rule) {
            Some(Self::LessThan(
                prop,
                num,
                Self::parse_target(target, rule_nums)?,
            ))
        } else {
            Some(Self::Unconditional(Self::parse_target(rule, rule_nums)?))
        }
    }
    fn get_rule_number(rule: &str, rule_nums: &[(usize, &str)]) -> Option<usize> {
        rule_nums
            .iter()
            .find_map(|(pos, r)| if *r == rule { Some(*pos) } else { None })
    }
}

pub struct Rules<'a> {
    pub rule_nums: &'a [(usize, &'a str)],
    pub rules: &'a [Rule],
    pub start: usize,
}

impl<'a> Rules<'a> {
    pub fn parse_rules<'b: 'a>(rules: &'b str, mem: &'a Mem<'a>) -> Self {
        let num_rules = rules.lines().map(|l| l.split(",").count()).sum::<usize>();
        let num_rule_names = rules.lines().count();
        let rule_nums = mem
            .alloc_slice_from_iter(
                num_rule_names,
                rules.lines().scan(0, |s, l| {
                    let prev = *s;
                    *s += l.split(",").count();
                    Some((prev, l.split_once("{").unwrap().0))
                }),
            )
            .unwrap();
        let rules_iter = rules.lines().flat_map(|l| {
            l.split_once("{")
                .unwrap()
                .1
                .trim_end_matches("}")
                .split(",")
                .map(|r| Rule::parse(r, rule_nums).unwrap())
        });
        Self {
            rule_nums,
            rules: mem.alloc_slice_from_iter(num_rules, rules_iter).unwrap(),
            start: Rule::get_rule_number("in", rule_nums).unwrap(),
        }
    }

    pub fn pass_part(&self, part: &Part) -> bool {
        let mut index = self.start;
        loop {
            let target = match self.rules[index] {
                Rule::GreaterThan(prop, num, target) => {
                    if part.get(prop) <= num {
                        index += 1;
                        continue;
                    } else {
                        target
                    }
                }
                Rule::LessThan(prop, num, target) => {
                    if part.get(prop) >= num {
                        index += 1;
                        continue;
                    } else {
                        target
                    }
                }
                Rule::Unconditional(target) => target,
            };
            match target {
                Target::Accept => return true,
                Target::Reject => return false,
                Target::Goto(i) => index = i,
            }
        }
    }
}

pub struct Part {
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

#[cfg(test)]
mod test {
    use super::part_1;
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
        assert_eq!(part_1(TEST_INPUT, &mut buffer), 19114);
    }
}
