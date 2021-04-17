use crate::lib::parse_to_vec;

fn parse(input: &str) -> (u64, u64) {
    let keys = parse_to_vec(input.lines());
    (keys[0], keys[1])
}

fn solve(card_pubkey: u64, door_pubkey: u64) -> u64 {
    static REMDIV: u64 = 20201227;

    let (loop_size, other_pubkey) = {
        let subject_number = 7;
        let mut loop_size = 0;
        let mut value = 1;
        loop {
            loop_size += 1;
            value = (value * subject_number) % REMDIV;
            if value == card_pubkey {
                break (loop_size, door_pubkey);
            }
            if value == door_pubkey {
                break (loop_size, card_pubkey);
            }
        }
    };

    let encryption_key = {
        let subject_number = other_pubkey;
        let mut value = 1;
        for _ in 0..loop_size {
            value = (value * subject_number) % REMDIV;
        }
        value
    };

    encryption_key
}

pub fn run() {
    let input = include_str!("input/day25.txt");
    let (key1, key2) = parse(input);
    println!("Day 25: {}", solve(key1, key2));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &'static str = "5764801\n17807724";

    #[test]
    fn test_day25_parse() {
        assert_eq!((5764801, 17807724), parse(SAMPLE));
    }

    #[test]
    fn test_day25_part1_sample() {
        let (key1, key2) = parse(SAMPLE);
        assert_eq!(14897079, solve(key1, key2));
    }
}
