use std::iter;

use crate::SolveResult;

type Report = Vec<u32>;

pub fn solve(input: &str) -> SolveResult {
    let reports = parse(input);
    Ok((part1(reports), part2()))
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect())
        .collect()
}

fn part1(reports: Vec<Report>) -> String {
    reports
        .iter()
        .filter(|r| is_safe(*r))
        .count()
        .to_string()
}

fn is_safe(report: &Report) -> bool {
    let diffs: Vec<i32> = iter::zip(report, &report[1..])
        .map(|(&a, &b)| (a as i32) - (b as i32))
        .collect();
    diffs.iter().all(|&x| x > 0 && x < 4)
    || diffs.iter().all(|&x| x > -4 && x < 0)
}

fn part2() -> String {
    String::new()
}