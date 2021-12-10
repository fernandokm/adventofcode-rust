use std::{
    hash::Hash,
    ops::{Add, Mul},
    str::FromStr,
};

use anyhow::Context;
use rustc_hash::FxHashMap;

use super::Op;

pub trait Word:
    Copy + Eq + Hash + Add<Output = Self> + Mul<Output = Self> + From<u8> + FromStr
{
}

impl<T> Word for T where
    T: Copy + Eq + Hash + Add<Output = Self> + Mul<Output = Self> + From<u8> + FromStr + Send
{
}

#[derive(thiserror::Error, Debug)]
pub enum Error<Word> {
    #[error("intcode error: invalid opcode: {0}")]
    InvalidOpcode(Word),
}

#[derive(Debug, Clone)]
pub struct Computer<W: Word> {
    pub ram: FxHashMap<W, W>,
    pub ip: W,
    pub halted: bool,
}

impl<W: Word, E> FromStr for Computer<W>
where
    W: FromStr<Err = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Computer<W>, Self::Err> {
        let mut ram = FxHashMap::default();
        let mut i = W::from(0);
        for raw_word in s.trim().split(',') {
            ram.insert(i, raw_word.parse().context("error parsing intcode")?);
            i = i + W::from(1);
        }
        Ok(Computer {
            ram,
            ip: W::from(0),
            halted: false,
        })
    }
}

impl<W: Word> Computer<W> {
    pub fn ram_at(&self, pos: W) -> W {
        self.ram.get(&pos).copied().unwrap_or_else(|| W::from(0))
    }

    pub fn next_word(&mut self) -> W {
        let w = self.ram_at(self.ip);
        self.ip = self.ip + W::from(1);
        w
    }

    pub fn exec_single(&mut self) -> Result<(), Error<W>> {
        let op = Op::from_opcode(self.next_word())?;
        op.exec(self)
    }

    pub fn exec(&mut self) -> Result<(), Error<W>> {
        while !self.halted {
            self.exec_single()?;
        }
        Ok(())
    }
}
