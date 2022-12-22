use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Context};
use aoc::{
    input::{self, Spec},
    ProblemOutput, Solver,
};
use clap::Args;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::terminal_writer;

const MAX_DROPPED_PERCENT: f64 = 0.25;

#[derive(Debug, Args)]
pub struct Cmd {
    #[clap(
        short = 'n',
        long,
        help = "Run each solver at least this many times",
        default_value = "0"
    )]
    min_runs: u64,

    #[clap(
        short = 't',
        long,
        value_parser = parse_duration_s,
        help = "Run each solver for at least this many seconds (we consider the total time for both parts of each day)",
        default_value = "0",
    )]
    min_duration_s: Duration,

    #[clap(
        name = "problems",
        help = "A list of problems to be solved, in the format yyyy[.dd][:variant] (ignored if \
                --all is specified)"
    )]
    problems_filters: Vec<ProblemFilter>,

    #[clap(
        short,
        long,
        default_value = "auto",
        help = "Controls colored output (always, auto, never)"
    )]
    color: ColorChoice,

    #[clap(short, long)]
    quiet: bool,
}

impl Cmd {
    pub fn exec(&self, default_inputs: &impl input::Source) -> anyhow::Result<()> {
        let specs = self.find_specs(default_inputs);
        let solvers = Solver::get_map();
        for spec in specs {
            let solver = solvers
                .get(&spec.id)
                .ok_or_else(|| anyhow!("No solver found for problem {}", spec.id))?;
            let input = default_inputs.get(spec).unwrap();
            self.run_solver(spec, solver, &input)?;
        }
        Ok(())
    }

    fn find_specs<'a>(&self, default_inputs: &'a impl input::Source) -> Vec<&'a Spec> {
        let mut specs = FxHashSet::default();
        let mut useful = vec![false; self.problems_filters.len()];
        for spec in default_inputs.keys() {
            for (i, pf) in self.problems_filters.iter().enumerate() {
                if pf.matches_spec(spec) {
                    useful[i] = true;
                    specs.insert(spec);
                }
            }
        }
        let not_useful = useful
            .into_iter()
            .enumerate()
            .filter(|(_i, useful)| !useful)
            .map(|(i, _useful)| &self.problems_filters[i].raw)
            .collect_vec();
        if !not_useful.is_empty() {
            println!(
                "Warning: the following filters didn't match any problems (or there were no \
                 inputs available):\n  {}",
                not_useful.into_iter().join("\n  ")
            );
        }
        specs.into_iter().sorted_unstable().collect()
    }

    fn run_solver(&self, spec: &Spec, solver: &Solver, input: &str) -> anyhow::Result<()> {
        let mut writer = terminal_writer::TerminalWriter {
            color_choice: self.color.into(),
            quiet: self.quiet,
        };

        if self.min_runs <= 1 && self.min_duration_s == Duration::ZERO {
            Self::run_solver_once(spec, solver, &mut writer, input)?;
        } else {
            self.run_solver_bench(spec, solver, &mut writer, input)?;
        }

        Ok(())
    }

    fn run_solver_once(
        spec: &Spec,
        solver: &Solver,
        writer: &mut terminal_writer::TerminalWriter,
        input: &str,
    ) -> anyhow::Result<()> {
        let mut out = ProblemOutput::start(spec, writer)?;
        if let Err(e) = solver.solve(input, &mut out) {
            writer.error(&e)?;
        }
        Ok(())
    }

    fn run_solver_bench(
        &self,
        spec: &Spec,
        solver: &Solver,
        writer: &mut terminal_writer::TerminalWriter,
        input: &str,
    ) -> anyhow::Result<()> {
        let mut out = ProblemOutput::start(spec, writer)?;
        out.hide_solutions();

        let mut err = None;
        for i in 0.. {
            out.reset_timer();
            if let Err(e) = solver.solve(input, &mut out) {
                err = Some(e);
                break;
            }
            let total_time = out.total_time();
            if i >= self.min_runs.saturating_sub(1) && total_time >= self.min_duration_s {
                break;
            }
        }
        if let Some(err) = err {
            writer.error(&err)?;
            return Ok(());
        }
        out.show_solutions()?;

        // Warn about dropped time
        let total_time = out.total_time();
        let dropped_time = out.dropped_time();
        let dropped_percent = dropped_time.as_secs_f64()
            / (dropped_time.as_secs_f64() + total_time.as_secs_f64())
            * 100.0;
        if !self.quiet && dropped_percent > MAX_DROPPED_PERCENT * 100.0 {
            writer.warn(&anyhow!(
                "Warning: wasted {dropped_percent:.1}% of execution time \
                 (dropped={dropped_time:.1?}, useful={total_time:.1?})"
            ))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ProblemFilter {
    pub raw: String,

    year: Option<u32>,
    day: Option<u32>,
    variant: Option<String>,
}

impl FromStr for ProblemFilter {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut pf = ProblemFilter {
            raw: s.to_owned(),
            year: None,
            day: None,
            variant: None,
        };
        let (raw_date, variant) = match s.split_once(':') {
            Some((raw_date, variant)) => (raw_date, Some(variant.to_owned())),
            None => (s, None),
        };
        pf.variant = variant;

        if let Some((year, day)) = raw_date.split_once('.') {
            if day != "*" {
                pf.day = Some(day.parse().with_context(|| format!("invalid day: {day}"))?);
            }
            s = year;
        } else {
            s = raw_date;
        }
        if s != "*" {
            pf.year = Some(s.parse().with_context(|| format!("invalid year: {s}"))?);
        }

        Ok(pf)
    }
}

impl ProblemFilter {
    pub fn matches_spec(&self, spec: &Spec) -> bool {
        self.day.map_or(true, |day| day == spec.id.day)
            && self.year.map_or(true, |year| year == spec.id.year)
            && self
                .variant
                .as_ref()
                .map_or(true, |variant| variant == &spec.variant)
    }
}

#[derive(Debug, Clone, Copy)]
enum ColorChoice {
    Always,
    Auto,
    Never,
}

impl FromStr for ColorChoice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "always" => Self::Always,
            "auto" => Self::Auto,
            "never" => Self::Never,
            _ => anyhow::bail!(
                "invalid color option \"{}\" (must be always, auto or never)",
                s.trim()
            ),
        })
    }
}

impl From<ColorChoice> for termcolor::ColorChoice {
    fn from(val: ColorChoice) -> Self {
        match val {
            ColorChoice::Always => termcolor::ColorChoice::Always,
            ColorChoice::Auto if atty::is(atty::Stream::Stdout) => termcolor::ColorChoice::Auto,
            ColorChoice::Never | ColorChoice::Auto => termcolor::ColorChoice::Never,
        }
    }
}

fn parse_duration_s(raw: &str) -> Result<Duration, <f64 as FromStr>::Err> {
    Ok(Duration::from_secs_f64(raw.parse()?))
}
