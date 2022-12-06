use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2020, 5);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut ids = input.split_whitespace().map(parse_id).collect_vec();
    ids.sort_unstable();

    if let Some(&m) = ids.iter().max() {
        out.set_part1(m);
    }

    if let Some((x, _)) = ids
        .iter()
        .zip(ids.iter().skip(1))
        .find(|&(&x, &y)| x + 1 != y)
    {
        out.set_part2(x + 1);
    }

    Ok(())
}

#[must_use]
pub fn parse_id(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| {
        if c == 'B' || c == 'R' {
            2 * acc + 1
        } else {
            2 * acc
        }
    })
}
