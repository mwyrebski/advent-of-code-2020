use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Mask {
    ones: u64,
    zeros: u64,
    flucts: Vec<u64>, // fluctuating positions (with 'X')
}

#[derive(Debug, PartialEq)]
enum Op {
    Mask(Mask),
    Mem(u64, u64),
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
                let zeros = u64::from_str_radix(&zeros_str, 2).unwrap();
                let ones = u64::from_str_radix(&ones_str, 2).unwrap();
                let flucts = right
                    .chars()
                    .enumerate()
                    .filter(|(_, v)| v == &'X')
                    .map(|(i, _)| 35 - i as u64)
                    .collect::<Vec<u64>>();
                Op::Mask(Mask {
                    zeros,
                    ones,
                    flucts,
                })
            } else {
                let address = left
                    .replace(|c| !char::is_numeric(c), "")
                    .parse::<u64>()
                    .unwrap();
                let value = right.parse::<u64>().unwrap();
                Op::Mem(address, value)
            }
        })
        .collect::<Vec<_>>()
}

fn part1(ops: &Vec<Op>) -> u64 {
    let mut hm = HashMap::new();
    let mut mask = &Mask {
        zeros: 0,
        ones: 0,
        flucts: vec![],
    };
    for op in ops {
        match op {
            Op::Mask(m) => mask = m,
            Op::Mem(address, value) => {
                let e = hm.entry(address).or_default();
                *e = (value & !mask.zeros) | mask.ones;
            }
        }
    }
    hm.values().sum()
}

#[inline]
fn set_bit(num: u64, bit: u64) -> u64 {
    num | (1 << bit)
}

#[inline]
fn clr_bit(num: u64, bit: u64) -> u64 {
    num & !(1 << bit)
}

fn part2(ops: &Vec<Op>) -> u64 {
    let mut hm = HashMap::new();
    let mut mask = &Mask {
        zeros: 0,
        ones: 0,
        flucts: vec![],
    };
    #[inline]
    fn permute(addr: u64, flucts: &[u64], value: u64, hm: &mut HashMap<u64, u64>) {
        if let Some((idx, tail)) = flucts.split_first() {
            permute(set_bit(addr, *idx as u64), tail, value, hm);
            permute(clr_bit(addr, *idx as u64), tail, value, hm);
        } else {
            let entry = hm.entry(addr).or_default();
            *entry = value.into();
        }
    }
    for op in ops {
        match op {
            Op::Mask(m) => mask = m,
            Op::Mem(address, value) => {
                let addr = address | mask.ones;
                permute(addr, &mask.flucts, *value, &mut hm);
            }
        }
    }
    hm.values().sum()
}

pub fn run() {
    let input = include_str!("input/day14.txt");
    let ops = parse_input(input);
    println!("Day 14/1: {}", part1(&ops));
    println!("Day 14/2: {}", part2(&ops));
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

    const SAMPLE2: &'static str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

    #[test]
    fn test_day14_parse_input_sample1() {
        let ops = parse_input(SAMPLE1);
        let flucts = (0..36)
            .rev()
            .filter(|n| *n != 1 && *n != 6)
            .collect::<Vec<_>>();
        assert_eq!(
            vec![
                Op::Mask(Mask {
                    zeros: 0b10,
                    ones: 0b1000000,
                    flucts,
                }),
                Op::Mem(8, 11),
                Op::Mem(7, 101),
                Op::Mem(8, 0),
            ],
            ops
        );
    }

    #[test]
    fn test_day14_parse_input_sample2() {
        let ops = parse_input(SAMPLE2);
        assert_eq!(
            vec![
                Op::Mask(Mask {
                    zeros: 0b111111111111111111111111111111001100,
                    ones: 0b000000000000000000000000000000010010,
                    flucts: vec![5, 0],
                }),
                Op::Mem(42, 100),
                Op::Mask(Mask {
                    zeros: 0b111111111111111111111111111111110100,
                    ones: 0b000000000000000000000000000000000000,
                    flucts: vec![3, 1, 0],
                }),
                Op::Mem(26, 1),
            ],
            ops
        );
    }

    #[test]
    fn test_day14_part1_sample1() {
        let ops = &parse_input(SAMPLE1);
        assert_eq!(165, part1(ops));
    }

    #[test]
    fn test_day14_part2_sample2() {
        let ops = &parse_input(SAMPLE2);
        assert_eq!(208, part2(ops));
    }

    #[test]
    fn test_day14_set_bit() {
        assert_eq!(1, set_bit(0, 0));
        assert_eq!(2, set_bit(0, 1));
        assert_eq!(8, set_bit(0, 3));
    }

    #[test]
    fn test_day14_clear_bit() {
        assert_eq!(0, clr_bit(1, 0));
        assert_eq!(5, clr_bit(5, 1));
        assert_eq!(5, clr_bit(7, 1));
    }
}
