use aoc::ProblemOutput;
use itertools::Itertools;

use super::intcode::Computer;

aoc::register!(solve, 2019, 9);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut comp: Computer<i64> = input.parse()?;

    comp.input.write(1)?;
    comp.exec()?;

    let comp_out = comp.output.iter().collect_vec();
    if comp_out.len() > 1 {
        anyhow::bail!(
            "BOOST program reported errors in the following operations: {}",
            comp_out[0..comp_out.len() - 1].iter().join(", ")
        );
    }
    out.set_part1(comp_out[0]);

    comp.reset();
    comp.input.write(2)?;
    comp.exec()?;
    out.set_part2(comp.output.read()?);

    Ok(())
}
