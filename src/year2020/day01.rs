use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2020, 1);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let nums: Vec<u64> = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .try_collect()?;

    out.set_part1(
        nums.iter()
            .copied()
            .tuple_combinations()
            .find(|(x, y)| x + y == 2020)
            .map_or(0, |(x, y)| x * y),
    );

    out.set_part2(
        nums.iter()
            .copied()
            .tuple_combinations()
            .find(|(x, y, z)| x + y + z == 2020)
            .map_or(0, |(x, y, z)| x * y * z),
    );

    Ok(())
}
