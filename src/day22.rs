use crate::lib::*;
use std::collections::BTreeSet;

type Cards = Vec<u8>;

fn parse(input: &str) -> (Cards, Cards) {
    let (p1str, p2str) = split_once_str(input, "\n\n");
    let p1 = parse_to_vec(p1str.lines().skip(1));
    let p2 = parse_to_vec(p2str.lines().skip(1));
    (p1, p2)
}

fn calc_score(cards: &[u8]) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * *x as usize)
        .sum()
}

fn part1(parsed: &(Cards, Cards)) -> usize {
    let mut player1: Cards = parsed.0.clone().into();
    let mut player2: Cards = parsed.1.clone().into();
    let winning_cards: Cards = loop {
        if player1.is_empty() {
            break player2;
        }
        if player2.is_empty() {
            break player1;
        }
        let c1 = player1.remove(0);
        let c2 = player2.remove(0);
        if c1 > c2 {
            player1.push(c1);
            player1.push(c2);
        } else {
            player2.push(c2);
            player2.push(c1);
        }
    };
    calc_score(&winning_cards)
}

fn part2(parsed: &(Cards, Cards)) -> usize {
    enum Winner {
        Player1(Cards),
        Player2(Cards),
    }
    fn play(player1: &[u8], player2: &[u8]) -> Winner {
        let mut p1 = player1.to_vec();
        let mut p2 = player2.to_vec();
        let mut previous_rounds: BTreeSet<(Cards, Cards)> = BTreeSet::new();
        loop {
            if previous_rounds.contains(&(p1.clone(), p2.clone())) || p2.is_empty() {
                break Winner::Player1(p1);
            } else if p1.is_empty() {
                break Winner::Player2(p2);
            }
            previous_rounds.insert((p1.clone(), p2.clone()));

            let card1 = p1.remove(0);
            let card2 = p2.remove(0);

            match {
                let (c1u, c2u) = (card1 as usize, card2 as usize);
                if p1.len() >= c1u && p2.len() >= c2u {
                    play(&p1[..c1u], &p2[..c2u]) // subgame
                } else if card1 > card2 {
                    Winner::Player1(vec![])
                } else {
                    Winner::Player2(vec![])
                }
            } {
                Winner::Player1(_) => {
                    p1.push(card1);
                    p1.push(card2);
                }
                Winner::Player2(_) => {
                    p2.push(card2);
                    p2.push(card1);
                }
            }
        }
    };

    let (player1, player2) = parsed;
    let winner = play(player1, player2);
    match winner {
        Winner::Player1(cards) | Winner::Player2(cards) => calc_score(&cards),
    }
}

pub fn run() {
    let input = include_str!("input/day22.txt");
    let parsed = &parse(input);
    println!("Day 22/1: {}", part1(parsed));
    println!("Day 22/2: {}", part2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    static TINY_SAMPLE: &'static str = "\
Player 1:
43
19

Player 2:
2
29
14
";

    #[test]
    fn test_day22_part1_sample1() {
        assert_eq!(306, part1(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day22_part2_sample1() {
        assert_eq!(291, part2(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day22_part2_tiny_sample() {
        assert_eq!(105, part2(&parse(TINY_SAMPLE)));
    }
}
