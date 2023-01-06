use anyhow::bail;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 25);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let nums: Vec<_> = input.lines().map(from_snafu).try_collect()?;
    let sum = nums.into_iter().sum();
    out.set_part1(to_snafu(sum));

    Ok(())
}

fn to_snafu(mut sum: i64) -> String {
    let mut digits = Vec::new();
    while sum > 0 {
        let d = (sum + 2) % 5 - 2;
        sum = (sum - d) / 5;
        digits.push(to_snafu_digit(d).unwrap());
    }
    digits.reverse();
    if digits.is_empty() {
        digits.push(b'0');
    }
    String::from_utf8(digits).unwrap()
}

fn from_snafu(s: &str) -> anyhow::Result<i64> {
    s.bytes()
        .map(from_snafu_digit)
        .try_fold(0, |acc, x| Ok(acc * 5 + x?))
}

fn to_snafu_digit(digit: i64) -> anyhow::Result<u8> {
    Ok(match digit {
        0..=2 => digit as u8 + b'0',
        -1 => b'-',
        -2 => b'=',
        _ => bail!("Invalid snafu digit value: {digit}"),
    })
}

fn from_snafu_digit(digit: u8) -> anyhow::Result<i64> {
    Ok(match digit {
        b'0'..=b'2' => (digit - b'0').into(),
        b'-' => -1,
        b'=' => -2,
        _ => bail!("Invalid char: {digit}"),
    })
}
