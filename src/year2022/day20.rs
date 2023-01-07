use std::cmp::Ordering;

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 20);

const DECRYPTION_KEY: i64 = 811_589_153;

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut nums: Vec<i64> = input
        .split_ascii_whitespace()
        .map(str::parse)
        .try_collect()?;

    out.set_part1(get_key(&nums, 1));

    for n in &mut nums {
        *n *= DECRYPTION_KEY;
    }
    out.set_part2(get_key(&nums, 10));

    Ok(())
}

fn get_key(nums: &[i64], rounds: usize) -> i64 {
    let mixed_nums = mix(nums, rounds);
    let izero = mixed_nums.iter().position(|&n| n == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| mixed_nums[(izero + offset) % mixed_nums.len()])
        .sum::<i64>()
}

fn mix(nums: &[i64], rounds: usize) -> Vec<i64> {
    let mut entries = nums
        .iter()
        .enumerate()
        .map(|(i, &n)| Entry::new(i, n))
        .collect_vec();
    for _ in 0..rounds {
        for id in 0..entries.len() {
            let (i, entry) = entries.iter().find_position(|&e| e.id == id).unwrap();
            let inew = (i as i64 + entry.val).rem_euclid(entries.len() as i64 - 1) as usize;
            match inew.cmp(&i) {
                Ordering::Less => entries[inew..=i].rotate_right(1),
                Ordering::Greater => entries[i..=inew].rotate_left(1),
                Ordering::Equal => (),
            }
        }
    }
    entries.into_iter().map(|e| e.val).collect_vec()
}

struct Entry {
    id: usize,
    val: i64,
}

impl Entry {
    fn new(id: usize, val: i64) -> Self {
        Self { id, val }
    }
}
