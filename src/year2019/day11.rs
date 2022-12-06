use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use super::intcode::{self, Computer};
use crate::util::Complex;

aoc::register!(solve, 2019, 11);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let mut comp: Computer<i64> = input.parse()?;
    let mut panels: FxHashMap<Complex<i64>, i64> = FxHashMap::default();

    paint_all(&mut comp, &mut panels)?;
    out.set_part1(panels.len());

    comp.reset();
    panels.clear();
    panels.insert(Complex::new(0, 0), 1);
    paint_all(&mut comp, &mut panels)?;

    out.set_part2(to_string(&panels));

    Ok(())
}

fn to_string(panels: &FxHashMap<Complex<i64>, i64>) -> String {
    let (xmin, xmax) = panels.keys().map(|c| c.re).minmax().into_option().unwrap();
    let (ymin, ymax) = panels.keys().map(|c| c.im).minmax().into_option().unwrap();
    let mut ident = String::with_capacity(((xmax - xmin + 1) * (ymax - ymin + 1)) as usize);
    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            let color = panels.get(&Complex::new(x, y)).copied().unwrap_or_default();
            ident.push(if color == 1 { '#' } else { ' ' })
        }
        ident.push('\n')
    }
    ident.pop();
    ident
}

fn paint_all(
    comp: &mut Computer<i64>,
    panels: &mut FxHashMap<Complex<i64>, i64>,
) -> Result<(), intcode::Error<i64>> {
    let i = Complex::new(0, 1);
    let mut pos = Complex::new(0, 0);
    let mut dir = i;
    let inner_result: Result<(), intcode::Error<i64>> = (|| loop {
        let color = panels.entry(pos).or_default();
        comp.input.write(*color)?;
        *color = read(comp)?;
        dir *= if read(comp)? == 0 { i } else { -i };
        pos += dir;
    })();

    match inner_result {
        Ok(_) => unreachable!(),
        Err(intcode::Error::Halted) => Ok(()),
        err => err,
    }
}

fn read(comp: &mut Computer<i64>) -> Result<i64, intcode::Error<i64>> {
    loop {
        match comp.output.read() {
            Ok(val) => return Ok(val),
            Err(intcode::Error::EndOfInput) => comp.exec_single()?,
            err => return err,
        }
    }
}
