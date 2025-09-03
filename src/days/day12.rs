use std::collections::HashMap;

use crate::{util::grid::Array2Ext, SolveResult};
use ndarray::Array2;
use ndarray_ndimage::{pad, PadMode};

type Farm = Array2<char>;

#[derive(Debug)]
struct Region {
    label: usize,
    area: usize,
    perimeter: usize,
    sides: usize,
}

pub fn solve(input: &str) -> SolveResult {
    let farm = Farm::from_string(input, |x| x)?;
    let (mut regions, plot_labels) = label_regions(&farm);
    count_sides(&mut regions, &plot_labels);
    Ok((part1(&regions), part2(&regions)))
}

fn part1(regions: &HashMap<usize, Region>) -> String {
    regions
        .values()
        .map(|r| r.area * r.perimeter)
        .sum::<usize>()
        .to_string()
}

fn part2(regions: &HashMap<usize, Region>) -> String {
    regions
        .values()
        .map(|r| r.area * r.sides)
        .sum::<usize>()
        .to_string()
}

fn label_regions(farm: &Farm) -> (HashMap<usize, Region>, Array2<usize>) {
    let mut regions: Vec<Region> = Vec::new();
    let mut plot_labels: Array2<usize> = Array2::zeros(farm.raw_dim());

    for ((r, c), &x) in farm.indexed_iter() {
        let north = if r > 0 { Some(farm[(r - 1, c)]) } else { None };
        let west = if c > 0 { Some(farm[(r, c - 1)]) } else { None };
        let current = Some(x);
        let plot_label = match (north == current, west == current) {
            (true, true) => {
                let north_label = plot_labels[(r - 1, c)];
                let west_label = plot_labels[(r, c - 1)];
                union_roots(&mut regions, north_label, west_label);
                if north_label > west_label {
                    west_label
                } else {
                    north_label
                }
            }
            (true, false) => {
                if west.is_some() {
                    regions[plot_labels[(r, c - 1)]].perimeter += 1;
                }
                let label = plot_labels[(r - 1, c)];
                regions[label].perimeter += 1;
                label
            }
            (false, true) => {
                if north.is_some() {
                    regions[plot_labels[(r - 1, c)]].perimeter += 1;
                }
                let label = plot_labels[(r, c - 1)];
                regions[label].perimeter += 1;
                label
            }
            (false, false) => {
                if north.is_some() {
                    regions[plot_labels[(r - 1, c)]].perimeter += 1;
                }
                if west.is_some() {
                    regions[plot_labels[(r, c - 1)]].perimeter += 1;
                }
                let label = regions.len();
                regions.push(Region {
                    label,
                    area: 0,
                    perimeter: 2,
                    sides: 0
                });
                label
            }
        };
        plot_labels[(r, c)] = plot_label;

        if c == farm.shape()[1] - 1 {
            regions[plot_label].perimeter += 1;
        }
        if r == farm.shape()[0] - 1 {
            regions[plot_label].perimeter += 1;
        }
        regions[plot_label].area += 1;
    }

    for plot_label in plot_labels.iter_mut() {
        *plot_label = find_root(&mut regions, *plot_label);
    }

    (combine_regions(regions), plot_labels)
}

fn find_root(regions: &mut Vec<Region>, label: usize) -> usize {
    let mut root = label;
    while regions[root].label != root {
        root = regions[root].label;
    }

    let mut current = label;
    while regions[current].label != root {
        let parent = regions[current].label;
        regions[current].label = root;
        current = parent;
    }
    root
}

fn union_roots(regions: &mut Vec<Region>, mut x: usize, mut y: usize) {
    x = find_root(regions, x);
    y = find_root(regions, y);
    match x.cmp(&y) {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Less => regions[y].label = x,
        std::cmp::Ordering::Greater => regions[x].label = y,
    }
}

fn combine_regions(mut regions: Vec<Region>) -> HashMap<usize, Region> {
    for i in 0..regions.len() {
        let root = find_root(&mut regions, i);
        if i == root {
            continue;
        }
        let (left, right) = regions.split_at_mut(i);
        left[root].area += right[0].area;
        left[root].perimeter += right[0].perimeter;
    }

    regions
        .into_iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if r.label == i {
                Some((r.label, r))
            } else {
                None
            }
        })
        .collect()
}

fn count_sides(regions: &mut HashMap<usize, Region>, plot_labels: &Array2<usize>) {
    let mut regions: HashMap<_, _> = regions.iter_mut().map(|(k, v)| (k + 1, v)).collect();
    let plot_labels = pad(
        &plot_labels.mapv(|x| x + 1),
        &[[1, 1]],
        PadMode::Constant(0),
    );

    for window in plot_labels.windows((2, 2)) {
        let mut frequency = HashMap::new();
        for label in window {
            *frequency.entry(*label).or_insert(0usize) += 1;
        }
        for (label, count) in frequency {
            match count {
                1 | 3 => {
                    if let Some(x) = regions.get_mut(&label) {
                        x.sides += 1;
                    }
                }
                2 if window[(0, 0)] == window[(1, 1)] || window[(0, 1)] == window[(1, 0)] => {
                    if let Some(x) = regions.get_mut(&label) {
                        x.sides += 2;
                    }
                }
                _ => {}
            }
        }
    }
}
