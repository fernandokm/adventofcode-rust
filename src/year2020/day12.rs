use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::coords::{xy, P2};

aoc::register!(solve, 2020, 12);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let instructions: Vec<(char, i64)> = input
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<_> {
            let action = line.chars().next().context("invalid input: empty line")?;
            let val = line[1..].trim().parse().context("invalid input")?;
            Ok((action, val))
        })
        .try_collect()?;

    let mut pos = P2(0, 0);
    let mut direction = xy::east();
    for &(action, val) in &instructions {
        match action {
            'N' => pos.1 += val,
            'S' => pos.1 -= val,
            'E' => pos.0 += val,
            'W' => pos.0 -= val,
            'L' => (0..(val / 90)).for_each(|_| direction *= xy::left_turn()),
            'R' => (0..(val / 90)).for_each(|_| direction *= xy::right_turn()),
            'F' => pos += P2(val, 0) * direction,
            _ => anyhow::bail!("Invalid action: {}", action),
        }
    }
    out.set_part1(pos.0.abs() + pos.1.abs());

    let mut pos = P2(0, 0);
    let mut waypoint = P2(10, 1);
    for &(action, val) in &instructions {
        match action {
            'N' => waypoint.1 += val,
            'S' => waypoint.1 -= val,
            'E' => waypoint.0 += val,
            'W' => waypoint.0 -= val,
            'L' => (0..(val / 90)).for_each(|_| waypoint *= xy::left_turn()),
            'R' => (0..(val / 90)).for_each(|_| waypoint *= xy::right_turn()),
            'F' => pos += P2(val, 0) * waypoint,
            _ => anyhow::bail!("Invalid action: {}", action),
        }
    }
    out.set_part2(pos.0.abs() + pos.1.abs());

    Ok(())
}
