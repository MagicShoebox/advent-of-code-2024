use crate::{SolveError, SolveResult};
use std::{collections::HashMap, iter};

type Columns = (Vec<u32>, Vec<u32>);

pub fn solve(input: &str) -> SolveResult {
    let mut cols = parse(input)?;
    Ok((part1(&mut cols), part2(cols)))
}

fn parse(input: &str) -> Result<Columns, SolveError> {
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
        (Some(x), Some(y)) => Ok((x, y)),
        _ => Err("Input did not have 2 columns.".into()),
    }
}

fn part1(cols: &mut Columns) -> String {
    cols.0.sort();
    cols.1.sort();
    iter::zip(&cols.0, &cols.1)
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>() // my first Turbofish
        .to_string()
}

fn part2(cols: Columns) -> String {
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for y in cols.1 {
        *counter.entry(y).or_insert(0) += 1;
    }
    cols.0
        .into_iter()
        .map(|x| x * counter.get(&x).unwrap_or(&0))
        .sum::<u32>()
        .to_string()
}
