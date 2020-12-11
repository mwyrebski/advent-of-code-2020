fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<u32> {
    let mut v = lines.map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
    v.sort();
    v.insert(0, 0);
    v.push(v.last().unwrap() + 3);
    v
}

fn part1(jolts: &Vec<u32>) -> u32 {
    let mut current = 0;
    let mut diffs1 = 0;
    let mut diffs3 = 0;
    for j in &jolts[1..] {
        match j - current {
            1 => diffs1 += 1,
            3 => diffs3 += 1,
            _ => panic!(),
        }
        current = *j;
    }
    diffs1 * diffs3
}

fn part2(jolts: &Vec<u32>) -> u64 {
    let mut track = vec![1; jolts.len()];
    for i in (0..jolts.len()).rev() {
        let can_connect_with = |q| jolts.get(q).map(|x| x - jolts[i] <= 3).unwrap_or_default();

        if can_connect_with(i + 1) {
            track[i] = track[i + 1];
        }
        if can_connect_with(i + 2) {
            track[i] += track[i + 2];
        }
        if can_connect_with(i + 3) {
            track[i] += track[i + 3];
        }
    }
    track[0]
}

pub fn run() {
    let input = include_str!("input/day10.txt");
    let jolts = parse_input(input.lines());
    println!("Day 10/1: {}", part1(&jolts));
    println!("Day 10/2: {}", part2(&jolts));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = "16
10
15
5
1
11
7
19
6
12
4
";

    const SAMPLE2: &'static str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn test_day10_part1_sample1() {
        let jolts = &parse_input(SAMPLE1.lines());
        assert_eq!(7 * 5, part1(jolts));
    }
    #[test]
    fn test_day10_part1_sample2() {
        let jolts = &parse_input(SAMPLE2.lines());
        assert_eq!(22 * 10, part1(jolts));
    }

    #[test]
    fn test_day10_part2_sample1() {
        let jolts = &parse_input(SAMPLE1.lines());
        assert_eq!(8, part2(jolts));
    }
    #[test]
    fn test_day10_part2_sample2() {
        let jolts = &parse_input(SAMPLE2.lines());
        assert_eq!(19208, part2(jolts));
    }
}
