use std::collections::VecDeque;

use anyhow::bail;
use aoc::ProblemOutput;
use itertools::Itertools;
use ndarray::{Array1, Array2, ArrayView2, Axis};

use crate::util::{coords::P2, grid::GridSpec};

aoc::register!(solve, 2022, 12);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let (map, start, end) = parse_input(input)?;

    out.set_part1(bfs_min_dist(map.view(), [start], end)?);

    out.set_part2(bfs_min_dist(
        map.view(),
        map.indexed_iter()
            .filter(|&(_, &val)| val == b'a')
            .map(|((i, j), _)| P2(i, j)),
        end,
    )?);

    Ok(())
}

fn bfs_min_dist(
    map: ArrayView2<'_, u8>,
    start: impl IntoIterator<Item = P2<usize>>,
    end: P2<usize>,
) -> anyhow::Result<u32> {
    let spec = GridSpec::new_indexed(map.dim().0, map.dim().1);
    let mut queue = VecDeque::new();
    for pos in start {
        queue.push_back((0, pos));
    }

    let mut dist = Array2::from_elem(map.dim(), None);
    while let Some((node_dist, node_pos)) = queue.pop_front() {
        let neighbor_dist = node_dist + 1;
        for neighbor_pos in spec.neighbors(&node_pos) {
            if map[neighbor_pos] > map[node_pos] + 1 {
                continue;
            }
            if neighbor_pos == end {
                return Ok(neighbor_dist);
            } else if dist[neighbor_pos].is_none() {
                dist[neighbor_pos] = Some(neighbor_dist);
                queue.push_back((neighbor_dist, neighbor_pos));
            }
        }
    }
    bail!("No path found");
}

fn parse_input(input: &str) -> anyhow::Result<(Array2<u8>, P2<usize>, P2<usize>)> {
    let rows = input
        .lines()
        .map(|line| Array1::from_iter(line.bytes()))
        .collect_vec();
    let row_views = rows.iter().map(Array1::view).collect_vec();
    let mut map = ndarray::stack(Axis(0), &row_views)?;

    let mut start = P2(0, 0);
    let mut end = P2(0, 0);
    for ((i, j), val) in map.indexed_iter_mut() {
        if *val == b'S' {
            *val = b'a';
            start = P2(i, j);
        } else if *val == b'E' {
            *val = b'z';
            end = P2(i, j);
        }
    }

    Ok((map, start, end))
}
