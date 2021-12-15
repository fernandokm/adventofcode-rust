use std::cell::Cell;

use anyhow::Context;
use aoc::ProblemOutput;
use rustc_hash::FxHashMap;

aoc::register!(solve, 2021, 12);

const START: &str = "start";
const END: &str = "end";

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let caves = parse_input(input)?;
    out.set_part1(count_paths(&caves, START, false));
    out.set_part2(count_paths(&caves, START, true));
    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<FxHashMap<&str, Cave>> {
    let mut caves: FxHashMap<&str, Cave> = FxHashMap::default();
    for line in input.trim().lines() {
        let (a, b) = line.split_once('-').context("invalid input")?;
        caves.entry(a).or_default().neighbors.push(b);
        caves.entry(b).or_default().neighbors.push(a);
    }
    Ok(caves)
}

fn count_paths<'a>(
    caves: &'a FxHashMap<&'a str, Cave>,
    name: &'a str,
    mut can_double_visit: bool,
) -> usize {
    let current_cave = &caves[name];
    if name == END {
        return 1;
    } else if name.chars().all(char::is_lowercase) && current_cave.visits.get() > 0 {
        if can_double_visit && name != START {
            can_double_visit = false;
        } else {
            return 0;
        }
    }

    current_cave.visits.set(current_cave.visits.get() + 1);
    let count = current_cave
        .neighbors
        .iter()
        .map(|n| count_paths(caves, n, can_double_visit))
        .sum();
    current_cave.visits.set(current_cave.visits.get() - 1);

    count
}

#[derive(Default)]
struct Cave<'a> {
    visits: Cell<usize>,
    neighbors: Vec<&'a str>,
}
