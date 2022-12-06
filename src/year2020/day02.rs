use anyhow::Context;
use aoc::ProblemOutput;
use lazy_static::lazy_static;
use regex::Regex;

aoc::register!(solve, 2020, 2);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\w): (\\w+)").unwrap();
    }

    let mut valid1 = 0;
    let mut valid2 = 0;
    for line in input.lines() {
        let cap = RE.captures(line).context("Invalid input")?;
        let a = cap
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .context("Invalid input")?;
        let b = cap
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .context("Invalid input")?;
        let char = cap.get(3).unwrap().as_str().chars().next().unwrap();
        let password = cap.get(4).unwrap().as_str();

        if (a..=b).contains(&password.chars().filter(|&c| c == char).count()) {
            valid1 += 1;
        }

        if (password.chars().nth(a - 1) == Some(char)) ^ (password.chars().nth(b - 1) == Some(char))
        {
            valid2 += 1;
        }
    }
    out.set_part1(valid1);
    out.set_part2(valid2);

    Ok(())
}
