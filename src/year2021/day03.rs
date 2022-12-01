use anyhow::Context;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 3);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let n_bits = input.trim().lines().next().context("empty input")?.len();
    let nums: Vec<u32> = input
        .trim()
        .lines()
        .map(|line| u32::from_str_radix(line, 2))
        .try_collect()?;

    let gamma = (0..n_bits)
        .map(|i| most_common_bit(i, &nums) << i)
        .sum::<u32>();
    let epsilon = (!gamma) & ((1 << n_bits) - 1);

    out.set_part1(gamma * epsilon);

    let o2_rating = filter_part_2(true, n_bits, &nums).context("no O2 rating found")?;
    let co2_rating = filter_part_2(false, n_bits, &nums).context("no CO2 rating found")?;

    out.set_part2(o2_rating * co2_rating);

    Ok(())
}

fn most_common_bit(pos: usize, nums: &[u32]) -> u32 {
    let bit_at_pos = 1 << pos;
    let num_bits_1 = nums.iter().filter(|&&n| n & bit_at_pos != 0).count();
    u32::from(num_bits_1 * 2 >= nums.len())
}

fn filter_part_2(keep_most_common: bool, n_bits: usize, nums: &[u32]) -> Option<u32> {
    let mut nums = nums.iter().copied().collect_vec();
    for pos in (0..n_bits).rev() {
        let desired_value = if keep_most_common {
            most_common_bit(pos, &nums) << pos
        } else {
            (most_common_bit(pos, &nums) ^ 1) << pos
        };
        nums = nums
            .iter()
            .copied()
            .filter(|&a| a & (1 << pos) == desired_value)
            .collect();
        if nums.len() == 1 {
            return Some(nums[0]);
        }
    }
    None
}
