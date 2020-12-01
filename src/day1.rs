static EXPECTED: i32 = 2020;

fn find_pair(nums: &Vec<i32>) -> Option<(i32, i32)> {
    for a in nums.into_iter() {
        for b in nums.into_iter() {
            if a + b == EXPECTED {
                let t = (*a, *b);
                return Some(t);
            }
        }
    }
    None
}

fn part1(nums: &Vec<i32>) -> i32 {
    let t = find_pair(&nums).unwrap();
    t.0 * t.1
}

fn find_triple(nums: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for a in nums.into_iter() {
        for b in nums.into_iter() {
            for c in nums.into_iter() {
                if a + b + c == EXPECTED {
                    let t = (*a, *b, *c);
                    return Some(t);
                }
            }
        }
    }
    None
}

fn part2(nums: &Vec<i32>) -> i32 {
    let t = find_triple(&nums).unwrap();
    t.0 * t.1 * t.2
}

use crate::lib;

pub fn run() {
    let input = include_str!("input/day1.txt");
    let nums = &lib::to_i32s(input);
    println!("Day 1/1: {}", part1(nums));
    println!("Day 1/2: {}", part2(nums));
}
