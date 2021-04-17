use std::collections::HashSet;

type Point = crate::lib::point::Point<i32>;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::East => (1, 0),
            Direction::SouthEast => (0, -1),
            Direction::SouthWest => (-1, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (0, 1),
            Direction::NorthEast => (1, 1),
        }
        .into()
    }
    fn all_as_points() -> Vec<Point> {
        vec![
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
        .iter()
        .map(Direction::to_point)
        .collect()
    }
}

type ParsedData = Vec<Vec<Direction>>;

fn parse(input: &str) -> ParsedData {
    input
        .lines()
        .map(|line| {
            let mut result = Vec::new();
            let mut it = line.chars();
            while let Some(c) = it.next() {
                match c {
                    'e' => result.push(Direction::East),
                    'w' => result.push(Direction::West),
                    'n' => match it.next() {
                        Some('e') => result.push(Direction::NorthEast),
                        Some('w') => result.push(Direction::NorthWest),
                        _ => panic!(),
                    },
                    's' => match it.next() {
                        Some('e') => result.push(Direction::SouthEast),
                        Some('w') => result.push(Direction::SouthWest),
                        _ => panic!(),
                    },
                    _ => panic!(format!("unexpected character '{}'", c)),
                };
            }
            result
        })
        .collect()
}

fn locate_black_points(data: &ParsedData) -> HashSet<Point> {
    let mut black_points_set = HashSet::new();
    for directions in data {
        let pt = directions
            .iter()
            .fold(Point::zero(), |acc, dir| acc + dir.to_point());
        if black_points_set.contains(&pt) {
            black_points_set.remove(&pt);
        } else {
            black_points_set.insert(pt);
        }
    }
    black_points_set
}

fn part1(data: &ParsedData) -> usize {
    let black_points_set = locate_black_points(data);
    black_points_set.len()
}

fn part2(data: &ParsedData) -> usize {
    let mut black_points_set = locate_black_points(data);

    let all_directions_as_points = Direction::all_as_points();
    let get_adjacent_points = |source| -> HashSet<Point> {
        all_directions_as_points
            .iter()
            .map(|&pt| pt + source)
            .collect()
    };

    for _ in 0..100 {
        let needs_flipping_from_black_to_white: HashSet<Point> = black_points_set
            .iter()
            .filter(|pt| {
                let adj = get_adjacent_points(**pt);
                let adjacent_blacks = black_points_set.intersection(&adj).count();
                0 == adjacent_blacks || adjacent_blacks > 2
            })
            .map(|p| p.clone())
            .collect();

        let needs_flipping_from_white_to_black = {
            let all_adjacent_points: HashSet<Point> = black_points_set
                .iter()
                .map(|black_pt| get_adjacent_points(*black_pt))
                .flatten()
                .collect();

            let white_points = all_adjacent_points
                .difference(&black_points_set)
                .map(|p| p.clone())
                .collect::<HashSet<Point>>();

            white_points
                .into_iter()
                .filter(|pt| {
                    let adj = get_adjacent_points(*pt);
                    let adjacent_blacks = black_points_set.intersection(&adj).count();
                    adjacent_blacks == 2
                })
                .collect::<HashSet<Point>>()
        };

        for flip_pt in needs_flipping_from_black_to_white {
            black_points_set.remove(&flip_pt);
        }
        for flip_pt in needs_flipping_from_white_to_black {
            black_points_set.insert(flip_pt);
        }
    }

    black_points_set.len()
}

pub fn run() {
    let input = include_str!("input/day24.txt");
    let parsed = &parse(input);
    println!("Day 24/1: {}", part1(parsed));
    println!("Day 24/2: {}", part2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    #[test]
    fn test_day24_parse() {
        assert_eq!(
            vec![vec![
                Direction::East,
                Direction::SouthEast,
                Direction::NorthWest,
                Direction::West,
            ]],
            parse("esenww")
        );
    }

    #[test]
    fn test_day24_part1_sample1() {
        assert_eq!(10, part1(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day24_part2_sample1() {
        assert_eq!(2208, part2(&parse(SAMPLE1)));
    }
}
