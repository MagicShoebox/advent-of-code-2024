use std::{
    collections::{BTreeSet, HashMap},
    ops::Bound::{Excluded, Unbounded},
};

use crate::SolveResult;

struct Grid {
    rows: HashMap<usize, BTreeSet<usize>>,
    cols: HashMap<usize, BTreeSet<usize>>,
    start: (usize, usize),
    size: (usize, usize),
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub fn solve(input: &str) -> SolveResult {
    let grid = parse(input);
    Ok((part1(&grid), String::new()))
}

fn parse(input: &str) -> Grid {
    let mut grid = Grid {
        rows: HashMap::new(),
        cols: HashMap::new(),
        start: (0, 0),
        size: (0, 0),
    };
    for (r, row) in input.lines().enumerate() {
        for (c, x) in row.chars().enumerate() {
            grid.size = (r, c);
            match x {
                '^' => grid.start = (r, c),
                '#' => {
                    grid.rows.entry(r).or_default().insert(c);
                    grid.cols.entry(c).or_default().insert(r);
                }
                _ => (),
            };
        }
    }
    grid.size = (grid.size.0 + 1, grid.size.1 + 1);
    grid
}

fn part1(grid: &Grid) -> String {
    let mut vertices = vec![grid.start];
    let mut pos = grid.start;
    let mut dir = Direction::North;
    loop {
        match calc_dest(grid, &pos, &dir) {
            Some(dest) => {
                vertices.push(dest);
                pos = dest;
                dir = dir.turn();
            }
            None => {
                vertices.push(final_dest(grid, &pos, &dir));
                break;
            }
        }
    }

    steps(grid, &vertices).to_string()
}

fn calc_dest(grid: &Grid, pos: &(usize, usize), dir: &Direction) -> Option<(usize, usize)> {
    Some(match dir {
        Direction::North => (
            *grid
                .cols
                .get(&pos.1)?
                .range((Unbounded, Excluded(&pos.0)))
                .next_back()?
                + 1,
            pos.1,
        ),
        Direction::East => (
            pos.0,
            *grid
                .rows
                .get(&pos.0)?
                .range((Excluded(&pos.1), Unbounded))
                .next()?
                - 1,
        ),
        Direction::South => (
            *grid
                .cols
                .get(&pos.1)?
                .range((Excluded(&pos.0), Unbounded))
                .next()?
                - 1,
            pos.1,
        ),
        Direction::West => (
            pos.0,
            *grid
                .rows
                .get(&pos.0)?
                .range((Unbounded, Excluded(&pos.1)))
                .next_back()?
                + 1,
        ),
    })
}

fn final_dest(grid: &Grid, pos: &(usize, usize), dir: &Direction) -> (usize, usize) {
    match dir {
        Direction::North => (0, pos.1),
        Direction::East => (pos.0, grid.size.1 - 1),
        Direction::South => (grid.size.0 - 1, pos.1),
        Direction::West => (pos.0, 0),
    }
}

fn steps(grid: &Grid, vertices: &[(usize, usize)]) -> usize {
    let mut buffer: Vec<Vec<bool>> = (0..grid.size.0)
        .map(|_| (0..grid.size.1).map(|_| false).collect())
        .collect();
    for (i, (p1, p2)) in vertices.iter().zip(vertices[1..].iter()).enumerate() {
        let r = match i % 4 {
            0 => p2.0..=p1.0,
            1 => p1.1..=p2.1,
            2 => p1.0..=p2.0,
            3 => p2.1..=p1.1,
            _ => panic!(),
        };
        for x in r {
            match i % 4 {
                0 => buffer[x][p1.1] = true,
                1 => buffer[p1.0][x] = true,
                2 => buffer[x][p1.1] = true,
                3 => buffer[p1.0][x] = true,
                _ => panic!(),
            }
        }
    }
    buffer.iter().flatten().filter(|&x| *x).count()
}


/*

* I wanted to find a way to calculate the steps without
* storing each step individually in an array or set
* (thus allowing for grids of arbitrary size).

* Unfortunately, accounting for the intersections and overlaps
* was more difficult than I expected.
* This is the last version I had before I abandoned the approach
* and used the method above in order to continue AoC.

fn steps_arbitrary_size(vertices: &[(usize, usize)]) -> usize {
    let mut horiz: BTreeMap<usize, Vec<RangeInclusive<usize>>> = BTreeMap::new();
    let mut verts: BTreeMap<usize, Vec<RangeInclusive<usize>>> = BTreeMap::new();
    let mut s: usize = 0; // initial position
    for (i, (p1, p2)) in vertices.iter().zip(vertices[1..].iter()).enumerate() {
        if i % 2 == 0 {
            let col = p1.1;
            let range = if i % 4 == 0 { p2.0..=p1.0 } else { p1.0..=p2.0 };
            let dist = p1.0.abs_diff(p2.0) + 1;
            s += dist;

            s -= verts
                .get(&col)
                .map_or(0, |v| v.iter().map(|r| overlap(&range, r)).sum());

            s -= horiz
                .range(range.clone())
                .flat_map(|(_, v)| v)
                .filter(|r| r.contains(&col))
                .count();

            if dist > 1 {
                verts.entry(col).or_default().push(range);
            }
        } else {
            let row = p1.0;
            let range = if i % 4 == 1 { p1.1..=p2.1 } else { p2.1..=p1.1 };
            let dist = p1.1.abs_diff(p2.1) + 1;
            s += dist;

            s -= horiz
                .get(&row)
                .map_or(0, |v| v.iter().map(|r| overlap(&range, r)).sum());

            s -= verts
                .range(range.clone())
                .flat_map(|(_, v)| v)
                .filter(|r| r.contains(&row))
                .count();

            if dist > 1 {
                horiz.entry(row).or_default().push(range);
            }
        }
    }
    s
}

fn overlap(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> usize {
    if r1.start() > r2.start() {
        return overlap(r2, r1);
    }
    if r1.end() < r2.start() {
        return 0;
    }
    min(r1.end(), r2.end()) - r2.start() + 1
}

*/