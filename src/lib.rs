pub fn to_lines(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn to_i32s(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

// poor man's implementation of the str::split_once since it's experimental
pub fn split_once(input: &str, delimiter: char) -> Option<(&str, &str)> {
    let v: Vec<&str> = input.splitn(2, delimiter).collect();
    if v.len() == 2 {
        Some((v[0], v[1]))
    } else {
        None
    }
}
