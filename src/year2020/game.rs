use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Game<'a> {
    acc: isize,
    pos: usize,
    mem: &'a [Instruction],
}

impl<'a> Game<'a> {
    pub fn new(mem: &'a [Instruction]) -> Self {
        Self {
            acc: 0,
            pos: 0,
            mem,
        }
    }

    pub fn offset(&mut self, offset: isize) -> anyhow::Result<()> {
        self.pos = if offset == -offset && offset != 0 {
            None
        } else if offset > 0 {
            self.pos.checked_add(offset as usize)
        } else {
            self.pos.checked_sub((-offset) as usize)
        }
        .ok_or_else(|| anyhow!("offset out of bounds: {}", offset))?;

        Ok(())
    }

    pub fn acc(&self) -> isize {
        self.acc
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn execute_single(&mut self) -> anyhow::Result<()> {
        let instruction = self.mem[self.pos as usize];
        instruction.apply(self)
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
    pub op: Op,
    pub arg: isize,
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
