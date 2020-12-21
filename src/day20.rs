use std::fmt;

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    data: Vec<Vec<bool>>,
    hashes: Vec<(i32, i32)>,
}

impl Tile {
    fn calc_border_hash(vec: &Vec<bool>) -> (i32, i32) {
        let set_bit = |value, index| (value as i32) << index;

        let p = vec
            .iter()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | set_bit(*b, i));
        let q = vec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | set_bit(*b, i));

        (p, q)
    }

    pub fn new(id: u32, data: Vec<Vec<bool>>) -> Tile {
        let top = &data[0];
        let right = &data.iter().map(|row| row[row.len() - 1]).collect();
        let bottom = &data[data.len() - 1];
        let left = &data.iter().map(|row| row[0]).collect();

        let hashes = vec![
            Tile::calc_border_hash(top),
            Tile::calc_border_hash(right),
            Tile::calc_border_hash(bottom),
            Tile::calc_border_hash(left),
        ];

        Tile { id, data, hashes }
    }

    pub fn any_side_matches(&self, other: &Tile) -> bool {
        self.hashes.iter().any(|&h| {
            other
                .hashes
                .iter()
                .any(|&oh| (h.0, h.1) == oh || (h.1, h.0) == oh)
        })
    }

    fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = lines
            .next()
            .unwrap()
            .replace(|c| !char::is_numeric(c), "")
            .parse::<u32>()
            .unwrap();
        let data = lines
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        Tile::new(id, data)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = format!("Tile: {}", self.id);
        let result = self.data.iter().fold(String::new(), |acc, row| {
            let line = row
                .into_iter()
                .map(|c| if *c { '#' } else { '.' })
                .collect::<String>();
            format!("{}{}\n", acc, line)
        });
        writeln!(f, "{}\n{}", header, result)
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input.trim().split("\n\n").map(Tile::parse).collect()
}

fn part1(data: &Vec<Tile>) -> u64 {
    let count_other_matching_tiles = |tile: &Tile| -> usize {
        data.iter()
            .map(|t| {
                if t.id != tile.id && tile.any_side_matches(t) {
                    1
                } else {
                    0
                }
            })
            .sum()
    };

    data.iter()
        .filter(|t| count_other_matching_tiles(t) == 2)
        .map(|tile| tile.id as u64)
        .product()
}

pub fn run() {
    let input = include_str!("input/day20.txt");
    let parsed = &parse(input);
    println!("Day 20/1: {}", part1(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_part1_sample1() {
        assert_eq!(20899048083289, part1(&parse(SAMPLE1)));
    }

    static SAMPLE1: &'static str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";
}
