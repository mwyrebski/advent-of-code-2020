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

fn part2(parsed: &Vec<u32>) -> usize {
    let extend = 1_000_000_usize;
    let count = 10_000_000_usize;

    let cups_list = {
        let parsed_max = *parsed.iter().max().unwrap() as usize;
        let mut cups_list: Vec<usize> = parsed.iter().map(|&x| x as usize).collect();
        for i in parsed_max + 1..=extend {
            cups_list.push(i);
        }
        cups_list
    };

    let min = *cups_list.iter().min().unwrap();
    let max = *cups_list.iter().max().unwrap();

    let mut rights = vec![0; cups_list.len() + 1];
    for win in cups_list.windows(2) {
        if let [this, next] = win {
            rights[*this] = *next;
        }
    }
    let first = *cups_list.first().unwrap();
    let last = *cups_list.last().unwrap();
    rights[last] = first;

    let mut current = first;
    for _ in 1..=count {
        let a = rights[current];
        let b = rights[a];
        let c = rights[b];
        let dest = {
            let mut dst = current;
            loop {
                dst -= 1;
                if dst < min {
                    dst = max;
                }
                if dst != a && dst != b && dst != c {
                    break dst;
                }
            }
        };
        let new_current_next = rights[c];
        let dest_next = rights[dest];
        rights[current] = new_current_next;
        rights[dest] = a;
        rights[c] = dest_next;

        current = rights[current];
    }

    let x = rights[1];
    let y = rights[x];

    x * y
}

pub fn run() {
    let input = include_str!("input/day23.txt");
    let parsed = &parse(input);
    println!("Day 23/1: {}", part1(parsed));
    println!("Day 23/2: {}", part2(parsed));
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
    fn test_day23_part1_sample1() {
        assert_eq!(67384529, part1(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day23_part2_sample1() {
        assert_eq!(149245887792, part2(&parse(SAMPLE1)));
    }
}
