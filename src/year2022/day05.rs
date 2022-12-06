use std::str::FromStr;

use anyhow::{anyhow, Context};
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 5);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let (input_stacks, input_moves) = input.split_once("\n\n").context("invalid input")?;
    let mut stacks1 = parse_stacks(input_stacks);
    let mut stacks2 = stacks1.clone();

    for line in input_moves.lines() {
        let m: Move = line.parse()?;

        // part 1
        for _ in 0..m.count {
            let item = stacks1[m.src].pop().context("unexpected empty stack")?;
            stacks1[m.dst].push(item);
        }

        // part 2
        let drain_range = stacks2[m.src].len() - m.count..;
        for el in stacks2[m.src].drain(drain_range).collect_vec() {
            stacks2[m.dst].push(el);
        }
    }
    out.set_part1(stacks_head(&stacks1));
    out.set_part2(stacks_head(&stacks2));

    Ok(())
}

fn stacks_head(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .map(|stack| *stack.last().unwrap_or(&' '))
        .collect()
}

fn parse_stacks(input_stacks: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for line in input_stacks.lines() {
        for (i, c) in line.chars().enumerate() {
            if !c.is_alphabetic() {
                continue;
            }
            let stack_idx = i / 4;
            if stack_idx >= stacks.len() {
                stacks.resize_with(stack_idx + 1, Vec::new);
            }
            stacks[stack_idx].push(c);
        }
    }
    for s in &mut stacks {
        s.reverse();
    }

    stacks
}

struct Move {
    count: usize,
    src: usize,
    dst: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut parts = s.split_ascii_whitespace();
        let mut parse_part = |name| -> anyhow::Result<_> {
            parts.next();
            Ok(parts
                .next()
                .ok_or_else(|| anyhow!("missing {name} in {s}"))?
                .parse()?)
        };
        Ok(Move {
            count: parse_part("count")?,
            src: parse_part("src")? - 1,
            dst: parse_part("dst")? - 1,
        })
    }
}
