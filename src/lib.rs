pub fn to_lines(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn to_i32s(input: &str) -> Vec<i32> {
    parse_to_vec(input.trim().split_whitespace())
}

pub fn parse_unwrap<T: std::str::FromStr>(s: &str) -> T {
    s.parse::<T>().ok().unwrap()
}

pub fn parse_to_vec<'a, T: std::str::FromStr>(it: impl Iterator<Item = &'a str>) -> Vec<T> {
    it.map(parse_unwrap).collect::<Vec<T>>()
}

// poor man's implementation of the str::split_once since it's experimental
pub fn split_once_char(input: &str, delimiter: char) -> (&str, &str) {
    vec2tuple(&input.splitn(2, delimiter).collect())
}
pub fn split_once_str<'a>(input: &'a str, delimiter: &str) -> (&'a str, &'a str) {
    vec2tuple(&input.splitn(2, delimiter).collect())
}

fn vec2tuple<'a>(v: &Vec<&'a str>) -> (&'a str, &'a str) {
    if v.len() != 2 {
        panic!("Expected 2 values in a vector");
    }
    (v[0], v[1])
}
