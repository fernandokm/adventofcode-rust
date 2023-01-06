#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms)]
// "Quality-of-life for common operations in AoC solutions"
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::match_on_vec_items,
    clippy::similar_names
)]
// No docs for AoC
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use aoc::input;
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
    let default_inputs = input::from_embedded::<EmbeddedInput>()?;
    app.exec(&default_inputs)
}
