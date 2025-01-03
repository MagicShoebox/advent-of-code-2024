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

// TODO: Generate via procedural macro
pub const DAYS: usize = 5;

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}

const DAY_FNS: [fn(&str) -> SolveResult; DAYS] = [
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
];
