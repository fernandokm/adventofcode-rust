use std::cell::Cell;

use anyhow::Context;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use aoc::ProblemOutput;

aoc::register!(solve, 2021, 14);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let (template, rules) = input.split_once("\n\n").context("invalid input")?;
    let mut pairs: FxHashMap<(char, char), ElementPair> = rules
        .trim()
        .lines()
        .map(ElementPair::parse_rule)
        .collect::<Option<_>>()
        .context("invalid input")?;
    template
        .trim()
        .chars()
        .tuple_windows()
        .for_each(|(c1, c2)| pairs.get_mut(&(c1, c2)).unwrap().count += 1);

    for _ in 0..10 {
        update(&mut pairs);
    }
    out.set_part1(minmax_counts_diff(&pairs, template));

    for _ in 10..40 {
        update(&mut pairs);
    }
    out.set_part2(minmax_counts_diff(&pairs, template));

    Ok(())
}

fn minmax_counts_diff(pairs: &FxHashMap<(char, char), ElementPair>, template: &str) -> u64 {
    let (min, max) = pairs
        .iter()
        .flat_map(|(&(e1, e2), pair)| [(e1, pair.count), (e2, pair.count)].into_iter())
        .chain(
            [
                (template.chars().next().unwrap(), 1),
                (template.chars().last().unwrap(), 1),
            ]
            .into_iter(),
        )
        .into_grouping_map()
        .sum()
        .into_values()
        .minmax()
        .into_option()
        .unwrap();
    (max - min) / 2
}

fn update(pairs: &mut FxHashMap<(char, char), ElementPair>) {
    for (&(e1, e2), pair) in pairs.iter() {
        increment(&pairs[&(e1, pair.new_element)].next_count, pair.count);
        increment(&pairs[&(pair.new_element, e2)].next_count, pair.count);
    }
    for pair in pairs.values_mut() {
        pair.count = pair.next_count.get();
        pair.next_count.set(0);
    }
}

fn increment(cell: &Cell<u64>, inc: u64) {
    cell.set(cell.get() + inc);
}

#[derive(Debug)]
struct ElementPair {
    count: u64,
    next_count: Cell<u64>,
    new_element: char,
}

impl ElementPair {
    fn parse_rule(rule: &str) -> Option<((char, char), ElementPair)> {
        let (raw_in, raw_out) = rule.split_once(" -> ")?;
        let key = raw_in.trim().chars().collect_tuple()?;
        let val = ElementPair {
            count: 0,
            next_count: Cell::new(0),
            new_element: raw_out.trim().chars().next()?,
        };
        Some((key, val))
    }
}
