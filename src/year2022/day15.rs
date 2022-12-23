use std::ops::{Range, RangeInclusive};

use anyhow::anyhow;
use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::coords::P2;

aoc::register!(solve, 2022, 15);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut sensors_beacons: Vec<_> = input
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<_> {
            let mut it = line.trim().split(&[',', ':', '=']);
            let mut get_i64 = || it.nth(1).unwrap_or("").parse::<i64>();
            let sensor = P2(get_i64()?, get_i64()?);
            let beacon = P2(get_i64()?, get_i64()?);
            Ok((sensor, beacon))
        })
        .try_collect()?;

    // sorting the sensors reduces memory allocations in MultiRange::push
    sensors_beacons.sort_by_key(|s| (s.0.0, s.0.1));
    let sensors_beacons = sensors_beacons;

    let is_real = out.variant() == "real";

    let target_y = if is_real { 2_000_000 } else { 10 };
    let beacons_at_target_y = sensors_beacons
        .iter()
        .map(|(_, beacon)| beacon)
        .filter(|&&P2(_x, y)| y == target_y)
        .unique()
        .count() as i64;
    let mut blocked_coords = MultiRange::default();
    push_range_at_y(&mut blocked_coords, &sensors_beacons, target_y);
    out.set_part1(blocked_coords.len() - beacons_at_target_y);

    let max_xy = if is_real { 4_000_000 } else { 20 } + 1;
    let xy_range = 0..max_xy;
    out.set_part2(
        xy_range
            .clone()
            .find_map(|y| {
                blocked_coords.clear();
                push_range_at_y(&mut blocked_coords, &sensors_beacons, y);
                let x = blocked_coords.find_missing_in_range(&xy_range)?;
                Some(x * 4_000_000 + y)
            })
            .ok_or_else(|| anyhow!("No empty space found"))?,
    );

    Ok(())
}

fn push_range_at_y(
    blocked_coords: &mut MultiRange,
    sensors_beacons: &[(P2<i64>, P2<i64>)],
    target_y: i64,
) {
    for &(sensor, beacon) in sensors_beacons {
        let dy = (sensor.1 - target_y).abs();
        let max_dist = (sensor - beacon).norm_l1();
        let range = max_dist - dy;
        if range >= 0 {
            blocked_coords.push_inclusive(sensor.0 - range..=sensor.0 + range);
        }
    }
}

#[derive(Debug, Clone, Default)]
struct MultiRange {
    ranges: Vec<(i64, i64)>,
}

impl MultiRange {
    fn clear(&mut self) {
        self.ranges.clear();
    }

    fn len(&self) -> i64 {
        self.ranges.iter().map(|(start, end)| end - start).sum()
    }

    fn push_inclusive(&mut self, range: RangeInclusive<i64>) {
        #[allow(clippy::range_plus_one)]
        self.push(*range.start()..*range.end() + 1);
    }

    fn push(&mut self, range: Range<i64>) {
        // i0 is the index of the first range that either intersects with `range`
        // or is to the right of `range`
        let i0 = self
            .ranges
            .iter()
            .position(|&(_, end)| end >= range.start)
            .unwrap_or(self.ranges.len());

        // i0 is the index of the first range that is to the right of `range`
        let i1 = self.ranges[i0..]
            .iter()
            .position(|&(start, _)| range.end < start)
            .map_or(self.ranges.len(), |pos| pos + i0);

        if i0 == i1 {
            // In this case, there are no ranges which intersect with `range`
            self.ranges.insert(i1, (range.start, range.end));
        } else {
            // Otherwise, the ranges at indexes i0..i1  intersect with `range`
            // and we should merge them
            self.ranges[i0] = (
                self.ranges[i0].0.min(range.start),
                self.ranges[i1 - 1].1.max(range.end),
            );
            // Since i0 < i1, we always have i0+1 <= i1, which ensures that
            // Vec::drain won't panic
            self.ranges.drain(i0 + 1..i1);
        }
    }

    fn find_missing_in_range(&self, range: &Range<i64>) -> Option<i64> {
        let &Range { mut start, end } = range;
        for &(rstart, rend) in &self.ranges {
            if start < rstart {
                return Some(start);
            }
            start = rend;
        }
        (start < end).then_some(start)
    }
}
