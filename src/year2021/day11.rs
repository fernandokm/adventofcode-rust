use aoc::ProblemOutput;
use ndarray::Array2;
use rustc_hash::FxHashSet;

aoc::register!(solve, 2021, 11);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let input = input.trim();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut energy = Array2::from_shape_vec(
        (width, height),
        input.chars().filter_map(|c| c.to_digit(10)).collect(),
    )?;

    let mut total_flashes = 0usize;
    let mut part1 = false;
    let mut part2 = false;
    for i in 1.. {
        let flashes = step(&mut energy);
        total_flashes = total_flashes.saturating_add(flashes);
        if i == 100 {
            out.set_part1(total_flashes);
            part1 = true;
        }
        if flashes == energy.len() {
            out.set_part2(i);
            part2 = true;
        }
        if part1 && part2 {
            break;
        }
    }

    Ok(())
}

fn flash(i: usize, j: usize, energy: &mut Array2<u32>, flashes: &mut FxHashSet<(usize, usize)>) {
    if !flashes.insert((i, j)) {
        return;
    }
    for ii in [i.wrapping_sub(1), i, i + 1] {
        for jj in [j.wrapping_sub(1), j, j + 1] {
            if ii == i && jj == j {
                continue;
            }
            if let Some(e) = energy.get_mut((ii, jj)) {
                *e += 1;
                if *e >= 10 {
                    flash(ii, jj, energy, flashes);
                }
            }
        }
    }
}

fn step(energy: &mut Array2<u32>) -> usize {
    energy.mapv_inplace(|v| v + 1);

    let mut flashes = FxHashSet::default();
    for i in 0..energy.shape()[0] {
        for j in 0..energy.shape()[1] {
            if energy[(i, j)] >= 10 {
                flash(i, j, energy, &mut flashes);
            }
        }
    }

    for &pos in flashes.iter() {
        energy[pos] = 0;
    }
    flashes.len()
}
