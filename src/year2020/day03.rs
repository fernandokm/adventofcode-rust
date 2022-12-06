use aoc::ProblemOutput;

aoc::register!(solve, 2020, 3);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let is_tree: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    out.set_part1(count_with_slope(3, 1, &is_tree));

    out.set_part2(
        count_with_slope(1, 1, &is_tree)
            * count_with_slope(3, 1, &is_tree)
            * count_with_slope(5, 1, &is_tree)
            * count_with_slope(7, 1, &is_tree)
            * count_with_slope(1, 2, &is_tree),
    );

    Ok(())
}

fn count_with_slope(dj: usize, di: usize, is_tree: &[Vec<bool>]) -> usize {
    is_tree
        .iter()
        .step_by(di)
        .enumerate()
        .filter(|(k, row)| row[dj * k % row.len()])
        .count()
}
