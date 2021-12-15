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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Op {
    pub fn from_opcode<W: Word>(opcode: W) -> Result<Op, Error<W>> {
        match () {
            _ if opcode == W::from(1) => Ok(Op::Add),
            _ if opcode == W::from(2) => Ok(Op::Mul),
            _ if opcode == W::from(3) => Ok(Op::Input),
            _ if opcode == W::from(4) => Ok(Op::Output),
            _ if opcode == W::from(5) => Ok(Op::JumpIfTrue),
            _ if opcode == W::from(6) => Ok(Op::JumpIfFalse),
            _ if opcode == W::from(7) => Ok(Op::LessThan),
            _ if opcode == W::from(8) => Ok(Op::Equals),
            _ if opcode == W::from(99) => Ok(Op::Halt),
            _ => Err(Error::InvalidOpcode(opcode)),
        }
    }

    pub fn param_count(self) -> usize {
        match self {
            Op::Halt => 0,
            Op::Input | Op::Output => 1,
            Op::JumpIfTrue | Op::JumpIfFalse => 2,
            Op::Add | Op::Mul | Op::LessThan | Op::Equals => 3,
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
            Op::Input => comp.input.read().and_then(|val| params[0].set(comp, val))?,
            Op::Output => comp.output.write(params[0].get(comp))?,
            Op::JumpIfTrue => Self::jump_if(comp, params, |x| x != W::from(0)),
            Op::JumpIfFalse => Self::jump_if(comp, params, |x| x == W::from(0)),
            Op::LessThan => Self::binary(comp, params, |x, y| if x < y { 1 } else { 0 })?,
            Op::Equals => Self::binary(comp, params, |x, y| if x == y { 1 } else { 0 })?,
            Op::Halt => comp.halted = true,
        }
        Ok(())
    }

    fn binary<W: Word, IW: Into<W>>(
        comp: &mut Computer<W>,
        params: &[Parameter<W>],
        f: impl Fn(W, W) -> IW,
    ) -> Result<(), Error<W>> {
        let in1 = params[0].get(comp);
        let in2 = params[1].get(comp);
        params[2].set(comp, f(in1, in2).into())
    }

    fn jump_if<W: Word>(comp: &mut Computer<W>, params: &[Parameter<W>], f: impl Fn(W) -> bool) {
        let in1 = params[0].get(comp);
        if f(in1) {
            // No need to worry about the instruction pointer being incremented
            // at the end of the instruction (as warned in AoC page),
            // since in out architecture the jump occurs after the ip is incremented.
            comp.ip = params[1].get(comp);
        }
    }
}

#[derive(Debug)]
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
