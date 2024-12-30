use std::iter;

use crate::{Error, SolveError, SolveResult};
use ndarray::{indices_of, prelude::*};

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    const VALUES: [Direction; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];
}

pub fn solve(input: &str) -> SolveResult {
    let grid = parse(input)?;
    Ok((part1(&grid), part2()))
}

fn parse(input: &str) -> Result<Array2<char>, SolveError> {
    let rows = input.lines().count();
    let cols = input
        .lines()
        .next()
        .ok_or(Error::InputError("Empty input"))?
        .len();
    let input = input.lines().flat_map(str::chars).collect();
    let arr = Array2::from_shape_vec((rows, cols), input)?;
    Ok(arr)
}

fn part1(grid: &Array2<char>) -> String {
    let token = "XMAS";
    let mut count: u32 = 0;
    for i in indices_of(grid) {
        for dir in Direction::VALUES {
            let slice = carordinal_slice(grid, i, dir);
            if starts_with(slice, token) {
                count += 1;
            }
        }
    }
    count.to_string()
}

fn part2() -> String {
    String::new()
}

fn starts_with<'a, I>(chars: I, token: &str) -> bool
where
    I: Iterator<Item = &'a char>,
{
    iter::zip(token.chars(), chars)
        .take_while(|(a, &b)| *a == b)
        .count()
        == token.len()
}

fn carordinal_slice<T>(
    grid: &Array2<T>,
    mut origin: (usize, usize),
    dir: Direction,
) -> impl iter::Iterator<Item = &T> {
    iter::successors(
        grid.get(origin),
        move |_| {
        let (r, c) = origin;
        origin = match dir {
            Direction::North => (r.checked_sub(1)?, c),
            Direction::NorthEast => (r.checked_sub(1)?, c.checked_add(1)?),
            Direction::East => (r, c.checked_add(1)?),
            Direction::SouthEast => (r.checked_add(1)?, c.checked_add(1)?),
            Direction::South => (r.checked_add(1)?, c),
            Direction::SouthWest => (r.checked_add(1)?, c.checked_sub(1)?),
            Direction::West => (r, c.checked_sub(1)?),
            Direction::NorthWest => (r.checked_sub(1)?, c.checked_sub(1)?),
        };
        grid.get(origin)
    })
}
