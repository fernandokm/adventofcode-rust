use std::str::FromStr;

use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

aoc::register!(solve, 2019, 3);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let (wire1, wire2) = input
        .trim()
        .lines()
        .map(|line| -> Result<Vec<_>, _> { line.split(',').map(Move::from_str).try_collect() })
        .collect_tuple()
        .context("invalid input: exepected 2 lines")?;
    let wire1 = wire1?;
    let wire2 = wire2?;
    let steps1 = get_steps(&wire1);
    let steps2 = get_steps(&wire2);

    let intersections = steps1
        .keys()
        .copied()
        .collect::<FxHashSet<_>>()
        .intersection(&steps2.keys().copied().collect())
        .copied()
        .collect_vec();
    out.set_part1(
        intersections
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .context("no intersections found")?,
    );

    out.set_part2(
        intersections
            .iter()
            .map(|pos| steps1[pos] + steps2[pos])
            .min()
            .unwrap(), /* at this points, we've already traversed intersections once and know it
                        * isn't empty */
    );

    Ok(())
}

fn get_steps(wire: &[Move]) -> FxHashMap<(i32, i32), i32> {
    let mut grid = FxHashMap::default();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    for m in wire {
        let mut xs = vec![x];
        let mut ys = vec![y];
        match m {
            Move::Up(val) => ys = (y + 1..=y + val).collect(),
            Move::Down(val) => ys = (y - val..=y - 1).rev().collect(),
            Move::Left(val) => xs = (x - val..=x - 1).rev().collect(),
            Move::Right(val) => xs = (x + 1..=x + val).collect(),
        }
        for &xx in &xs {
            for &yy in &ys {
                steps += 1;
                grid.insert((xx, yy), steps);
            }
        }
        x = xs.last().copied().unwrap_or(x);
        y = ys.last().copied().unwrap_or(y);
    }
    grid
}

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s[1..].parse()?;
        Ok(match s.chars().next().unwrap() {
            'U' => Move::Up(val),
            'D' => Move::Down(val),
            'L' => Move::Left(val),
            'R' => Move::Right(val),
            x => anyhow::bail!("invalid direction: {}", x),
        })
    }
}
