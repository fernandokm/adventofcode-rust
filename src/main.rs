#![deny(rust_2018_idioms)]

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "inputs"]
struct EmbeddedInput;

pub mod util;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;

fn main() -> anyhow::Result<()> {
    let app = aoc_cli::parse();
    let default_inputs = aoc::input::from_embedded::<EmbeddedInput>()?;
    app.exec(default_inputs)
}
