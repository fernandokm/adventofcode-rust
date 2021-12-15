use anyhow::Context;

use aoc::ProblemOutput;

use super::intcode::Computer;

aoc::register!(solve, 2019, 5);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let mut comp1: Computer<i32> = input.parse()?;
    let mut comp2 = comp1.clone();

    comp1.input.write(1)?;
    comp1.exec()?;
    out.set_part1(
        comp1
            .output
            .iter()
            .last()
            .context("no diagnostic code (part 1)")?,
    );

    comp2.input.write(5)?;
    comp2.exec()?;
    out.set_part2(comp2.output.read().context("no diagnostic code (part 2)")?);

    Ok(())
}
