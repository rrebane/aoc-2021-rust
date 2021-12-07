use aoc_runner_derive::{aoc, aoc_generator};

use pest::iterators::Pair;
use pest::Parser;
use std::cmp;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT+ }
    input = _{ (","? ~ number)+ ~ NEWLINE?  }
"#]
struct InputParser;

fn token_to_number(token: &Pair<Rule>) -> u16 {
    assert_eq!(token.as_rule(), Rule::number);
    token
        .as_str()
        .parse::<u16>()
        .unwrap_or_else(|e| panic!("{}", e))
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<u16> {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));
    input_tokens.map(|token| token_to_number(&token)).collect()
}

fn calculate_fuel_cost(positions: &[u16], cost: fn(u16, u16) -> usize) -> usize {
    let min: u16 = *positions.iter().min().unwrap();
    let max: u16 = *positions.iter().max().unwrap();

    let mut min_distance: usize = usize::MAX;

    for line in min..=max {
        let mut distance: usize = 0;

        for &pos in positions {
            distance += cost(pos, line);
        }

        if distance < min_distance {
            min_distance = distance;
        }
    }

    min_distance
}

#[aoc(day7, part1)]
fn part1(input: &[u16]) -> usize {
    calculate_fuel_cost(input, |a, b| (cmp::max(a, b) - cmp::min(a, b)) as usize)
}

#[aoc(day7, part2)]
fn part2(input: &[u16]) -> usize {
    fn nonlinear_cost(a: u16, b: u16) -> usize {
        let steps = (cmp::max(a, b) - cmp::min(a, b)) as usize;
        (1..=steps).sum()
    }

    calculate_fuel_cost(input, nonlinear_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
    }
}
