use crate::days::*;
use std::error;

pub type SolveError = Box<dyn error::Error>;
pub type SolveResult = Result<(String, String), SolveError>;

pub fn solve(day: usize, input: &str) -> SolveResult {
    DAY_FNS[day - 1](input)
}

// TODO: Generate via procedural macro
pub const DAYS: usize = 3;

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
}

const DAY_FNS: [fn(&str) -> SolveResult; DAYS] = [day01::solve, day02::solve, day03::solve];
