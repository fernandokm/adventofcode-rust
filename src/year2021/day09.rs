use std::collections::BTreeSet;

use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::{coords::P2, grid::GridSpec};

aoc::register!(solve, 2021, 9);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut grid: Vec<Vec<Point>> = input
        .trim()
        .lines()
        .map(|line| line.bytes().map(|c| Point::new(c - b'0')).collect())
        .collect();
    let grid_spec = GridSpec::new_indexed(grid.len(), grid[0].len());

    let low_points = grid_spec
        .iter()
        .filter(|&P2(i, j)| {
            grid_spec
                .neighbors(&P2(i, j))
                .all(|P2(ii, jj)| grid[ii][jj].height > grid[i][j].height)
        })
        .map(|P2(i, j)| (i, j))
        .collect_vec();

    out.set_part1(
        low_points
            .iter()
            .map(|&(i, j)| grid[i][j].height as u32 + 1)
            .sum::<u32>(),
    );

    for (i, j) in low_points {
        grid[i][j].low_points.insert((i, j));
        propagate_low_point(&grid_spec, &mut grid, i, j);
    }

    out.set_part2(
        grid.iter()
            .flat_map(|row| row.iter())
            .filter_map(Point::get_single_low_point)
            .counts()
            .values()
            .sorted_unstable()
            .rev()
            .take(3)
            .product::<usize>(),
    );

    Ok(())
}

fn propagate_low_point(grid_spec: &GridSpec<usize>, grid: &mut [Vec<Point>], i: usize, j: usize) {
    let low_points = grid[i][j].low_points.clone();
    let height = grid[i][j].height;
    for P2(ni, nj) in grid_spec.neighbors(&P2(i, j)) {
        let pt = &mut grid[ni][nj];
        if pt.height < height || pt.height == 9 {
            continue;
        }
        let old_size = pt.low_points.len();
        pt.low_points.extend(low_points.iter());
        if pt.low_points.len() > old_size {
            propagate_low_point(grid_spec, grid, ni, nj);
        }
    }
}

struct Point {
    height: u8,
    low_points: BTreeSet<(usize, usize)>,
}

impl Point {
    fn new(height: u8) -> Point {
        Point {
            height,
            low_points: BTreeSet::new(),
        }
    }

    fn get_single_low_point(&self) -> Option<(usize, usize)> {
        let mut it = self.low_points.iter();
        let first = *it.next()?;
        if it.next().is_some() {
            None
        } else {
            Some(first)
        }
    }
}
