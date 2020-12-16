use crate::lib::*;

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    rule1: (u32, u32),
    rule2: (u32, u32),
}

#[derive(Debug, PartialEq)]
struct Notes {
    fields: Vec<Field>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn parse_input<'a>(input: &str) -> Notes {
    let sp = input.split("\n\n").collect::<Vec<&str>>();
    let p_fields = sp[0];
    let p_nearby = sp[2];

    let fields = p_fields
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

    let nearby = p_nearby
        .lines()
        .skip(1)
        .map(|line| parse_to_vec(line.split(',')))
        .collect::<Vec<_>>();

    Notes {
        fields: fields,
        nearby_tickets: nearby,
    }
}

fn part1(notes: &Notes) -> u32 {
    let is_valid = |x| {
        notes.fields.iter().any(|field| {
            let (a, b) = field.rule1;
            let (c, d) = field.rule2;
            (a <= x && x <= b) || (c <= x && x <= d)
        })
    };

    notes
        .nearby_tickets
        .iter()
        .map(|ticket| ticket.iter().filter(|&f| !is_valid(*f)))
        .flatten()
        .sum()
}

pub fn run() {
    let input = include_str!("input/day16.txt");
    let notes = &parse_input(input);
    println!("Day 16/1: {}", part1(notes));
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
    fn test_day16_part1_samples() {
        assert_eq!(71, part1(&parse_input(SAMPLE1)));
    }
}
