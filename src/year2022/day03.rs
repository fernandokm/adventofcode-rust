use std::{collections::BTreeSet, ops::Add};

use anyhow::{bail, Context};
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 3);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    out.set_part1(
        input
            .lines()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);
                priority(find_common([first, second])?)
            })
            .fold_ok(0, u32::add)?,
    );

    out.set_part2(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|chunk| priority(find_common(chunk)?))
            .fold_ok(0, u32::add)?,
    );

    Ok(())
}

fn find_common<'a>(seqs: impl IntoIterator<Item = &'a str>) -> anyhow::Result<u8> {
    seqs.into_iter()
        .map(|seq| seq.bytes().collect::<BTreeSet<_>>())
        .reduce(|a, b| (&a & &b))
        .and_then(|solution| solution.into_iter().next())
        .context("No common characters found")
}

fn priority(item: u8) -> anyhow::Result<u32> {
    Ok(match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 26 + 1,
        _ => bail!("Invalid character in input: {}", item as char),
    }
    .into())
}
