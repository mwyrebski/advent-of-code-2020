#[derive(PartialEq, Debug)]
struct Notes {
    depart_timestamp: u32,
    buses: Vec<u32>,
}

fn parse_input<'a>(input: &str) -> Notes {
    let mut lines = input.lines();
    let depart_timestamp = lines.next().unwrap().parse::<u32>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    Notes {
        depart_timestamp,
        buses,
    }
}

#[inline]
fn div_ceil(num: u32, div: u32) -> u32 {
    let t = num / div;
    if t % div > 0 {
        t + 1
    } else {
        t
    }
}

fn part1(notes: &Notes) -> u32 {
    let (id, diff) = notes
        .buses
        .iter()
        .map(|id| {
            let id = *id;
            let freq = div_ceil(notes.depart_timestamp, id);
            let timestamp = freq * id;
            (id, timestamp - notes.depart_timestamp)
        })
        .min_by_key(|(_, diff)| *diff)
        .unwrap();
    id * diff
}

pub fn run() {
    let input = include_str!("input/day13.txt");
    let notes = parse_input(input);
    println!("Day 13/1: {}", part1(&notes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = "\
939
7,13,x,x,59,x,31,19
";

    #[test]
    fn test_day13_parse_input() {
        let notes = parse_input(SAMPLE1);
        assert_eq!(
            Notes {
                depart_timestamp: 939,
                buses: vec![7, 13, 59, 31, 19]
            },
            notes
        );
    }

    #[test]
    fn test_day13_part1_sample1() {
        let notes = &parse_input(SAMPLE1);
        assert_eq!(295, part1(notes));
    }
}
