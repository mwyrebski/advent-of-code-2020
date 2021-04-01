use crate::lib::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Literal(char),
    Ref(u8),
    Rec(u8),
    Seq(Vec<Rule>),
    Alt(Vec<Rule>),
}

fn parse_rules(rules: &str) -> HashMap<u8, Rule> {
    let parse_refs = |id: u8, s: &str| {
        let refs: Vec<Rule> = s
            .split(' ')
            .map(|s| {
                let v = parse_unwrap(s);
                if v == id {
                    Rule::Rec(v)
                } else {
                    Rule::Ref(v)
                }
            })
            .collect();
        Rule::Seq(refs)
    };

    rules
        .lines()
        .map(|l| {
            let (id_s, value_s) = split_once_str(l, ": ");
            let id = parse_unwrap(id_s);
            let rule = if value_s.contains('"') {
                Rule::Literal(value_s.chars().nth(1).unwrap())
            } else if value_s.contains('|') {
                Rule::Alt(value_s.split(" | ").map(|v| parse_refs(id, v)).collect())
            } else {
                parse_refs(id, value_s)
            };
            (id, rule)
        })
        .collect()
}

fn parse(input: &str) -> (HashMap<u8, Rule>, Vec<String>) {
    let (rules, messages) = split_once_str(input.trim(), "\n\n");
    let messages = messages.lines().map(|l| l.to_string()).collect();
    (parse_rules(rules), messages)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Chr(char),
    Alt(Vec<Vec<Node>>),
}

fn expand(rules: &HashMap<u8, Rule>) -> Vec<Node> {
    fn exp(rules: &HashMap<u8, Rule>, rule: &Rule, rec: usize) -> Vec<Node> {
        if rec > 4 {
            return vec![];
        }
        match rule {
            Rule::Literal(c) => vec![Node::Chr(*c)],
            Rule::Alt(alt) => vec![Node::Alt(alt.iter().map(|n| exp(rules, n, rec)).collect())],
            Rule::Seq(seq) => seq.iter().map(|n| exp(rules, n, rec)).flatten().collect(),
            Rule::Ref(id) => exp(rules, &rules[id], rec),
            Rule::Rec(id) => exp(rules, &rules[id], rec + 1),
        }
    }

    let root = &rules[&0];
    exp(rules, root, 0)
}

fn is_match(nodes: &Vec<Node>, message: &str) -> bool {
    fn check_ex(nodes: &[Node], msg: &[char]) -> bool {
        if let Some((node, rest)) = nodes.split_first() {
            return match node {
                Node::Chr(c) => {
                    if let Some((m, msg_rest)) = msg.split_first() {
                        if c == m {
                            return check_ex(&rest.to_vec(), msg_rest);
                        }
                    }
                    false
                }
                Node::Alt(alt) => alt.iter().any(|n| {
                    let alt_nodes = [n, rest].concat();
                    check_ex(&alt_nodes, msg)
                }),
            };
        }
        msg.is_empty()
    }

    let msg = message.chars().collect::<Vec<char>>();
    check_ex(&nodes, &msg)
}

fn part1(data: &(HashMap<u8, Rule>, Vec<String>)) -> usize {
    let (rules, messages) = data;
    let expanded = expand(&rules);
    messages.iter().filter(|m| is_match(&expanded, m)).count()
}

fn part2(data: &(HashMap<u8, Rule>, Vec<String>)) -> usize {
    let (rules, messages) = data;
    let new_rules = {
        let mut map = rules.clone();
        let updated_rules = parse_rules("8: 42 | 42 8\n11: 42 31 | 42 11 31");
        for (id, rule) in updated_rules {
            map.insert(id, rule);
        }
        map
    };

    let expanded = expand(&new_rules);
    messages.iter().filter(|m| is_match(&expanded, m)).count()
}

pub fn run() {
    let input = include_str!("input/day19.txt");
    let parsed = &parse(input);
    println!("Day 19/1: {}", part1(parsed));
    println!("Day 19/2: {}", part2(parsed));
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

    static SAMPLE2: &'static str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

    #[test]
    fn test_day19_parse_rules_sample_rules1() {
        let rules = &parse_rules(SAMPLE_RULES1);
        let expanded = &expand(rules);
        assert_eq!(false, is_match(expanded, "a"));
        assert_eq!(false, is_match(expanded, "b"));
        assert_eq!(false, is_match(expanded, "ab"));
        assert_eq!(false, is_match(expanded, "ba"));
        assert_eq!(true, is_match(expanded, "aab"));
        assert_eq!(true, is_match(expanded, "aba"));
    }

    #[test]
    fn test_day19_part1_sample1() {
        let rules = &parse_rules(
            "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
",
        );

        assert_eq!(true, is_match(&expand(rules), "ababbb"));
        assert_eq!(true, is_match(&expand(rules), "abbbab"));
        assert_eq!(false, is_match(&expand(rules), "aaaabbb"));

        assert_eq!(2, part1(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day19_part2_extended() {
        // last example does not match because max rec depth is 4
        assert_eq!(
            4,
            part2(&parse(
                "\
0: 3 4
1: \"a\"
2: \"b\"
3: 1 2 | 1 2 3
4: 1 1 1

abaaa
ababaaa
abababaaa
abababababaaa
ababababababaaa
"
            ))
        );
    }
    #[test]
    fn test_day19_part2_simple2() {
        // only first 8 should match
        assert_eq!(
            8,
            part2(&parse(
                "\
0: 3 4 4 4
1: \"a\"
2: \"b\"
3: 1 | 1 1
4: 2 | 2 2

abbb
abbbb
abbbbb
abbbbbb
aabbb
aabbbb
aabbbbb
aabbbbbb
bbb
aaabbb
aabb
abbbbbbb
aabbbbbbb
"
            ))
        );
    }

    #[test]
    fn test_day19_part2_sample2() {
        assert_eq!(12, part2(&parse(SAMPLE2)));
    }
}
