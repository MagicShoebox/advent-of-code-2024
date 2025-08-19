use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::Bound::{Excluded, Unbounded},
};

use crate::SolveResult;

struct Grid {
    rows: HashMap<usize, BTreeSet<usize>>,
    cols: HashMap<usize, BTreeSet<usize>>,
    start: (usize, usize),
    size: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    let mut grid = parse(input);
    let vertices = calc_vertices(&grid);
    Ok((part1(&grid, &vertices), part2(&mut grid, &vertices)))
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

fn calc_vertices(grid: &Grid) -> Vec<(usize, usize)>
{
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
    vertices
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

fn part1(grid: &Grid, vertices: &[(usize, usize)]) -> String {
    let mut buffer = vec![vec![false; grid.size.1]; grid.size.0];
    let mut dir = Direction::North;
    for (p1, p2) in vertices.iter().zip(vertices[1..].iter()) {
        let r = match dir {
            Direction::North => p2.0..=p1.0,
            Direction::East => p1.1..=p2.1,
            Direction::South => p1.0..=p2.0,
            Direction::West => p2.1..=p1.1,
        };
        for x in r {
            match dir {
                Direction::North | Direction::South => buffer[x][p1.1] = true,
                Direction::East | Direction::West => buffer[p1.0][x] = true,
            }
        }
        dir = dir.turn();
    }
    buffer.iter().flatten().filter(|&x| *x).count().to_string()
}

fn part2(grid: &mut Grid, vertices: &[(usize, usize)]) -> String {
    let mut loop_count = 0;
    let mut dir = Direction::North;
    for (p1, p2) in vertices.iter().zip(vertices[1..].iter()) {
        let r = match dir {
            Direction::North => p2.0+1..p1.0+1,
            Direction::East => p1.1..p2.1,
            Direction::South => p1.0..p2.0,
            Direction::West => p2.1+1..p1.1+1,
        };
        for x in r {
            let (pos, block) = match dir {
                Direction::North => ((x, p1.1), (x - 1, p1.1)),
                Direction::South => ((x, p1.1), (x + 1, p1.1)),
                Direction::East => ((p1.0, x), (p1.0, x + 1)),
                Direction::West => ((p1.0, x), (p1.0, x - 1)),
            };
            if block == grid.start {
                continue;
            }
            grid.rows.entry(block.0).or_default().insert(block.1);
            grid.cols.entry(block.1).or_default().insert(block.0);
            if check_loop(grid, pos, dir) {
                loop_count += 1;
            }
            grid.rows.get_mut(&block.0).map(|v| v.remove(&block.1));
            grid.cols.get_mut(&block.1).map(|v| v.remove(&block.0));
        }
        dir = dir.turn();
    }
    loop_count.to_string()
}

fn check_loop(grid: &Grid, mut pos: (usize, usize), mut dir: Direction) -> bool {
    let mut loop_set = HashSet::new();
    loop {
        match calc_dest(grid, &pos, &dir) {
            Some(dest) if loop_set.contains(&(dest, dir)) => {
                return true
            },
            Some(dest) => {
                loop_set.insert((dest, dir));
                pos = dest;
                dir = dir.turn();
            }
            None => return false
        }
    }
}