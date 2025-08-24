use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::SolveResult;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec2(i64, i64);

struct Antennas {
    locations: HashMap<char, HashSet<Vec2>>,
    size: Vec2,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

pub fn solve(input: &str) -> SolveResult {
    let antennas = parse(input);
    Ok((part1(&antennas), part2(&antennas)))
}

fn parse(input: &str) -> Antennas {
    let mut antennas = Antennas {
        locations: HashMap::new(),
        size: Vec2(0, 0),
    };
    for (r, row) in input.lines().enumerate() {
        for (c, x) in row.chars().enumerate() {
            antennas.size = Vec2(r as i64, c as i64);
            if x != '.' {
                antennas
                    .locations
                    .entry(x)
                    .or_default()
                    .insert(Vec2(r as i64, c as i64));
            }
        }
    }
    antennas.size += Vec2(1, 1);
    antennas
}

fn part1(antennas: &Antennas) -> String {
    let get_antinodes = |nodes: &[Vec2]| {
        let mut result = HashSet::new();
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let i_to_j = nodes[j] - nodes[i];
                let an1 = nodes[i] - i_to_j;
                if is_valid(antennas, &an1) {
                    result.insert(an1);
                }
                let an2 = nodes[j] + i_to_j;
                if is_valid(antennas, &an2) {
                    result.insert(an2);
                }
            }
        }
        result
    };
    count_antinodes(antennas, get_antinodes).to_string()
}

fn part2(antennas: &Antennas) -> String {
    let get_antinodes = |nodes: &[Vec2]| {
        let mut result = HashSet::new();
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let i_to_j = nodes[j] - nodes[i];
                let mut an = nodes[i];
                while is_valid(antennas, &an) {
                    result.insert(an);
                    an -= i_to_j;
                }
                an = nodes[j];
                while is_valid(antennas, &an) {
                    result.insert(an);
                    an += i_to_j;
                }
            }
        }
        result
    };
    count_antinodes(antennas, get_antinodes).to_string()
}

fn count_antinodes<F>(antennas: &Antennas, get_antinodes: F) -> usize
where
    F: Fn(&[Vec2]) -> HashSet<Vec2>,
{
    antennas
        .locations
        .values()
        .map(|nodes| get_antinodes(&nodes.iter().copied().collect::<Vec<Vec2>>()))
        .reduce(|mut acc, x| {
            acc.extend(x);
            acc
        })
        .map_or(0, |v| v.len())
}

fn is_valid(antennas: &Antennas, antinode: &Vec2) -> bool {
    (0..antennas.size.0).contains(&antinode.0) && (0..antennas.size.1).contains(&antinode.1)
}
