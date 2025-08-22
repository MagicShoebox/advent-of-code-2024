use std::num::ParseIntError;

use regex::Regex;

use crate::{SolveError, SolveResult};

struct CalibrationEquation {
    test_value: u64,
    terms: Vec<u64>,
}

pub fn solve(input: &str) -> SolveResult {
    let equations = parse(input)?;
    Ok((part1(&equations), part2(&equations)))
}

fn parse(input: &str) -> Result<Vec<CalibrationEquation>, SolveError> {
    let re = Regex::new(r"(\d+): (\d+(?: \d+)*)")?;
    let equations: Result<Vec<CalibrationEquation>, ParseIntError> = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [v, ts])| {
            Ok(CalibrationEquation {
                test_value: v.parse()?,
                terms: parse_terms(ts)?,
            })
        })
        .collect();
    Ok(equations?)
}

fn parse_terms(terms: &str) -> Result<Vec<u64>, ParseIntError> {
    terms.split_whitespace().map(str::parse).collect()
}

fn part1(equations: &[CalibrationEquation]) -> String {
    let get_candidates = |x, y| [x + y, x * y];
    total_calibration(equations, get_candidates).to_string()
}

fn part2(equations: &[CalibrationEquation]) -> String {
    let get_candidates = |x, y| [x + y, x * y, concat_digits(x, y)];
    total_calibration(equations, get_candidates).to_string()
}

fn concat_digits(x: u64, y: u64) -> u64 {
    let n = y.checked_ilog10().unwrap_or_default() + 1;
    x * 10u64.pow(n) + y
}

fn total_calibration<F, I>(equations: &[CalibrationEquation], get_candidates: F) -> u64
where
    F: Fn(u64, u64) -> I,
    I: IntoIterator<Item = u64>,
{
    equations
        .iter()
        .filter(|eq| validate(eq, &get_candidates))
        .map(|eq| eq.test_value)
        .sum::<u64>()
}

fn validate<F, I>(equation: &CalibrationEquation, get_candidates: F) -> bool
where
    F: Fn(u64, u64) -> I,
    I: IntoIterator<Item = u64>,
{
    let mut stack = vec![(1, equation.terms[0])];
    while let Some((i, t)) = stack.pop() {
        if i >= equation.terms.len() {
            if t == equation.test_value {
                return true;
            }
            continue;
        }
        let candidates = get_candidates(t, equation.terms[i]);
        for candidate in candidates {
            if candidate <= equation.test_value {
                stack.push((i + 1, candidate));
            }
        }
    }
    false
}
