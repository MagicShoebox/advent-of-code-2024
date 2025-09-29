use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    num::ParseIntError,
};

use ndarray::Array2;
use regex::Regex;

use crate::{util::grid::ArrayExt, SolveError, SolveResult};

type Memory = Array2<bool>;

pub fn solve(input: &str) -> SolveResult {
    let falling_bytes = parse(input)?;
    let mut max_r = 0;
    let mut max_c = 0;
    for [r, c] in falling_bytes.iter() {
        max_r = max_r.max(*r);
        max_c = max_c.max(*c);
    }
    let mut memory = Memory::from_elem((max_r + 1, max_c + 1), true);
    let partial = if falling_bytes.len() < 50 { 12 } else { 1024 };
    Ok((
        part1(&falling_bytes, &mut memory, partial),
        part2(&falling_bytes, &mut memory, partial),
    ))
}

fn parse(input: &str) -> Result<Vec<[usize; 2]>, SolveError> {
    let re = Regex::new(r"(\d+),(\d+)")?;
    Ok(re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| Ok::<_, ParseIntError>([y.parse()?, x.parse()?])) // switch from x,y to r,c
        .collect::<Result<Vec<_>, _>>()?)
}

fn part1(falling_bytes: &Vec<[usize; 2]>, memory: &mut Memory, partial: usize) -> String {
    for ix in &falling_bytes[..partial] {
        memory[*ix] = false;
    }
    match shortest_path(memory) {
        Some(d) => d.to_string(),
        None => "No path!".to_string(),
    }
}

fn part2(falling_bytes: &Vec<[usize; 2]>, memory: &mut Memory, partial: usize) -> String {
    for ix in &falling_bytes[partial..] {
        memory[*ix] = false;
        match shortest_path(memory) {
            Some(_) => continue,
            None => return format!("{},{}", ix[1], ix[0]), // back to x,y
        }
    }
    "Always path!".to_string()
}

fn shortest_path(memory: &Memory) -> Option<usize> {
    let start = (0, 0);
    let end = (memory.shape()[0] - 1, memory.shape()[1] - 1);
    let mut shortest = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    shortest.insert(start, 0);
    priority_queue.push(Reverse((0, 0, start)));
    while let Some(Reverse((_, d, ix))) = priority_queue.pop() {
        if ix == end {
            return Some(d);
        }
        let neighbors = memory.neighbors(ix).filter(|n_ix| memory[*n_ix]);
        for n_ix in neighbors {
            if !matches!(shortest.get(&n_ix), Some(ex) if *ex <= d + 1) {
                let d = d + 1;
                let s = d + n_ix.0.abs_diff(end.0) + n_ix.1.abs_diff(end.1);
                shortest.insert(n_ix, d);
                priority_queue.push(Reverse((s, d, n_ix)));
            }
        }
    }
    None
}
