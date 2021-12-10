use anyhow::Context;
use itertools::Itertools;

use aoc::ProblemOutput;

use super::intcode::{self, Computer};

aoc::register!(solve, 2019, 2);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let comp: Computer<u32> = input.parse()?;

    out.set_part1(get_output(12, 2, comp.clone())?);

    const TARGET_VALUE: u32 = 19690720;
    let (noun, verb) = (0..=99)
        .cartesian_product(0..=99)
        .find(|&(noun, verb)| get_output(noun, verb, comp.clone()).ok() == Some(TARGET_VALUE))
        .context("no (noun, verb) pair found for part 2")?;
    out.set_part2(noun * 100 + verb);

    Ok(())
}

fn get_output(noun: u32, verb: u32, comp: Computer<u32>) -> Result<u32, intcode::Error<u32>> {
    let mut comp = comp;
    comp.ram.insert(1, noun);
    comp.ram.insert(2, verb);
    comp.exec()?;
    Ok(comp.ram_at(0))
}
