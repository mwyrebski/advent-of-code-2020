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
