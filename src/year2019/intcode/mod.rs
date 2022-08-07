pub use self::core::{Computer, Error, Word};
pub use self::io::Channel;
pub use self::op::{Instruction, Op, Parameter};

mod core;
mod io;
mod op;
