use std::collections::hash_map::Entry;

use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::util::{
    coords::{xy, P2},
    signed::Signed,
};

aoc::register!(solve, 2022, 17);

const WIDTH: usize = 7;
const ROCK_SHAPES: [&[P2<usize>]; 5] = [
    &[P2(0, 0), P2(1, 0), P2(2, 0), P2(3, 0)],
    &[P2(1, 0), P2(0, 1), P2(1, 1), P2(2, 1), P2(1, 2)],
    &[P2(0, 0), P2(1, 0), P2(2, 0), P2(2, 1), P2(2, 2)],
    &[P2(0, 0), P2(0, 1), P2(0, 2), P2(0, 3)],
    &[P2(0, 0), P2(1, 0), P2(0, 1), P2(1, 1)],
];

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut jets = input
        .trim()
        .bytes()
        .map(|b| {
            if b == b'<' {
                xy::signed_left()
            } else {
                xy::signed_right()
            }
        })
        .enumerate()
        .cycle();
    let mut rock_shapes = ROCK_SHAPES.iter().copied().enumerate().cycle();

    let mut map = Map::default();
    for (i_shape, shape) in rock_shapes.by_ref().take(2022) {
        map.drop_shape(i_shape, shape, &mut jets);
    }
    out.set_part1(map.rows.len());

    let mut rocks_remaining = 1_000_000_000_000 - 2022;
    let mut extra_elevation = 0;
    for (i_shape, shape) in rock_shapes {
        rocks_remaining -= 1;
        if let Some(period) = map.drop_shape(i_shape, shape, &mut jets) {
            extra_elevation += rocks_remaining / period.num_rocks * period.elevation_change;
            rocks_remaining %= period.num_rocks;
        }
        if rocks_remaining == 0 {
            break;
        }
    }
    out.set_part2(map.rows.len() + extra_elevation);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StateKey {
    shape_index: usize,
    jet_index: usize,
    pattern: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
struct StateData {
    elevation: usize,
    rock_index: usize,
}

struct Period {
    num_rocks: usize,
    elevation_change: usize,
}

#[derive(Default)]
struct Map {
    rows: Vec<u8>,
    states: FxHashMap<StateKey, StateData>,
    num_rocks: usize,
}

impl Map {
    fn insert(&mut self, points: impl IntoIterator<Item = P2<usize>>) {
        for P2(x, y) in points {
            if y >= self.rows.len() {
                self.rows.resize(y + 1, 0);
            }
            self.rows[y] |= 1 << x;
        }
        self.num_rocks += 1;
    }

    fn contains(&self, P2(x, y): P2<usize>) -> bool {
        self.rows
            .get(y)
            .map_or(false, |row| row & (1 << x) == (1 << x))
    }

    fn drop_shape(
        &mut self,
        shape_index: usize,
        shape: &[P2<usize>],
        jets: &mut impl Iterator<Item = (usize, P2<Signed<usize>>)>,
    ) -> Option<Period> {
        let mut points = shape.to_vec();
        self.try_move_points(&mut points, P2(2, self.rows.len() + 3));

        let mut last_jet_index;

        loop {
            let (i_jet, jet) = jets.next().unwrap();
            last_jet_index = i_jet;
            self.try_move_points(&mut points, jet);
            if !self.try_move_points(&mut points, xy::signed_down()) {
                break;
            }
        }

        let elevation = points.iter().map(|&P2(_, y)| y).min().unwrap();
        self.insert(points);

        if self.rows[elevation..]
            .iter()
            .tuple_windows()
            .all(|(r1, r2)| r1 | r2 != 0x7F)
        {
            return None;
        }

        let state_key = StateKey {
            shape_index,
            jet_index: last_jet_index,
            pattern: self.rows[elevation..].to_vec(),
        };
        let state_data = StateData {
            elevation,
            rock_index: self.num_rocks - 1,
        };
        match self.states.entry(state_key) {
            Entry::Occupied(e) => Some(Period {
                num_rocks: state_data.rock_index - e.get().rock_index,
                elevation_change: state_data.elevation - e.get().elevation,
            }),
            Entry::Vacant(e) => {
                e.insert(state_data);
                None
            }
        }
    }

    fn try_move_points(
        &self,
        points: &mut [P2<usize>],
        offset: impl Into<P2<Signed<usize>>>,
    ) -> bool {
        let offset: P2<Signed<usize>> = offset.into();
        for p in points.iter() {
            let Some(P2(x, y)) = p.checked_add_signed(&offset) else {return false;};
            if x >= WIDTH || self.contains(P2(x, y)) {
                return false;
            }
        }
        for p in points {
            *p = p.checked_add_signed(&offset).unwrap();
        }
        true
    }
}
