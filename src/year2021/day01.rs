use std::str::FromStr;

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 1);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let depths: Vec<u64> = input.trim().lines().map(FromStr::from_str).try_collect()?;

    out.set_part1(
        depths
            .iter()
            .zip(depths.iter().skip(1))
            .filter(|&(x, y)| x < y)
            .count(),
    );

    let windowed = depths
        .iter()
        .tuple_windows()
        .map(|(x1, x2, x3)| x1 + x2 + x3)
        .collect_vec();

    out.set_part2(
        windowed
            .iter()
            .zip(windowed.iter().skip(1))
            .filter(|&(x, y)| x < y)
            .count(),
    );

    Ok(())
}
