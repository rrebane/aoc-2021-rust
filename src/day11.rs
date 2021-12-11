use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;
use std::collections::HashSet;

use crate::util;

#[derive(Parser)]
#[grammar_inline = r#"
    cell = { ASCII_DIGIT }
    input = _{ NEWLINE? ~ ((" "+)? ~ cell+ ~ NEWLINE?)+ }
"#]
struct InputParser;

const ROW_SIZE: usize = 10;
const COL_SIZE: usize = 10;
const GRID_SIZE: usize = ROW_SIZE * COL_SIZE;

type Grid = [u8; GRID_SIZE];

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Grid {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));

    let mut grid: Grid = [0; GRID_SIZE];

    for (i, token) in input_tokens.into_iter().enumerate() {
        grid[i] =
            util::parse::token_to_number(token, Rule::cell).unwrap_or_else(|e| panic!("{}", e));
    }

    grid
}

fn index_to_coord(index: usize) -> (isize, isize) {
    ((index % ROW_SIZE) as isize, (index / ROW_SIZE) as isize)
}

fn coord_to_index(coordinate: (isize, isize)) -> usize {
    coordinate.1 as usize * ROW_SIZE + coordinate.0 as usize
}

fn neighbor_coordinates(coordinate: (isize, isize)) -> Vec<(isize, isize)> {
    (-1..=1)
        .flat_map(|y| (-1..=1).map(move |x| (coordinate.0 + x, coordinate.1 + y)))
        .filter(|neighbor| *neighbor != coordinate)
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < ROW_SIZE as isize && *y < COL_SIZE as isize)
        .collect()
}

fn simulate_steps(grid: Grid, steps: usize, acc_count: usize) -> usize {
    if steps == 0 {
        return acc_count;
    }

    let mut next_grid: Grid = [0; GRID_SIZE];
    next_grid.clone_from_slice(&grid);

    let mut flashed_indices: HashSet<usize> = HashSet::new();
    let mut to_propagate_indices = vec![];

    for (idx, _level) in grid.iter().enumerate() {
        next_grid[idx] += 1;
        if next_grid[idx] > 9 {
            flashed_indices.insert(idx);
            to_propagate_indices.push(idx);
        }
    }

    while !to_propagate_indices.is_empty() {
        let idx = to_propagate_indices.pop().unwrap();
        let flash_coord = index_to_coord(idx);
        let neighbor_coords = neighbor_coordinates(flash_coord);

        for neighbor_coord in neighbor_coords {
            let neighbor_idx = coord_to_index(neighbor_coord);
            if flashed_indices.contains(&neighbor_idx) {
                continue;
            }

            next_grid[neighbor_idx] += 1;
            if next_grid[neighbor_idx] > 9 {
                flashed_indices.insert(neighbor_idx);
                to_propagate_indices.push(neighbor_idx);
            }
        }

        next_grid[idx] = 0;
    }

    simulate_steps(next_grid, steps - 1, acc_count + flashed_indices.len())
}

#[aoc(day11, part1)]
fn part1(input: &Grid) -> usize {
    simulate_steps(input.to_owned(), 100, 0)
}

fn simulate_until_synchronized(grid: Grid, current_step: usize) -> usize {
    let mut next_grid: Grid = [0; GRID_SIZE];
    next_grid.clone_from_slice(&grid);

    let mut flashed_indices: HashSet<usize> = HashSet::new();
    let mut to_propagate_indices = vec![];

    for (idx, _level) in grid.iter().enumerate() {
        next_grid[idx] += 1;
        if next_grid[idx] > 9 {
            flashed_indices.insert(idx);
            to_propagate_indices.push(idx);
        }
    }

    while !to_propagate_indices.is_empty() {
        let idx = to_propagate_indices.pop().unwrap();
        let flash_coord = index_to_coord(idx);
        let neighbor_coords = neighbor_coordinates(flash_coord);

        for neighbor_coord in neighbor_coords {
            let neighbor_idx = coord_to_index(neighbor_coord);
            if flashed_indices.contains(&neighbor_idx) {
                continue;
            }

            next_grid[neighbor_idx] += 1;
            if next_grid[neighbor_idx] > 9 {
                flashed_indices.insert(neighbor_idx);
                to_propagate_indices.push(neighbor_idx);
            }
        }

        next_grid[idx] = 0;
    }

    if flashed_indices.len() >= GRID_SIZE {
        current_step
    } else {
        simulate_until_synchronized(next_grid, current_step + 1)
    }
}

#[aoc(day11, part2)]
fn part2(input: &Grid) -> usize {
    simulate_until_synchronized(input.to_owned(), 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 1656);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 195);
    }
}
