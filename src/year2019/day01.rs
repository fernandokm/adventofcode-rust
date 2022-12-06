use std::str::FromStr;

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2019, 1);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let masses: Vec<_> = input.trim().lines().map(u32::from_str).try_collect()?;

    out.set_part1(total_fuel_requirement(&masses, false));
    out.set_part2(total_fuel_requirement(&masses, true));

    Ok(())
}

fn total_fuel_requirement(masses: &[u32], recursive: bool) -> u32 {
    masses
        .iter()
        .map(|&m| fuel_requirement(m, recursive))
        .sum::<u32>()
}

fn fuel_requirement(mass: u32, recursive: bool) -> u32 {
    let base_fuel = (mass / 3).saturating_sub(2);
    if recursive && base_fuel >= 9 {
        base_fuel + fuel_requirement(base_fuel, true)
    } else {
        base_fuel
    }
}
