use aoc::ProblemOutput;

use crate::util::{coords::P2, grid::GridSpec, signed::Signed};

aoc::register!(solve, 2022, 8);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let grid_spec = GridSpec::new_indexed(grid.len(), grid[0].len());

    out.set_part1(
        grid_spec
            .iter()
            .filter(|&pos| {
                GridSpec::directions().any(|d| viewing_distance(grid_spec, &grid, pos, d).1)
            })
            .count(),
    );

    out.set_part2(
        grid_spec
            .iter()
            .map(|pos| {
                GridSpec::directions()
                    .map(|d| viewing_distance(grid_spec, &grid, pos, d).0)
                    .product::<u64>()
            })
            .max()
            .unwrap(),
    );

    Ok(())
}

fn viewing_distance(
    grid_spec: GridSpec<usize>,
    grid: &[&[u8]],
    pos: P2<usize>,
    d: P2<Signed<usize>>,
) -> (u64, bool) {
    let mut count = 0;
    let P2(i0, j0) = pos;
    for P2(i, j) in grid_spec.iter_direction(pos, &d).skip(1) {
        count += 1;
        if grid[i][j] >= grid[i0][j0] {
            return (count, false);
        }
    }
    (count, true)
}
