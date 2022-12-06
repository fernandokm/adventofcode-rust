pub use self::{
    core::{Computer, Error, Word},
    io::Channel,
    op::{Instruction, Op, Parameter},
};

mod core;
mod io;
mod op;
