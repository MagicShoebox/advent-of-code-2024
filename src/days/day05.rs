use std::collections::HashMap;

use crate::{Error, SolveError, SolveResult};

struct Rules {
    depends_on: HashMap<u32, Vec<u32>>,
    fulfills: HashMap<u32, Vec<u32>>,
}

type Update = Vec<u32>;

pub fn solve(input: &str) -> SolveResult {
    let (rules, updates) = parse(input)?;
    Ok((part1(&rules, &updates), String::new()))
}

fn parse(input: &str) -> Result<(Rules, Vec<Update>), SolveError> {
    let mut lines = input.lines();
    let mut rules = Rules {
        depends_on: HashMap::new(),
        fulfills: HashMap::new(),
    };
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        // u|v == v depends on u
        let (u, v) = line
            .split_once('|')
            .ok_or(Error::InputError("No delimiter in graph input"))?;
        let (u, v) = (u.parse()?, v.parse()?);
        rules.fulfills.entry(u).or_default().push(v);
        rules.depends_on.entry(v).or_default().push(u);
    }
    let updates: Result<Vec<Update>, _> = lines
        .map(|line| line.split(',').map(|p| p.parse()).collect())
        .collect();
    Ok((rules, updates?))
}

fn part1(rules: &Rules, updates: &[Update]) -> String {
    updates
        .iter()
        .filter(|&u| is_valid(rules, u))
        .map(|u| u[u.len() / 2])
        .sum::<u32>()
        .to_string()
}

fn is_valid(rules: &Rules, update: &Update) -> bool {
    let mut order = HashMap::new();
    for (i, page) in update.iter().enumerate() {
        order.insert(page, i);
    }
    for (i, page) in update.iter().enumerate() {
        if let Some(pages) = rules.depends_on.get(page) {
            if pages.iter().flat_map(|p| order.get(p)).any(|&j| j > i) {
                return false;
            }
        }
    }
    true
}
