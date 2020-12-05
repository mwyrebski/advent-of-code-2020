use crate::lib::to_lines;

fn seat_id(l: &String) -> u32 {
    let row_code = &l[..7];
    let col_code = &l[7..];
    let mut row = 0b0111_1111; // 127
    let mut x = 0b0100_0000; // 64
    for c in row_code.chars() {
        if c == 'F' {
            row ^= x; // clear bit
        }
        x /= 2;
    }
    let mut col = 0b0111; // 7
    let mut x = 0b0100; // 4
    for c in col_code.chars() {
        if c == 'L' {
            col ^= x // clear bit
        };
        x /= 2;
    }
    row * 8 + col
}

fn part1(lines: &Vec<String>) -> u32 {
    lines.iter().map(seat_id).max().unwrap()
}

fn part2(lines: &Vec<String>) -> u32 {
    0
}

pub fn run() {
    let input = include_str!("input/day5.txt");
    let lines = &to_lines(input);
    println!("Day 5/1: {}", part1(lines));
    println!("Day 5/2: {}", part2(lines));
}
