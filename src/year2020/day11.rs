use aoc::ProblemOutput;

use crate::util::{coords::P2, grid::GridSpec};

aoc::register!(solve, 2020, 11);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut seats1: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut aux = seats1.clone();
    let mut seats2 = seats1.clone();

    while update(&mut seats1, &mut aux, true) {}
    out.set_part1(count_occupied(&seats1));

    while update(&mut seats2, &mut aux, false) {}
    out.set_part2(count_occupied(&seats2));

    Ok(())
}

#[must_use]
pub fn count_occupied(seats: &[Vec<char>]) -> usize {
    seats
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

pub fn update(seats: &mut Vec<Vec<char>>, aux: &mut [Vec<char>], part1: bool) -> bool {
    let mut updated = false;
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            let min_count = if part1 { 4 } else { 5 };
            match seats[i][j] {
                'L' if count_occupied_neighbors(i, j, part1, seats) == 0 => {
                    updated = true;
                    aux[i][j] = '#';
                }
                '#' if count_occupied_neighbors(i, j, part1, seats) >= min_count => {
                    updated = true;
                    aux[i][j] = 'L';
                }
                c => aux[i][j] = c,
            }
        }
    }
    seats.swap_with_slice(aux);
    updated
}

fn count_occupied_neighbors(i: usize, j: usize, part1: bool, seats: &[Vec<char>]) -> usize {
    let spec = GridSpec::new_indexed(seats.len(), seats[0].len());
    GridSpec::directions_with_diag()
        .filter(|dir| {
            spec.iter_direction(P2(i, j), dir)
                .skip(1) // skip the node at (i, j)
                .map(|P2(ii, jj)| seats[ii][jj])
                .find(|&c| {
                    // in part 2, consider the first seat (not neighbor) in each direction
                    part1 || (c != '.')
                })
                .map_or(false, |c: char| c == '#')
        })
        .count()
}
