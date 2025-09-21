use std::{collections::HashSet, fmt::Debug};

use ndarray::{Array2, Axis};

use crate::{Error, SolveError, SolveResult};

#[derive(Debug, PartialEq, Clone)]
struct Item {
    kind: ItemKind,
    position: (usize, usize),
}

impl Item {
    fn new(kind: ItemKind, position: (usize, usize)) -> Self {
        Self { kind, position }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ItemKind {
    Wall,
    Box1,
    Box2,
    Robot,
}

type Warehouse = Vec<Item>;

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str) -> SolveResult {
    let (warehouse, moves) = parse(input)?;
    let expanded = expand_warehouse(warehouse.clone());
    Ok((
        move_and_score(warehouse, &moves),
        move_and_score(expanded, &moves),
    ))
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
    let warehouse = warehouse
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars().enumerate().filter_map(move |(c, x)| match x {
                '.' => None,
                '#' => Some(Ok(Item::new(ItemKind::Wall, (r, c)))),
                'O' => Some(Ok(Item::new(ItemKind::Box1, (r, c)))),
                '@' => Some(Ok(Item::new(ItemKind::Robot, (r, c)))),
                _ => Some(Err("Unexpected character in warehouse map")),
            })
        })
        .collect::<Result<_, _>>()?;
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

fn expand_warehouse(warehouse: Warehouse) -> Warehouse {
    warehouse
        .into_iter()
        .flat_map(|item| {
            let p1 = (item.position.0, 2 * item.position.1);
            let p2 = (item.position.0, 2 * item.position.1 + 1);
            match item.kind {
                ItemKind::Wall => {
                    vec![Item::new(ItemKind::Wall, p1), Item::new(ItemKind::Wall, p2)]
                }
                ItemKind::Box1 => vec![Item::new(ItemKind::Box2, p1)],
                ItemKind::Robot => vec![Item::new(ItemKind::Robot, p1)],
                _ => unreachable!(),
            }
        })
        .collect()
}

fn _display(warehouse: &Warehouse, mve: Option<&Move>) {
    let max_row = warehouse.iter().map(|item| item.position.0).max().unwrap();
    let max_col = warehouse.iter().map(|item| item.position.1).max().unwrap();
    let mut buffer = Array2::from_elem((max_row + 1, max_col + 1), '.');
    for item in warehouse {
        match item.kind {
            ItemKind::Wall => buffer[item.position] = '#',
            ItemKind::Box1 => buffer[item.position] = 'O',
            ItemKind::Box2 => {
                let (r, c) = item.position;
                buffer[item.position] = '[';
                buffer[(r, c + 1)] = ']';
            }
            ItemKind::Robot => buffer[item.position] = '@',
        }
    }
    if let Some(m) = mve {
        println!("Moved {:?}", m);
    }
    for row in buffer.axis_iter(Axis(0)) {
        let row: String = row.as_slice().unwrap().into_iter().collect();
        print!("{}", row);
        println!();
    }
}

fn move_and_score(mut warehouse: Warehouse, moves: &[Move]) -> String {
    //_display(&warehouse, None);
    for mve in moves {
        apply_move(&mut warehouse, mve);
        //_display(&warehouse, Some(mve));
    }
    warehouse
        .into_iter()
        .filter_map(|item| match (item.kind, item.position) {
            (ItemKind::Box1 | ItemKind::Box2, (r, c)) => Some(100 * r + c),
            _ => None,
        })
        .sum::<usize>()
        .to_string()
}

fn apply_move(warehouse: &mut Warehouse, mve: &Move) {
    let robot = warehouse
        .iter()
        .position(|x| matches!(x.kind, ItemKind::Robot))
        .unwrap();
    let mut moving = HashSet::new();
    let mut stack = vec![robot];
    while let Some(i) = stack.pop() {
        if moving.contains(&i) {
            continue;
        }
        match (warehouse[i].kind, mve) {
            (ItemKind::Wall, _) => return,
            (ItemKind::Robot | ItemKind::Box1, _) => {
                moving.insert(i);
                if let Some(x) = index_of_item_at(warehouse, delta(warehouse[i].position, mve)) {
                    stack.push(x);
                }
            }
            (ItemKind::Box2, Move::Up | Move::Down) => {
                let p = warehouse[i].position;
                moving.insert(i);
                if let Some(x) = index_of_item_at(warehouse, delta(p, mve)) {
                    stack.push(x);
                }
                if let Some(x) = index_of_item_at(warehouse, delta((p.0, p.1 + 1), mve)) {
                    stack.push(x);
                }
            }
            (ItemKind::Box2, Move::Left) => {
                moving.insert(i);
                if let Some(x) = index_of_item_at(warehouse, delta(warehouse[i].position, mve)) {
                    stack.push(x);
                }
            }
            (ItemKind::Box2, Move::Right) => {
                let p = warehouse[i].position;
                moving.insert(i);
                if let Some(x) = index_of_item_at(warehouse, delta((p.0, p.1 + 1), mve)) {
                    stack.push(x);
                }
            }
        }
    }
    for i in moving {
        let item = &mut warehouse[i];
        if matches!(item.kind, ItemKind::Robot | ItemKind::Box1 | ItemKind::Box2) {
            item.position = delta(item.position, mve);
        }
    }
}

fn delta((r, c): (usize, usize), mve: &Move) -> (usize, usize) {
    match mve {
        Move::Up => (r - 1, c),
        Move::Down => (r + 1, c),
        Move::Left => (r, c - 1),
        Move::Right => (r, c + 1),
    }
}

fn index_of_item_at(warehouse: &Warehouse, p: (usize, usize)) -> Option<usize> {
    warehouse.iter().position(|x| match x.kind {
        ItemKind::Wall | ItemKind::Robot | ItemKind::Box1 if x.position == p => true,
        ItemKind::Box2
            if x.position.0 == p.0 && (x.position.1 == p.1 || x.position.1 + 1 == p.1) =>
        {
            true
        }
        _ => false,
    })
}
