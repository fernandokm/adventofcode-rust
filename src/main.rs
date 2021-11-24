use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "inputs"]
struct EmbeddedInput;

mod year2020;

fn main() -> anyhow::Result<()> {
    let app = aoc_cli::from_args();
    let default_inputs = aoc::input::from_embedded::<EmbeddedInput>()?;
    app.exec(default_inputs)
}
