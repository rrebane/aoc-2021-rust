use aoc_runner_derive::{aoc, aoc_generator};

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT+ }
    input = _{ (","? ~ number)+ ~ NEWLINE?  }
"#]
struct InputParser;

fn token_to_number(token: &Pair<Rule>) -> u8 {
    assert_eq!(token.as_rule(), Rule::number);
    token
        .as_str()
        .parse::<u8>()
        .unwrap_or_else(|e| panic!("{}", e))
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<u8> {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));
    input_tokens.map(|token| token_to_number(&token)).collect()
}

fn count_fish(fishes: &[u8], days: usize) -> usize {
    let mut count: [usize; 9] = [0; 9];

    for &fish in fishes {
        count[fish as usize] += 1;
    }

    for _day in 0..days {
        count.rotate_left(1);
        count[6] += count[8];
    }

    count.iter().sum()
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    count_fish(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    count_fish(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[3, 4, 3, 1, 2]), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[3, 4, 3, 1, 2]), 26984457539);
    }
}
