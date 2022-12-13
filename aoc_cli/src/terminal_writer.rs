use std::{fmt::Display, io::Write};

use aoc::{input, stats::Stats, Part, SolutionWriter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use thousands::Separable;

#[derive(Debug, Clone, Copy)]
enum OutputType {
    Inline,
    Block,
}

#[derive(Debug, Clone, Copy)]
pub struct TerminalWriter {
    pub color_choice: ColorChoice,
    pub quiet: bool,
}

impl TerminalWriter {
    fn write(stdout: &mut StandardStream, content: &dyn Display) -> aoc::Result<OutputType> {
        let s = content.to_string();
        let s = s.trim();
        if s.contains('\n') {
            let indent = str::repeat(" ", 8);
            for line in s.lines() {
                write!(stdout, "\n{}{}", indent, line)?;
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
        Self::write(&mut stdout, &format!("{:?}", err))?;
        writeln!(stdout)?;
        stdout.reset()?;
        Ok(())
    }

    pub fn warn(&self, msg: &dyn std::fmt::Debug) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        Self::write(&mut stdout, &format!("{:?}", msg))?;
        writeln!(stdout)?;
        stdout.reset()?;
        Ok(())
    }
}

impl SolutionWriter for TerminalWriter {
    fn write_heading(&mut self, spec: &input::Spec) -> aoc::Result<()> {
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

    fn write_solution(
        &mut self,
        part: Part,
        stats: &Stats,
        solution: &dyn Display,
    ) -> aoc::Result<()> {
        let mut stdout = StandardStream::stdout(self.color_choice);
        write!(stdout, "    [part {}] ", part)?;
        let out_type = Self::write(&mut stdout, solution)?;

        if self.quiet {
            writeln!(stdout)?;
        } else {
            match out_type {
                OutputType::Block => write!(stdout, "        ")?,
                OutputType::Inline => write!(stdout, "    ")?,
            }
            stdout.set_color(ColorSpec::new().set_dimmed(true))?;
            let Stats {
                exec_count,
                exec_time_total,
                exec_time_mean,
                exec_time_std,
            } = stats;
            if let Some(exec_time_std) = exec_time_std {
                let exec_count = exec_count.separate_with_underscores();
                #[allow(clippy::cast_precision_loss)]
                let std_percent =
                    (exec_time_std.as_nanos() as f64) / (exec_time_mean.as_nanos() as f64) * 100.0;
                writeln!(
                    stdout,
                    "(finished in {exec_time_mean:.1?} ± {exec_time_std:.1?} \
                     (±{std_percent:.1}%), {exec_time_total:.1?}/{exec_count} runs)"
                )?;
            } else {
                writeln!(stdout, "(finished in {exec_time_mean:.1?})")?;
            }
        }

        stdout.reset()?;
        Ok(())
    }
}
