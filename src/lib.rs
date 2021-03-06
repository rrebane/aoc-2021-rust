extern crate num;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use aoc_runner_derive::aoc_lib;

aoc_lib! { year = 2021 }
