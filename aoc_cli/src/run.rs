use std::str::FromStr;

use aoc::{ProblemId, Solver};
use itertools::Itertools;
use structopt::StructOpt;

use crate::terminal_backend;

#[derive(Debug, StructOpt)]
pub struct RunCmd {
    #[structopt(short, long, help = "Run all the solvers")]
    all: bool,

    #[structopt(
        short,
        long,
        help = "Run each solver multiple times",
        default_value = "1"
    )]
    repeat: u64,

    #[structopt(
        name = "problems",
        required_unless("all"),
        help = "A list of problems to be solved, in the format yyyy[.dd][:variant] (ignored if --all is specified)"
    )]
    problems_filters: Vec<ProblemFilter>,

    #[structopt(
        short,
        long,
        default_value = "auto",
        help = "Controls colored output (always, auto, never)"
    )]
    color: ColorChoice,

    #[structopt(short, long)]
    quiet: bool,
}

impl RunCmd {
    pub fn exec(&self, default_inputs: impl aoc::input::Input) -> anyhow::Result<()> {
        let solvers = Solver::get_map();
        if self.all {
            for solver in solvers.values().sorted_by_key(|s| s.problem_id) {
                self.run_solver(solver, &default_inputs)?;
            }
            return Ok(());
        }
        let problems_by_year = solvers.keys().copied().into_group_map_by(|val| val.year);
        for pf in &self.problems_filters {
            if let Some(day) = pf.day {
                let solver = solvers.get(&ProblemId { year: pf.year, day }).unwrap();
                self.run_solver(solver, &default_inputs)?;
            } else if let Some(problems) = problems_by_year.get(&pf.year) {
                for &p in problems.iter().sorted() {
                    self.run_solver(solvers.get(&p).unwrap(), &default_inputs)?;
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
        default_inputs: &impl aoc::input::Input,
    ) -> anyhow::Result<()> {
        let mut input_specs = default_inputs
            .keys()
            .into_iter()
            .filter(|spec| spec.id == solver.problem_id)
            .sorted_unstable_by(|spec1, spec2| spec1.variant.cmp(&spec2.variant))
            .peekable();
        let backend = terminal_backend::TerminalOutputBackend {
            color_choice: self.color.into(),
            quiet: self.quiet,
        };
        if input_specs.peek().is_none() {
            backend.error(&format!("No input files found for: {}", solver.problem_id))?;
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

            let mut out = aoc::ProblemOutput::start(spec, backend)?;
            out.disable_output();
            let input = default_inputs.get(spec).unwrap();

            for i in 0..self.repeat {
                if i == self.repeat - 1 {
                    out.enable_output();
                }
                if let Err(e) = solver.solve(&input, &mut out) {
                    backend.error(&e)?;
                    break;
                }
            }
        }

        if !target_variants.is_empty() {
            backend.error(&format!(
                "Missing inputs: {}",
                target_variants.iter().join(", ")
            ))?;
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
        if let Some((rest, variant)) = s.split_once(":") {
            s = rest;
            pf.variant = Some(variant.to_owned());
        }
        if let Some((rest, day)) = s.split_once(".") {
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
            ColorChoice::Never => termcolor::ColorChoice::Never,
            ColorChoice::Auto if atty::is(atty::Stream::Stdout) => termcolor::ColorChoice::Auto,
            ColorChoice::Auto => termcolor::ColorChoice::Never,
        }
    }
}
