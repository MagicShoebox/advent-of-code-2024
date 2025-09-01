use std::collections::HashSet;

use crate::{util::grid::ArrayExt, Error, SolveError, SolveResult};
use ndarray::{prelude::*, IntoDimension};

pub fn solve(input: &str) -> SolveResult {
    let top_map = parse(input)?;
    Ok((part1(&top_map), part2(&top_map)))
}

fn parse(input: &str) -> Result<Array2<u32>, SolveError> {
    let rows = input.lines().count();
    let cols = input
        .lines()
        .next()
        .ok_or(Error::InputError("Empty input"))?
        .len();
    let top_map = input
        .lines()
        .flat_map(str::chars)
        .map(|h| {
            h.to_digit(10)
                .or(Some(10))
                .ok_or(Error::InputError("Invalid digit"))
        })
        .collect::<Result<_, _>>()?;
    Ok(Array2::from_shape_vec((rows, cols), top_map)?)
}

// The shared parts of part1 & 2 should really be extracted into a shared function,
// but I spent so long on that stupid neighbors() implementation above
// I don't have it in me right now.

fn part1(top_map: &Array2<u32>) -> String {
    let mut level = 9;
    let level9 = top_map.indexed_iter().filter(|(_, &h)| h == level);

    let mut flood_map = Array2::default(top_map.raw_dim());
    let mut current = HashSet::new();
    for (ix, _) in level9 {
        flood_map[ix] = HashSet::from([ix]);
        current.insert(ix.into_dimension());
    }

    let mut next = HashSet::new();
    while level > 0 && current.len() > 0 {
        level -= 1;
        for ix in current.drain() {
            let lower = top_map.neighbors(ix).filter(|&nix| top_map[nix] == level);
            for nix in lower {
                // Couldn't figure out how to avoid this clone buffer
                let source = flood_map[ix].clone();
                flood_map[nix].extend(source);
                next.insert(nix);
            }
        }
        (current, next) = (next, current);
    }

    top_map
        .indexed_iter()
        .filter(|(_, &h)| h == 0)
        .map(|(ix, _)| flood_map[ix].len())
        .sum::<usize>()
        .to_string()
}

fn part2(top_map: &Array2<u32>) -> String {
    let mut level = 9;
    let level9 = top_map.indexed_iter().filter(|(_, &h)| h == level);

    let mut flood_map = Array2::zeros(top_map.raw_dim());
    let mut current = HashSet::new();
    for (ix, _) in level9 {
        flood_map[ix] = 1;
        current.insert(ix.into_dimension());
    }

    let mut next = HashSet::new();
    while level > 0 && current.len() > 0 {
        level -= 1;
        for ix in current.drain() {
            let lower = top_map.neighbors(ix).filter(|&nix| top_map[nix] == level);
            for nix in lower {
                flood_map[nix] += flood_map[ix];
                next.insert(nix);
            }
        }
        (current, next) = (next, current);
    }

    top_map
        .indexed_iter()
        .filter(|(_, &h)| h == 0)
        .map(|(ix, _)| flood_map[ix])
        .sum::<usize>()
        .to_string()
}
