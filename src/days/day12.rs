use crate::{util::grid::Array2Ext, SolveResult};
use ndarray::Array2;

type Farm = Array2<char>;

#[derive(Debug)]
struct Region {
    label: usize,
    area: usize,
    perimeter: usize,
}

pub fn solve(input: &str) -> SolveResult {
    let farm = Farm::from_string(input, |x| x)?;
    Ok((part1(&farm), String::new()))
}

fn part1(farm: &Farm) -> String {
    let mut regions = label_regions(farm);
    combine_regions(&mut regions);
    regions
        .iter()
        .map(|r| r.area * r.perimeter)
        .sum::<usize>()
        .to_string()
}

fn label_regions(farm: &Farm) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut plot_labels: Array2<usize> = Array2::zeros(farm.raw_dim());

    for r in 0..farm.shape()[0] {
        for c in 0..farm.shape()[1] {
            let north = if r > 0 { Some(farm[(r - 1, c)]) } else { None };
            let west = if c > 0 { Some(farm[(r, c - 1)]) } else { None };
            let current = Some(farm[(r, c)]);
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
    }

    regions
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

fn combine_regions(regions: &mut Vec<Region>) {
    for i in 0..regions.len() {
        let root = find_root(regions, i);
        if i == root {
            continue;
        }
        let (left, right) = regions.split_at_mut(i);
        left[root].area += right[0].area;
        left[root].perimeter += right[0].perimeter;
    }

    let mut i = 0..regions.len();
    regions.retain(|r| r.label == i.next().unwrap());
}
