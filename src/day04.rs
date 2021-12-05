use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    number = { ASCII_DIGIT{1,2} }
    drawn_numbers = { (","? ~ number)+ ~ NEWLINE }
    board = { (((" "+)? ~ number){5} ~ NEWLINE){5} }
    input = _{ drawn_numbers ~ (NEWLINE? ~ board)+ }
"#]
struct InputParser;

type Board = [u8; 25];
type Mask = [bool; 25];

struct Bingo {
    numbers: Vec<u8>,
    boards: Vec<Board>
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Bingo {
    let input_tokens = InputParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));

    let mut numbers = vec![];
    let mut boards = vec![];

    for token in input_tokens {
        match token.as_rule() {
            Rule::drawn_numbers => {
                for number_token in token.into_inner() {
                    assert_eq!(number_token.as_rule(), Rule::number);
                    numbers.push(number_token.as_str().parse::<u8>().unwrap_or_else(|e| panic!("{}", e)));
                }
            },
            Rule::board => {
                let mut index: usize = 0;
                let mut board = [0; 25];

                for number_token in token.into_inner() {
                    assert_eq!(number_token.as_rule(), Rule::number);
                    board[index] = number_token.as_str().parse::<u8>().unwrap_or_else(|e| panic!("{}", e));
                    index += 1;
                }

                boards.push(board);
            },
            _ => unreachable!()
        }
    }

    Bingo {
        numbers: numbers,
        boards: boards
    }
}

const WINNING_COMBINATIONS: [[usize; 5]; 10] = [
    // Rows
    [ 0, 1, 2, 3, 4],
    [ 5, 6, 7, 8, 9],
    [10,11,12,13,14],
    [15,16,17,18,19],
    [20,21,22,23,24],
    // Columns
    [ 0, 5,10,15,20],
    [ 1, 6,11,16,21],
    [ 2, 7,12,17,22],
    [ 3, 8,13,18,23],
    [ 4, 9,14,19,24]
];

fn mark_number(board: &Board, marked: &mut [bool], drawn_number: u8) {
    for (i, &number) in board.iter().enumerate() {
        if number == drawn_number {
            marked[i] = true;
            break
        }
    }
}

fn has_won(marked: &[bool]) -> bool {
    for combination in WINNING_COMBINATIONS {
        let mut has_won = true;
        for index in combination {
            if !marked[index] {
                has_won = false;
                break;
            }
        }
        if has_won {
            return true;
        }
    }

    false
}

fn score(board: &Board, marked: &[bool], drawn_number: u8) -> usize {
    let mut sum: usize = 0;

    for (i, &number) in board.iter().enumerate() {
        if !marked[i] {
            sum += number as usize;
        }
    }

    sum * drawn_number as usize
}

#[aoc(day4, part1)]
fn part1(input: &Bingo) -> usize {
    let mut marked: Vec<Mask> = vec![[false; 25]; input.boards.len()];

    for &drawn_number in &input.numbers {
        for (board_nr, &board) in input.boards.iter().enumerate() {
            mark_number(&board, &mut marked[board_nr], drawn_number);
            if has_won(&marked[board_nr]) {
                return score(&board, &marked[board_nr], drawn_number);
            }
        }
    }

    unreachable!();
}

#[aoc(day4, part2)]
fn part2(input: &Bingo) -> usize {
    let mut marked: Vec<Mask> = vec![[false; 25]; input.boards.len()];
    let mut board_has_won = vec![false; input.boards.len()];

    for &drawn_number in &input.numbers {
        for (board_nr, &board) in input.boards.iter().enumerate() {
            if board_has_won[board_nr] {
                continue;
            }
            mark_number(&board, &mut marked[board_nr], drawn_number);
            if has_won(&marked[board_nr]) {
                board_has_won[board_nr] = true;
                if board_has_won.iter().all(|&val| val) {
                    return score(&board, &marked[board_nr], drawn_number);
                }
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                &Bingo {
                    numbers: vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
                    boards: vec![
                        [22, 13, 17, 11,  0,
                          8,  2, 23,  4, 24,
                         21,  9, 14, 16,  7,
                          6, 10,  3, 18,  5,
                          1, 12, 20, 15, 19],
                        [ 3, 15,  0,  2, 22,
                          9, 18, 13, 17,  5,
                         19,  8,  7, 25, 23,
                         20, 11, 10, 24,  4,
                         14, 21, 16, 12,  6],
                        [14, 21, 17, 24,  4,
                         10, 16, 15,  9, 19,
                         18,  8, 23, 26, 20,
                         22, 11, 13,  6,  5,
                          2,  0, 12,  3,  7]
                    ]
                }
            ),
            4512
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                &Bingo {
                    numbers: vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
                    boards: vec![
                        [22, 13, 17, 11,  0,
                          8,  2, 23,  4, 24,
                         21,  9, 14, 16,  7,
                          6, 10,  3, 18,  5,
                          1, 12, 20, 15, 19],
                        [ 3, 15,  0,  2, 22,
                          9, 18, 13, 17,  5,
                         19,  8,  7, 25, 23,
                         20, 11, 10, 24,  4,
                         14, 21, 16, 12,  6],
                        [14, 21, 17, 24,  4,
                         10, 16, 15,  9, 19,
                         18,  8, 23, 26, 20,
                         22, 11, 13,  6,  5,
                          2,  0, 12,  3,  7]
                    ]
                }
            ),
            1924
        );
    }
}
