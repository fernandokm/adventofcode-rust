use std::fmt::Display;

pub static NONE_ERR: NoneErrType = NoneErrType();

#[derive(Debug, Clone, Copy)]
pub struct NoneErrType();

impl Display for NoneErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("got Option::None")
    }
}

impl std::error::Error for NoneErrType {}
