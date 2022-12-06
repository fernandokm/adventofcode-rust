use std::str::FromStr;

use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 4);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut blocks = input.trim().split("\n\n");
    let numbers: Vec<u32> = blocks
        .next()
        .context("invalid input")?
        .split(',')
        .map(|x| x.parse())
        .try_collect()?;

    let mut boards: Vec<Board> = blocks.map(FromStr::from_str).try_collect()?;
    let mut scores = Vec::with_capacity(boards.len());
    for &n in &numbers {
        for board in &mut boards {
            if let Some(score) = board.update(n) {
                scores.push(score)
            }
        }
    }

    out.set_part1(scores.first().unwrap());
    out.set_part2(scores.last().unwrap());

    Ok(())
}

struct Board {
    g: Vec<Vec<u32>>,
    done: bool,
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g = s
            .trim()
            .lines()
            .map(|line| line.split_whitespace().map(FromStr::from_str).collect())
            .try_collect()?;
        Ok(Board { g, done: false })
    }
}

impl Board {
    fn update(&mut self, n: u32) -> Option<u32> {
        if self.done {
            return None;
        }

        let (i, j) = self.find_pos(n)?;
        self.g[i][j] = 0;
        if self.g[i].iter().all(|&k| k == 0) || self.g.iter().all(|row| row[j] == 0) {
            self.done = true;
            Some(self.score(n))
        } else {
            None
        }
    }

    fn find_pos(&self, n: u32) -> Option<(usize, usize)> {
        self.g.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &m)| if n == m { Some((i, j)) } else { None })
        })
    }

    fn score(&self, n: u32) -> u32 {
        n * self
            .g
            .iter()
            .map(|row| row.iter().sum::<u32>())
            .sum::<u32>()
    }
}
