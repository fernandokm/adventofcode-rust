use anyhow::Context;
use aoc::ProblemOutput;

aoc::register!(solve, 2021, 2);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut x = 0;
    let mut depth1 = 0;
    let mut depth2 = 0;
    let mut aim = 0;

    for line in input.trim().lines() {
        let (direction, amount) = line
            .split_once(' ')
            .context(format!("Invalid input line: {}", line))?;
        let amount: i32 = amount.parse()?;
        match direction {
            "forward" => {
                x += amount;
                depth2 += aim * amount;
            }
            "up" => {
                depth1 -= amount;
                aim -= amount;
            }
            "down" => {
                depth1 += amount;
                aim += amount;
            }
            _ => (),
        }
    }
    out.set_part1(x * depth1);
    out.set_part2(x * depth2);
    Ok(())
}
