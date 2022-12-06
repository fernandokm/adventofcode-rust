use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashSet;

aoc::register!(solve, 2021, 13);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let (dots, instructions) = input.split_once("\n\n").context("invalid input")?;
    let mut dots: FxHashSet<(usize, usize)> = dots
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<_> {
            let (x, y) = line.split_once(',').context("invalid input")?;
            Ok((x.parse()?, y.parse()?))
        })
        .try_collect()?;

    for (i, line) in instructions.trim().lines().enumerate() {
        let (axis, pos) = line
            .strip_prefix("fold along ")
            .and_then(|s| s.split_once('='))
            .context("invalid input")?;
        let pos = pos.parse()?;
        if axis == "x" {
            dots = dots
                .into_iter()
                .map(|(x, y)| if x > pos { (2 * pos - x, y) } else { (x, y) })
                .collect();
        } else {
            dots = dots
                .into_iter()
                .map(|(x, y)| if y > pos { (x, 2 * pos - y) } else { (x, y) })
                .collect();
        }
        if i == 0 {
            out.set_part1(dots.len());
        }
    }

    let xmax = dots.iter().map(|&(x, _)| x).max().unwrap();
    let ymax = dots.iter().map(|&(_, y)| y).max().unwrap();
    let mut part2 = String::with_capacity((xmax + 1) * (ymax + 1));
    for y in 0..=ymax {
        for x in 0..=xmax {
            part2.push(if dots.contains(&(x, y)) { '#' } else { ' ' });
        }
        part2.push('\n');
    }
    part2.pop();

    out.set_part2(part2);
    Ok(())
}
