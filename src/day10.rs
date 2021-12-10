use aoc_runner_derive::{aoc, aoc_generator};

use phf::phf_map;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

const BRACE_MAP: phf::Map<char, char> = phf_map! {'(' => ')', '[' => ']', '{' => '}', '<' => '>'};
const ERROR_SCORE_MAP: phf::Map<char, usize> =
    phf_map! {')' => 3, ']' => 57, '}' => 1197, '>' => 25137};

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let mut score: usize = 0;

    for line in input.lines() {
        let mut stack: Vec<char> = vec![];

        for c in line.chars() {
            if BRACE_MAP.contains_key(&c) {
                stack.push(c);
            } else if BRACE_MAP.get(stack.last().unwrap()) == Some(&c) {
                stack.pop();
            } else {
                score += *ERROR_SCORE_MAP.get(&c).unwrap();
                break;
            }
        }
    }

    score
}

const AUTOCOMPLETE_SCORE_MAP: phf::Map<char, usize> =
    phf_map! {'(' => 1, '[' => 2, '{' => 3, '<' => 4};

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut line_scores: Vec<usize> = vec![];

    for line in input.lines() {
        let mut stack: Vec<char> = vec![];
        let mut is_corrupted = false;

        for c in line.chars() {
            if BRACE_MAP.contains_key(&c) {
                stack.push(c);
            } else if BRACE_MAP.get(stack.last().unwrap()) == Some(&c) {
                stack.pop();
            } else {
                is_corrupted = true;
                break;
            }
        }

        if is_corrupted {
            continue;
        }

        let mut line_score: usize = 0;

        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            line_score *= 5;
            line_score += *AUTOCOMPLETE_SCORE_MAP.get(&c).unwrap();
        }

        line_scores.push(line_score);
    }

    let median_idx = line_scores.len() / 2;
    line_scores.select_nth_unstable(median_idx);
    line_scores[median_idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 26397);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 288957);
    }
}
