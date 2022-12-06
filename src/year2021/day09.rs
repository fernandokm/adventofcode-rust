use std::collections::BTreeSet;

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 9);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut grid: Vec<Vec<Point>> = input
        .trim()
        .lines()
        .map(|line| line.bytes().map(|c| Point::new(c - b'0')).collect())
        .collect();

    let low_points = enumerate_grid(&grid)
        .filter(|&((i, j), pt)| neighbors(&grid, (i, j)).all(|npt| npt.height > pt.height))
        .map(|((i, j), _)| (i, j))
        .collect_vec();

    out.set_part1(
        low_points
            .iter()
            .map(|&(i, j)| grid[i][j].height as u32 + 1)
            .sum::<u32>(),
    );

    for (i, j) in low_points {
        grid[i][j].low_points.insert((i, j));
        propagate_low_point(&mut grid, i, j);
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

fn propagate_low_point(grid: &mut [Vec<Point>], i: usize, j: usize) {
    let low_points = grid[i][j].low_points.clone();
    let height = grid[i][j].height;
    for (ni, nj) in neighbor_coords(i, j) {
        if let Some(pt) = grid.get_mut(ni).and_then(|row| row.get_mut(nj)) {
            if pt.height < height || pt.height == 9 {
                continue;
            }
            let old_size = pt.low_points.len();
            pt.low_points.extend(low_points.iter());
            if pt.low_points.len() > old_size {
                propagate_low_point(grid, ni, nj);
            }
        }
    }
}

fn enumerate_grid<T>(grid: &[Vec<T>]) -> impl Iterator<Item = ((usize, usize), &T)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, val)| ((i, j), val)))
}

fn neighbor_coords(i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    let minus1 = 0usize.wrapping_sub(1);
    [(0, 1), (0, minus1), (1, 0), (minus1, 0)]
        .into_iter()
        .map(move |(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
}

fn get<T>(grid: &[Vec<T>], (i, j): (usize, usize)) -> Option<&T> {
    grid.get(i).and_then(|row| row.get(j))
}

fn neighbors<T>(grid: &[Vec<T>], (i, j): (usize, usize)) -> impl Iterator<Item = &T> {
    let minus1 = 0usize.wrapping_sub(1);
    [(0, 1), (0, minus1), (1, 0), (minus1, 0)]
        .into_iter()
        .flat_map(move |(di, dj)| get(grid, (i.wrapping_add(di), j.wrapping_add(dj))))
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
