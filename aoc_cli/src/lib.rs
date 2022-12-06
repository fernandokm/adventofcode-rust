#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms)]
// TODO: enable docs lints (remove the following line)
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use aoc::input;
use clap::Parser;
pub use terminal_backend::TerminalOutputBackend;
mod terminal_backend;

pub mod list;
pub mod run;

#[must_use]
pub fn parse() -> AocApp {
    AocApp::parse()
}

#[derive(Debug, Parser)]
pub enum AocApp {
    Run(run::Cmd),
    List(list::Cmd),
}

impl AocApp {
    pub fn exec(&self, default_inputs: &impl input::Source) -> anyhow::Result<()> {
        match self {
            AocApp::Run(cmd) => cmd.exec(default_inputs),
            AocApp::List(cmd) => cmd.exec(default_inputs),
        }
    }
}
