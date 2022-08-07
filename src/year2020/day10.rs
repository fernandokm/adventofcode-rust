use std::str::FromStr;

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2020, 10);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let mut adapters: Vec<u32> = input
        .split_whitespace()
        .map(FromStr::from_str)
        .try_collect()?;
    adapters.push(0); // treat the outlet as an adapter
    adapters.push(adapters.iter().max().unwrap() + 3); // the built-in adapter
    adapters.sort_unstable();

    let mut diffs = [0; 4];

    for (x, y) in adapters.iter().zip(adapters.iter().skip(1)) {
        diffs[(y - x) as usize] += 1;
    }
    out.set_part1(diffs[1] * diffs[3]);

    out.set_part2(count_arrangements(&adapters));

    Ok(())
}

fn count_arrangements(adapters: &[u32]) -> u64 {
    debug_assert!(
        !adapters.is_empty()
            && adapters
                .iter()
                .zip(adapters.iter().skip(1))
                .all(|(x, y)| x <= y),
        "adapters must be sorted and non-empty"
    );

    // count[i] is the number of ways to go from adapters[0] to adapters[i]
    let mut count = vec![0_u64; adapters.len()];
    count[0] = 1;

    for (i, &a) in adapters.iter().enumerate() {
        adapters
            .iter()
            .enumerate()
            .skip(i + 1)
            .take_while(|&(_, b)| b - a <= 3)
            .for_each(|(j, _)| count[j] += count[i]);
    }
    *count.last().unwrap()
}
