use std::ops::RangeInclusive;

use anyhow::Context;
use aoc::ProblemOutput;

aoc::register!(solve, 2022, 4);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let pairs: Vec<_> = input
        .lines()
        .map(parse_line)
        .collect::<Option<_>>()
        .context("invalid input")?;

    out.set_part1(
        pairs
            .iter()
            .filter(|(first, second)| is_subset_of(first, second) || is_subset_of(second, first))
            .count(),
    );

    out.set_part2(
        pairs
            .iter()
            .filter(|(first, second)| {
                is_subset_of(first, second)
                    || is_subset_of(second, first)
                    || second.contains(first.start())
                    || second.contains(first.end())
            })
            .count(),
    );

    Ok(())
}

fn parse_line(line: &str) -> Option<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (r1, r2) = line.split_once(',')?;
    let r1 = parse_range(r1)?;
    let r2 = parse_range(r2)?;
    Some((r1, r2))
}

fn parse_range(raw: &str) -> Option<RangeInclusive<u32>> {
    let (start, end) = raw.split_once('-')?;
    let start = start.parse().ok()?;
    let end = end.parse().ok()?;
    Some(start..=end)
}

fn is_subset_of(subset: &RangeInclusive<u32>, set: &RangeInclusive<u32>) -> bool {
    subset.start() >= set.start() && subset.end() <= set.end()
}
