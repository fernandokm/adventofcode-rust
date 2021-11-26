use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::Complex;

aoc::register!(solve, 2020, 12);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let instructions: Vec<(char, i64)> = input
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<_> {
            let action = line.chars().next().context("invalid input: empty line")?;
            let val = line[1..].trim().parse().context("invalid input")?;
            Ok((action, val))
        })
        .try_collect()?;

    let i = Complex::new(0, 1);
    let mut pos = Complex::new(0, 0);
    let mut direction = Complex::new(1, 0);
    for &(action, val) in &instructions {
        match action {
            'N' => pos.im += val,
            'S' => pos.im -= val,
            'E' => pos.re += val,
            'W' => pos.re -= val,
            'L' => (0..(val / 90)).for_each(|_| direction *= i),
            'R' => (0..(val / 90)).for_each(|_| direction *= -i),
            'F' => pos += Complex::new(val, 0) * direction,
            _ => anyhow::bail!("Invalid action: {}", action),
        }
    }
    out.set_part1(pos.re.abs() + pos.im.abs());

    let mut pos = Complex::new(0, 0);
    let mut waypoint = Complex::new(10, 1);
    for &(action, val) in &instructions {
        match action {
            'N' => waypoint.im += val,
            'S' => waypoint.im -= val,
            'E' => waypoint.re += val,
            'W' => waypoint.re -= val,
            'L' => (0..(val / 90)).for_each(|_| waypoint *= i),
            'R' => (0..(val / 90)).for_each(|_| waypoint *= -i),
            'F' => pos += Complex::new(val, 0) * waypoint,
            _ => anyhow::bail!("Invalid action: {}", action),
        }
    }
    out.set_part2(pos.re.abs() + pos.im.abs());

    Ok(())
}
