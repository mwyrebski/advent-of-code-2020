use crate::lib::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Field {
    name: String,
    rule1: (u32, u32),
    rule2: (u32, u32),
}

impl Field {
    pub fn is_valid(&self, value: u32) -> bool {
        let (a, b) = self.rule1;
        let (c, d) = self.rule2;
        (a <= value && value <= b) || (c <= value && value <= d)
    }
}

#[derive(Debug, PartialEq)]
struct Notes {
    fields: Vec<Field>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Notes {
    pub fn has_valid_field(&self, value: u32) -> bool {
        self.fields.iter().any(|f| f.is_valid(value))
    }
}

fn parse_input<'a>(input: &str) -> Notes {
    let areas = input.split("\n\n").collect::<Vec<&str>>();
    let a_fields = areas[0];
    let a_your = areas[1];
    let a_nearby = areas[2];

    let fields = a_fields
        .lines()
        .map(|line| {
            let (name, rules) = split_once_str(line, ": ");
            let (r1, r2) = split_once_str(rules, " or ");
            let (r1a, r1b) = split_once_char(r1, '-');
            let (r2a, r2b) = split_once_char(r2, '-');

            Field {
                name: name.to_string(),
                rule1: (parse_unwrap(r1a), parse_unwrap(r1b)),
                rule2: (parse_unwrap(r2a), parse_unwrap(r2b)),
            }
        })
        .collect::<Vec<_>>();

    let your = parse_to_vec(a_your.lines().into_iter().last().unwrap().split(','));

    let nearby = a_nearby
        .lines()
        .skip(1)
        .map(|line| parse_to_vec(line.split(',')))
        .collect::<Vec<_>>();

    Notes {
        fields: fields,
        your_ticket: your,
        nearby_tickets: nearby,
    }
}

fn part1(notes: &Notes) -> u32 {
    notes
        .nearby_tickets
        .iter()
        .map(|ticket| ticket.iter().filter(|&&x| !notes.has_valid_field(x)))
        .flatten()
        .sum()
}

fn detect_fields(notes: &Notes) -> HashMap<String, u32> {
    let valid_tickets = notes
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|&x| notes.has_valid_field(x)))
        .collect::<Vec<_>>();

    let is_field_valid_for_all_tickets_by_index =
        |field: &Field, index| valid_tickets.iter().all(|t| field.is_valid(t[index]));

    let mut unmatched_fields = (0..notes.fields.len()).collect::<Vec<usize>>();
    let mut unmatched_indexes = (0..notes.fields.len()).collect::<Vec<usize>>();
    let mut map = HashMap::new();
    loop {
        for f_idx in 0..notes.fields.len() {
            if !unmatched_fields.contains(&f_idx) {
                continue;
            }

            let field = &notes.fields[f_idx];
            let matching_indexes = unmatched_indexes
                .iter()
                .filter(|&&i| is_field_valid_for_all_tickets_by_index(field, i))
                .collect::<Vec<&usize>>();

            if matching_indexes.len() == 1 {
                let i = *matching_indexes[0];
                map.insert(field.name.clone(), notes.your_ticket[i]);
                unmatched_indexes.retain(|&u| u != i);
                unmatched_fields.retain(|&n| n != f_idx);
            }
        }

        if unmatched_fields.is_empty() {
            break;
        }
    }
    map
}

fn part2(notes: &Notes) -> usize {
    detect_fields(notes)
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| *v as usize)
        .product()
}

pub fn run() {
    let input = include_str!("input/day16.txt");
    let notes = &parse_input(input);
    println!("Day 16/1: {}", part1(notes));
    println!("Day 16/2: {}", part2(notes));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    static SAMPLE2: &'static str = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

    #[test]
    fn test_day16_parse_input() {
        let notes = parse_input(SAMPLE1);
        let expected = Notes {
            fields: vec![
                Field {
                    name: "class".to_string(),
                    rule1: (1, 3),
                    rule2: (5, 7),
                },
                Field {
                    name: "row".to_string(),
                    rule1: (6, 11),
                    rule2: (33, 44),
                },
                Field {
                    name: "seat".to_string(),
                    rule1: (13, 40),
                    rule2: (45, 50),
                },
            ],
            your_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };
        assert_eq!(expected, notes);
    }

    #[test]
    fn test_day16_part1_sample1() {
        assert_eq!(71, part1(&parse_input(SAMPLE1)));
    }

    #[test]
    fn test_day16_part2_sample2() {
        let detected = detect_fields(&parse_input(SAMPLE2));
        assert_eq!(detected.len(), 3);
        assert_eq!(detected.get("class"), Some(&12));
        assert_eq!(detected.get("row"), Some(&11));
        assert_eq!(detected.get("seat"), Some(&13));
    }
}
