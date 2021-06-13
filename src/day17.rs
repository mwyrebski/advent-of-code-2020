use std::cmp::max;
use std::collections::BTreeSet;

type Cube = BTreeSet<(i8, i8, i8)>;
type HyperCube = BTreeSet<(i8, i8, i8, i8)>;

fn parse_input<'a>(input: &str) -> Cube {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .enumerate()
                .filter_map(|(x, ch)| if ch == '#' { Some(x) } else { None })
        })
        .enumerate()
        .map(|(y, xs)| xs.map(|x| (x as i8, y as i8, 0i8)).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let mut set = Cube::new();
    for d in data {
        set.insert(d);
    }
    set
}

fn max_key(cube: &Cube) -> i8 {
    *cube
        .iter()
        .map(|(x, y, z)| max(x, max(y, z)))
        .max()
        .unwrap()
}

fn is_new_state_active(state: bool, active_neighbours: i32) -> bool {
    match state {
        true => active_neighbours == 2 || active_neighbours == 3,
        false => active_neighbours == 3,
    }
}

fn part1(cube: &Cube) -> usize {
    let mut min: i8 = 0;
    let mut max: i8 = max_key(cube);
    let mut cube = cube.clone();

    for _ in 1..=6 {
        let mut new_cube = Cube::new();
        min -= 1;
        max += 1;

        for x in min..=max {
            for y in min..=max {
                for z in min..=max {
                    let at_position = (x, y, z);

                    let mut active_neighbours = 0;
                    for nx in x - 1..=x + 1 {
                        for ny in y - 1..=y + 1 {
                            for nz in z - 1..=z + 1 {
                                let at_neighbour = (nx, ny, nz);
                                if at_neighbour == at_position {
                                    continue;
                                }
                                if cube.contains(&at_neighbour) {
                                    active_neighbours += 1;
                                }
                            }
                        }
                    }

                    if is_new_state_active(cube.contains(&at_position), active_neighbours) {
                        new_cube.insert(at_position);
                    }
                }
            }
        }
        cube = new_cube;
    }

    cube.len()
}

fn part2(cube: &Cube) -> usize {
    let mut min: i8 = 0;
    let mut max: i8 = max_key(cube);
    let mut hypercube = HyperCube::new();
    for (x, y, z) in cube {
        hypercube.insert((*x, *y, *z, 0));
    }

    for _ in 1..=6 {
        let mut new_hypercube = HyperCube::new();
        min -= 1;
        max += 1;

        for x in min..=max {
            for y in min..=max {
                for z in min..=max {
                    for w in min..=max {
                        let at_position = (x, y, z, w);

                        let mut active_neighbours = 0;
                        for nx in x - 1..=x + 1 {
                            for ny in y - 1..=y + 1 {
                                for nz in z - 1..=z + 1 {
                                    for nw in w - 1..=w + 1 {
                                        let at_neighbour = (nx, ny, nz, nw);
                                        if at_neighbour == at_position {
                                            continue;
                                        }
                                        if hypercube.contains(&at_neighbour) {
                                            active_neighbours += 1;
                                        }
                                    }
                                }
                            }
                        }

                        if is_new_state_active(hypercube.contains(&at_position), active_neighbours)
                        {
                            new_hypercube.insert(at_position);
                        }
                    }
                }
            }
        }
        hypercube = new_hypercube;
    }

    hypercube.len()
}

pub fn run() {
    let input = include_str!("input/day17.txt");
    let cube = &parse_input(input);
    println!("Day 17/1: {}", part1(cube));
    println!("Day 17/2: {}", part2(cube));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
.#.
..#
###
";

    #[test]
    fn test_day17_parse_input() {
        let cube = parse_input(SAMPLE1);
        assert_eq!(5, cube.len());
        assert!(cube.contains(&(0, 2, 0)));
        assert!(cube.contains(&(1, 0, 0)));
        assert!(cube.contains(&(1, 2, 0)));
        assert!(cube.contains(&(2, 1, 0)));
        assert!(cube.contains(&(2, 2, 0)));
    }

    #[test]
    fn test_day17_part1_sample1() {
        assert_eq!(112, part1(&parse_input(SAMPLE1)));
    }

    #[test]
    fn test_day17_part2_sample1() {
        assert_eq!(848, part2(&parse_input(SAMPLE1)));
    }
}
