use crate::lib::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Node {
    Literal(char),
    Ref(u8),
    Seq(Vec<Node>),
    Alt(Box<(Node, Node)>),
}

fn parse_rules(rules: &str) -> HashMap<u8, Node> {
    let str2seq = |s: &str| {
        let refs = s
            .split(' ')
            .map(|s| Node::Ref(parse_unwrap(s)))
            .collect::<Vec<Node>>();
        Node::Seq(refs)
    };

    let parsed_rules = rules
        .lines()
        .map(|l| {
            let (id_s, value_s) = split_once_str(l, ": ");
            let id: u8 = parse_unwrap(id_s);
            let node = if value_s.contains('"') {
                let c = value_s.chars().nth(1).unwrap();
                Node::Literal(c)
            } else if value_s.contains('|') {
                let (sa, sb) = split_once_str(value_s, " | ");
                let a = str2seq(sa);
                let b = str2seq(sb);
                Node::Alt(Box::new((a, b)))
            } else {
                str2seq(value_s)
            };
            (id, node)
        })
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    for (id, node) in parsed_rules {
        map.insert(id, node);
    }
    map
}

fn parse(input: &str) -> (HashMap<u8, Node>, Vec<String>) {
    let (rules, messages) = split_once_str(input.trim(), "\n\n");
    let messages = messages.lines().map(|l| l.to_string()).collect();
    (parse_rules(rules), messages)
}

fn is_match(rules: &HashMap<u8, Node>, message: &str) -> bool {
    fn check(
        rules: &HashMap<u8, Node>,
        node: &Node,
        msg: &Vec<char>,
        i: usize,
        tab: usize,
    ) -> (bool, usize) {
        if i >= msg.len() {
            return (false, 0);
        }

        match node {
            Node::Literal(c) => (c == &msg[i], 1),
            Node::Ref(id) => check(rules, &rules[id], msg, i, tab + 1),
            Node::Seq(seq) => {
                let mut z = 0;
                let seq_result = seq.iter().all(|seq_node| {
                    let (b, j) = check(rules, seq_node, msg, i + z, tab + 1);
                    z += j;
                    b
                });

                if seq_result {
                    (seq_result, z)
                } else {
                    (seq_result, 0)
                }
            }
            Node::Alt(alt) => {
                let (p, q) = &**alt;
                let (p_result, p_next_i) = check(rules, p, msg, i, tab + 1);
                if p_result {
                    (p_result, p_next_i)
                } else {
                    check(rules, q, msg, i, tab + 1)
                }
            }
        }
    }

    let root = &rules[&0];
    let msg = message.chars().collect::<Vec<char>>();
    let (matches, i) = check(rules, root, &msg, 0, 0);
    matches && i >= message.len()
}

fn part1(data: &(HashMap<u8, Node>, Vec<String>)) -> usize {
    let (rules, messages) = data;
    messages.iter().filter(|m| is_match(rules, m)).count()
}

pub fn run() {
    let input = include_str!("input/day19.txt");
    let parsed = &parse(input);
    println!("Day 19/1: {}", part1(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_RULES1: &'static str = "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"
";

    static SAMPLE1: &'static str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";

    #[test]
    fn test_day19_parse_rules_sample_rules1() {
        let rules = &parse_rules(SAMPLE_RULES1);
        assert_eq!(false, is_match(rules, "a"));
        assert_eq!(false, is_match(rules, "b"));
        assert_eq!(false, is_match(rules, "ab"));
        assert_eq!(false, is_match(rules, "ba"));
        assert_eq!(true, is_match(rules, "aab"));
        assert_eq!(true, is_match(rules, "aba"));
    }

    #[test]
    fn test_day19_part1_sample1() {
        let (rules_input, _) = split_once_str(SAMPLE1, "\n\n");
        let rules = &parse_rules(rules_input);

        assert_eq!(true, is_match(rules, "ababbb"));
        assert_eq!(true, is_match(rules, "abbbab"));
        assert_eq!(false, is_match(rules, "aaaabbb"));

        assert_eq!(2, part1(&parse(SAMPLE1)));
    }
}
