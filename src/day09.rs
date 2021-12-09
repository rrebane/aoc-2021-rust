use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;

use crate::util;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT }
    line = { number+ }
    input = _{ (line ~ NEWLINE?)+  }
"#]
struct InputParser;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));

    let mut grid = vec![];

    for token in input_tokens {
        assert_eq!(token.as_rule(), Rule::line);

        let mut row = vec![];

        for number_token in token.into_inner() {
            row.push(
                util::parse::token_to_number(number_token, Rule::number)
                    .unwrap_or_else(|e| panic!("{}", e)),
            );
        }

        grid.push(row);
    }

    grid
}

fn is_lowest(grid: &[Vec<u8>], width: usize, height: usize, x: usize, y: usize) -> Option<u8> {
    let cell = grid[y][x];

    let is_lowest_cell = match (x, y) {
        (0, 0) => cell < grid[y + 1][x] && cell < grid[y][x + 1],
        (0, y) if y == height - 1 => cell < grid[y - 1][x] && cell < grid[y][x + 1],
        (x, 0) if x == width - 1 => cell < grid[y + 1][x] && cell < grid[y][x - 1],
        (x, y) if x == width - 1 && y == height - 1 => {
            cell < grid[y - 1][x] && cell < grid[y][x - 1]
        }
        (0, y) => cell < grid[y - 1][x] && cell < grid[y + 1][x] && cell < grid[y][x + 1],
        (x, 0) => cell < grid[y + 1][x] && cell < grid[y][x - 1] && cell < grid[y][x + 1],
        (x, y) if x == width - 1 => {
            cell < grid[y - 1][x] && cell < grid[y + 1][x] && cell < grid[y][x - 1]
        }
        (x, y) if y == height - 1 => {
            cell < grid[y - 1][x] && cell < grid[y][x - 1] && cell < grid[y][x + 1]
        }
        _ => {
            cell < grid[y - 1][x]
                && cell < grid[y + 1][x]
                && cell < grid[y][x - 1]
                && cell < grid[y][x + 1]
        }
    };

    if is_lowest_cell {
        Some(cell)
    } else {
        None
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    let mut local_min_score: usize = 0;

    let width = input[0].len();
    let height = input.len();

    for y in 0..height {
        for x in 0..width {
            if let Some(local_height) = is_lowest(input, width, height, x, y) {
                local_min_score += local_height as usize + 1
            }
        }
    }

    local_min_score
}

fn neighbor_coordinates(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    match (x, y) {
        (0, 0) => vec![(y + 1, x), (y, x + 1)],
        (0, y) if y == height - 1 => vec![(y - 1, x), (y, x + 1)],
        (x, 0) if x == width - 1 => vec![(y + 1, x), (y, x - 1)],
        (x, y) if x == width - 1 && y == height - 1 => vec![(y - 1, x), (y, x - 1)],
        (0, y) => vec![(y - 1, x), (y + 1, x), (y, x + 1)],
        (x, 0) => vec![(y + 1, x), (y, x - 1), (y, x + 1)],
        (x, y) if x == width - 1 => vec![(y - 1, x), (y + 1, x), (y, x - 1)],
        (x, y) if y == height - 1 => vec![(y - 1, x), (y, x - 1), (y, x + 1)],
        _ => vec![(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)],
    }
}

fn empty_neighbor_coordinates(
    grid: &[Vec<u8>],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    neighbor_coordinates(width, height, x, y)
        .into_iter()
        .filter(|&(y, x)| grid[y][x] == 0)
        .collect()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    let mut basin_grid = input.to_vec();

    let width = input[0].len();
    let height = input.len();

    // Generate a grid with walls
    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        for x in 0..width {
            if basin_grid[y][x] != 9 {
                basin_grid[y][x] = 0;
            } else {
                basin_grid[y][x] = u8::MAX;
            }
        }
    }

    // Fill the empty areas
    let mut next_fill_index: u8 = 1;

    let mut basin_sizes: Vec<usize> = vec![];

    for y in 0..height {
        for x in 0..width {
            if basin_grid[y][x] == 0 {
                basin_sizes.push(0);
                basin_grid[y][x] = next_fill_index;
                basin_sizes[next_fill_index as usize - 1] += 1;
                let mut coordinates_to_check =
                    empty_neighbor_coordinates(&basin_grid, width, height, x, y);

                while !coordinates_to_check.is_empty() {
                    let (y, x) = coordinates_to_check.pop().unwrap();

                    if basin_grid[y][x] == 0 {
                        basin_grid[y][x] = next_fill_index;
                        basin_sizes[next_fill_index as usize - 1] += 1;
                        coordinates_to_check.append(&mut empty_neighbor_coordinates(
                            &basin_grid,
                            width,
                            height,
                            x,
                            y,
                        ));
                    }
                }

                next_fill_index += 1;
            }
        }
    }

    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
            ]),
            15
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
            ]),
            1134
        );
    }
}
