use ndarray::Array2;
use num::Rational64;
use regex::Regex;

use crate::{SolveError, SolveResult};

#[derive(Debug, Clone)]
struct ClawMachine {
    buttons: Array2<i64>,
    prize: Array2<i64>,
}

impl ClawMachine {
    fn det(&self) -> i64 {
        self.buttons[(0, 0)] * self.buttons[(1, 1)] - self.buttons[(0, 1)] * self.buttons[(1, 0)]
    }

    fn inv(&self) -> Array2<Rational64> {
        Array2::from_shape_vec(
            (2, 2),
            vec![
                self.buttons[(1, 1)],
                -self.buttons[(0, 1)],
                -self.buttons[(1, 0)],
                self.buttons[(0, 0)],
            ],
        )
        .unwrap()
        .mapv(|x| Rational64::new(x, self.det()))
    }
}

pub fn solve(input: &str) -> SolveResult {
    let machines = parse(input)?;
    Ok((part1(&machines), part2(&machines)))
}

fn parse(input: &str) -> Result<Vec<ClawMachine>, SolveError> {
    let mut pattern = String::new();
    pattern.push_str(r"Button A: X\+(\d+), Y\+(\d+)\r?\n");
    pattern.push_str(r"Button B: X\+(\d+), Y\+(\d+)\r?\n");
    pattern.push_str(r"Prize: X=(\d+), Y=(\d+)(?:\r?\n)*");
    let re = Regex::new(pattern.as_str())?;
    let machines = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [ax, ay, bx, by, px, py])| {
            Ok::<_, SolveError>(ClawMachine {
                buttons: Array2::from_shape_vec(
                    (2, 2),
                    vec![ax.parse()?, bx.parse()?, ay.parse()?, by.parse()?],
                )?,
                prize: Array2::from_shape_vec((2, 1), vec![px.parse()?, py.parse()?])?,
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(machines)
}

fn part1(machines: &[ClawMachine]) -> String {
    machines
        .iter()
        .filter_map(min_tokens)
        .filter(|(a, b)| *a <= 100 && *b <= 100)
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>()
        .to_string()
}

fn part2(machines: &[ClawMachine]) -> String {
    let machines: Vec<_> = machines
        .iter()
        .cloned()
        .map(|m| ClawMachine {
            prize: m.prize + 1e13 as i64,
            ..m
        })
        .collect();
    machines
        .iter()
        .filter_map(min_tokens)
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>()
        .to_string()
}

fn min_tokens(machine: &ClawMachine) -> Option<(i64, i64)> {
    if machine.det() == 0 {
        // I had originally expected some of the machines to have
        // singular matrices, which I'm not actually sure how to solve.
        // Fortunately, however, they all turned out to be invertible,
        // so I haven't bothered and am just leaving the panic.
        panic!("singular");
    }
    let b = machine.prize.mapv(|x| Rational64::from_integer(x));
    let x = machine.inv().dot(&b).into_flat();
    match x[0].is_integer() && x[1].is_integer() {
        true => Some((x[0].to_integer(), x[1].to_integer())),
        false => None,
    }
}
