use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 6);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let input = input.trim().bytes().map(|b| (b - b'a') as usize).collect_vec();
    out.set_part1(find_marker(&input, 4));
    out.set_part2(find_marker(&input, 14));

    Ok(())
}

fn find_marker(input: &[usize], size: usize) -> usize {
    (size..=input.len())
        .find(|&i| is_unique(&input[i - size..i]))
        .unwrap()
}

fn is_unique(arr: &[usize]) -> bool {
    let mut seen = [false; 26];
    for &v in arr.iter() {
        if seen[v] {
            return false;
        }
        seen[v] = true;
    }
    true
}
