use aoc::ProblemOutput;
use itertools::Itertools;

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
    let minus1 = 0usize.overflowing_sub(1).0;

    [minus1, 0, 1]
        .into_iter()
        .cartesian_product([minus1, 0, 1].into_iter())
        .filter(|&(di, dj)| {
            if di == 0 && dj == 0 {
                return false;
            }
            let mut i = i.overflowing_add(di).0;
            let mut j = j.overflowing_add(dj).0;
            if !part1 {
                // in part 2, consider the first seat (not neighbor) in each direction
                while get(seats, i, j).map_or(false, |c| c == '.') {
                    i = i.overflowing_add(di).0;
                    j = j.overflowing_add(dj).0;
                }
            }
            get(seats, i, j).map_or(false, |c| c == '#')
        })
        .count()
}

fn get(seats: &[Vec<char>], i: usize, j: usize) -> Option<char> {
    seats.get(i).and_then(|row| row.get(j).copied())
}
