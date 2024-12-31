use std::collections::{HashMap, HashSet};

use crate::{Error, SolveError, SolveResult};

struct Rules {
    depends_on: HashMap<u32, HashSet<u32>>,
    fulfills: HashMap<u32, Vec<u32>>,
}

type Update = Vec<u32>;

pub fn solve(input: &str) -> SolveResult {
    let (rules, updates) = parse(input)?;
    let (valid, invalid): (Vec<Update>, Vec<Update>) =
        updates.into_iter().partition(|u| is_valid(&rules, u));

    let part1 = score(&valid);
    Ok((part1, part2(rules, invalid)))
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
        rules.depends_on.entry(v).or_default().insert(u);
    }

    let updates: Result<Vec<Update>, _> = lines
        .map(|line| line.split(',').map(|p| p.parse()).collect())
        .collect();

    Ok((rules, updates?))
}

fn score(updates: &[Update]) -> String {
    updates
        .iter()
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

fn part2(rules: Rules, updates: Vec<Update>) -> String {
    let mut valid: Vec<Update> = Vec::new();
    for update in updates {
        let update_set: HashSet<u32> = HashSet::from_iter(update);
        valid.push(reorder(&rules, update_set));
    }
    score(&valid)
}

fn reorder(rules: &Rules, update: HashSet<u32>) -> Update {
    let mut pending: Vec<&u32> = update
        .iter()
        .filter(|&p| {
            rules
                .depends_on
                .get(p)
                .map_or(true, |d| d.is_disjoint(&update))
        })
        .collect();

    let mut completed: HashSet<&u32> = HashSet::with_capacity(update.len());
    let mut ordered: Vec<u32> = Vec::with_capacity(update.len());
    while let Some(u) = pending.pop() {
        completed.insert(u);
        ordered.push(*u);
        if let Some(vs) = rules.fulfills.get(u) {
            for v in vs.iter().filter(|&v| update.contains(v)) {
                if rules
                    .depends_on
                    .get(v)
                    .unwrap()
                    .intersection(&update)
                    .all(|p| completed.contains(p))
                {
                    pending.push(v);
                }
            }
        }
    }

    ordered
}
