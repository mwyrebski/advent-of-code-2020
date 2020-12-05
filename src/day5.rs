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

fn part1(seat_ids: &Vec<u32>) -> u32 {
    *seat_ids.iter().max().unwrap()
}

fn part2(seat_ids: &Vec<u32>) -> u32 {
    let max_id = *seat_ids.iter().max().unwrap() as usize;
    let mut v: Vec<bool> = vec![false; max_id + 1];
    for id in seat_ids.iter() {
        v[*id as usize] = true;
    }
    let mut my_seat_id = 0;
    for n in 1..(max_id - 1) {
        if !v[n] && v[n - 1] && v[n + 1] {
            my_seat_id = n;
            break;
        }
    }
    my_seat_id as u32
}

pub fn run() {
    let input = include_str!("input/day5.txt");
    let lines = &to_lines(input);
    let seat_ids = &lines.iter().map(seat_id).collect();
    println!("Day 5/1: {}", part1(seat_ids));
    println!("Day 5/2: {}", part2(seat_ids));
}
