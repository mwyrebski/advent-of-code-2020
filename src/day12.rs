use points::Point;

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

mod points {
    use std::ops::{AddAssign, Mul};

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
        pub fn zero() -> Point {
            Point::new(0, 0)
        }
        pub fn manhattan_dist(&self, from: Point) -> i32 {
            (from.x - self.x).abs() + (from.y - self.y).abs()
        }
        pub fn move_north(&mut self, value: i32) {
            self.y += value;
        }
        pub fn move_south(&mut self, value: i32) {
            self.y -= value;
        }
        pub fn move_east(&mut self, value: i32) {
            self.x += value;
        }
        pub fn move_west(&mut self, value: i32) {
            self.x -= value;
        }
        pub fn rotate_right(&mut self, angle: i32) {
            let a = angle / 90 % 4;
            for _ in 0..a {
                let x2 = self.y;
                let y2 = -self.x;
                self.x = x2;
                self.y = y2;
            }
        }
        pub fn rotate_left(&mut self, angle: i32) {
            self.rotate_right(360 - angle % 360);
        }
    }

    impl Mul<i32> for Point {
        type Output = Self;

        fn mul(self, rhs: i32) -> Self {
            Self::new(self.x * rhs, self.y * rhs)
        }
    }

    impl AddAssign for Point {
        fn add_assign(&mut self, other: Self) {
            *self = Self::new(self.x + other.x, self.y + other.y)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_day12_point_rotate_right() {
            let mut p = Point::new(10, 4);
            p.rotate_right(90);
            assert_eq!(Point::new(4, -10), p);
            p.rotate_right(90);
            assert_eq!(Point::new(-10, -4), p);
            p.rotate_right(360);
            assert_eq!(Point::new(-10, -4), p);
            p.rotate_right(270);
            assert_eq!(Point::new(4, -10), p);
            p.rotate_right(180);
            assert_eq!(Point::new(-4, 10), p);
        }

        #[test]
        fn test_day12_point_rotate_left() {
            let mut p = Point::new(2, 4);
            p.rotate_left(90);
            assert_eq!(Point::new(-4, 2), p);
            p.rotate_left(90);
            assert_eq!(Point::new(-2, -4), p);
            p.rotate_left(360);
            assert_eq!(Point::new(-2, -4), p);
            p.rotate_left(270);
            assert_eq!(Point::new(-4, 2), p);
            p.rotate_left(180);
            assert_eq!(Point::new(4, -2), p);
        }
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
    let mut pt = Point::zero();

    for Move(action, value) in moves {
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

    pt.manhattan_dist(Point::zero())
}

fn part2(moves: &Vec<Move>) -> i32 {
    let mut ship = Point::zero();
    let mut waypoint = Point::new(10, 1);

    for Move(action, value) in moves {
        let v = *value;
        match action {
            Action::N => waypoint.move_north(v),
            Action::S => waypoint.move_south(v),
            Action::E => waypoint.move_east(v),
            Action::W => waypoint.move_west(v),
            Action::L => waypoint.rotate_left(v),
            Action::R => waypoint.rotate_right(v),
            Action::F => ship += waypoint * v,
        }
    }

    ship.manhattan_dist(Point::zero())
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
        assert_eq!(17 + 8, Point::new(-17, -8).manhattan_dist(Point::zero()));
        assert_eq!(17 + 8, Point::new(-17, 8).manhattan_dist(Point::zero()));
        assert_eq!(17 + 8, Point::new(17, 8).manhattan_dist(Point::zero()));
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
