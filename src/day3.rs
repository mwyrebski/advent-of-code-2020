use crate::lib;
use std::cmp::max;
use std::collections::HashMap;

#[derive(PartialEq)]
enum LocationType {
    Open,
    Tree,
}

#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

struct Map {
    width: usize,
    height: usize,
    points: HashMap<Point, LocationType>,
}

impl Map {
    fn new(points: HashMap<Point, LocationType>) -> Map {
        let mut max_w = 0;
        let mut max_h = 0;
        for (Point(x, y), _) in &points {
            max_w = max(max_w, *x);
            max_h = max(max_h, *y);
        }
        Map {
            width: max_w + 1,
            height: max_h + 1,
            points: points,
        }
    }
    fn get(&self, x: usize, y: usize) -> &LocationType {
        let x = x % self.width;
        self.points.get(&Point(x, y)).unwrap()
    }
}

fn parse_map(lines: &Vec<String>) -> Map {
    let mut points = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.char_indices() {
            let pt = Point(x, y);
            let typ = if ch == '#' {
                LocationType::Tree
            } else {
                LocationType::Open
            };
            points.insert(pt, typ);
        }
    }
    Map::new(points)
}

fn count_trees(slope_right: usize, slope_down: usize, map: &Map) -> usize {
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;
    loop {
        x += slope_right;
        y += slope_down;
        let t = map.get(x, y);
        if *t == LocationType::Tree {
            count += 1;
        }
        if y == map.height - 1 {
            break;
        }
    }
    count
}

fn part1(map: &Map) -> usize {
    count_trees(3, 1, map)
}

fn part2(map: &Map) -> usize {
    vec![
        count_trees(1, 1, map),
        count_trees(3, 1, map),
        count_trees(5, 1, map),
        count_trees(7, 1, map),
        count_trees(1, 2, map),
    ]
    .iter()
    .product()
}

pub fn run() {
    let input = include_str!("input/day3.txt");
    let lines = &lib::to_lines(input);
    let map = &parse_map(lines);
    println!("Day 3/1: {}", part1(map));
    println!("Day 3/2: {}", part2(map));
}
