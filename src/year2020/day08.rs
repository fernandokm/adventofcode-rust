use std::str::FromStr;

use anyhow::anyhow;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashSet;

aoc::register!(solve, 2020, 8);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut game = Game::new(input.lines().map(FromStr::from_str).try_collect()?);
    game.execute_to_end()?;
    out.set_part1(game.acc);

    for i in 0..game.mem.len() {
        let old_op = game.mem[i].op;
        match old_op {
            Op::Jmp => game.mem[i].op = Op::Nop,
            Op::Nop => game.mem[i].op = Op::Jmp,
            Op::Acc => continue,
        };

        game.reset();
        game.execute_to_end()?;
        if game.pos == game.mem.len() {
            out.set_part2(game.acc);
            return Ok(());
        }

        game.mem[i].op = old_op;
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Game {
    acc: isize,
    pos: usize,
    mem: Vec<Instruction>,
    visited: FxHashSet<usize>,
}

impl Game {
    #[must_use]
    pub fn new(mem: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            pos: 0,
            mem,
            visited: FxHashSet::default(),
        }
    }

    pub fn offset(&mut self, offset: isize) -> anyhow::Result<()> {
        self.pos = if offset == -offset && offset != 0 {
            None
        } else if offset > 0 {
            self.pos.checked_add(offset.unsigned_abs())
        } else {
            self.pos.checked_sub(offset.unsigned_abs())
        }
        .ok_or_else(|| anyhow!("offset out of bounds: {}", offset))?;

        Ok(())
    }

    pub fn execute_single(&mut self) -> anyhow::Result<()> {
        let instruction = self.mem[self.pos as usize];
        instruction.apply(self)
    }

    pub fn execute_to_end(&mut self) -> anyhow::Result<()> {
        while self.visited.insert(self.pos) && self.pos < self.mem.len() {
            self.execute_single()?;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.acc = 0;
        self.visited.clear();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            "nop" => Op::Nop,
            _ => anyhow::bail!("invalid operation: {}", s),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    op: Op,
    arg: isize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid instruction: {}", s))?;
        Ok(Instruction::new(op.parse()?, arg.parse()?))
    }
}

impl Instruction {
    #[must_use]
    pub fn new(op: Op, arg: isize) -> Self {
        Self { op, arg }
    }

    pub fn apply(&self, game: &mut Game) -> anyhow::Result<()> {
        match self.op {
            Op::Acc => {
                game.acc += self.arg;
                game.offset(1)
            }
            Op::Jmp => game.offset(self.arg),
            Op::Nop => game.offset(1),
        }
    }
}
