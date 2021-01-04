use std::fmt;

#[derive(Debug, Clone)]
struct Tile {
    id: i64,
    data: Vec<Vec<bool>>,
    all_hashes: Vec<i32>,
    top_hash: i32,
    right_hash: i32,
    bottom_hash: i32,
    left_hash: i32,
}

trait Image {
    fn new(id: i64, data: Vec<Vec<bool>>) -> Self
    where
        Self: Sized;
    fn id(&self) -> i64;
    fn data(&self) -> &Vec<Vec<bool>>;

    fn width(&self) -> usize {
        self.data()[0].len()
    }
    fn height(&self) -> usize {
        self.data().len()
    }
    fn get(&self, x: usize, y: usize) -> bool {
        self.data()[y][x]
    }

    fn rotate_right(&self) -> Self
    where
        Self: Sized,
    {
        let mut new_data = Vec::new();
        for x in 0..self.width() {
            let mut new_row = Vec::new();
            for y in (0..self.height()).rev() {
                new_row.push(self.get(x, y));
            }
            new_data.push(new_row);
        }
        Self::new(self.id(), new_data)
    }

    fn flip_horizontally(&self) -> Self
    where
        Self: Sized,
    {
        let mut new_data = Vec::new();
        for y in 0..self.height() {
            let mut new_row = Vec::new();
            for x in (0..self.width()).rev() {
                new_row.push(self.get(x, y));
            }
            new_data.push(new_row);
        }
        Self::new(self.id(), new_data)
    }

    fn contains(&self, inner: &dyn Image) -> bool {
        let contains_in_location = |ix: usize, iy: usize| {
            for my in 0..inner.height() {
                for mx in 0..inner.width() {
                    if inner.get(mx, my) && !self.get(ix + mx, iy + my) {
                        return false;
                    }
                }
            }
            true
        };

        for iy in 0..self.height() - inner.height() {
            for ix in 0..self.width() - inner.width() {
                if contains_in_location(ix, iy) {
                    return true;
                }
            }
        }
        return false;
    }

    fn remove(&self, other: &dyn Image) -> Self
    where
        Self: Sized,
    {
        let contains_in_location = |ix: usize, iy: usize| {
            for my in 0..other.height() {
                for mx in 0..other.width() {
                    if other.get(mx, my) && !self.get(ix + mx, iy + my) {
                        return false;
                    }
                }
            }
            true
        };

        let mut new_data = self.data().clone();
        for iy in 0..self.height() - other.height() {
            for ix in 0..self.width() - other.width() {
                if contains_in_location(ix, iy) {
                    for my in 0..other.height() {
                        for mx in 0..other.width() {
                            if other.get(mx, my) {
                                new_data[iy + my][ix + mx] = false;
                            }
                        }
                    }
                }
            }
        }
        Self::new(self.id(), new_data)
    }

    fn count_by_value(&self, value: bool) -> usize {
        let mut count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get(x, y) == value {
                    count += 1;
                }
            }
        }
        count
    }

    fn format_data(&self) -> String {
        self.data()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn iter_variations(&self) -> ImageVariationsIter<Self>
    where
        Self: Clone,
    {
        ImageVariationsIter {
            image: self.clone(),
            pos: 0,
        }
    }
}

struct ImageVariationsIter<T>
where
    T: Image + Clone,
{
    image: T,
    pos: u8,
}

impl<T: Image + Clone> Iterator for ImageVariationsIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.pos;
        self.pos += 1;
        match p {
            0..=3 | 5..=8 => {
                self.image = self.image.rotate_right();
                Some(self.image.clone())
            }
            4 => {
                self.image = self.image.flip_horizontally();
                Some(self.image.clone())
            }
            _ => None,
        }
    }
}

