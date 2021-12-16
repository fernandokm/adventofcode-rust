use std::{fmt::Display, io::Write, time::Duration};

use aoc::input::InputSpec;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Clone, Copy)]
enum OutputType {
    Inline,
    Block,
}

#[derive(Debug, Clone, Copy)]
pub struct TerminalOutputBackend {
    pub color_choice: ColorChoice,
    pub quiet: bool,
}

impl TerminalOutputBackend {
    fn write(&self, stdout: &mut StandardStream, content: &dyn Display) -> aoc::Result<OutputType> {
        let s = content.to_string();
        let s = s.trim();
        if s.contains('\n') {
            let indent = str::repeat(" ", 8);
            for line in s.lines() {
                write!(stdout, "\n{}{}", indent, line)?
            }
            Ok(OutputType::Block)
        } else {
            write!(stdout, "{}", s)?;
            Ok(OutputType::Inline)
        }
    }

    pub fn error(&self, err: &dyn std::fmt::Debug) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        self.write(&mut stdout, &format!("{:?}", err))?;
        writeln!(stdout)?;
        stdout.reset()?;
        Ok(())
    }
}

impl aoc::ProblemOutputBackend for TerminalOutputBackend {
    fn start(&mut self, spec: &InputSpec) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.set_color(ColorSpec::new().set_bold(true))?;

        writeln!(
            stdout,
            "Problem {}.{} ({})",
            spec.id.year, spec.id.day, spec.variant
        )?;

        stdout.reset()?;
        Ok(())
    }

    fn set_solution(
        &mut self,
        part: u32,
        exec_time: Duration,
        exec_time_err: Option<Duration>,
        solution: &dyn Display,
    ) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        write!(stdout, "    [part {}] ", part)?;
        let out_type = self.write(&mut stdout, solution)?;

        if self.quiet {
            writeln!(stdout)?;
        } else {
            match out_type {
                OutputType::Block => write!(stdout, "        ")?,
                OutputType::Inline => write!(stdout, "    ")?,
            }
            stdout.set_color(ColorSpec::new().set_dimmed(true))?;
            if let Some(err) = exec_time_err {
                writeln!(stdout, "(finished in {:.1?} ± {:.1?})", exec_time, err)?;
            } else {
                writeln!(stdout, "(finished in {:.1?})", exec_time)?;
            }
        }

        stdout.reset()?;
        Ok(())
    }
}
