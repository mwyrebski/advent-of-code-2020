use crate::lib::split_once;
use std::collections::HashMap;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn parse_passport(data: &str) -> Passport {
    let fields: Vec<(String, String)> = data
        .split_whitespace()
        .map(|field| split_once(field, ':').unwrap())
        .map(|(k, v)| (String::from(k), String::from(v)))
        .collect();

    let mut h = HashMap::new();
    for (k, v) in fields {
        h.insert(k, v);
    }

    Passport {
        byr: h.get("byr").map(String::from),
        iyr: h.get("iyr").map(String::from),
        eyr: h.get("eyr").map(String::from),
        hgt: h.get("hgt").map(String::from),
        hcl: h.get("hcl").map(String::from),
        ecl: h.get("ecl").map(String::from),
        pid: h.get("pid").map(String::from),
        cid: h.get("cid").map(String::from),
    }
}

fn is_valid(p: &Passport) -> bool {
    p.byr.is_some()
        && p.iyr.is_some()
        && p.eyr.is_some()
        && p.hgt.is_some()
        && p.hcl.is_some()
        && p.ecl.is_some()
        && p.pid.is_some()
        && (p.cid.is_some() || p.cid.is_none())
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(parse_passport).collect()
}

fn part1(passports: &Vec<Passport>) -> usize {
    passports.iter().map(is_valid).map(|b| b as usize).sum()
}

fn has_valid_fields(p: &Passport) -> bool {
    static VALID_ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let allowed_color_chars: Vec<char> = "0123456789abcdef".chars().collect();
    let is_between = |n, a, b| a <= n && n <= b;
    let to_i32 = |x: &String| x.parse::<i32>().unwrap();

    p.byr
        .as_ref()
        .map(|x| is_between(to_i32(x), 1920, 2002))
        .unwrap_or_default()
        && p.iyr
            .as_ref()
            .map(|x| is_between(to_i32(x), 2010, 2020))
            .unwrap_or_default()
        && p.eyr
            .as_ref()
            .map(|x| is_between(to_i32(x), 2020, 2030))
            .unwrap_or_default()
        && p.hgt
            .as_ref()
            .map(|x| {
                let n = to_i32(&x.replace(|c: char| c.is_ascii_alphabetic(), ""));
                if x.ends_with("cm") {
                    is_between(n, 150, 193)
                } else if x.ends_with("in") {
                    is_between(n, 59, 76)
                } else {
                    false
                }
            })
            .unwrap_or_default()
        && p.hcl
            .as_ref()
            .map(|x| {
                x.len() == 7
                    && x.starts_with('#')
                    && x[1..]
                        .replace(|c: char| allowed_color_chars.contains(&c), "")
                        .len()
                        == 0
            })
            .unwrap_or_default()
        && p.pid
            .as_ref()
            .map(|x| x.len() == 9 && !x.contains(char::is_alphabetic))
            .unwrap_or_default()
        && p.ecl
            .as_ref()
            .map(|x| VALID_ECL.contains(&x.as_str()))
            .unwrap_or_default()
}

fn part2(passports: &Vec<Passport>) -> usize {
    passports
        .iter()
        .map(has_valid_fields)
        .map(|b| b as usize)
        .sum()
}

pub fn run() {
    let input = include_str!("input/day4.txt");
    let passports = &parse_input(input);
    println!("Day 4/1: {}", part1(passports));
    println!("Day 4/2: {}", part2(passports));
}
