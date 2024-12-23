use regex::Regex;

use crate::{SolveError, SolveResult};

pub fn solve(input: &str) -> SolveResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> Result<String, SolveError> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let mut sum: u32 = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        let (x, y): (u32, u32) = (x.parse()?, y.parse()?);
        sum += x * y;
    }
    Ok(sum.to_string())
}

fn part2(input: &str) -> Result<String, SolveError> {
    let cmd_re = Regex::new(r"mul|don't|do")?;
    let mul_re = Regex::new(r"^\((\d{1,3}),(\d{1,3})\)")?;
    let other_re = Regex::new(r"^\(\)")?;
    let mut sum: u32 = 0;
    let mut pos: usize = 0;
    let mut enabled = true;
    while let Some(m) = cmd_re.find_at(input, pos) {
        let next_re = match m.as_str() {
            "mul" => &mul_re,
            "do" | "don't" => &other_re,
            _ => panic!("Missing regex arm"),
        };
        match next_re.captures(&input[m.end()..]) {
            Some(n) => {
                match m.as_str() {
                    "do" => enabled = true,
                    "don't" => enabled = false,
                    "mul" if enabled => {
                        let (x, y): (u32, u32) = (n[1].parse()?, n[2].parse()?);
                        sum += x * y;
                    }
                    _ => {} // nothing to do
                }
                pos = m.end() + n.get(0).unwrap().end();
            }
            None => {
                pos = m.end();
            }
        }
    }
    Ok(sum.to_string())
}
