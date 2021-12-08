use aoc_runner_derive::{aoc, aoc_generator};

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    segment = { ("a" | "b" | "c" | "d" | "e" | "f" | "g") }
    digit = { segment+ }
    patterns = { (" "? ~ digit){10} }
    output = { (" "? ~ digit){4} }
    entry = { patterns ~ " | " ~ output }
    input = _{ NEWLINE? ~ ((" "+)? ~ entry ~ NEWLINE?)+ }
"#]
struct InputParser;

#[derive(Debug, PartialEq, PartialOrd)]
struct Entry {
    patterns: [u8; 10],
    output: [u8; 4],
}

fn digit_to_number(token: Pair<Rule>) -> u8 {
    let mut number: u8 = 0;

    assert_eq!(token.as_rule(), Rule::digit);
    for segment_token in token.into_inner() {
        assert_eq!(segment_token.as_rule(), Rule::segment);
        match segment_token.as_str() {
            "a" => number |= 0b1000000,
            "b" => number |= 0b100000,
            "c" => number |= 0b10000,
            "d" => number |= 0b1000,
            "e" => number |= 0b100,
            "f" => number |= 0b10,
            "g" => number |= 0b1,
            _ => unreachable!(),
        }
    }

    number
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Entry> {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));

    let mut entries = vec![];

    for token in input_tokens {
        assert_eq!(token.as_rule(), Rule::entry);

        let mut entry_tokens = token.into_inner();
        let pattern_tokens = entry_tokens.next().unwrap();
        assert_eq!(pattern_tokens.as_rule(), Rule::patterns);

        let output_tokens = entry_tokens.next().unwrap();
        assert_eq!(output_tokens.as_rule(), Rule::output);

        let mut patterns = [0; 10];
        let mut output = [0; 4];

        for (i, digit_token) in pattern_tokens.into_inner().enumerate() {
            patterns[i] = digit_to_number(digit_token);
        }

        for (i, digit_token) in output_tokens.into_inner().enumerate() {
            output[i] = digit_to_number(digit_token);
        }

        entries.push(Entry { patterns, output })
    }

    entries
}

fn is_unique_segment_count(digit: u8) -> bool {
    let bit_count = digit.count_ones();
    bit_count == 2 || bit_count == 4 || bit_count == 3 || bit_count == 7
}

#[aoc(day8, part1)]
fn part1(input: &[Entry]) -> usize {
    let mut count: usize = 0;

    for entry in input {
        for digit in entry.output {
            if is_unique_segment_count(digit) {
                count += 1;
            }
        }
    }

    count
}

fn map_segment(solution: &[u8], pos: usize, count: usize) -> (usize, u8) {
    let val: u8 = 0b1 << (6 - pos);

    match count {
        // e
        4 => (4, val),
        // b
        6 => (1, val),
        // d or g
        7 => {
            // Only d is in 4
            if solution[4] & (0b1 << (6 - pos)) != 0 {
                (3, val)
            } else {
                (6, val)
            }
        }
        // a or c
        8 => {
            // Only c is in 1
            if solution[1] & (0b1 << (6 - pos)) != 0 {
                (2, val)
            } else {
                (0, val)
            }
        }
        // f
        9 => (5, val),
        _ => unreachable!(),
    }
}

fn solve_digits(patterns: &[u8]) -> [u8; 10] {
    let mut solution: [u8; 10] = [0; 10];
    let mut segments: [u8; 7] = [0; 7];

    for digit in patterns {
        let segement_count = digit.count_ones();
        match segement_count {
            2 => solution[1] = *digit,
            4 => solution[4] = *digit,
            3 => solution[7] = *digit,
            7 => solution[8] = *digit,
            _ => (),
        }
    }

    let mut segment_occurrences: [usize; 7] = [0; 7];
    for digit in patterns {
        segment_occurrences[0] += if (*digit & 0b1000000) as usize != 0 {
            1
        } else {
            0
        };
        segment_occurrences[1] += if (*digit & 0b100000) as usize != 0 {
            1
        } else {
            0
        };
        segment_occurrences[2] += if (*digit & 0b10000) as usize != 0 {
            1
        } else {
            0
        };
        segment_occurrences[3] += if (*digit & 0b1000) as usize != 0 {
            1
        } else {
            0
        };
        segment_occurrences[4] += if (*digit & 0b100) as usize != 0 { 1 } else { 0 };
        segment_occurrences[5] += if (*digit & 0b10) as usize != 0 { 1 } else { 0 };
        segment_occurrences[6] += if (*digit & 0b1) as usize != 0 { 1 } else { 0 };
    }

    // Map segments
    for (pos, segment_occurrence) in segment_occurrences.iter().enumerate() {
        let (map_pos, map_value) = map_segment(&solution, pos, *segment_occurrence);
        segments[map_pos] = map_value;
    }

    // Map digits
    for digit in patterns {
        let segement_count = digit.count_ones();
        match segement_count {
            5 => {
                // 3 has c and f
                // 2 has c
                // 5 has f
                if *digit & segments[2] != 0 && *digit & segments[5] != 0 {
                    solution[3] = *digit;
                } else if *digit & segments[2] != 0 {
                    solution[2] = *digit;
                } else {
                    solution[5] = *digit;
                }
            }
            6 => {
                // 0 has c and e
                // 6 has e
                // 9 has c
                if *digit & segments[2] != 0 && *digit & segments[4] != 0 {
                    solution[0] = *digit;
                } else if *digit & segments[4] != 0 {
                    solution[6] = *digit;
                } else {
                    solution[9] = *digit;
                }
            }
            _ => (),
        }
    }

    solution
}

fn solve_output(solution: &[u8; 10], output: &[u8; 4]) -> usize {
    let mut number: usize = 0;

    for (pos, digit) in output.iter().enumerate() {
        number += solution.iter().position(|&val| val == *digit).unwrap()
            * usize::pow(10, 3 - pos as u32);
    }

    number
}

#[aoc(day8, part2)]
fn part2(input: &[Entry]) -> usize {
    let mut output: usize = 0;

    for entry in input {
        let solution = solve_digits(&entry.patterns);
        output += solve_output(&solution, &entry.output);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn input_parse() {
        assert_eq!(
            parse_input(&"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"),
            vec![
                Entry {
                    patterns: [0b1111111, 0b111110, 0b1011011, 0b1111010, 0b1101000, 0b1111110, 0b111111, 0b1100110, 0b1111101, 0b1100000],
                    output: [0b111110, 0b1111010, 0b111110, 0b1111010]
                }
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 61229);
    }
}
