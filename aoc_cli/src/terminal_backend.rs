use std::{fmt::Display, io::Write, time::Duration};

use aoc::input::InputSpec;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Clone, Copy)]
pub struct TerminalOutputBackend {
    pub color_choice: ColorChoice,
}

impl TerminalOutputBackend {
    fn write_block(&self, stdout: &mut StandardStream, block: &dyn Display) -> aoc::Result<()> {
        let s = block.to_string();
        let s = s.trim();
        if s.contains('\n') {
            for line in s.lines() {
                write!(stdout, "\n    {}", line)?
            }
            write!(stdout, "\n    ")?;
        } else {
            write!(stdout, "{}", s)?;
        }
        Ok(())
    }

    pub fn error(&self, err: &dyn std::fmt::Debug) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        self.write_block(&mut stdout, &format!("{:?}", err))?;
        writeln!(stdout)?;
        Ok(())
    }
}

impl aoc::ProblemOutputBackend for TerminalOutputBackend {
    fn start(&mut self, spec: &InputSpec) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.reset()?;
        stdout.set_color(ColorSpec::new().set_bold(true))?;

        write!(stdout, "Year {}, day {}", spec.id.year, spec.id.day)?;
        write!(stdout, " ({})", spec.variant)?;
        writeln!(stdout)?;
        Ok(())
    }

    fn set_solution(
        &mut self,
        part: u32,
        exec_time: Duration,
        solution: &dyn Display,
    ) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.reset()?;
        write!(stdout, "    [part {}] ", part)?;
        self.write_block(&mut stdout, solution)?;

        stdout.set_color(ColorSpec::new().set_dimmed(true))?;
        writeln!(stdout, "    (finished in {:.1?})", exec_time)?;
        Ok(())
    }
}
