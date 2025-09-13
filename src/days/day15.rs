use std::fmt::Debug;

use ndarray::{s, Array2, Axis};

use crate::{util::grid::Array2Ext, Error, SolveError, SolveResult};

#[derive(PartialEq, Clone, Copy)]
enum Item {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Box => write!(f, "O"),
            Self::Robot => write!(f, "@"),
        }
    }
}

type Warehouse = Array2<Item>;

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str) -> SolveResult {
    let (mut warehouse, moves) = parse(input)?;
    Ok((part1(&mut warehouse, &moves), String::new()))
}

fn parse(input: &str) -> Result<(Warehouse, Vec<Move>), SolveError> {
    let blank = input.find("\n\n").ok_or(Error::InputError(
        "No blank line between warehouse map and move list",
    ))?;
    let warehouse: String = input
        .char_indices()
        .take_while(|(i, _)| *i <= blank)
        .map(|(_, c)| c)
        .collect();
    let warehouse = Warehouse::from_string(&warehouse, |c| match c {
        '.' => Ok(Item::Empty),
        '#' => Ok(Item::Wall),
        'O' => Ok(Item::Box),
        '@' => Ok(Item::Robot),
        _ => Err("Unexpected character in warehouse map"),
    })?;
    let warehouse = Array2::from_shape_vec(
        warehouse.raw_dim(),
        warehouse.into_iter().collect::<Result<_, _>>()?,
    )?;
    let moves: String = input
        .char_indices()
        .skip_while(|(i, _)| *i <= blank)
        .map(|(_, c)| c)
        .collect();
    let moves = moves
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Ok(Move::Up),
                'v' => Ok(Move::Down),
                '<' => Ok(Move::Left),
                '>' => Ok(Move::Right),
                _ => Err("Unexpected character in move list"),
            })
        })
        .collect::<Result<_, _>>()?;
    Ok((warehouse, moves))
}

fn part1(warehouse: &mut Warehouse, moves: &[Move]) -> String {
    for mve in moves {
        apply_move(warehouse, mve);
    }
    warehouse
        .indexed_iter()
        .filter(|(_, x)| **x == Item::Box)
        .map(|((r, c), _)| 100 * r + c)
        .sum::<usize>()
        .to_string()
}

fn apply_move(warehouse: &mut Warehouse, mve: &Move) {
    let robot = warehouse
        .indexed_iter()
        .find(|(_, x)| **x == Item::Robot)
        .unwrap()
        .0;
    let mut slice = match mve {
        Move::Up => {
            let mut s = warehouse.slice_mut(s![..=robot.0, robot.1]);
            s.invert_axis(Axis(0));
            s
        }
        Move::Down => warehouse.slice_mut(s![robot.0.., robot.1]),
        Move::Left => {
            let mut s = warehouse.slice_mut(s![robot.0, ..=robot.1]);
            s.invert_axis(Axis(0));
            s
        }
        Move::Right => warehouse.slice_mut(s![robot.0, robot.1..]),
    };
    let first = slice
        .indexed_iter()
        .find(|(_, x)| **x == Item::Empty || **x == Item::Wall);
    let first = match first {
        Some((_, f)) if *f == Item::Wall => return,
        Some((i, _)) => i,
        None => panic!("Could not find wall"),
    };
    for i in (0..first).rev() {
        slice[i + 1] = slice[i];
    }
    slice[0] = Item::Empty;
}
