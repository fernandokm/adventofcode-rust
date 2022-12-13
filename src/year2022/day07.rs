use std::iter::Peekable;

use aoc::ProblemOutput;
use rustc_hash::FxHashMap;

aoc::register!(solve, 2022, 7);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut root = Directory::default();
    root.parse_commands(&mut input.lines().peekable());
    root.compute_sizes();
    out.set_part1(
        root.iter_dir_sizes()
            .filter(|&size| size <= 100_000)
            .sum::<u64>(),
    );

    let unused_space = 70_000_000 - root.size;
    let required_space = 30_000_000_u64.saturating_sub(unused_space);
    out.set_part2(
        root.iter_dir_sizes()
            .filter(|&size| size >= required_space)
            .min()
            .unwrap(),
    );
    Ok(())
}

#[derive(Default)]
struct Directory {
    dirs: FxHashMap<String, Directory>,
    files_size: u64,
    size: u64,
}

impl Directory {
    fn iter_dir_sizes(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(
            std::iter::once(self.size)
                .chain(self.dirs.values().flat_map(Directory::iter_dir_sizes)),
        )
    }

    fn compute_sizes(&mut self) -> u64 {
        if self.size == 0 {
            let dirs_size: u64 = self.dirs.values_mut().map(Directory::compute_sizes).sum();
            self.size = self.files_size + dirs_size;
        }
        self.size
    }

    fn parse_commands<'a>(&mut self, lines: &mut Peekable<impl Iterator<Item = &'a str>>) {
        while let Some(line) = lines.next() {
            let line = line.trim();
            if let Some(target) = line.strip_prefix("$ cd") {
                match target.trim() {
                    // Ignore this case
                    // (it only happens as the first line of input)
                    "/" => (),
                    ".." => return,
                    target => self
                        .dirs
                        .entry(target.to_string())
                        .or_default()
                        .parse_commands(lines),
                }
            } else if line == "$ ls" {
                while let Some(entry) = lines.next_if(|entry| !entry.starts_with('$')) {
                    let (size, _name) = entry.split_once(' ').unwrap();
                    if size != "dir" {
                        self.files_size += size.parse::<u64>().unwrap();
                    }
                }
            }
        }
    }
}
