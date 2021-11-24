use input::InputSpec;
use linkme::distributed_slice;
use rustc_hash::FxHashMap;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub mod input;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Not implemented")]
    NotImplemented,

    #[error("Solver error: {0}")]
    SolverError(#[source] anyhow::Error),
}

#[macro_export]
macro_rules! not_implemented {
    () => {
        return Err(::aoc::Error::NotImplemented.into())
    };
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProblemId {
    pub year: u32,
    pub day: u32,
}

impl Display for ProblemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}.{}", self.year, self.day)
    }
}

pub struct ProblemOutput<'a> {
    inner: Box<dyn ProblemOutputBackend + 'a>,
    last_instant: Instant,
}

impl<'a> ProblemOutput<'a> {
    pub fn start<B>(spec: &InputSpec, mut inner: B) -> Result<ProblemOutput<'a>>
    where
        B: ProblemOutputBackend + 'a,
    {
        inner.start(spec)?;
        Ok(ProblemOutput {
            inner: Box::new(inner),
            last_instant: Instant::now(),
        })
    }

    pub fn set_part1(&mut self, solution: impl Display) {
        self.try_set_part1(solution)
            .expect("Unexpected error setting the output for part 1")
    }

    pub fn try_set_part1(&mut self, solution: impl Display) -> Result<()> {
        self.inner
            .set_solution(1, self.last_instant.elapsed(), &solution)?;
        self.reset_elapsed_time();
        Ok(())
    }

    pub fn set_part2(&mut self, solution: impl Display) {
        self.try_set_part2(solution)
            .expect("Unexpected error setting the output for part 2")
    }

    pub fn try_set_part2(&mut self, solution: impl Display) -> Result<()> {
        self.inner
            .set_solution(2, self.last_instant.elapsed(), &solution)?;
        self.reset_elapsed_time();
        Ok(())
    }

    pub fn reset_elapsed_time(&mut self) {
        self.last_instant = Instant::now()
    }
}

pub trait ProblemOutputBackend {
    fn start(&mut self, spec: &InputSpec) -> Result<()>;
    fn set_solution(
        &mut self,
        part: u32,
        exec_time: Duration,
        solution: &dyn Display,
    ) -> Result<()>;
}

pub struct Solver {
    pub problem_id: ProblemId,
    pub raw_solve: fn(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()>,
}

// TODO: derive(Debug) causes an error. Why?
impl std::fmt::Debug for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut builder = f.debug_struct("Solver");
        builder.field("problem_id", &self.problem_id);
        builder.field("raw_solve", &(self.raw_solve as *const ()));
        builder.finish()
    }
}

impl Solver {
    pub fn solve(&self, input: &str, out: &mut ProblemOutput) -> Result<()> {
        (self.raw_solve)(input, out).map_err(|e| match e.downcast() {
            Ok(e) => e,
            Err(e) => Error::SolverError(e),
        })
    }

    pub fn get_map() -> FxHashMap<ProblemId, &'static Solver> {
        let mut m: FxHashMap<ProblemId, &'static Solver> = FxHashMap::default();
        for s in SOLVERS {
            if m.insert(s.problem_id, s).is_some() {
                panic!("Multiple solver implementations for {}", s.problem_id);
            }
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
