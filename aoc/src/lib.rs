//! Core `AoC` code.
//!
//! Provides the tools used to define `AoC` solutions ([`register!`])
//! and to read input data ([`input::Input`]).

#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms)]
// TODO: enable docs lints (remove the following line)
#![allow(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]
// TODO: enable docs in private items (uncomment the following line)
// #![deny(clippy::missing_docs_in_private_items)]

use std::fmt::{Display, Write};

use input::Spec;
use linkme::distributed_slice;
use rustc_hash::FxHashMap;
use stats::Stats;

pub mod input;
pub mod stats;

/// The error type for `AoC` solver errors.
#[derive(Debug, thiserror::Error)]
pub enum SolverError {
    /// Indicates that there was an IO error when printing the
    /// result given by a solver.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Indicates that a given solver is not implemented.
    /// This is usually generated by calling the [`not_implemented!`] macro.
    #[error("Not implemented")]
    NotImplemented,

    /// Wraps any other error raised by the solver.
    #[error("Solver error: {0}")]
    SolverError(#[source] anyhow::Error),
}

/// Indicates that a solver is not implemented by returning a
/// [`SolverError::NotImplemented`] error.
#[macro_export]
macro_rules! not_implemented {
    () => {
        return Err(::aoc::SolverError::NotImplemented.into())
    };
}

/// A specialized [`Result`] for `AoC` solver errors.
pub type Result<T> = std::result::Result<T, SolverError>;

/// Uniquely identifies an `AoC` problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProblemId {
    /// The year in which the problem was published.
    pub year: u32,
    /// The day in which the problem was published
    /// (usually an integer from 1 to 25, inclusive).
    pub day: u32,
}

impl Display for ProblemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}.{}", self.year, self.day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

impl Part {
    #[must_use]
    pub fn to_index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => f.write_char('1'),
            Part::Two => f.write_char('2'),
        }
    }
}

// TODO: do we need both lifetimes here?
pub struct ProblemOutput<'a> {
    writer: &'a mut (dyn SolutionWriter + 'a),
    monitor: stats::Monitor,
}

impl<'a> ProblemOutput<'a> {
    pub fn start(spec: &Spec, writer: &'a mut (impl SolutionWriter + 'a)) -> Result<Self> {
        writer.write_heading(spec)?;

        let mut monitor = stats::Monitor::default();
        monitor.reset();
        Ok(Self { writer, monitor })
    }

    #[must_use]
    pub fn stats(&self, part: Part) -> Stats {
        self.monitor.stats(part)
    }

    pub fn reset_timer(&mut self) {
        self.monitor.reset();
    }

    fn try_set(&mut self, part: Part, solution: impl Display) -> Result<()> {
        self.monitor.finish(part);

        let stats = self.monitor.stats(part);
        self.writer.write_solution(part, &stats, &solution)?;

        self.monitor.reset();
        Ok(())
    }

    pub fn set_part1(&mut self, solution: impl Display) {
        self.try_set(Part::One, solution)
            .expect("Unexpected error setting the output for part 1");
    }

    pub fn set_part2(&mut self, solution: impl Display) {
        self.try_set(Part::Two, solution)
            .expect("Unexpected error setting the output for part 2");
    }
}

pub trait SolutionWriter {
    fn write_heading(&mut self, spec: &Spec) -> Result<()>;
    fn write_solution(&mut self, part: Part, stats: &Stats, solution: &dyn Display) -> Result<()>;
}

#[derive(Debug, Clone, Default)]
pub struct NullWriter {
    pub heading: Option<Spec>,
    pub solutions: [Option<(Stats, String)>; 2],
}

impl NullWriter {
    pub fn pipe_to(&self, other: &mut impl SolutionWriter) -> Result<()> {
        if let Some(heading) = &self.heading {
            other.write_heading(heading)?;
        }
        for part in [Part::One, Part::Two] {
            if let Some((stats, solution)) = &self.solutions[part.to_index()] {
                other.write_solution(Part::One, stats, solution)?;
            }
        }
        Ok(())
    }
}

impl SolutionWriter for NullWriter {
    fn write_heading(&mut self, spec: &Spec) -> Result<()> {
        self.heading = Some(spec.clone());
        Ok(())
    }

    fn write_solution(&mut self, part: Part, stats: &Stats, solution: &dyn Display) -> Result<()> {
        self.solutions[part.to_index()] = Some((*stats, solution.to_string()));
        Ok(())
    }
}

pub struct Solver {
    pub problem_id: ProblemId,
    pub raw_solve: fn(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()>,
}

// TODO: derive(Debug) causes an error. Why?
impl std::fmt::Debug for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("Solver");
        builder.field("problem_id", &self.problem_id);
        builder.field("raw_solve", &(self.raw_solve as *const ()));
        builder.finish()
    }
}

impl Solver {
    pub fn solve(&self, input: &str, out: &mut ProblemOutput<'_>) -> Result<()> {
        (self.raw_solve)(input, out).map_err(|e| match e.downcast() {
            Ok(e) => e,
            Err(e) => SolverError::SolverError(e),
        })
    }

    #[must_use]
    pub fn get_map() -> FxHashMap<ProblemId, &'static Solver> {
        let mut m: FxHashMap<ProblemId, &'static Solver> = FxHashMap::default();
        for s in SOLVERS {
            assert!(
                m.insert(s.problem_id, s).is_none(),
                "Multiple solver implementations for {}",
                s.problem_id
            );
        }
        m
    }
}

#[distributed_slice]
pub static SOLVERS: [Solver] = [..];

#[macro_export]
macro_rules! register {
    ($solve_fn:path, $year:expr, $day:expr) => {
        ::paste::paste! {
            #[::linkme::distributed_slice(::aoc::SOLVERS)]
            static [<SOLVER_ $solve_fn _ $year _ $day>]: ::aoc::Solver = ::aoc::Solver {
                problem_id: ::aoc::ProblemId {
                    year: $year,
                    day: $day,
                },
                raw_solve: $solve_fn,
            };
        }
    };
}