impl Image for Tile {
    fn new(id: i64, data: Vec<Vec<bool>>) -> Tile {
        let top = &data[0];
        let right = &data.iter().map(|row| row[row.len() - 1]).collect();
        let bottom = &data.last().unwrap();
        let left = &data.iter().map(|row| row[0]).collect();

        let top_hash = Tile::calc_border_hash(top);
        let right_hash = Tile::calc_border_hash(right);
        let bottom_hash = Tile::calc_border_hash(bottom);
        let left_hash = Tile::calc_border_hash(left);

        let all_hashes = vec![
            top_hash,
            right_hash,
            bottom_hash,
            left_hash,
            Tile::calc_border_rev_hash(top),
            Tile::calc_border_rev_hash(right),
            Tile::calc_border_rev_hash(bottom),
            Tile::calc_border_rev_hash(left),
        ];

        Tile {
            id,
            data,
            all_hashes,
            top_hash,
            right_hash,
            bottom_hash,
            left_hash,
        }
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn data(&self) -> &Vec<Vec<bool>> {
        &self.data
    }
}

impl Tile {
    #[inline]
    fn set_bit(value: bool, index: usize) -> i32 {
        (value as i32) << index
    }

    fn calc_border_rev_hash(vec: &Vec<bool>) -> i32 {
        vec.iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | Tile::set_bit(*b, i))
    }

    fn calc_border_hash(vec: &Vec<bool>) -> i32 {
        vec.iter()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | Tile::set_bit(*b, i))
    }

    pub fn any_side_matches(&self, other: &Tile) -> bool {
        self.all_hashes.iter().any(|h| other.all_hashes.contains(h))
    }

    pub fn count_matching_sides(&self, other_tiles: &Vec<Tile>) -> usize {
        other_tiles
            .iter()
            .map(|t| t.id != self.id && self.any_side_matches(t))
            .filter(|&b| b)
            .count()
    }

    pub fn match_left_hash(&self, target: i32) -> Tile {
        for var_tile in self.iter_variations() {
            if var_tile.left_hash == target {
                return var_tile;
            }
        }
        panic!("No left_hash match found!");
    }
    pub fn match_top_hash(&self, target: i32) -> Tile {
        for var_tile in self.iter_variations() {
            if var_tile.top_hash == target {
                return var_tile;
            }
        }
        panic!("No top_hash match found!");
    }

    fn parse(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = lines
            .next()
            .unwrap()
            .replace(|c| !char::is_numeric(c), "")
            .parse::<i64>()
            .unwrap();
        let data = lines
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        Tile::new(id, data)
    }
}

struct ActualImage {
    data: Vec<Vec<bool>>,
}

impl Image for ActualImage {
    fn new(_id: i64, data: Vec<Vec<bool>>) -> Self {
        ActualImage { data }
    }
    fn id(&self) -> i64 {
        -1
    }
    fn data(&self) -> &Vec<Vec<bool>> {
        &self.data
    }
}

impl ActualImage {
    pub fn from_tiles(tiles: &Vec<Vec<Tile>>) -> ActualImage {
        let mut data = Vec::new();

        for tiles_row in tiles {
            let width = tiles_row[0].width();
            let height = tiles_row[0].height();

            for y in 1..height - 1 {
                let mut img_row = Vec::new();
                for tile in tiles_row {
                    for x in 1..width - 1 {
                        img_row.push(tile.get(x, y));
                    }
                }
                data.push(img_row);
            }
        }

        ActualImage::new(-1, data)
    }
}

#[derive(Clone)]
struct Monster {
    data: Vec<Vec<bool>>,
}

impl Image for Monster {
    fn new(_: i64, data: Vec<Vec<bool>>) -> Self {
        Monster { data }
    }
    fn id(&self) -> i64 {
        -2
    }
    fn data(&self) -> &Vec<Vec<bool>> {
        &self.data
    }
}

impl Default for Monster {
    fn default() -> Self {
        const STR: [&'static str; 3] = [
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ];
        let data: Vec<Vec<bool>> = STR
            .iter()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        Self::new(-2, data)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tile {}:\n{}", self.id, self.format_data())
    }
}

