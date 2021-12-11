use anyhow::Context;

use aoc::ProblemOutput;

use super::intcode::Computer;

aoc::register!(solve, 2019, 5);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let mut comp1: Computer<i32> = input.parse()?;
    let mut comp2 = comp1.clone();

    comp1.input.push(1);
    comp1.exec()?;
    let diagnostic = comp1.output.pop().context("no diagnostic code (part 1)")?;
    if !comp1.output.iter().all(|&out| out == 0) {
        anyhow::bail!(
            "got nonzero test results: {:?}  (diagnostic code = {})",
            comp1.output,
            diagnostic,
        );
    }
    out.set_part1(diagnostic);

    comp2.input.push(5);
    comp2.exec()?;
    let diagnostic = comp2.output.pop().context("no diagnostic code (part 2)")?;
    out.set_part2(diagnostic);

    Ok(())
}
