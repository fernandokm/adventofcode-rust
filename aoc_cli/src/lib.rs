use structopt::StructOpt;

pub use terminal_backend::TerminalOutputBackend;
mod terminal_backend;

pub mod list;
pub mod run;

pub fn from_args() -> AocApp {
    AocApp::from_args()
}

#[derive(Debug, StructOpt)]
pub enum AocApp {
    Run(run::RunCmd),
    List(list::ListCmd),
}

impl AocApp {
    pub fn exec(&self, default_inputs: impl aoc::input::Input) -> anyhow::Result<()> {
        match self {
            AocApp::Run(cmd) => cmd.exec(default_inputs),
            AocApp::List(cmd) => cmd.exec(default_inputs),
        }
    }
}
