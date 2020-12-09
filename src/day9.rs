fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<u64> {
    lines.map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>()
}

fn find_invalid_number(vec: &Vec<u64>, preamble_len: usize) -> u64 {
    let is_valid = |num, slice: &[_]| {
        for (i, a) in slice.iter().enumerate() {
            for (j, b) in slice.iter().enumerate() {
                if i != j && a + b == num {
                    return true;
                }
            }
        }
        false
    };
    for win in vec.windows(preamble_len + 1) {
        let num = win[preamble_len];
        let slice = &win[..preamble_len];

        if !is_valid(num, slice) {
            return num;
        }
    }
    0
}

fn part1(cypher: &Vec<u64>) -> u64 {
    find_invalid_number(cypher, 25)
}

fn part2(cypher: &Vec<u64>, invalid_number: u64) -> u64 {
    let len = cypher.len();
    for i in 0..len - 1 {
        let a = cypher[i];
        let mut sum = a;
        let mut min = a;
        let mut max = a;
        for j in (i + 1)..len {
            let b = cypher[j];
            sum += b;
            if sum > invalid_number {
                break;
            }
            min = std::cmp::min(min, b);
            max = std::cmp::max(max, b);
            if sum == invalid_number {
                return min + max;
            }
        }
    }
    0
}

pub fn run() {
    let input = include_str!("input/day9.txt");
    let cypher = &parse_input(input.lines());
    let invalid_number = part1(cypher);
    println!("Day 9/1: {}", invalid_number);
    println!("Day 9/2: {}", part2(cypher, invalid_number));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    #[test]
    fn test_day9_part1_sample() {
        let cypher = &parse_input(SAMPLE.lines());
        assert_eq!(127, find_invalid_number(cypher, 5));
    }

    #[test]
    fn test_day9_part2_sample() {
        let cypher = &parse_input(SAMPLE.lines());
        assert_eq!(62, part2(cypher, 127));
    }
}
