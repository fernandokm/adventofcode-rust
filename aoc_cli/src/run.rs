use std::{str::FromStr, time::Duration};

use aoc::{
    input::{self, Spec},
    ProblemId, ProblemOutput, Solver,
};
use clap::Args;
use itertools::Itertools;

use crate::terminal_writer;

const MAX_DROPPED_PERCENT: f64 = 0.05;

#[derive(Debug, Args)]
pub struct Cmd {
    #[clap(short, long, help = "Run all the solvers")]
    all: bool,

    #[clap(
        short = 'n',
        long,
        help = "Run each solver at least this many times",
        default_value = "1"
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
        required_unless_present("all"),
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
        let solvers = Solver::get_map();
        if self.all {
            for solver in solvers.values().sorted_by_key(|s| s.problem_id) {
                self.run_solver(solver, default_inputs)?;
            }
            return Ok(());
        }
        let problems_by_year = solvers.keys().copied().into_group_map_by(|val| val.year);
        for pf in &self.problems_filters {
            if let Some(day) = pf.day {
                let solver = solvers.get(&ProblemId { year: pf.year, day }).unwrap();
                self.run_solver(solver, default_inputs)?;
            } else if let Some(problems) = problems_by_year.get(&pf.year) {
                for &p in problems.iter().sorted() {
                    self.run_solver(solvers.get(&p).unwrap(), default_inputs)?;
                }
            } else {
                println!("No problems found for \"{}\"", pf.raw);
            }
        }
        Ok(())
    }

    fn run_solver(
        &self,
        solver: &Solver,
        default_inputs: &impl input::Source,
    ) -> anyhow::Result<()> {
        let mut input_specs = default_inputs
            .keys()
            .into_iter()
            .filter(|spec| spec.id == solver.problem_id)
            .sorted_unstable_by(|spec1, spec2| spec1.variant.cmp(&spec2.variant))
            .peekable();
        let mut writer = terminal_writer::TerminalWriter {
            color_choice: self.color.into(),
            quiet: self.quiet,
        };
        if input_specs.peek().is_none() {
            writer.error(&format!("No input files found for: {}", solver.problem_id))?;
        }

        let target_pfs = self
            .problems_filters
            .iter()
            .filter(|pf| {
                pf.year == solver.problem_id.year
                    && pf.day.map_or(true, |day| day == solver.problem_id.day)
            })
            .collect_vec();
        let all_variants = target_pfs.iter().any(|pf| pf.variant.is_none());
        let mut target_variants = target_pfs
            .iter()
            .filter_map(|pf| pf.variant.as_ref())
            .collect_vec();

        for spec in input_specs {
            let variant_pos = target_variants
                .iter()
                .find_position(|&&v| v == &spec.variant)
                .map(|(i, _)| i);
            if let Some(i) = variant_pos {
                target_variants.swap_remove(i);
            }
            if !self.all && !all_variants && variant_pos.is_none() {
                continue;
            }

            if self.min_runs <= 1 {
                Self::run_solver_once(spec, solver, &mut writer, default_inputs)?;
            } else {
                self.run_solver_bench(spec, solver, &mut writer, default_inputs)?;
            }
        }

        if !target_variants.is_empty() {
            writer.error(&format!(
                "Missing inputs: {}",
                target_variants.iter().join(", ")
            ))?;
        }

        Ok(())
    }

    fn run_solver_once(
        spec: &Spec,
        solver: &Solver,
        writer: &mut terminal_writer::TerminalWriter,
        default_inputs: &impl input::Source,
    ) -> anyhow::Result<()> {
        let mut out = ProblemOutput::start(spec, writer)?;
        let input = default_inputs.get(spec).unwrap();

        if let Err(e) = solver.solve(&input, &mut out) {
            writer.error(&e)?;
        }
        Ok(())
    }

    fn run_solver_bench(
        &self,
        spec: &Spec,
        solver: &Solver,
        writer: &mut terminal_writer::TerminalWriter,
        default_inputs: &impl input::Source,
    ) -> anyhow::Result<()> {
        let mut out = ProblemOutput::start(spec, writer)?;
        out.hide_solutions();
        let input = default_inputs.get(spec).unwrap();

        let mut err = None;
        for i in 0.. {
            out.reset_timer();
            if let Err(e) = solver.solve(&input, &mut out) {
                err = Some(e);
                break;
            }
            let total_time = out.total_time();
            if i >= self.min_runs - 1 && total_time >= self.min_duration_s {
                break;
            }
        }
        if let Some(err) = err {
            writer.error(&err)?;
            return Ok(());
        }
        out.show_solutions()?;
        let total_time = out.total_time();
        let dropped_time = out.dropped_time();
        let dropped_percent = dropped_time.as_secs_f64()
            / (dropped_time.as_secs_f64() + total_time.as_secs_f64())
            * 100.0;
        if dropped_percent > MAX_DROPPED_PERCENT * 100.0 {
            println!(
                "Warning: wasted {dropped_percent:.1}% of execution time \
                 (dropped={dropped_time:.1?}, useful={total_time:.1?})"
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ProblemFilter {
    raw: String,

    year: u32,
    day: Option<u32>,
    variant: Option<String>,
}

impl FromStr for ProblemFilter {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut pf = ProblemFilter {
            raw: s.to_owned(),
            year: 0,
            day: None,
            variant: None,
        };
        if let Some((rest, variant)) = s.split_once(':') {
            s = rest;
            pf.variant = Some(variant.to_owned());
        }
        if let Some((rest, day)) = s.split_once('.') {
            s = rest;
            pf.day = Some(day.parse()?);
        }
        pf.year = s.parse()?;
        Ok(pf)
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
