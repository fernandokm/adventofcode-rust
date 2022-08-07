use std::str::FromStr;

use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2020, 4);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let passports: Vec<Passport> = input
        .trim()
        .split("\n\n")
        .map(str::parse)
        .try_collect()
        .context("Invalid input")?;

    out.set_part1(passports.iter().filter(|p| p.is_valid_part1()).count());
    out.set_part2(passports.iter().filter(|p| p.is_valid_part2()).count());

    Ok(())
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_part2(&self) -> bool {
        self.byr
            .as_ref()
            .map_or(false, |val| is_in_range(1920, 2002, val))
            && self
                .iyr
                .as_ref()
                .map_or(false, |val| is_in_range(2010, 2020, val))
            && self
                .eyr
                .as_ref()
                .map_or(false, |val| is_in_range(2020, 2030, val))
            && self.hgt.as_ref().map_or(false, |val| {
                let i = val.len() - 2;
                match &val[i..] {
                    "cm" => is_in_range(150, 193, &val[..i]),
                    "in" => is_in_range(59, 76, &val[..i]),
                    _ => false,
                }
            })
            && self.hcl.as_ref().map_or(false, |val| {
                val.len() == 7
                    && val.starts_with('#')
                    && val.chars().skip(1).all(|c| "0123456789abcdef".contains(c))
            })
            && self.ecl.as_ref().map_or(false, |val| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val.as_str())
            })
            && self.pid.as_ref().map_or(false, |val| val.len() == 9)
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Passport::default();
        for kv in s.split_whitespace() {
            let (k, v) = kv
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("missing separator \":\""))?;

            match k {
                "byr" => p.byr = Some(v.to_string()),
                "iyr" => p.iyr = Some(v.to_string()),
                "eyr" => p.eyr = Some(v.to_string()),
                "hgt" => p.hgt = Some(v.to_string()),
                "hcl" => p.hcl = Some(v.to_string()),
                "ecl" => p.ecl = Some(v.to_string()),
                "pid" => p.pid = Some(v.to_string()),
                "cid" => (),
                _ => anyhow::bail!("unexpected key {}", k),
            }
        }

        Ok(p)
    }
}

fn is_in_range(low: usize, high: usize, val: &str) -> bool {
    match val.parse() {
        Ok(val) => (low..=high).contains(&val),
        Err(_) => false,
    }
}
