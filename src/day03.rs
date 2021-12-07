use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    binary = { ("0" | "1") }
    report = { binary+ }
    input = _{ (report ~ NEWLINE?)+ }
"#]
struct InputParser;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<u16> {
    let input_tokens = InputParser::parse(Rule::input, input).unwrap_or_else(|e| panic!("{}", e));
    input_tokens
        .map(|token| u16::from_str_radix(token.as_str(), 2).unwrap_or_else(|e| panic!("{}", e)))
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[u16]) -> usize {
    let report_acc: u16 = input.iter().fold(0, |acc, report| acc | report);
    let report_size: usize = 16 - report_acc.leading_zeros() as usize;

    let mut frequency = vec![0; report_size];

    for report in input {
        for i in 0..report_size {
            frequency[report_size - i - 1] += if ((report >> i) & 0b1) == 0b1 { 1 } else { 0 };
        }
    }

    let gamma_rate: u16 = frequency
        .into_iter()
        .map(|count| if count >= (input.len() / 2) { 1 } else { 0 })
        .fold(0, |acc, val| (acc << 1) + val);

    let epsilon_rate: u16 = !gamma_rate & (!(0_u16) >> (16 - report_size));

    gamma_rate as usize * epsilon_rate as usize
}

#[allow(clippy::ptr_arg)]
fn common_bit_at(
    values: &Vec<&u16>,
    idx: usize,
    predicate: fn(usize, usize) -> bool,
) -> (u16, u16) {
    let mask: u16 = 0b1 << idx;

    let mut bit_count: usize = 0;
    let mut total_count: usize = 0;

    for &val in values {
        bit_count += if val & mask != 0 { 1 } else { 0 };
        total_count += 1;
    }

    if predicate(bit_count, total_count) {
        (mask, mask)
    } else {
        (0, mask)
    }
}

#[allow(clippy::ptr_arg)]
fn oxygen_criteria(values: &Vec<&u16>, idx: usize) -> (u16, u16) {
    common_bit_at(values, idx, |bit_count, total_count| {
        bit_count * 2 >= total_count
    })
}

#[allow(clippy::ptr_arg)]
fn co2_criteria(values: &Vec<&u16>, idx: usize) -> (u16, u16) {
    common_bit_at(values, idx, |bit_count, total_count| {
        bit_count * 2 < total_count
    })
}

fn filter_values(
    values: Vec<&u16>,
    idx: usize,
    criteria: fn(&Vec<&u16>, usize) -> (u16, u16),
) -> u16 {
    let (target, mask) = criteria(&values, idx);
    let next_values = Vec::from_iter(
        values
            .into_iter()
            .filter(|&&value| (value & mask) == target),
    );

    match next_values.len() {
        1 => *next_values[0],
        0 => unreachable!(),
        _ => filter_values(next_values, idx - 1, criteria),
    }
}

#[aoc(day3, part2)]
fn part2(input: &[u16]) -> usize {
    let report_acc: u16 = input.iter().fold(0, |acc, report| acc | report);
    let report_size: usize = 16 - report_acc.leading_zeros() as usize;
    let oxygen_rate = filter_values(
        Vec::from_iter(input.iter()),
        report_size - 1,
        oxygen_criteria,
    );
    let co2_rate = filter_values(Vec::from_iter(input.iter()), report_size - 1, co2_criteria);

    oxygen_rate as usize * co2_rate as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[0b10, 0b10]), 2);

        assert_eq!(
            part1(&[
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010
            ]),
            198
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010
            ]),
            230
        );
    }
}
