use aoc::{input, ProblemId, Solver};
use clap::Args;
use itertools::Itertools;

#[derive(Debug, Args)]
pub struct Cmd {
    #[clap(
        short,
        long,
        help = "Show detailed information for each problem (available input files and solvers)"
    )]
    verbose: bool,

    #[clap(short, long, help = "List only problems from the specified years")]
    year: Vec<u32>,
}

impl Cmd {
    pub fn exec(&self, default_inputs: &impl input::Source) -> anyhow::Result<()> {
        if self.verbose {
            for (year, group) in &Solver::get_map().into_iter().group_by(|(id, _)| id.year) {
                if !self.year.is_empty() && !self.year.contains(&year) {
                    continue;
                }

                println!("[{}]", year);
                for (id, _) in group {
                    print!("  {:2}", id.day);

                    Self::print_section(
                        "inputs",
                        32,
                        default_inputs
                            .keys()
                            .into_iter()
                            .filter(|spec| spec.id == id)
                            .map(|spec| spec.variant.as_str()),
                    );

                    println!();
                }
            }
        } else {
            let intervals = get_intervals(Solver::get_map().keys().copied().sorted());
            for (year, mut group) in &intervals.into_iter().group_by(|(id, _)| id.year) {
                if !self.year.is_empty() && !self.year.contains(&year) {
                    continue;
                }

                print!("[{}]  ", year);
                print_interval(group.next().unwrap());
                for interval in group {
                    print!(",  ");
                    print_interval(interval);
                }
                println!();
            }
        }

        Ok(())
    }

    fn print_section<'a>(heading: &str, width: usize, values: impl Iterator<Item = &'a str> + 'a) {
        let value_list = values
            .into_iter()
            .map(|s| if s.is_empty() { "<empty>" } else { s })
            .join(", ");
        print!(
            "  [{}: {}]{}",
            heading,
            value_list,
            " ".repeat(width.saturating_sub(6 + heading.len() + value_list.len())),
        );
    }
}

fn print_interval((start, end): (ProblemId, ProblemId)) {
    if start == end {
        print!("{}", start.day);
    } else {
        print!("{}..={}", start.day, end.day);
    }
}

fn get_intervals(mut ids: impl Iterator<Item = ProblemId>) -> Vec<(ProblemId, ProblemId)> {
    let current_start = ids.next();
    if current_start.is_none() {
        return Vec::new();
    }

    let mut intervals = Vec::new();
    let mut current_start = current_start.unwrap();
    let mut current_end = current_start;

    for id in ids {
        if id.year == current_end.year && id.day == current_end.day + 1 {
            current_end = id;
        } else {
            intervals.push((current_start, current_end));
            current_start = id;
            current_end = current_start;
        }
    }

    let last_interval = (current_start, current_end);
    if intervals.last() != Some(&last_interval) {
        intervals.push(last_interval);
    }
    intervals
}
