use std::{cmp::Ordering, collections::HashSet};

use ndarray::{azip, Array1, Array2, Axis};
use regex::Regex;

use crate::{SolveError, SolveResult};

#[derive(Debug, Clone)]
struct Robot {
    origin: Array1<i64>,   // [row, col]
    position: Array1<i64>, // [row, col]
    velocity: Array1<i64>, // [v_row, v_col]
}

pub fn solve(input: &str) -> SolveResult {
    let robots = parse(input)?;
    // Sample input: 12, full input: 500
    let space = if robots.len() < 50 {
        vec![7, 11]
    } else {
        vec![103, 101]
    };
    let space = Array1::from_vec(space);
    Ok((part1(robots.clone(), &space), part2(robots, &space)))
}

fn parse(input: &str) -> Result<Vec<Robot>, SolveError> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;
    let robots = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [px, py, vx, vy])| {
            // note we switch from x,y to r,c here
            Ok::<_, SolveError>(Robot {
                origin: Array1::from_vec(vec![py.parse()?, px.parse()?]),
                position: Array1::from_vec(vec![py.parse()?, px.parse()?]),
                velocity: Array1::from_vec(vec![vy.parse()?, vx.parse()?]),
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(robots)
}

fn part1(mut robots: Vec<Robot>, space: &Array1<i64>) -> String {
    for _ in 0..100 {
        tick(&mut robots, space);
    }
    let mut counts: [u64; 4] = [0, 0, 0, 0];
    let quadrant_digit = |r: &Robot, i| r.position[i].cmp(&(space[i] / 2));
    for robot in robots {
        match (quadrant_digit(&robot, 0), quadrant_digit(&robot, 1)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => {}
            (Ordering::Less, Ordering::Less) => counts[0] += 1,
            (Ordering::Less, Ordering::Greater) => counts[1] += 1,
            (Ordering::Greater, Ordering::Less) => counts[2] += 1,
            (Ordering::Greater, Ordering::Greater) => counts[3] += 1,
        }
    }
    counts
        .into_iter()
        .reduce(|acc, c| acc * c)
        .unwrap()
        .to_string()
}

fn part2(mut robots: Vec<Robot>, space: &Array1<i64>) -> String {
    tick(&mut robots, space);
    let mut ticks = 1;
    while robots.iter().any(|r| r.position != r.origin) {
        tick(&mut robots, space);
        ticks += 1;
        let score = xmas_score(&robots);
        if score < 500 {
            continue;
        }
        println!("{esc}c", esc = 27 as char);
        println!("Tick {} - Score {}", ticks, score);
        display(&robots, space);
        return ticks.to_string();
    }
    ticks.to_string()
}

fn tick(robots: &mut [Robot], space: &Array1<i64>) {
    for robot in robots.iter_mut() {
        robot.position += &robot.velocity;
        azip!((p in &mut robot.position, &s in space) *p = p.rem_euclid(s));
    }
}

fn xmas_score(robots: &[Robot]) -> u64 {
    let positions: HashSet<_> = robots
        .iter()
        .map(|r| (r.position[0], r.position[1]))
        .collect();
    let corners = [
        Array1::from_vec(vec![-1, -1]),
        Array1::from_vec(vec![-1, 1]),
        Array1::from_vec(vec![1, 1]),
        Array1::from_vec(vec![1, -1]),
    ];
    robots
        .iter()
        .map(|r| {
            corners
                .iter()
                .map(|c| &r.position + c)
                .map(|p| positions.contains(&(p[0], p[1])) as u64)
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn display(robots: &[Robot], space: &Array1<i64>) {
    let size = space.mapv(|s| s as usize);
    let mut grid = Array2::from_elem((size[0], size[1]), ' ');
    for robot in robots {
        let position = robot.position.mapv(|p| p as usize);
        grid[(position[0], position[1])] = 'X';
    }
    let output: String = grid
        .map_axis(Axis(0), |r| r.into_iter().collect::<String>())
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);
}
