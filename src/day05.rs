use aoc_runner_derive::{aoc, aoc_generator};

use pest::iterators::Pair;
use pest::Parser;
use std::cmp;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT+ }
    line = { number ~ "," ~ number ~ " -> " ~ number ~ "," ~ number }
    input = _{ (line ~ NEWLINE?)+  }
"#]
struct InputParser;

type Line = [u16; 4];

fn token_to_number(token: Pair<Rule>) -> u16 {
    assert_eq!(token.as_rule(), Rule::number);
    token
        .as_str()
        .parse::<u16>()
        .unwrap_or_else(|e| panic!("{}", e))
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Line> {
    let input_tokens = InputParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));

    let mut lines = vec![];

    for token in input_tokens {
        match token.as_rule() {
            Rule::line => {
                let mut line_tokens = token.into_inner();
                let x1 = token_to_number(line_tokens.next().unwrap());
                let y1 = token_to_number(line_tokens.next().unwrap());
                let x2 = token_to_number(line_tokens.next().unwrap());
                let y2 = token_to_number(line_tokens.next().unwrap());
                lines.push([x1, y1, x2, y2]);
            }
            _ => unreachable!(),
        }
    }

    lines
}

fn largest_coordinate(lines: &[Line]) -> usize {
    let mut largest: u16 = 0;

    for line in lines {
        for &coordinate in line {
            if coordinate > largest {
                largest = coordinate;
            }
        }
    }

    largest as usize
}

fn coord_to_index(x: u16, y: u16, grid_size: usize) -> usize {
    y as usize * grid_size + x as usize
}

fn draw_line(grid: &mut [u8], line: &Line, grid_size: usize) {
    match line {
        // Horizontal
        &[x1, y1, x2, y2] if y1 == y2 => {
            let min = cmp::min(x1, x2);
            let max = cmp::max(x1, x2);

            for x in min..=max {
                let index = coord_to_index(x, y1, grid_size);
                grid[index] += 1;
            }
        }
        // Vertical
        &[x1, y1, x2, y2] if x1 == x2 => {
            let min = cmp::min(y1, y2);
            let max = cmp::max(y1, y2);

            for y in min..=max {
                let index = coord_to_index(x1, y, grid_size);
                grid[index] += 1;
            }
        }
        // Other
        _ => (),
    }
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> usize {
    let grid_size = largest_coordinate(input) + 1;
    let mut grid = vec![0; grid_size * grid_size];

    for line in input {
        draw_line(&mut grid, line, grid_size);
    }

    let mut count: usize = 0;

    for cell in grid {
        if cell >= 2 {
            count += 1;
        }
    }

    count
}

fn draw_line2(grid: &mut [u8], line: &Line, grid_size: usize) {
    match line {
        // Horizontal
        &[x1, y1, x2, y2] if y1 == y2 => {
            let min = cmp::min(x1, x2);
            let max = cmp::max(x1, x2);

            for x in min..=max {
                let index = coord_to_index(x, y1, grid_size);
                grid[index] += 1;
            }
        }
        // Vertical
        &[x1, y1, x2, y2] if x1 == x2 => {
            let min = cmp::min(y1, y2);
            let max = cmp::max(y1, y2);

            for y in min..=max {
                let index = coord_to_index(x1, y, grid_size);
                grid[index] += 1;
            }
        }
        // Diagonal
        &[x1, y1, x2, y2] => {
            let mut x = x1;
            let mut y = y1;

            loop {
                let index = coord_to_index(x, y, grid_size);
                grid[index] += 1;
                if x == x2 && y == y2 {
                    break;
                }
                if x1 < x2 {
                    x += 1
                } else {
                    x -= 1
                };
                if y1 < y2 {
                    y += 1
                } else {
                    y -= 1
                };
            }
        }
    }
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> usize {
    let grid_size = largest_coordinate(input) + 1;
    let mut grid = vec![0; grid_size * grid_size];

    for line in input {
        draw_line2(&mut grid, line, grid_size);
    }

    let mut count: usize = 0;

    for cell in grid {
        if cell >= 2 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[
                [0, 9, 5, 9],
                [8, 0, 0, 8],
                [9, 4, 3, 4],
                [2, 2, 2, 1],
                [7, 0, 7, 4],
                [6, 4, 2, 0],
                [0, 9, 2, 9],
                [3, 4, 1, 4],
                [0, 0, 8, 8],
                [5, 5, 8, 2]
            ]),
            5
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[
                [0, 9, 5, 9],
                [8, 0, 0, 8],
                [9, 4, 3, 4],
                [2, 2, 2, 1],
                [7, 0, 7, 4],
                [6, 4, 2, 0],
                [0, 9, 2, 9],
                [3, 4, 1, 4],
                [0, 0, 8, 8],
                [5, 5, 8, 2]
            ]),
            12
        );
    }
}
