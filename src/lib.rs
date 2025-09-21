use crate::days::*;
use core::fmt;
use std::error::{self};

pub type SolveError = Box<dyn error::Error>;
pub type SolveResult = Result<(String, String), SolveError>;

#[derive(Debug)]
enum Error<'a> {
    InputError(&'a str),
}

impl error::Error for Error<'_> {}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputError(msg) => write!(f, "{msg}"),
        }
    }
}

pub fn solve(day: usize, input: &str) -> SolveResult {
    DAY_FNS[day - 1](input)
}

mod util {
    pub mod grid;
}

// TODO: Generate via procedural macro
pub const DAYS: usize = 16;

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
}

const DAY_FNS: [fn(&str) -> SolveResult; DAYS] = [
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
    day16::solve,
];
