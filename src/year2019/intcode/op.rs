use super::{Computer, Error, Word};

#[derive(Debug, Clone, Copy)]
pub enum Op0 {
    Halt,
}

impl Op0 {
    pub fn exec<W: Word>(self, comp: &mut Computer<W>) -> Result<(), Error<W>> {
        comp.halted = true;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op3 {
    Add,
    Mul,
}

impl Op3 {
    pub fn exec<W: Word>(
        self,
        xaddr: W,
        yaddr: W,
        zaddr: W,
        comp: &mut Computer<W>,
    ) -> Result<(), Error<W>> {
        let x = comp.ram_at(xaddr);
        let y = comp.ram_at(yaddr);
        match self {
            Op3::Add => comp.ram.insert(zaddr, x + y),
            Op3::Mul => comp.ram.insert(zaddr, x * y),
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Op3(Op3),
    Op0(Op0),
}

impl Op {
    pub fn exec<W: Word>(&self, comp: &mut Computer<W>) -> Result<(), Error<W>> {
        match self {
            Op::Op3(op3) => op3.exec(comp.next_word(), comp.next_word(), comp.next_word(), comp),
            Op::Op0(op0) => op0.exec(comp),
        }
    }
}

impl Op {
    pub fn from_opcode<W: Word>(opcode: W) -> Result<Op, Error<W>> {
        match () {
            _ if opcode == W::from(1) => Ok(Op::Op3(Op3::Add)),
            _ if opcode == W::from(2) => Ok(Op::Op3(Op3::Mul)),
            _ if opcode == W::from(99) => Ok(Op::Op0(Op0::Halt)),
            _ => Err(Error::InvalidOpcode(opcode)),
        }
    }
}
