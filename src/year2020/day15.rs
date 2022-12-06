use std::{mem::size_of, str::FromStr};

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2020, 15);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let numbers: Vec<usize> = input
        .trim()
        .split(',')
        .map(FromStr::from_str)
        .try_collect()?;

    out.set_part1(get(&numbers, 2020));
    out.set_part2(get(&numbers, 30000000));

    Ok(())
}

fn get(numbers: &[usize], idx: usize) -> usize {
    assert!(idx >= numbers.len() && size_of::<usize>() >= size_of::<u32>());

    let max_num = numbers.iter().copied().max().unwrap();
    let mut last_indices = vec![0; idx.max(max_num + 1)];
    for (i, &n) in numbers[..numbers.len() - 1].iter().enumerate() {
        last_indices[n] = i + 1;
    }

    let mut current_number = *numbers.last().unwrap();
    for i in numbers.len()..idx {
        let next_number = match last_indices[current_number] {
            0 => 0,
            last_index => i - last_index,
        };
        last_indices[current_number] = i;
        current_number = next_number;
    }
    current_number
}
