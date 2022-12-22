use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use super::intcode::{self, Computer};
use crate::util::coords::{xy, P2};

aoc::register!(solve, 2019, 11);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut comp: Computer<i64> = input.parse()?;
    let mut panels: FxHashMap<P2<i64>, i64> = FxHashMap::default();

    paint_all(&mut comp, &mut panels)?;
    out.set_part1(panels.len());

    comp.reset();
    panels.clear();
    panels.insert(P2(0, 0), 1);
    paint_all(&mut comp, &mut panels)?;

    out.set_part2(to_string(&panels));

    Ok(())
}

fn to_string(panels: &FxHashMap<P2<i64>, i64>) -> String {
    fn minmax(it: impl Iterator<Item = i64>) -> (i64, i64) {
        it.minmax().into_option().unwrap()
    }
    let (xmin, xmax) = minmax(panels.keys().map(|&P2(x, _)| x));
    let (ymin, ymax) = minmax(panels.keys().map(|&P2(_, y)| y));
    let width: usize = (xmax - xmin + 1).try_into().unwrap();
    let height: usize = (ymax - ymin + 1).try_into().unwrap();
    let mut ident = String::with_capacity(width * height);
    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            let color = panels.get(&P2(x, y)).copied().unwrap_or_default();
            ident.push(if color == 1 { '#' } else { ' ' });
        }
        ident.push('\n');
    }
    ident.pop();
    ident
}

fn paint_all(
    comp: &mut Computer<i64>,
    panels: &mut FxHashMap<P2<i64>, i64>,
) -> Result<(), intcode::Error<i64>> {
    let mut pos = P2(0, 0);
    let mut dir = xy::up::<i64>();
    let inner_result: Result<(), intcode::Error<i64>> = (|| loop {
        let color = panels.entry(pos).or_default();
        comp.input.write(*color)?;
        *color = read(comp)?;
        dir *= if read(comp)? == 0 {
            xy::left_turn()
        } else {
            xy::right_turn()
        };
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
