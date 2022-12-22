use std::{cmp::Reverse, collections::BinaryHeap};

use aoc::ProblemOutput;
use itertools::Itertools;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};

use crate::util::grid::GridSpec;

aoc::register!(solve, 2021, 15);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let input_elems: Vec<Node> = input
        .chars()
        .filter_map(|c| Some(Node::new(c.to_digit(10)?)))
        .collect();
    let width = input.trim().lines().next().unwrap().len();
    let height = input_elems.len() / width;
    let mut map1 = Array2::from_shape_vec((width, height), input_elems)?;
    let mut map2 = repeat_map_incresing_risk(
        Axis(1),
        5,
        repeat_map_incresing_risk(Axis(0), 5, map1.view())?.view(),
    )?;

    djikstra(map1.view_mut(), (0, 0), (width - 1, height - 1));
    out.set_part1(map1.last().unwrap().total_risk);

    djikstra(map2.view_mut(), (0, 0), (5 * width - 1, 5 * height - 1));
    out.set_part2(map2.last().unwrap().total_risk);

    Ok(())
}

fn repeat_map_incresing_risk(
    axis: Axis,
    len: usize,
    map: ArrayView2<'_, Node>,
) -> anyhow::Result<Array2<Node>> {
    let maps = &(0..len)
        .map(|i| {
            map.mapv(|mut v| {
                v.risk = (v.risk - 1 + i as u32) % 9 + 1;
                v
            })
        })
        .collect_vec();
    Ok(ndarray::concatenate(
        axis,
        &maps.iter().map(Array2::view).collect_vec(),
    )?)
}

fn djikstra(mut map: ArrayViewMut2<'_, Node>, start: (usize, usize), end: (usize, usize)) {
    let grid_spec = GridSpec::new_indexed(map.dim().0, map.dim().1);
    let mut tentative_total_risks: BinaryHeap<_> = map
        .indexed_iter()
        .map(|(pos, _)| (Reverse(u32::MAX), pos))
        .collect();
    tentative_total_risks.push((Reverse(0), start));

    while let Some((Reverse(total_risk), pos)) = tentative_total_risks.pop() {
        let node = &mut map[pos];
        if !node.is_tentative {
            continue;
        }
        node.total_risk = total_risk;
        node.is_tentative = false;
        if pos == end {
            return;
        }
        let node_risk = node.total_risk;

        for pos2 in grid_spec.neighbors(&pos.into()) {
            let node2 = &mut map[pos2.into_tuple()];
            let total_risk2 = node_risk + node2.risk;
            if node2.is_tentative && total_risk2 < node2.total_risk {
                node2.total_risk = total_risk2;
                tentative_total_risks.push((Reverse(total_risk2), pos2.into_tuple()));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    risk: u32,
    total_risk: u32,
    is_tentative: bool,
}

impl Node {
    fn new(risk: u32) -> Node {
        Self {
            risk,
            total_risk: u32::MAX,
            is_tentative: true,
        }
    }
}
