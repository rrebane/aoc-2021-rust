use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT+ }
    input = _{ (number ~ NEWLINE)+ }
"#]
struct InputParser;

fn token_to_number(token: &Pair<Rule>) -> u32 {
    let token_str = token.as_str();
    return token_str.parse::<u32>().unwrap_or_else(|e| panic!("{}", e));
}

pub fn part1(input: String) {
    let input_tokens = InputParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));

    let mut prev_number: Option<u32> = None;
    let mut increase_count: u32 = 0;

    for number_token in input_tokens {
        let number = token_to_number(&number_token);

        match prev_number {
            Some(prev_number) => increase_count += if prev_number < number { 1 } else { 0 },
            _ => (),
        }

        prev_number = Some(number);
    }

    println!("{}", increase_count);
}

const WINDOW_SIZE: usize = 3;

pub fn part2(input: String) {
    let input_tokens = InputParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));

    let increase_count: u32 = input_tokens
        .map(|number_token| token_to_number(&number_token))
        .collect::<Vec<u32>>()
        .windows(WINDOW_SIZE)
        .map(|window| window.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum();

    println!("{}", increase_count);
}
