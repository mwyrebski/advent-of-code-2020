use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn part1(groups: &Vec<&str>) -> usize {
    let count_questions_in_group = |group: &&str| {
        let mut group_answers = HashSet::new();
        for ch in group.chars() {
            if char::is_alphabetic(ch) {
                group_answers.insert(ch);
            }
        }
        group_answers.len()
    };
    groups.iter().map(count_questions_in_group).sum()
}

fn part2(groups: &Vec<&str>) -> usize {
    let count_questions_in_group = |group: &&str| {
        let answers_vec = group
            .lines()
            .map(|line| {
                line.chars().fold(HashSet::new(), |mut acc, ch| {
                    acc.insert(ch);
                    acc
                })
            })
            .collect::<Vec<_>>();

        answers_vec
            .iter()
            .skip(1)
            .fold(answers_vec[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            })
            .len()
    };
    groups.iter().map(count_questions_in_group).sum()
}

pub fn run() {
    let input = include_str!("input/day6.txt");
    let groups = &parse_input(input);
    println!("Day 6/1: {}", part1(groups));
    println!("Day 6/2: {}", part2(groups));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Vec<&'static str> {
        vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"]
    }

    #[test]
    fn test_example_part1() {
        let groups = &sample();
        assert_eq!(11, part1(groups));
    }

    #[test]
    fn test_example_part2() {
        let groups = &sample();
        assert_eq!(6, part2(groups));
    }
}
