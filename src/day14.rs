use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Mask {
    ones: i64,
    zeros: i64,
}

#[derive(Debug, PartialEq)]
enum Op {
    Mask(Mask),
    Mem(u32, i64),
}

fn parse_input<'a>(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let p = line.split(" = ").collect::<Vec<_>>();
            let left = p[0];
            let right = p[1];
            if left == "mask" {
                let zeros_str = right.replace('1', "X").replace('0', "1").replace('X', "0");
                let ones_str = right.replace('X', "0");
                let zeros = i64::from_str_radix(&zeros_str, 2).unwrap();
                let ones = i64::from_str_radix(&ones_str, 2).unwrap();
                Op::Mask(Mask { zeros, ones })
            } else {
                let addr = left
                    .replace(|c| !char::is_numeric(c), "")
                    .parse::<u32>()
                    .unwrap();
                let val = right.parse::<i64>().unwrap();
                Op::Mem(addr, val)
            }
        })
        .collect::<Vec<_>>()
}

fn part1(ops: &Vec<Op>) -> i64 {
    let mut hm = HashMap::new();
    let mut mask = &Mask { zeros: 0, ones: 0 };
    for op in ops {
        match op {
            Op::Mask(m) => mask = m,
            Op::Mem(addr, value) => {
                let e = hm.entry(addr).or_default();
                *e = (value & !mask.zeros) | mask.ones;
            }
        }
    }
    hm.values().sum()
}

pub fn run() {
    let input = include_str!("input/day14.txt");
    let ops = parse_input(input);
    println!("Day 14/1: {}", part1(&ops));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

    #[test]
    fn test_day14_parse_input() {
        let ops = parse_input(SAMPLE1);
        assert_eq!(
            vec![
                Op::Mask(Mask {
                    zeros: 0b10,
                    ones: 0b1000000
                }),
                Op::Mem(8, 11),
                Op::Mem(7, 101),
                Op::Mem(8, 0),
            ],
            ops
        );
    }

    #[test]
    fn test_day14_part1_sample1() {
        let ops = &parse_input(SAMPLE1);
        assert_eq!(165, part1(ops));
    }
}
