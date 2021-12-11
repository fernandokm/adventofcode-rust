use itertools::Itertools;
use tinyvec::ArrayVec;

use super::{Computer, Error, Word};

#[derive(Debug, Clone, Copy)]
pub enum Parameter<W: Word> {
    Position(W),
    Immediate(W),
}

impl<W: Word> Default for Parameter<W> {
    fn default() -> Self {
        Self::Immediate(W::from(0))
    }
}

impl<W: Word> Parameter<W> {
    pub fn from_mode_and_val(mode: W, val: W) -> Result<Self, Error<W>> {
        match () {
            _ if mode == W::from(0) => Ok(Parameter::Position(val)),
            _ if mode == W::from(1) => Ok(Parameter::Immediate(val)),
            _ => Err(Error::InvalidParameterMode(mode)),
        }
    }

    pub fn get(self, comp: &Computer<W>) -> W {
        match self {
            Parameter::Position(pos) => comp.ram_at(pos),
            Parameter::Immediate(val) => val,
        }
    }

    pub fn set(self, comp: &mut Computer<W>, val: W) -> Result<(), Error<W>> {
        match self {
            Parameter::Position(pos) => {
                comp.ram.insert(pos, val);
            }
            Parameter::Immediate(_) => return Err(Error::ReadonlyParameter { mode: "immediate" }),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Mul,
    Input,
    Output,
    Halt,
}

impl Op {
    pub fn from_opcode<W: Word>(opcode: W) -> Result<Op, Error<W>> {
        match () {
            _ if opcode == W::from(1) => Ok(Op::Add),
            _ if opcode == W::from(2) => Ok(Op::Mul),
            _ if opcode == W::from(3) => Ok(Op::Input),
            _ if opcode == W::from(4) => Ok(Op::Output),
            _ if opcode == W::from(99) => Ok(Op::Halt),
            _ => Err(Error::InvalidOpcode(opcode)),
        }
    }

    pub fn param_count(self) -> usize {
        match self {
            Op::Add | Op::Mul => 3,
            Op::Input | Op::Output => 1,
            Op::Halt => 0,
        }
    }

    pub fn exec<W: Word>(
        self,
        comp: &mut Computer<W>,
        params: &[Parameter<W>],
    ) -> Result<(), Error<W>> {
        match self {
            Op::Add => Self::binary(comp, params, |x, y| x + y)?,
            Op::Mul => Self::binary(comp, params, |x, y| x * y)?,
            Op::Input => {
                let in1 = comp.input.pop().ok_or(Error::EndOfInput)?;
                params[0].set(comp, in1)?;
            }
            Op::Output => comp.output.push(params[0].get(comp)),
            Op::Halt => comp.halted = true,
        }
        Ok(())
    }

    fn binary<W: Word>(
        comp: &mut Computer<W>,
        params: &[Parameter<W>],
        f: impl Fn(W, W) -> W,
    ) -> Result<(), Error<W>> {
        params[2].set(comp, f(params[0].get(comp), params[1].get(comp)))
    }
}

pub struct Instruction<W: Word> {
    op: Op,
    params: ArrayVec<[Parameter<W>; 3]>,
}

impl<W: Word> Instruction<W> {
    pub fn next_from(comp: &mut Computer<W>) -> Result<Self, Error<W>> {
        let mut w = comp.next_word();
        let op = Op::from_opcode(w % W::from(100))?;
        w = w / W::from(100);

        let params = (0..op.param_count())
            .map(|_| {
                let mode = w % W::from(10);
                w = w / W::from(10);
                Parameter::from_mode_and_val(mode, comp.next_word())
            })
            .try_collect()?;

        Ok(Instruction { op, params })
    }

    pub fn exec(&self, comp: &mut Computer<W>) -> Result<(), Error<W>> {
        self.op.exec(comp, &self.params)
    }
}
