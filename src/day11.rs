#[derive(Copy, Clone, PartialEq)]
enum PosType {
    Floor,
    SeatFree,
    SeatOccupied,
}

fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Vec<PosType>> {
    lines
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => PosType::SeatFree,
                    '.' => PosType::Floor,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn count_occupied(seats: &Vec<Vec<PosType>>) -> usize {
    seats
        .iter()
        .flatten()
        .filter(|p| *p == &PosType::SeatOccupied)
        .count()
}

static DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(seats: &Vec<Vec<PosType>>) -> usize {
    let rows_len = seats.len();
    let cols_len = seats[0].len();

    let count_adjacent_occupied = |seats: &Vec<Vec<_>>, i, j| {
        let mut adjacent_occupied = 0;
        for (x, y) in &DIRECTIONS {
            let a = i as i32 + x;
            let b = j as i32 + y;
            if a < 0 || b < 0 || a >= rows_len as i32 || b >= cols_len as i32 {
                continue;
            }
            if seats[a as usize][b as usize] == PosType::SeatOccupied {
                adjacent_occupied += 1
            }
        }
        adjacent_occupied
    };

    let mut old_seats = seats.clone();
    loop {
        let mut new_seats = vec![vec![PosType::Floor; cols_len]; rows_len];
        for i in 0..rows_len {
            for j in 0..cols_len {
                let occupied = count_adjacent_occupied(&old_seats, i, j);
                new_seats[i][j] = match old_seats[i][j] {
                    PosType::SeatFree if occupied == 0 => PosType::SeatOccupied,
                    PosType::SeatOccupied if occupied >= 4 => PosType::SeatFree,
                    state => state,
                };
            }
        }
        if old_seats == new_seats {
            return count_occupied(&old_seats);
        }
        old_seats = new_seats;
    }
}

fn part2(seats: &Vec<Vec<PosType>>) -> usize {
    let rows_len = seats.len();
    let cols_len = seats[0].len();

    let count_visible_occupied = |seats: &Vec<Vec<_>>, i, j| {
        let mut visible_occupied = 0;
        for (x, y) in &DIRECTIONS {
            let mut row = i as i32;
            let mut col = j as i32;
            loop {
                row += x;
                col += y;
                if row < 0 || col < 0 || row >= rows_len as i32 || col >= cols_len as i32 {
                    break;
                }
                match seats[row as usize][col as usize] {
                    PosType::Floor => continue,
                    PosType::SeatFree => (),
                    PosType::SeatOccupied => visible_occupied += 1,
                }
                break;
            }
        }
        visible_occupied
    };

    let mut old_seats = seats.clone();
    loop {
        let mut new_seats = vec![vec![PosType::Floor; cols_len]; rows_len];
        for i in 0..rows_len {
            for j in 0..cols_len {
                let occupied = count_visible_occupied(&old_seats, i, j);
                new_seats[i][j] = match old_seats[i][j] {
                    PosType::SeatFree if occupied == 0 => PosType::SeatOccupied,
                    PosType::SeatOccupied if occupied >= 5 => PosType::SeatFree,
                    state => state,
                };
            }
        }
        if old_seats == new_seats {
            return count_occupied(&old_seats);
        }
        old_seats = new_seats;
    }
}

pub fn run() {
    let input = include_str!("input/day11.txt");
    let seats = parse_input(input.lines());
    println!("Day 11/1: {}", part1(&seats));
    println!("Day 11/2: {}", part2(&seats));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn test_day11_part1_sample1() {
        let seats = &parse_input(SAMPLE1.lines());
        assert_eq!(37, part1(seats));
    }

    #[test]
    fn test_day11_part2_sample1() {
        let seats = &parse_input(SAMPLE1.lines());
        assert_eq!(26, part2(seats));
    }
}
