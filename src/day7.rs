use std::collections::HashMap;

static SHINY_GOLD: &'static str = "shiny gold";

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<String, Vec<(u8, String)>> {
    lines.fold(HashMap::new(), |mut acc, line| {
        let split = line.split(" contain ").collect::<Vec<&str>>();
        let key = split[0].trim_end_matches(" bags").to_string();
        let value = split[1]
            .trim_end_matches('.')
            .split(", ")
            .map(|s| s.trim_end_matches(" bags").trim_end_matches(" bag"))
            .filter(|s| s != &"no other")
            .map(|s| {
                let i = s[..1].parse::<u8>().unwrap();
                let n = String::from(&s[2..]);
                (i, n)
            })
            .collect::<Vec<_>>();

        acc.insert(key, value);
        acc
    })
}

fn part1(map: &HashMap<String, Vec<(u8, String)>>) -> usize {
    fn can_reach_shiny_gold(map: &HashMap<String, Vec<(u8, String)>>, name: &String) -> bool {
        let v = map.get(name).unwrap();
        v.iter().any(|(_, n)| n == SHINY_GOLD)
            || v.into_iter().any(|(_, n)| can_reach_shiny_gold(map, n))
    };

    map.keys().filter(|k| can_reach_shiny_gold(map, k)).count()
}

fn part2(map: &HashMap<String, Vec<(u8, String)>>) -> u32 {
    fn count_items_in_bag(
        map: &HashMap<String, Vec<(u8, String)>>,
        name: &String,
        sum: u32,
    ) -> u32 {
        let v = map.get(name).unwrap();
        let sub: u32 = v
            .iter()
            .map(|(i, n)| {
                let i = *i as u32;
                i + i * count_items_in_bag(map, n, sum)
            })
            .sum();
        sum + sub
    };

    count_items_in_bag(map, &String::from(SHINY_GOLD), 0)
}

pub fn run() {
    let input = include_str!("input/day7.txt");
    let map = &parse_input(input.lines());
    println!("Day 7/1: {}", part1(map));
    println!("Day 7/2: {}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
    const SAMPLE2: &'static str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn test_part1_sample() {
        let map = parse_input(SAMPLE.lines());
        assert_eq!(4, part1(&map));
    }

    #[test]
    fn test_part2_sample() {
        let map = parse_input(SAMPLE.lines());
        assert_eq!(32, part2(&map));
    }

    #[test]
    fn test_part2_sample2() {
        let map = parse_input(SAMPLE2.lines());
        assert_eq!(126, part2(&map));
    }
}
