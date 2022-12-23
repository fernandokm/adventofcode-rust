use std::{iter::Peekable, str::FromStr};

use anyhow::{anyhow, ensure};
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 13);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut signal: Vec<Item> = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::parse)
        .try_collect()?;

    out.set_part1(
        signal
            .iter()
            .tuples()
            .enumerate()
            .filter(|(_, (p1, p2))| p1 <= p2)
            .map(|(i, _)| i + 1)
            .sum::<usize>(),
    );

    let loc1 = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let loc2 = Item::List(vec![Item::List(vec![Item::Int(6)])]);
    signal.extend([loc1.clone(), loc2.clone()]);
    signal.sort_unstable();
    let idx1 = signal.iter().position(|x| x == &loc1).unwrap_or_default() + 1;
    let idx2 = signal.iter().position(|x| x == &loc2).unwrap_or_default() + 1;
    out.set_part2(idx1 * idx2);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Int(u32),
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        fn inner<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a str>>) -> anyhow::Result<Item> {
            let t = tokens
                .next()
                .ok_or_else(|| anyhow!("unexpected end of input"))?;
            if t != "[" {
                return Ok(Item::Int(t.parse()?));
            }
            if tokens.peek() == Some(&"]") {
                tokens.next();
                return Ok(Item::List(Vec::new()));
            }
            let mut vals = Vec::new();
            while tokens.peek() != Some(&"]") {
                vals.push(inner(tokens)?);
            }
            tokens.next();
            Ok(Item::List(vals))
        }

        let mut tokens = get_tokens(s).fuse().peekable();
        let item = inner(&mut tokens)?;
        ensure!(tokens.next().is_none());
        Ok(item)
    }
}

fn get_tokens(s: &str) -> impl Iterator<Item = &'_ str> {
    let mut i = 0;
    std::iter::from_fn(move || {
        if i == s.len() {
            return None;
        }

        // Skip list separators
        if s.as_bytes()[i] == b',' {
            i += 1;
        }

        // If the current char is a list delimiter, return it
        if b"[]".contains(&s.as_bytes()[i]) {
            let tok = &s[i..=i];
            i += 1;
            return Some(tok);
        }
        // Otherwise it's an integer; return it
        for j in i + 1..s.len() {
            if b",[]".contains(&s.as_bytes()[j]) {
                let tok = &s[i..j];
                i = j;
                return Some(tok);
            }
        }
        let tok = &s[i..];
        i = s.len();
        Some(tok)
    })
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Int(lhs), Item::Int(rhs)) => lhs.cmp(rhs),
            (Item::List(lhs), Item::List(rhs)) => lhs.cmp(rhs),
            (Item::List(lhs), Item::Int(rhs)) => lhs.cmp(&vec![Item::Int(*rhs)]),
            (Item::Int(lhs), Item::List(rhs)) => vec![Item::Int(*lhs)].cmp(rhs),
        }
    }
}
