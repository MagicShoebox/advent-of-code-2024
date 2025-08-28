use std::collections::HashMap;

use crate::{SolveError, SolveResult};

type Stones = HashMap<u64, usize>;

pub fn solve(input: &str) -> SolveResult {
    let mut stones = parse(input)?;
    Ok((part1(&mut stones), part2(&mut stones)))
}

fn parse(input: &str) -> Result<Stones, SolveError> {
    let mut stones = HashMap::new();
    for stone in input.split_whitespace().map(str::parse) {
        *stones.entry(stone?).or_default() += 1;
    }
    Ok(stones)
}

fn part1(stones: &mut Stones) -> String {
    blink_n(stones, 25);
    stones.values().sum::<usize>().to_string()
}

fn part2(stones: &mut Stones) -> String {
    blink_n(stones, 50);
    stones.values().sum::<usize>().to_string()
}

fn blink_n(stones: &mut Stones, n: usize) {
    let mut current = stones;
    let mut other = Stones::new();
    let mut next = &mut other;
    for _ in 0..n {
        for (stone, count) in current.drain() {
            match stone {
                0 => *next.entry(1).or_default() += count,
                x if (x.ilog10() + 1) % 2 == 0 => {
                    let (left, right) = cleave(x);
                    *next.entry(left).or_default() += count;
                    *next.entry(right).or_default() += count;
                }
                x => *next.entry(x * 2024).or_default() += count,
            }
        }
        (current, next) = (next, current);
    }
    if n % 2 != 0 {
        next.extend(current.drain());
    }
}

fn cleave(stone: u64) -> (u64, u64) {
    let magnitude = 10u64.pow((stone.ilog10() + 1) / 2);
    (stone / magnitude, stone % magnitude)
}
