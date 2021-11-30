use std::{collections::BTreeMap, ops::RangeInclusive, str::FromStr};

use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::err::NONE_ERR;

aoc::register!(solve, 2020, 16);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let input = parse_input(input).context("error parsing input")?;

    let mut err_rate = 0;
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            let invalid_sum = ticket
                .iter()
                .filter(|&&val| input.rules.iter().all(|(_, r)| !r.validate(val)))
                .sum1::<u64>();

            err_rate += invalid_sum.unwrap_or(0);
            invalid_sum.is_none()
        })
        .collect_vec();

    out.set_part1(err_rate);

    let fields = input.rules.keys().copied().collect_vec();
    let mut field_names = vec![None; fields.len()];
    let mut possible_names_per_field = (0..fields.len())
        .map(|i| {
            input
                .rules
                .iter()
                .filter(|(_name, rule)| valid_tickets.iter().all(|ticket| rule.validate(ticket[i])))
                .map(|(&name, _rule)| name)
                .collect_vec()
        })
        .collect_vec();

    while field_names.iter().any(|name| name.is_none()) {
        let mut changed = false;
        for &field in &fields {
            let mut idx = possible_names_per_field
                .iter()
                .enumerate()
                .filter(|(_, names)| names.contains(&field))
                .map(|(i, _)| i);
            if let Some(idx0) = idx.next() {
                if idx.next().is_none() {
                    let (j, _) = possible_names_per_field[idx0]
                        .iter()
                        .find_position(|&&s| s == field)
                        .unwrap();
                    field_names[idx0] = Some(possible_names_per_field[idx0][j]);
                    possible_names_per_field[idx0].clear();
                }
            }
        }
        for i in 0..fields.len() {
            if possible_names_per_field[i].len() == 1 {
                changed = true;
                let field_name = possible_names_per_field[i].remove(0);
                field_names[i] = Some(field_name);
                for possible_names in possible_names_per_field.iter_mut() {
                    if let Some((j, _)) = possible_names.iter().find_position(|&&s| s == field_name)
                    {
                        possible_names.swap_remove(j);
                    }
                }
            }
        }
        if !changed {
            dbg!(&field_names, &possible_names_per_field);
            anyhow::bail!("no solution found for part 2");
        }
    }

    out.set_part2(
        field_names
            .iter()
            .enumerate()
            .filter(|&(_, field_name)| field_name.unwrap().starts_with("departure"))
            .map(|(i, _)| input.own_ticket[i])
            .product::<u64>(),
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let mut sections = input.trim().split("\n\n");
    let mut get_section =
        move || -> anyhow::Result<&str> { Ok(sections.next().ok_or(NONE_ERR)?.trim()) };

    Ok(Input {
        rules: get_section()?.lines().map(parse_rule).try_collect()?,
        own_ticket: parse_ticket(get_section()?.lines().next_back().ok_or(NONE_ERR)?)?,
        nearby_tickets: get_section()?
            .lines()
            .skip(1)
            .map(parse_ticket)
            .try_collect()?,
    })
}

fn parse_rule(line: &str) -> anyhow::Result<(&str, Rule)> {
    let (field, raw_rule) = line.split_once(':').ok_or(NONE_ERR)?;
    let ranges = raw_rule
        .split("or")
        .map(|raw_range| -> anyhow::Result<_> {
            let (lo, hi) = raw_range.trim().split_once('-').ok_or(NONE_ERR)?;
            Ok(lo.parse()?..=hi.parse()?)
        })
        .try_collect()?;
    Ok((field, Rule(ranges)))
}

fn parse_ticket(line: &str) -> anyhow::Result<Vec<u64>> {
    Ok(line.split(',').map(FromStr::from_str).try_collect()?)
}

struct Rule(Vec<RangeInclusive<u64>>);
impl Rule {
    fn validate(&self, val: u64) -> bool {
        self.0.iter().any(|range| range.contains(&val))
    }
}

struct Input<'a> {
    rules: BTreeMap<&'a str, Rule>,
    own_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}
