use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

type PathMap = HashMap<Position, (usize, Vec<Position>)>;
type Maze = Array2<MazePoint>;

pub fn solve(input: &str) -> SolveResult {
    let maze = parse(input)?;
    let (path_map, end) = navigate(&maze);
    Ok((part1(&path_map, end), part2(&path_map, end)))
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

fn part1(path_map: &PathMap, end: (usize, usize)) -> String {
    path_map[&Position(end, Direction::East)].0.to_string()
}

fn part2(path_map: &PathMap, end: (usize, usize)) -> String {
    let mut tiles = HashSet::new();
    let mut stack = vec![Position(end, Direction::East)];
    while let Some(current) = stack.pop() {
        tiles.insert(current.0);
        stack.extend(&path_map[&current].1);
    }

    tiles.len().to_string()
}

fn navigate(maze: &Maze) -> (PathMap, (usize, usize)) {
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
    let mut path_map = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    let start_position = Position(start, Direction::East);
    path_map.insert(start_position, (0, vec![]));
    priority_queue.push(Reverse((0, start_position)));
    while let Some(Reverse((_, position))) = priority_queue.pop() {
        let (score, _) = path_map[&position];
        if position.0 == end {
            if position.1 != Direction::East {
                path_map
                    .entry(Position(position.0, Direction::East))
                    .or_insert((score, vec![]))
                    .1
                    .push(position);
            }
            continue;
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
            match path_map.get_mut(&pos_n) {
                Some((best, path)) if *best == s => {
                    path.push(position);
                }
                Some((best, path)) if *best > s => {
                    *best = s;
                    *path = vec![position];
                    let s = s + pos_n.dist_from(end);
                    priority_queue.push(Reverse((s, pos_n)));
                }
                None => {
                    path_map.insert(pos_n, (s, vec![position]));
                    let s = s + pos_n.dist_from(end);
                    priority_queue.push(Reverse((s, pos_n)));
                }
                _ => {}
            }
        }
    }

    return (path_map, end);
}
