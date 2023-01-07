use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct NoneErr;

impl Display for NoneErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("got Option::None")
    }
}

impl std::error::Error for NoneErr {}
