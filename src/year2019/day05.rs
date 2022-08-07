use anyhow::Context;

use aoc::ProblemOutput;

use super::intcode::Computer;

aoc::register!(solve, 2019, 5);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let mut comp: Computer<i32> = input.parse()?;

    comp.input.write(1)?;
    comp.exec()?;
    out.set_part1(
        comp.output
            .iter()
            .last()
            .context("no diagnostic code (part 1)")?,
    );

    comp.reset();
    comp.input.write(5)?;
    comp.exec()?;
    out.set_part2(comp.output.read().context("no diagnostic code (part 2)")?);

    Ok(())
}
