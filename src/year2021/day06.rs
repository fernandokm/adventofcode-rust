use aoc::ProblemOutput;

aoc::register!(solve, 2021, 6);

static INITIAL_TIMER: usize = 6;
static FIRST_INITIAL_TIMER: usize = INITIAL_TIMER + 2;

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut counts = vec![0; FIRST_INITIAL_TIMER + 1];
    for n in input.trim().split(',') {
        counts[n.parse::<usize>()?] += 1;
    }

    for _ in 0..80 {
        update(&mut counts);
    }
    out.set_part1(counts.iter().sum::<u64>());

    for _ in 80..256 {
        update(&mut counts);
    }
    out.set_part2(counts.iter().sum::<u64>());

    Ok(())
}

fn update(counts: &mut [u64]) {
    let counts0 = counts[0];
    for i in 1..counts.len() {
        counts[i - 1] = counts[i];
    }
    *counts.last_mut().unwrap() = 0;

    counts[INITIAL_TIMER] += counts0;
    counts[FIRST_INITIAL_TIMER] += counts0;
}
