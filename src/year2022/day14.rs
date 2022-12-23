use anyhow::anyhow;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::util::{
    coords::{xy, P2},
    grid::GridSpec,
    iter::IterExt,
};

aoc::register!(solve, 2022, 14);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut blocked = FxHashSet::default();
    let unbounded_spec = GridSpec::from(..);
    for line in input.trim().lines() {
        let points: Vec<_> = line.split(" -> ").map(parse_point).try_collect()?;
        for (start, end) in points.into_iter().tuple_windows() {
            for pos in iter_path(&unbounded_spec, start, end) {
                blocked.insert(pos);
            }
        }
    }

    let ymin = blocked.iter().map(|p| p.1).min().unwrap();
    let mut count = 0;
    while drop_sand(ymin - 2, &mut blocked) > ymin {
        count += 1;
    }
    out.set_part1(count);

    while drop_sand(ymin - 2, &mut blocked) != 0 {
        count += 1;
    }
    // The loops above always ignore the last unit of sand, so we need to add it to
    // our count:
    out.set_part2(count + 2);

    Ok(())
}

fn drop_sand(yfloor: i32, blocked: &mut FxHashSet<P2<i32>>) -> i32 {
    let mut pos = P2(500, 0);
    let possible_dirs = [xy::down::<i32>(), xy::downleft(), xy::downright()];
    loop {
        let dir = possible_dirs
            .iter()
            .find(|&&d| (pos + d).1 > yfloor && !blocked.contains(&(pos + d)));
        if let Some(&dir) = dir {
            pos += dir;
        } else {
            blocked.insert(pos);
            return pos.1;
        }
    }
}

fn iter_path(
    spec: &GridSpec<i32>,
    start: P2<i32>,
    end: P2<i32>,
) -> impl '_ + Iterator<Item = P2<i32>> {
    let dir = end - start;
    let dir = dir / P2(dir.0.abs().max(dir.1.abs()), 0);
    spec.iter_direction(start, dir.into_signed())
        .take_until_inclusive(move |x| x == &end)
}

fn parse_point(s: &str) -> anyhow::Result<P2<i32>> {
    let (x, y) = s
        .split_once(',')
        .ok_or_else(|| anyhow!("invalid coordinate: {s}"))?;
    Ok(P2(x.parse()?, -y.parse()?))
}
