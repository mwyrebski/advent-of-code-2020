#[derive(PartialEq, Debug)]
struct Bus {
    id: i64,
    offset: usize,
}

#[derive(PartialEq, Debug)]
struct Notes {
    depart_timestamp: u32,
    buses: Vec<Bus>,
}

fn parse_input<'a>(input: &str) -> Notes {
    let mut lines = input.lines();
    let depart_timestamp = lines.next().unwrap().parse::<u32>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(offset, bus_id)| bus_id.parse::<i64>().ok().map(|id| Bus { id, offset }))
        .collect::<Vec<_>>();
    Notes {
        depart_timestamp,
        buses,
    }
}

#[inline]
fn div_ceil(num: i64, div: i64) -> i64 {
    let t = num / div;
    if t % div > 0 {
        t + 1
    } else {
        t
    }
}

fn part1(notes: &Notes) -> i64 {
    let (id, diff) = notes
        .buses
        .iter()
        .map(|bus| {
            let depart = notes.depart_timestamp as i64;
            let freq = div_ceil(depart, bus.id);
            let timestamp = freq * bus.id;
            (bus.id, timestamp - depart)
        })
        .min_by_key(|(_, diff)| *diff)
        .unwrap();
    id * diff
}

// source: https://github.com/simon-andrews/rust-modinverse/
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn part2(notes: &Notes) -> i64 {
    let ns: i64 = notes.buses.iter().map(|bus| bus.id).product();

    let xs: i64 = notes
        .buses
        .iter()
        .map(|bus| {
            let n = ns / bus.id;
            let (_, m, _) = egcd(n, bus.id);
            let x = bus.offset as i64 * n * m;
            x
        })
        .sum();

    ns - xs.rem_euclid(ns)
}

pub fn run() {
    let input = include_str!("input/day13.txt");
    let notes = parse_input(input);
    println!("Day 13/1: {}", part1(&notes));
    println!("Day 13/2: {}", part2(&notes));
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
        let b = |id, offset| -> Bus { Bus { id, offset } };
        let notes = parse_input(SAMPLE1);
        assert_eq!(
            Notes {
                depart_timestamp: 939,
                buses: vec![b(7, 0), b(13, 1), b(59, 4), b(31, 6), b(19, 7)]
            },
            notes
        );
    }

    #[test]
    fn test_day13_part1_sample1() {
        let notes = &parse_input(SAMPLE1);
        assert_eq!(295, part1(notes));
    }

    #[test]
    fn test_day13_part2_sample1() {
        let notes = &parse_input(SAMPLE1);
        assert_eq!(1068781, part2(notes));
    }

    #[test]
    //#[ignore]
    fn test_day13_part2_more_samples() {
        assert_eq!(3417, part2(&parse_input("0\n17,x,13,19")));
        assert_eq!(754018, part2(&parse_input("0\n67,7,59,61")));
        assert_eq!(779210, part2(&parse_input("0\n67,x,7,59,61")));
        assert_eq!(1261476, part2(&parse_input("0\n67,7,x,59,61")));
        assert_eq!(1202161486, part2(&parse_input("0\n1789,37,47,1889")));
    }
}
