use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use ndarray::Array2;

use crate::{util::grid::Array2Ext, SolveError, SolveResult};

#[derive(PartialEq)]
enum MazePoint {
    Start,
    End,
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position((usize, usize), Direction);

impl Position {
    fn counterclockwise(&self) -> Self {
        match self.1 {
            Direction::North => Position(self.0, Direction::West),
            Direction::East => Position(self.0, Direction::North),
            Direction::South => Position(self.0, Direction::East),
            Direction::West => Position(self.0, Direction::South),
        }
    }
    fn clockwise(&self) -> Self {
        match self.1 {
            Direction::North => Position(self.0, Direction::East),
            Direction::East => Position(self.0, Direction::South),
            Direction::South => Position(self.0, Direction::West),
            Direction::West => Position(self.0, Direction::North),
        }
    }

    fn forward(&self) -> Self {
        match self.1 {
            Direction::North => Position((self.0 .0 - 1, self.0 .1), self.1),
            Direction::East => Position((self.0 .0, self.0 .1 + 1), self.1),
            Direction::South => Position((self.0 .0 + 1, self.0 .1), self.1),
            Direction::West => Position((self.0 .0, self.0 .1 - 1), self.1),
        }
    }

    fn dist_from(&self, point: (usize, usize)) -> usize {
        self.0 .0.abs_diff(point.0) + self.0 .1.abs_diff(point.1)
    }
}

type Maze = Array2<MazePoint>;

pub fn solve(input: &str) -> SolveResult {
    let maze = parse(input)?;
    Ok((part1(&maze), String::new()))
}

fn parse(input: &str) -> Result<Maze, SolveError> {
    let maze = Maze::from_string(input, |c| match c {
        'S' => Ok(MazePoint::Start),
        'E' => Ok(MazePoint::End),
        '#' => Ok(MazePoint::Wall),
        '.' => Ok(MazePoint::Empty),
        _ => Err("Unexpected character in input"),
    })?;
    let maze = Maze::from_shape_vec(maze.raw_dim(), maze.into_iter().collect::<Result<_, _>>()?)?;
    Ok(maze)
}

fn part1(maze: &Maze) -> String {
    let start = maze
        .indexed_iter()
        .find(|(_, x)| **x == MazePoint::Start)
        .map(|(i, _)| i)
        .unwrap();
    let end = maze
        .indexed_iter()
        .find(|(_, x)| **x == MazePoint::End)
        .map(|(i, _)| i)
        .unwrap();
    let mut best_score = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    let start_position = Position(start, Direction::East);
    best_score.insert(start_position, 0);
    priority_queue.push(Reverse((0, start_position)));
    while let Some(Reverse((_, position))) = priority_queue.pop() {
        let score = best_score[&position];
        if position.0 == end {
            return score.to_string();
        }
        let nghbrs = [
            (score + 1, position.forward()),
            (score + 1000, position.counterclockwise()),
            (score + 1000, position.clockwise()),
        ];
        for (s, pos_n) in nghbrs {
            if maze[pos_n.0] == MazePoint::Wall {
                continue;
            }
            match best_score.get_mut(&pos_n) {
                Some(ex) if *ex > s => {
                    *ex = s;
                    let s = s + pos_n.dist_from(end);
                    priority_queue.push(Reverse((s, pos_n)));
                }
                None => {
                    best_score.insert(pos_n, s);
                    let s = s + pos_n.dist_from(end);
                    priority_queue.push(Reverse((s, pos_n)));
                }
                _ => {}
            }
        }
    }

    "No path to end found".to_string()
}
