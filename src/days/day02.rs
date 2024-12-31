use std::iter;

use crate::SolveResult;

pub fn solve(input: &str) -> SolveResult {
    let reports = parse(input);
    let report_diffs: Vec<Vec<i32>> = reports.iter().map(level_diffs).collect();
    Ok((part1(&report_diffs), part2(&report_diffs)))
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect()
}

fn level_diffs(report: &Vec<u32>) -> Vec<i32> {
    iter::zip(report, &report[1..])
        .map(|(&a, &b)| (a as i32) - (b as i32))
        .collect()
}

fn part1(report_diffs: &[Vec<i32>]) -> String {
    report_diffs
        .iter()
        .flat_map(|x| analyze(x))
        .count()
        .to_string()
}

fn analyze(diffs: &[i32]) -> Result<(), usize> {
    let increasing = diffs.first().unwrap_or(&0).signum();
    for (i, &x) in diffs.iter().enumerate() {
        let diff = increasing * x;
        if !(1..=3).contains(&diff) {
            return Err(i);
        }
    }
    Ok(())
}

fn part2(report_diffs: &[Vec<i32>]) -> String {
    report_diffs
        .iter()
        .filter(|x| is_safe_with_dampener(x))
        .count()
        .to_string()
}

fn is_safe_with_dampener(diffs: &[i32]) -> bool {
    let i = match analyze(diffs) {
        Ok(()) => return true,
        Err(i) => i,
    };

    // Must try removing first when i == 1
    // for corner case where first two diffs are -,+ or +,-
    if i == 1 && analyze(&diffs[1..]).is_ok() {
        return true;
    }

    let last = diffs.len() - 1;
    if i == 0 {
        analyze(&diffs[1..]).or_else(|_| analyze(&combine(diffs, 1)))
    } else if i == last {
        analyze(&diffs[..last]).or_else(|_| analyze(&combine(diffs, last)))
    } else {
        analyze(&combine(diffs, i)).or_else(|_| analyze(&combine(diffs, i + 1)))
    }
    .is_ok()
}

fn combine(diffs: &[i32], i: usize) -> Vec<i32> {
    [&diffs[..i - 1], &[diffs[i - 1] + diffs[i]], &diffs[i + 1..]].concat()
}
