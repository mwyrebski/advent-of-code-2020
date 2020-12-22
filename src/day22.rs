use crate::lib::*;
use std::collections::VecDeque;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (p1str, p2str) = split_once_str(input, "\n\n");
    let p1 = parse_to_vec(p1str.lines().skip(1));
    let p2 = parse_to_vec(p2str.lines().skip(1));
    (p1, p2)
}

fn part1(parsed: &(Vec<u32>, Vec<u32>)) -> usize {
    let mut p1: VecDeque<u32> = parsed.0.clone().into();
    let mut p2: VecDeque<u32> = parsed.1.clone().into();
    let winning_cards: &VecDeque<u32> = loop {
        if p1.is_empty() {
            break &p2;
        }
        if p2.is_empty() {
            break &p1;
        }
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    };
    winning_cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * *x as usize)
        .sum()
}

pub fn run() {
    let input = include_str!("input/day22.txt");
    let parsed = &parse(input);
    println!("Day 22/1: {}", part1(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    #[test]
    fn test_day22_part1_sample1() {
        assert_eq!(306, part1(&parse(SAMPLE1)));
    }
}
