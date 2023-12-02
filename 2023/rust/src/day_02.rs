use std::collections::HashMap;

fn add_results<'a, 'b>(res: &'a mut HashMap<&'b str, u32>, merge: &'a HashMap<&'b str, u32>) {
    merge.iter().for_each(|(s, c)| {
        res.entry(*s)
            .and_modify(|e| {
                if c > e {
                    *e = *c
                }
            })
            .or_insert(*c);
    })
}

fn parse_line_draws(line: &str) -> HashMap<&str, u32> {
    let mut line_map = HashMap::new();
    line.split("; ").for_each(|s| {
        let mut hand_map = HashMap::new();
        s.split(", ").for_each(|ss| {
            let (num, col) = ss.split_once(" ").unwrap();
            let num = num.parse().unwrap();
            hand_map.entry(col).and_modify(|e| *e += num).or_insert(num);
        });
        add_results(&mut line_map, &hand_map);
    });
    line_map
}

fn parse_line(line: &str) -> (u32, HashMap<&str, u32>) {
    let (g, rest) = line.split_once(": ").unwrap();
    let game_number = (&g[5..]).parse().unwrap();
    (game_number, parse_line_draws(rest))
}

fn agrees(line: &HashMap<&str, u32>, limits: &HashMap<&str, u32>) -> bool {
    line.iter()
        .all(|(k, v)| limits.get(k).map_or(false, |lim_v| v <= lim_v))
}

pub fn part_1(input: &str) -> String {
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let total: u32 = input
        .lines()
        .filter_map(|l| {
            let (n, m) = parse_line(l);
            if agrees(&m, &limits) {
                Some(n)
            } else {
                None
            }
        })
        .sum();
    format!("{}", total)
}

pub fn part_2(input: &str) -> String {
    let total: u32 = input
        .lines()
        .map(|l| {
            let (_, m) = parse_line(l);
            m.get("red").unwrap_or(&0) * m.get("green").unwrap_or(&0) * m.get("blue").unwrap_or(&0)
        })
        .sum();
    format!("{}", total)
}

#[cfg(test)]
mod test {
    use super::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(data), String::from("8"));
    }

    #[test]
    fn part_2_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(data), String::from("2286"));
    }
}
