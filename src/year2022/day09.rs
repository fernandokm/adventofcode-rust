use anyhow::bail;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::util::coords::{xy, P2};

aoc::register!(solve, 2022, 9);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let instructions: Vec<(P2<i32>, u32)> =
        input.trim().lines().filter_map(parse_line).try_collect()?;

    out.set_part1(simulate::<2>(&instructions));
    out.set_part2(simulate::<10>(&instructions));

    Ok(())
}

#[must_use]
pub fn simulate<const N: usize>(instructions: &[(P2<i32>, u32)]) -> usize {
    let mut knots = [P2(0, 0); N];
    let mut visited = FxHashSet::default();
    visited.insert(*knots.last().unwrap());
    for &(head_dir, count) in instructions {
        for _ in 0..count {
            knots[0] += head_dir;
            for i in 1..N {
                let diff = knots[i - 1] - knots[i];
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    let dir = P2(diff.0.clamp(-1, 1), diff.1.clamp(-1, 1));
                    knots[i] += dir;
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }
    visited.len()
}

#[must_use]
pub fn parse_line(line: &str) -> Option<anyhow::Result<(P2<i32>, u32)>> {
    let raw_dir = line.chars().next()?;
    let raw_count = line[raw_dir.len_utf8()..].trim();

    // Define a new function in order to be able to use ? and bail!
    // to return errors
    let inner = || {
        let dir = match raw_dir {
            'R' => xy::right(),
            'U' => xy::up(),
            'L' => xy::left(),
            'D' => xy::down(),
            _ => bail!("Invalid direction: {raw_dir}"),
        };

        let count = raw_count.parse()?;
        Ok((dir, count))
    };
    Some(inner())
}
