use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

aoc::register!(solve, 2020, 7);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let direct: FxHashMap<_, _> = input.lines().map(parse_line).try_collect()?;

    let mut full = FxHashMap::default();
    for color in direct.keys() {
        load_color(color, &direct, &mut full);
    }

    out.set_part1(
        full.values()
            .filter(|bags| bags.keys().contains(&"shiny gold"))
            .count(),
    );
    out.set_part2(full.get("shiny gold").unwrap().values().sum::<usize>());

    Ok(())
}

fn load_color<'a>(
    color: &'a str,
    direct: &FxHashMap<&'a str, FxHashMap<&'a str, usize>>,
    full: &mut FxHashMap<&'a str, FxHashMap<&'a str, usize>>,
) {
    if full.contains_key(color) {
        return;
    }

    let mut bags: FxHashMap<&'a str, usize> = FxHashMap::default();
    for (&direct_color, &direct_count) in direct.get(color).unwrap() {
        *bags.entry(direct_color).or_default() += direct_count;

        load_color(direct_color, direct, full);
        for (&indirect_color, &indirect_count) in full.get(direct_color).unwrap() {
            *bags.entry(indirect_color).or_default() += direct_count * indirect_count;
        }
    }
    full.insert(color, bags);
}

fn parse_line(line: &str) -> anyhow::Result<(&str, FxHashMap<&str, usize>)> {
    let line = line.trim_end_matches('.');
    let (key, val) = line
        .split_once("contain")
        .ok_or_else(|| anyhow::anyhow!("invalid input: {}", line))?;

    let key = trim_color(key);
    let val = if val.contains("no other bags") {
        FxHashMap::default()
    } else {
        val.split(',').map(parse_color_and_count).try_collect()?
    };

    Ok((key, val))
}

fn parse_color_and_count(s: &str) -> anyhow::Result<(&str, usize)> {
    let (count, color) = s
        .trim()
        .split_once(' ')
        .ok_or_else(|| anyhow::anyhow!("invalid input: {}", s))?;
    Ok((trim_color(color), count.parse()?))
}

fn trim_color(color: &str) -> &str {
    color
        .trim()
        .trim_end_matches("bag")
        .trim_end_matches("bags")
        .trim()
}
