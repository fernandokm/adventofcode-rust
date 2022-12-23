use std::{
    cmp::Reverse,
    collections::VecDeque,
    ops::{Add, Mul},
};

use anyhow::{anyhow, bail};
use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::math;

aoc::register!(solve, 2022, 11);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut monkeys: Vec<_> = input.split("\n\n").map(parse_monkey).try_collect()?;

    out.set_part1(run_rounds(&mut monkeys.clone(), 20, true));
    out.set_part2(run_rounds(&mut monkeys, 10_000, false));

    Ok(())
}

#[derive(Debug, Clone)]
struct Monkey {
    worry_levels: VecDeque<u64>,
    op: fn(u64, u64) -> u64,
    op_arg: u64,
    test_divisor: u64,
    target_if_true: usize,
    target_if_false: usize,

    total_inspections: u64,
}

fn parse_monkey(raw: &str) -> anyhow::Result<Monkey> {
    let mut lines = raw.trim().lines().skip(1);
    let mut get = |prefix| {
        lines
            .next()
            .map(str::trim)
            .and_then(|line| line.strip_prefix(prefix))
            .map(str::trim)
            .ok_or_else(|| anyhow!("invalid input: expected prefix \"{prefix}\""))
    };

    let worry_levels = get("Starting items:")?
        .split(", ")
        .map(str::parse)
        .try_collect()?;

    let (op_symbol, operand) = get("Operation: new = old")?.split_at(1);
    let (op, op_arg): (fn(u64, u64) -> u64, u64) = match (op_symbol, operand.trim().parse()) {
        ("*", _) if operand == " old" => (|x, _| x * x, 0),
        ("*", Ok(v)) => (u64::mul, v),
        ("+", Ok(v)) => (u64::add, v),
        _ => bail!("invalid input: op='{op_symbol}' operand=\"{operand}\""),
    };

    Ok(Monkey {
        worry_levels,
        op,
        op_arg,
        test_divisor: get("Test: divisible by")?.parse()?,
        target_if_true: get("If true: throw to monkey")?.parse()?,
        target_if_false: get("If false: throw to monkey")?.parse()?,
        total_inspections: 0,
    })
}

fn run_rounds(monkeys: &mut [Monkey], rounds: usize, divide_worry: bool) -> u64 {
    let monkey_lcm = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .reduce(math::lcm)
        .unwrap();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut worry) = monkeys[i].worry_levels.pop_front() {
                let m = &monkeys[i];
                let worry_divisor = if divide_worry { 3 } else { 1 };
                worry = (m.op)(worry, m.op_arg) / worry_divisor % monkey_lcm;
                let target = if worry % m.test_divisor == 0 {
                    m.target_if_true
                } else {
                    m.target_if_false
                };
                monkeys[target].worry_levels.push_back(worry);
                monkeys[i].total_inspections += 1;
            }
        }
    }

    monkeys
        .iter()
        .map(|m| Reverse(m.total_inspections))
        .k_smallest(2)
        .map(|Reverse(val)| val)
        .product()
}
