use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

aoc::register!(solve, 2020, 14);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let instructions: Vec<_> = input
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<Instruction<'_>> {
            if let Some(line) = line.strip_prefix("mem[") {
                let (address, value) = line.split_once("] = ").context("invalid input")?;
                Ok(Instruction::Assign {
                    address: address.parse()?,
                    value: value.parse()?,
                })
            } else {
                Ok(Instruction::Mask(
                    line.split_once('=').context("invalid input")?.1.trim(),
                ))
            }
        })
        .try_collect()?;

    out.set_part1(solve_part1(&instructions));

    let max_floating_bits = instructions
        .iter()
        .filter_map(|inst| match inst {
            Instruction::Mask(s) => Some(s.chars().filter(|&c| c == 'X').count()),
            _ => None,
        })
        .max()
        .unwrap();
    if max_floating_bits < 16 {
        out.set_part2(solve_part2(&instructions));
    } else {
        // The real input has few enough bits that it's possible to run the simulation.
        // One of the tests doesn't.
        out.set_part2("<skipped>");
    }

    Ok(())
}

fn solve_part1(instructions: &[Instruction<'_>]) -> u64 {
    let mut mem: FxHashMap<u64, u64> = FxHashMap::default();
    let mut zero_mask = 0;
    let mut one_mask = 0;
    for inst in instructions {
        match inst {
            &Instruction::Assign { address, value } => {
                mem.insert(address, (value & (!zero_mask)) | one_mask);
            }
            Instruction::Mask(s) => {
                zero_mask = 0;
                one_mask = 0;
                for (i, c) in s.chars().enumerate() {
                    match c {
                        '0' => zero_mask += 1 << (35 - i),
                        '1' => one_mask += 1 << (35 - i),
                        _ => (),
                    }
                }
            }
        }
    }
    mem.values().sum()
}

fn solve_part2(instructions: &[Instruction<'_>]) -> u64 {
    let mut mem: FxHashMap<u64, u64> = FxHashMap::default();
    let mut one_mask = 0;
    let mut floating_bits: Vec<usize> = Vec::new();
    for inst in instructions {
        match inst {
            &Instruction::Assign { address, value } => {
                assign_all(&mut mem, address | one_mask, value, &floating_bits);
            }
            Instruction::Mask(s) => {
                one_mask = 0;
                floating_bits.clear();
                for (i, c) in s.chars().enumerate() {
                    match c {
                        '1' => one_mask += 1 << (35 - i),
                        'X' => floating_bits.push(35 - i),
                        _ => (),
                    }
                }
            }
        }
    }
    mem.values().sum()
}

fn assign_all(mem: &mut FxHashMap<u64, u64>, address: u64, value: u64, floating_bits: &[usize]) {
    if floating_bits.is_empty() {
        mem.insert(address, value);
    } else {
        let i = floating_bits[0];
        assign_all(mem, address, value, &floating_bits[1..]);
        assign_all(mem, address ^ (1 << i), value, &floating_bits[1..]);
    }
}

enum Instruction<'a> {
    Assign { address: u64, value: u64 },
    Mask(&'a str),
}
