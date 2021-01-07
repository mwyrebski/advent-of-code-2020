use crate::lib::parse_unwrap;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| parse_unwrap(&c.to_string()))
        .collect()
}

fn calc_moves(count: usize, parsed: &Vec<u32>) -> u32 {
    let min = *parsed.iter().min().unwrap();
    let max = *parsed.iter().max().unwrap();

    let mut cups: VecDeque<u32> = parsed.clone().into();
    let mut picked = VecDeque::with_capacity(3);

    for _ in 0..count {
        let current = cups.pop_front().unwrap();
        picked.push_front(cups.pop_front().unwrap());
        picked.push_front(cups.pop_front().unwrap());
        picked.push_front(cups.pop_front().unwrap());
        let destination = {
            let mut dst = current;
            loop {
                dst -= 1;
                if dst < min {
                    dst = max;
                }
                if let Some(pos) = cups.iter().position(|&x| x == dst) {
                    break pos + 1;
                }
            }
        };
        cups.insert(destination, picked.pop_front().unwrap());
        cups.insert(destination, picked.pop_front().unwrap());
        cups.insert(destination, picked.pop_front().unwrap());
        cups.push_front(current);
        cups.rotate_left(1);
    }

    let pos1 = cups.iter().position(|&x| x == 1).unwrap();
    cups.rotate_left(pos1);

    cups.iter()
        .skip(1)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * (10u32.pow(i as u32)))
}

fn part1(parsed: &Vec<u32>) -> u32 {
    calc_moves(100, parsed)
}

pub fn run() {
    let input = include_str!("input/day23.txt");
    let parsed = &parse(input);
    println!("Day 23/1: {}", part1(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "389125467";

    #[test]
    fn test_day23_calc_moves_sample1() {
        assert_eq!(92658374, calc_moves(10, &parse(SAMPLE1)));
    }

    #[test]
    #[ignore]
    fn test_day23_part1_sample1() {
        assert_eq!(67384529, part1(&parse(SAMPLE1)));
    }
}
