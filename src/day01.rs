use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT+ }
    input = _{ (number ~ NEWLINE)+ }
"#]
struct InputParser;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    let input_tokens = InputParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));
    input_tokens.map(|token| token_to_number(&token)).collect()
}

fn token_to_number(token: &Pair<Rule>) -> Result<u32, ParseIntError> {
    let token_str = token.as_str();
    token_str.parse::<u32>()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> usize {
    let mut prev_number: Option<u32> = None;
    let mut increase_count: usize = 0;

    for number in input {
        match prev_number {
            Some(prev_number) => increase_count += if prev_number < *number { 1 } else { 0 },
            _ => (),
        }

        prev_number = Some(*number);
    }

    increase_count
}

const WINDOW_SIZE: usize = 3;

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> usize {
    let increase_count = input
        .windows(WINDOW_SIZE)
        .map(|window| window.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum();

    increase_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