impl fmt::Display for ActualImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.format_data())
    }
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.format_data())
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input.trim().split("\n\n").map(Tile::parse).collect()
}

fn part1(data: &Vec<Tile>) -> i64 {
    data.iter()
        .filter(|t| t.count_matching_sides(data) == 2)
        .map(|tile| tile.id as i64)
        .product()
}

fn part2(data: &Vec<Tile>) -> usize {
    let corners = data
        .iter()
        .filter(|t| t.count_matching_sides(data) == 2)
        .collect::<Vec<&Tile>>();

    let (top_left, mut rest) = {
        let mut corner_tile = corners[0].clone();
        let others = data
            .into_iter()
            .filter(|x| x.id != corner_tile.id)
            .collect::<Vec<_>>();

        let is_matching_others_by_hash = |h| others.iter().any(|o| o.all_hashes.contains(&h));

        while is_matching_others_by_hash(corner_tile.top_hash)
            || is_matching_others_by_hash(corner_tile.left_hash)
        {
            corner_tile = corner_tile.rotate_right();
        }

        (corner_tile, others)
    };

    let mut first_row = Vec::new();
    let mut prev_right_hash = top_left.right_hash;
    first_row.push(top_left);
    loop {
        let next_opt = rest
            .iter()
            .find(|x| x.all_hashes.contains(&prev_right_hash));

        if let Some(next) = next_opt {
            let next = next.match_left_hash(prev_right_hash);
            prev_right_hash = next.right_hash;
            rest.retain(|x| x.id != next.id);
            first_row.push(next);
        } else {
            break;
        }
    }

    let mut prev_row = first_row;
    let mut rows = vec![prev_row.clone()];

    while !rest.is_empty() {
        let mut row = Vec::new();
        for i in 0..prev_row.len() {
            let prev_hash = prev_row[i].bottom_hash;

            let next = rest
                .iter()
                .find(|x| x.all_hashes.contains(&prev_hash))
                .unwrap();
            let next = next.match_top_hash(prev_hash);
            rest.retain(|x| x.id != next.id);
            row.push(next);
        }

        rows.push(row.clone());
        prev_row = row;
    }

    let img = ActualImage::from_tiles(&rows);

    let img_without_monster = {
        let mut result = None;
        for var_monster in Monster::default().iter_variations() {
            if img.contains(&var_monster) {
                result = Some(img.remove(&var_monster));
                break;
            }
        }
        result
    }
    .unwrap();

    let habitat = img_without_monster.count_by_value(true);
    habitat
}

pub fn run() {
    let input = include_str!("input/day20.txt");
    let parsed = &parse(input);
    println!("Day 20/1: {}", part1(parsed));
    println!("Day 20/2: {}", part2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_part2_sample1() {
        assert_eq!(273, part2(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day20_image_rotate_right() {
        let t = Tile::parse(TINY_TILE);

        assert_eq!(
            "\
Tile 123:
###
#..
#..
",
            format!("{}", t.rotate_right())
        );
    }

    #[test]
    fn test_day20_image_flip_horizontally() {
        let t = Tile::parse(TINY_TILE);

        assert_eq!(
            "\
Tile 123:
..#
..#
###
",
            format!("{}", t.flip_horizontally())
        );
    }

    #[test]
    fn test_day20_default_monster() {
        assert_eq!(
            vec!(
                "..................#.",
                "#....##....##....###",
                ".#..#..#..#..#..#...",
                "",
            )
            .join("\n"),
            format!("{}", Monster::default())
        );
    }

    #[test]
    fn test_day20_part1_sample1() {
        assert_eq!(20899048083289, part1(&parse(SAMPLE1)));
    }

    static TINY_TILE: &'static str = "\
Tile 123:
#..
#..
###
";

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
