#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

struct Move(Action, i32);

#[derive(Debug)]
struct Point(i32, i32);

impl From<i32> for Direction {
    fn from(item: i32) -> Self {
        match item % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!(),
        }
    }
}

impl From<Direction> for i32 {
    fn from(item: Direction) -> Self {
        match item {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

impl Direction {
    pub fn right_by_angle(&mut self, angle: i32) {
        let d = i32::from(*self);
        let a = angle / 90;
        *self = (d + a).into();
    }
    pub fn left_by_angle(&mut self, angle: i32) {
        self.right_by_angle(360 - angle)
    }
}

impl Point {
    pub fn manhattan_dist(&self, from: Point) -> i32 {
        (from.0 - self.0).abs() + (from.1 - self.1).abs()
    }
    pub fn move_north(&mut self, value: i32) {
        self.1 -= value;
    }
    pub fn move_south(&mut self, value: i32) {
        self.1 += value;
    }
    pub fn move_east(&mut self, value: i32) {
        self.0 += value;
    }
    pub fn move_west(&mut self, value: i32) {
        self.0 -= value;
    }
}

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Move> {
    lines
        .map(|line| {
            let action = match line.chars().next().unwrap() {
                'N' => Action::N,
                'S' => Action::S,
                'E' => Action::E,
                'W' => Action::W,
                'L' => Action::L,
                'R' => Action::R,
                'F' => Action::F,
                _ => panic!(),
            };
            let value = line[1..].parse::<i32>().unwrap();
            Move(action, value)
        })
        .collect::<Vec<_>>()
}

fn part1(moves: &Vec<Move>) -> i32 {
    let mut dir = Direction::East;
    let mut pt = Point(0, 0);

    for m in moves {
        let Move(action, value) = m;
        let v = *value;
        match action {
            Action::N => pt.move_north(v),
            Action::S => pt.move_south(v),
            Action::E => pt.move_east(v),
            Action::W => pt.move_west(v),
            Action::L => dir.left_by_angle(v),
            Action::R => dir.right_by_angle(v),
            Action::F => match dir {
                Direction::North => pt.move_north(v),
                Direction::South => pt.move_south(v),
                Direction::East => pt.move_east(v),
                Direction::West => pt.move_west(v),
            },
        }
    }

    pt.manhattan_dist(Point(0, 0))
}

fn part2(_moves: &Vec<Move>) -> usize {
    0
}

pub fn run() {
    let input = include_str!("input/day12.txt");
    let moves = parse_input(input.lines());
    println!("Day 12/1: {}", part1(&moves));
    println!("Day 12/2: {}", part2(&moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = "\
F10
N3
F7
R90
F11
";

    #[test]
    fn test_day12_direction_right_by_angle() {
        let mut dir = Direction::North;
        dir.right_by_angle(90);
        assert_eq!(Direction::East, dir);
        dir.right_by_angle(180);
        assert_eq!(Direction::West, dir);
        dir.right_by_angle(270);
        assert_eq!(Direction::South, dir);
    }

    #[test]
    fn test_day12_direction_left_by_angle() {
        let mut dir = Direction::North;
        dir.left_by_angle(90);
        assert_eq!(Direction::West, dir);
        dir.left_by_angle(180);
        assert_eq!(Direction::East, dir);
        dir.left_by_angle(270);
        assert_eq!(Direction::South, dir);
    }

    #[test]
    fn test_day12_direction_manhattan_dist_from() {
        assert_eq!(17 + 8, Point(-17, -8).manhattan_dist(Point(0, 0)));
        assert_eq!(17 + 8, Point(-17, 8).manhattan_dist(Point(0, 0)));
        assert_eq!(17 + 8, Point(17, 8).manhattan_dist(Point(0, 0)));
    }

    #[test]
    fn test_day12_part1_sample1() {
        let parsed = &parse_input(SAMPLE1.lines());
        assert_eq!(25, part1(parsed));
    }

    #[test]
    fn test_day12_part2_sample1() {
        let parsed = &parse_input(SAMPLE1.lines());
        assert_eq!(286, part2(parsed));
    }
}
