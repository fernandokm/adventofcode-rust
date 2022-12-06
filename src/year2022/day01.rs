use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 1);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut calories_per_elf = input
        .split("\n\n")
        .map(|elf| {
            elf.split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect_vec();
    calories_per_elf.sort();

    out.set_part1(calories_per_elf.last().unwrap());
    out.set_part1(calories_per_elf.iter().rev().take(3).sum::<u64>());

    Ok(())
}
