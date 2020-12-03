struct Policy {
    c: char,
    x: usize,
    y: usize,
}

struct Line {
    policy: Policy,
    password: String,
}

fn parse_line(s: &String) -> Line {
    let parts: Vec<&str> = s.split(": ").collect();
    let policy_parts: Vec<&str> = parts[0].split(' ').collect();
    let number_part: Vec<&str> = policy_parts[0].split('-').collect();

    let min = number_part[0].parse::<usize>().unwrap();
    let max = number_part[1].parse::<usize>().unwrap();
    let ch = policy_parts[1].chars().next().unwrap();
    let password = String::from(parts[1]);

    Line {
        policy: Policy {
            c: ch,
            x: min,
            y: max,
        },
        password: password,
    }
}

fn is_line_valid(l: &Line) -> bool {
    let count = l.password.chars().filter(|c| l.policy.c == *c).count();
    l.policy.x <= count && count <= l.policy.y
}

fn part1(lines: &Vec<String>) -> usize {
    lines.iter().map(parse_line).filter(is_line_valid).count()
}

fn is_line_valid_new(l: &Line) -> bool {
    let x_opt = l.password.chars().nth(l.policy.x - 1);
    let y_opt = l.password.chars().nth(l.policy.y - 1);
    if let (Some(a), Some(b)) = (x_opt, y_opt) {
        return (a == l.policy.c) ^ (b == l.policy.c);
    }
    false
}

fn part2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(parse_line)
        .filter(is_line_valid_new)
        .count()
}

use crate::lib;

pub fn run() {
    let input = include_str!("input/day2.txt");
    let lines = &lib::to_lines(input);
    println!("Day 2/1: {}", part1(lines));
    println!("Day 2/2: {}", part2(lines));
}
