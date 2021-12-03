use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    up = { "up" }
    down = { "down" }
    forward = { "forward" }
    number = { ASCII_DIGIT+ }
    instruction = { (up | down | forward) ~ " "+ ~ number }
    input = _{ (instruction ~ NEWLINE?)+ }
"#]
struct InputParser;

#[derive(Debug)]
enum Direction {
    Up(isize),
    Down(isize),
    Forward(isize),
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Direction> {
    let input_tokens = InputParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));

    let mut directions = vec![];

    for token in input_tokens {
        match token.as_rule() {
            Rule::instruction => {
                let mut inner_tokens = token.into_inner();
                let direction_token = inner_tokens.next().unwrap();
                let distance_token = inner_tokens.next().unwrap();
                assert_eq!(distance_token.as_rule(), Rule::number);
                let distance = distance_token.as_str().parse::<isize>().unwrap_or_else(|e| panic!("{}", e));

                match direction_token.as_rule() {
                    Rule::up => directions.push(Direction::Up(distance)),
                    Rule::down => directions.push(Direction::Down(distance)),
                    Rule::forward => directions.push(Direction::Forward(distance)),
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    directions
}

#[aoc(day2, part1)]
fn part1(input: &[Direction]) -> isize {
    let mut depth: isize = 0;
    let mut horizontal: isize = 0;

    for direction in input {
        match direction {
            Direction::Up(distance) => depth -= distance,
            Direction::Down(distance) => depth += distance,
            Direction::Forward(distance) => horizontal += distance,
        }
    }

    horizontal * depth
}

#[aoc(day2, part2)]
fn part2(input: &[Direction]) -> isize {
    let mut aim: isize = 0;
    let mut depth: isize = 0;
    let mut horizontal: isize = 0;

    for direction in input {
        match direction {
            Direction::Up(distance) => aim -= distance,
            Direction::Down(distance) => aim += distance,
            Direction::Forward(distance) => {
                horizontal += distance;
                depth += aim * distance;
            },
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[
                Direction::Forward(5),
                Direction::Down(5),
                Direction::Forward(8),
                Direction::Up(3),
                Direction::Down(8),
                Direction::Forward(2)
            ]),
            150
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[
                Direction::Forward(5),
                Direction::Down(5),
                Direction::Forward(8),
                Direction::Up(3),
                Direction::Down(8),
                Direction::Forward(2)
            ]),
            900
        );
    }
}
