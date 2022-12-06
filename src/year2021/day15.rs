use std::{cmp::Reverse, collections::BinaryHeap};

use aoc::ProblemOutput;
use itertools::Itertools;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};

aoc::register!(solve, 2021, 15);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
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
    map: ArrayView2<Node>,
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

fn djikstra(mut map: ArrayViewMut2<Node>, start: (usize, usize), end: (usize, usize)) {
    let mut tentative_total_risks =
        BinaryHeap::from_iter(map.indexed_iter().map(|(pos, _)| (Reverse(u32::MAX), pos)));
    tentative_total_risks.push((Reverse(0), start));

    while let Some((Reverse(total_risk), pos)) = tentative_total_risks.pop() {
        if !map[pos].is_tentative {
            continue;
        }
        map[pos].total_risk = total_risk;
        map[pos].is_tentative = false;
        if pos == end {
            return;
        }
        let neighbors_pos = [
            (pos.0.wrapping_sub(1), pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0, pos.1 + 1),
        ];
        for pos2 in neighbors_pos {
            if map.get(pos2).is_none() {
                continue;
            }
            let total_risk2 = map[pos].total_risk + map[pos2].risk;
            if map[pos2].is_tentative && total_risk2 < map[pos2].total_risk {
                map[pos2].total_risk = total_risk2;
                tentative_total_risks.push((Reverse(total_risk2), pos2));
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
