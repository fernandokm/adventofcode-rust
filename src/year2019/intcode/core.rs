use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Div, Mul, Rem},
    str::FromStr,
};

use anyhow::Context;
use rustc_hash::FxHashMap;

use super::{Channel, Instruction};

pub trait Word:
    Copy
    + Eq
    + Ord
    + Hash
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + From<u8>
    + FromStr
    + Debug
{
}

impl<T> Word for T where
    T: Copy
        + Eq
        + Ord
        + Hash
        + Add<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + From<u8>
        + FromStr
        + Send
        + Debug
{
}

#[derive(thiserror::Error, Debug)]
pub enum Error<Word> {
    #[error("intcode error: computer is halted")]
    Halted,

    #[error("intcode error: invalid opcode: {0}")]
    InvalidOpcode(Word),

    #[error("intcode error: invalid parameter mode: {0}")]
    InvalidParameterMode(Word),

    #[error("intcode error: cannot set readonly parameter (parameter mode: {mode})")]
    ReadonlyParameter { mode: &'static str },

    #[error("intcode error: end of input (no more input available)")]
    EndOfInput,

    #[error("intcode error: deadlock detected")]
    Deadlock,

    #[error("intcode error: output buffer overflow (max_len={max_len})")]
    OutputBufferOverflow { max_len: usize },
}

#[derive(Debug, Clone)]
pub struct Computer<W: Word> {
    pub ram: FxHashMap<W, W>,
    pub ip: W,
    pub relative_base: W,
    pub halted: bool,
    pub input: Channel<W>,
    pub output: Channel<W>,

    initial_ram: FxHashMap<W, W>,
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
            initial_ram: ram.clone(),
            ram,
            ip: W::from(0),
            relative_base: W::from(0),
            halted: false,
            input: Channel::default(),
            output: Channel::default(),
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
        if self.halted {
            return Err(Error::Halted);
        }

        let old_ip = self.ip;
        let inst = Instruction::next_from(self)?;

        let result = inst.exec(self);
        if result.is_err() {
            self.ip = old_ip; // undo instruction pointer changes
        }
        result
    }

    pub fn exec(&mut self) -> Result<(), Error<W>> {
        while !self.halted {
            self.exec_single()?;
        }
        Ok(())
    }

    pub fn exec_all(comps: &mut [Computer<W>]) -> Result<(), Error<W>> {
        loop {
            let mut deadlock = true;
            for comp in comps.iter_mut() {
                if !comp.halted {
                    match comp.exec_single() {
                        Ok(_) => deadlock = false,
                        Err(Error::EndOfInput | Error::OutputBufferOverflow { .. }) => continue,
                        err @ Err(_) => return err,
                    }
                }
            }
            if deadlock {
                if comps.iter().all(|c| c.halted) {
                    return Ok(());
                } else {
                    return Err(Error::Deadlock);
                };
            }
        }
    }

    pub fn reset(&mut self) {
        self.ip = W::from(0);
        self.relative_base = W::from(0);
        self.ram = self.initial_ram.clone();
        self.halted = false;

        self.input.clear();
        self.output.clear();
    }
}
