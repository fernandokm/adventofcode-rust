use aoc::ProblemOutput;

aoc::register!(solve, 2020, 6);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let (count_part1, count_part2) = input
        .trim()
        .split("\n\n")
        .map(count)
        .reduce(|(x1, x2), (y1, y2)| (x1 + y1, x2 + y2))
        .unwrap_or_default();
    out.set_part1(count_part1);
    out.set_part2(count_part2);

    Ok(())
}

fn count(group: &str) -> (usize, usize) {
    let mut questions = [0; 26];
    for c in group.bytes() {
        if (b'a'..=b'z').contains(&c) {
            questions[usize::from(c - b'a')] += 1;
        }
    }

    let total = group.bytes().filter(|&c| c == b'\n').count() + 1;
    let part1 = questions.iter().filter(|&&c| c > 0).count();
    let part2 = questions.iter().filter(|&&c| c == total).count();
    (part1, part2)
}
