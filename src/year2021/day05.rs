use std::{
    ops::{AddAssign, RangeInclusive},
    str::FromStr,
};

use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;
use ndarray::{s, Array2};

aoc::register!(solve, 2021, 5);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let lines: Vec<_> = input.trim().lines().map(Line::from_str).try_collect()?;

    let xmax = lines.iter().map(|line| line.x1.max(line.x2)).max().unwrap();
    let ymax = lines.iter().map(|line| line.y1.max(line.y2)).max().unwrap();

    let mut occupancy = Array2::zeros((xmax + 1, ymax + 1));
    let mut occupancy_diag = occupancy.clone();

    for line in &lines {
        if !line.is_diagonal() {
            occupancy
                .slice_mut(s![line.xrange(), line.yrange()])
                .add_assign(1);
        } else if (line.x2 > line.x1) == (line.y2 > line.y1) {
            occupancy_diag
                .slice_mut(s![line.xrange(), line.yrange()])
                .diag_mut()
                .add_assign(1);
        } else {
            occupancy_diag
                .slice_mut(s![line.xrange(), line.yrange(); -1])
                .diag_mut()
                .add_assign(1);
        }
    }

    out.set_part1(occupancy.fold(0, |acc, &x| if x < 2 { acc } else { acc + 1 }));

    occupancy += &occupancy_diag;
    out.set_part2(occupancy.fold(0, |acc, &x| if x < 2 { acc } else { acc + 1 }));

    Ok(())
}

#[derive(Clone, Copy)]
struct Line {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s
            .split("->")
            .flat_map(|p| p.split(','))
            .map(|x| x.trim().parse());
        Ok(Line {
            x1: vals.next().context("invalid input")??,
            y1: vals.next().context("invalid input")??,
            x2: vals.next().context("invalid input")??,
            y2: vals.next().context("invalid input")??,
        })
    }
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.x1 != self.x2 && self.y1 != self.y2
    }

    fn xrange(&self) -> RangeInclusive<usize> {
        self.x1.min(self.x2)..=self.x1.max(self.x2)
    }

    fn yrange(&self) -> RangeInclusive<usize> {
        self.y1.min(self.y2)..=self.y1.max(self.y2)
    }
}
