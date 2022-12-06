use anyhow::{bail, Context};
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 2);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let strategy: Vec<_> = input
        .lines()
        .map(|line| {
            Some((
                line.as_bytes().first()? - b'A',
                line.as_bytes().get(2)? - b'X',
            ))
        })
        .collect::<Option<_>>()
        .context("Invalid input")?;

    out.set_part1(compute_score(&strategy, true)?);
    out.set_part2(compute_score(&strategy, false)?);

    Ok(())
}

fn compute_score(moves: &[(u8, u8)], is_part_1: bool) -> anyhow::Result<u32> {
    moves
        .iter()
        .map(|&(r, s)| {
            let move_opponent: Move = r.try_into()?;
            let move_self: Move = if is_part_1 {
                s.try_into()?
            } else {
                let target_score: u32 = (3 * s).into();
                Move::all()
                    .find(|x| x.play_score(move_opponent) == target_score)
                    .context("Invalid input")?
            };
            Ok(move_self.self_score() + move_self.play_score(move_opponent))
        })
        .fold_ok(0, u32::wrapping_add)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Move {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> anyhow::Result<Self> {
        Ok(match value {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => bail!("Invalid input: {value}"),
        })
    }
}

impl Move {
    pub fn all() -> impl Iterator<Item = Move> {
        [Move::Rock, Move::Paper, Move::Scissors].into_iter()
    }

    pub fn self_score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn play_score(self, other: Self) -> u32 {
        use Move::{Paper, Rock, Scissors};
        match (self, other) {
            (x, y) if (x == y) => 3,
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            _ => 0,
        }
    }
}
