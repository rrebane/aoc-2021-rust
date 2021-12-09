use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;

use crate::util;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT+ }
    input = _{ (number ~ NEWLINE?)+  }
"#]
struct InputParser;

#[aoc_generator(day0)]
fn parse_input(input: &str) -> Vec<u32> {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));
    input_tokens
        .map(|token| util::parse::token_to_number(&token, Rule::number))
        .collect()
}

#[aoc(day0, part1)]
fn part1(input: &[u32]) -> usize {
    0
}

#[aoc(day0, part2)]
fn part2(input: &[u32]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[]), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[]), 0);
    }
}
