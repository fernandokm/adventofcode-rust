use std::fmt::Display;

use aoc::ProblemOutput;
use itertools::Itertools;

use super::intcode::{self, Channel, Computer};

aoc::register!(solve, 2019, 7);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    const MAX_LEN: Option<usize> = Some(5);
    let mut comps: Vec<Computer<i32>> = vec![input.parse()?; 5];

    for i in 1..5 {
        comps[i].input = Channel::new_shared(MAX_LEN);
        comps[i - 1].output = comps[i].input.clone();
    }
    out.set_part1(unwrap_either_boxed(
        (0..5)
            .permutations(5)
            .map(|phases| run_with_phases(&mut comps, &phases))
            .fold_ok(0, |acc, x| acc.max(x)),
    ));

    comps[0].input = Channel::new_shared(MAX_LEN);
    comps[4].output = comps[0].input.clone();
    out.set_part2(unwrap_either_boxed(
        (5..10)
            .permutations(5)
            .map(|phases| run_with_phases(&mut comps, &phases))
            .fold_ok(0, |acc, x| acc.max(x)),
    ));

    Ok(())
}

fn run_with_phases(
    comps: &mut [Computer<i32>],
    phases: &[i32],
) -> Result<i32, intcode::Error<i32>> {
    comps.iter_mut().for_each(Computer::reset);
    for (comp, &phase) in comps.iter_mut().zip(phases.iter()) {
        comp.input.write(phase)?;
    }
    comps[0].input.write(0)?;

    Computer::exec_all(comps)?;

    comps.last_mut().unwrap().output.read()
}

fn unwrap_either_boxed<'a, T, E>(r: Result<T, E>) -> Box<dyn Display + 'a>
where
    T: Display + 'a,
    E: Display + 'a,
{
    match r {
        Ok(v) => Box::new(v),
        Err(v) => Box::new(v),
    }
}
