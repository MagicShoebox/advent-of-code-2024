use std::iter;

use crate::{SolveError, SolveResult};

pub fn solve(input: &str) -> SolveResult {
    let mut cols = parse(input)?;
    Ok((part1(&mut cols), part2(&mut cols)))
}

fn parse(input: &str) -> Result<[Vec<u32>; 2], SolveError> {
    let mut cols: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        for (i, num) in line.split_whitespace().enumerate() {
            let mut col = cols.get_mut(i);
            while let None = col {
                cols.push(vec![]);
                col = cols.get_mut(i);
            }
            col.unwrap().push(num.parse()?);
        }
    }
    match (cols.pop(), cols.pop()) {
        (Some(x), Some(y)) => Ok([x, y]),
        _ => Err("Input did not have 2 columns.".into()),
    }
}

fn part1(cols: &mut [Vec<u32>; 2]) -> String {
    cols[0].sort();
    cols[1].sort();
    let diff_sum: u32 = iter::zip(&cols[0], &cols[1])
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    diff_sum.to_string()
}

fn part2(cols: &mut [Vec<u32>; 2]) -> String {
    String::new()
}
