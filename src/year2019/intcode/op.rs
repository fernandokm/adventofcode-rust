use super::{Computer, Error, Word};

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Mul,
    Halt,
}

impl Op {
    pub fn exec<W: Word>(&self, comp: &mut Computer<W>) -> Result<(), Error<W>> {
        match self {
            Op::Add => Self::binary_op(comp, |x, y| x + y),
            Op::Mul => Self::binary_op(comp, |x, y| x * y),
            Op::Halt => comp.halted = true,
        }

        Ok(())
    }

    fn binary_op<W: Word>(comp: &mut Computer<W>, f: impl Fn(W, W) -> W) {
        let in1_addr = comp.next_word();
        let in1 = comp.ram_at(in1_addr);
        let in2_addr = comp.next_word();
        let in2 = comp.ram_at(in2_addr);
        let out_addr = comp.next_word();
        comp.ram.insert(out_addr, f(in1, in2));
    }
}

impl Op {
    pub fn from_opcode<W: Word>(opcode: W) -> Result<Op, Error<W>> {
        match () {
            _ if opcode == W::from(1) => Ok(Op::Add),
            _ if opcode == W::from(2) => Ok(Op::Mul),
            _ if opcode == W::from(99) => Ok(Op::Halt),
            _ => Err(Error::InvalidOpcode(opcode)),
        }
    }
}
