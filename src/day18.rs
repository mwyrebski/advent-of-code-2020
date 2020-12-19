use crate::lib::parse_unwrap;

#[derive(Debug, Clone)]
enum Token {
    Sum(Box<(Token, Token)>),
    Product(Box<(Token, Token)>),
    Number(u64),
    Paren(Box<Token>),
}

#[derive(Debug, Clone)]
enum Lexeme {
    Product,
    Sum,
    Num(u64),
    ParenStart,
    ParenEnd,
}

fn lex(input: &str) -> Vec<Lexeme> {
    input
        .chars()
        .filter_map(|c| match c {
            '0'..='9' => Some(Lexeme::Num(parse_unwrap(&c.to_string()))),
            '+' => Some(Lexeme::Sum),
            '*' => Some(Lexeme::Product),
            '(' => Some(Lexeme::ParenStart),
            ')' => Some(Lexeme::ParenEnd),
            ' ' | '\n' => None,
            _ => {
                panic!(format!("unexpected character '{}'", c));
            }
        })
        .collect::<Vec<_>>()
}

fn parse_from_left(lexemes: &Vec<Lexeme>) -> Token {
    fn parse_expr(tokens: &Vec<Lexeme>, pos: isize) -> (Token, isize) {
        let (node_term, next_pos) = parse_term(tokens, pos);
        match tokens.get(next_pos as usize) {
            Some(&Lexeme::Sum) => {
                let (rhs, i) = parse_expr(tokens, next_pos - 1);
                let product = Token::Sum(Box::new((node_term, rhs)));
                (product, i)
            }
            Some(&Lexeme::Product) => {
                let (rhs, i) = parse_expr(tokens, next_pos - 1);
                let sum = Token::Product(Box::new((node_term, rhs)));
                (sum, i)
            }
            _ => (node_term, next_pos),
        }
    }

    fn parse_term(tokens: &Vec<Lexeme>, pos: isize) -> (Token, isize) {
        match &tokens[pos as usize] {
            &Lexeme::Num(n) => {
                let node = Token::Number(n);
                (node, pos - 1)
            }
            &Lexeme::ParenEnd => {
                let (node, next_pos) = parse_expr(tokens, pos - 1);
                if let Some(&Lexeme::ParenStart) = tokens.get(next_pos as usize) {
                    let paren = Token::Paren(Box::new(node));
                    (paren, next_pos - 1)
                } else {
                    panic!("no matching paren found");
                }
            }
            _ => panic!("unexpected token"),
        }
    }

    // in order to generate syntax tree to be unwind sequentially,
    // it needs to be parsed backwards.
    let (token, _) = parse_expr(&lexemes, lexemes.len() as isize - 1);
    token
}

fn parse_with_sum_before_mul(lexemes: &Vec<Lexeme>) -> Token {
    fn parse_expr(tokens: &Vec<Lexeme>, pos: usize) -> (Token, usize) {
        let (node_term, next_pos) = parse_sub_expr(tokens, pos);
        match tokens.get(next_pos) {
            Some(&Lexeme::Product) => {
                let (rhs, i) = parse_expr(tokens, next_pos + 1);
                let sum = Token::Product(Box::new((node_term, rhs)));
                (sum, i)
            }
            _ => (node_term, next_pos),
        }
    }
    fn parse_sub_expr(tokens: &Vec<Lexeme>, pos: usize) -> (Token, usize) {
        let (node_term, next_pos) = parse_term(tokens, pos);
        match tokens.get(next_pos) {
            Some(&Lexeme::Sum) => {
                let (rhs, i) = parse_sub_expr(tokens, next_pos + 1);
                let product = Token::Sum(Box::new((node_term, rhs)));
                (product, i)
            }
            _ => (node_term, next_pos),
        }
    }
    fn parse_term(tokens: &Vec<Lexeme>, pos: usize) -> (Token, usize) {
        match &tokens[pos] {
            &Lexeme::Num(n) => {
                let node = Token::Number(n);
                (node, pos + 1)
            }
            &Lexeme::ParenStart => {
                let (node, next_pos) = parse_expr(tokens, pos + 1);
                if let Some(&Lexeme::ParenEnd) = tokens.get(next_pos) {
                    let paren = Token::Paren(Box::new(node));
                    (paren, next_pos + 1)
                } else {
                    panic!("no matching paren found");
                }
            }
            _ => panic!("unexpected token"),
        }
    }

    let (token, _) = parse_expr(&lexemes, 0);
    token
}

fn exec(root_token: &Token) -> u64 {
    fn run(item: &Token) -> Token {
        match item {
            z @ Token::Number(_) => z.clone(),
            Token::Paren(x) => run(x),
            Token::Sum(s) => match &**s {
                (Token::Number(l), Token::Number(r)) => Token::Number(l + r),
                (lhs, rhs) => {
                    let sum = Token::Sum(Box::new((run(&lhs), run(&rhs))));
                    run(&sum)
                }
            },
            Token::Product(s) => match &**s {
                (Token::Number(l), Token::Number(r)) => Token::Number(l * r),
                (lhs, rhs) => {
                    let product = Token::Product(Box::new((run(&lhs), run(&rhs))));
                    run(&product)
                }
            },
        }
    }

    match run(&root_token) {
        Token::Number(n) => n,
        _ => panic!(),
    }
}

fn parse_and_exec<F>(lex_lines: &Vec<Vec<Lexeme>>, f: F) -> u64
where
    F: Fn(&Vec<Lexeme>) -> Token,
{
    lex_lines
        .iter()
        .map(|line| f(line))
        .map(|root_token| exec(&root_token))
        .sum()
}

fn part1(lex_lines: &Vec<Vec<Lexeme>>) -> u64 {
    parse_and_exec(lex_lines, parse_from_left)
}

fn part2(lex_lines: &Vec<Vec<Lexeme>>) -> u64 {
    parse_and_exec(lex_lines, parse_with_sum_before_mul)
}

pub fn run() {
    let input = include_str!("input/day18.txt");
    let lex_lines = &input.lines().map(|line| lex(line)).collect::<Vec<_>>();
    println!("Day 18/1: {}", part1(lex_lines));
    println!("Day 18/2: {}", part2(lex_lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";

    static SAMPLE2: &'static str = "\
1 + (2 * 3) + (4 * (5 + 6)) 
2 * 3 + (4 * 5) 
5 + (8 * 3 + 9 + 3 * 4 * 3) 
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) 
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 
";

    fn p1(s: &str) -> u64 {
        let lex_lines = &s.lines().map(|line| lex(line)).collect::<Vec<_>>();
        part1(lex_lines)
    }
    fn p2(s: &str) -> u64 {
        let lex_lines = &s.lines().map(|line| lex(line)).collect::<Vec<_>>();
        part2(lex_lines)
    }

    #[test]
    fn test_day18_part1_sample1() {
        assert_eq!(26 + 437 + 12240 + 13632, p1(&SAMPLE1));
    }

    #[test]
    fn test_day18_part2_sample2() {
        assert_eq!(51 + 46 + 1445 + 669060 + 23340, p2(&SAMPLE2));
    }

    #[test]
    fn test_day18_part1_other_samples() {
        assert_eq!(7, p1("1 + 6"));
        assert_eq!(6, p1("2 * 3"));
        assert_eq!(5, p1("1 * 2 + 3"));
        assert_eq!(21, p1("2 * 8 + 5"));
        assert_eq!(13632, p1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn test_day18_part1_sampl333222() {
        assert_eq!(46, p2("2 * 3 + (4 * 5)"));
        assert_eq!(51, p2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(1445, p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669060, p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(23340, p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}
