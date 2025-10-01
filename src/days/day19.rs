use regex::Regex;

use crate::{Error, SolveError, SolveResult};

pub fn solve(input: &str) -> SolveResult {
    let (towels, designs) = parse(input)?;
    Ok((part1(&towels, &designs), part2(&towels, &designs)))
}

fn parse(input: &str) -> Result<(Vec<&str>, Vec<&str>), SolveError> {
    let blank = Regex::new(r"\r?\n\r?\n")?;
    if let [towels, designs] = blank.splitn(input, 2).collect::<Vec<_>>()[..] {
        let towels = towels.split(", ").collect::<Vec<_>>();
        let designs = designs.lines().collect::<Vec<_>>();
        Ok((towels, designs))
    } else {
        Err(Error::InputError("Couldn't find blank line between registers and program").into())
    }
}

fn part1(towels: &Vec<&str>, designs: &Vec<&str>) -> String {
    designs
        .iter()
        .map(|d| count(towels, d))
        .filter(|x| *x > 0)
        .count()
        .to_string()
}

fn part2(towels: &Vec<&str>, designs: &Vec<&str>) -> String {
    designs
        .iter()
        .map(|d| count(towels, d))
        .sum::<usize>()
        .to_string()
}

fn count(towels: &Vec<&str>, design: &str) -> usize {
    let mut counts = vec![0; design.len() + 1];
    counts[0] = 1;
    for i in 0..=design.len() {
        for t in towels {
            if matches!(design.get(i..i + t.len()), Some(d) if *t == d) {
                counts[i + t.len()] += counts[i]
            }
        }
    }
    counts[design.len()]
}
