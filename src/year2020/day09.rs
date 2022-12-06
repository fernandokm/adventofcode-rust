use std::{cmp::Ordering, str::FromStr};

use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2020, 9);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let (preamble_len, nums) = input.split_once("---").context("invalid input")?;
    let preamble_len: usize = preamble_len.trim().parse()?;
    let nums: Vec<u64> = nums
        .split_whitespace()
        .map(FromStr::from_str)
        .try_collect()?;

    let invalid = nums
        .iter()
        .copied()
        .enumerate()
        .skip(preamble_len)
        .find(|&(i, n)| !is_valid(n, &nums[i - preamble_len..i]))
        .context("no invalid number found")?
        .1;

    out.set_part1(invalid);

    let slice = find_contiguous_slice(invalid, &nums)
        .context("no contiguous slice adding up to the desired value was found")?;
    out.set_part2(match slice.iter().minmax() {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(x) => x + x,
        itertools::MinMaxResult::MinMax(x, y) => x + y,
    });

    Ok(())
}

fn is_valid(n: u64, previous_nums: &[u64]) -> bool {
    assert!(previous_nums.len() > 1);

    let mut sorted = previous_nums.to_vec();
    sorted.sort_unstable();

    let mut lo = 0;
    let mut hi = sorted.len() - 1;
    while hi > lo {
        let sum = sorted[lo] + sorted[hi];
        match sum.cmp(&n) {
            Ordering::Greater => hi -= 1,
            Ordering::Less => lo += 1,
            Ordering::Equal => return true,
        }
    }
    false
}

fn find_contiguous_slice(target_sum: u64, nums: &[u64]) -> Option<&[u64]> {
    for i in 0..nums.len() {
        let mut sum = 0;
        for (j, nj) in nums.iter().enumerate().skip(i) {
            sum += nj;
            match sum.cmp(&target_sum) {
                Ordering::Less => (),
                Ordering::Equal => return Some(&nums[i..=j]),
                Ordering::Greater => break,
            }
        }
    }
    None
}
