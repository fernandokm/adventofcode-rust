use anyhow::anyhow;
use aoc::ProblemOutput;
use itertools::{iproduct, Itertools};

aoc::register!(solve, 2022, 18);

type Vec3<T> = Vec<Vec<Vec<T>>>;

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut points = Point::parse_vec(input)?;
    let indices = iproduct!(0..points.len(), 0..points[0].len(), 0..points[0][0].len());

    out.set_part1(count_droplet_sides(indices.clone(), &points, |p| {
        p != Point::Droplet
    }));

    for (x, y, z) in indices.clone() {
        let is_extreme = [0, points.len() - 1].contains(&x)
            || [0, points[0].len() - 1].contains(&y)
            || [0, points[0][0].len() - 1].contains(&z);
        if points[x][y][z] == Point::Interior && is_extreme {
            mark_and_propagate_exterior_point(&mut points, x, y, z);
        }
    }
    out.set_part2(count_droplet_sides(indices, &points, |p| {
        p == Point::Exterior
    }));

    Ok(())
}

fn count_droplet_sides(
    indices: impl Iterator<Item = (usize, usize, usize)>,
    points: &Vec3<Point>,
    neighbor_pred: impl Fn(Point) -> bool,
) -> usize {
    indices
        .filter(|&(x, y, z)| points[x][y][z] == Point::Droplet)
        .map(|(x, y, z)| {
            // This is equivalent to counting sides for which neighbor_pred(p) is true,
            // except that we also count neighbors which are out of bounds in indices
            // (and therefore are certainly exterior points).
            let ignored_sides = neighbors(points, x, y, z)
                .filter(|(_, &p)| !neighbor_pred(p))
                .count();
            6 - ignored_sides
        })
        .sum::<usize>()
}

fn mark_and_propagate_exterior_point(points: &mut Vec3<Point>, x: usize, y: usize, z: usize) {
    points[x][y][z] = Point::Exterior;
    let neighbors = neighbors(points, x, y, z)
        .filter(|&(_, &p)| p == Point::Interior)
        .map(|(coords, _)| coords)
        .collect_vec();
    for (nx, ny, nz) in neighbors {
        mark_and_propagate_exterior_point(points, nx, ny, nz);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Point {
    Droplet,
    Interior,
    Exterior,
}

impl Point {
    fn parse_coords(line: &str) -> anyhow::Result<(usize, usize, usize)> {
        let mut it = line.split(',');
        let mut get_val = || -> anyhow::Result<_> {
            Ok(it
                .next()
                .ok_or_else(|| anyhow!("Invalid input line: {line}"))?
                .trim()
                .parse()?)
        };
        Ok((get_val()?, get_val()?, get_val()?))
    }

    fn parse_vec(input: &str) -> anyhow::Result<Vec3<Point>> {
        let droplet_coords: Vec<_> = input.trim().lines().map(Self::parse_coords).try_collect()?;

        let max_x = droplet_coords.iter().map(|p| p.0).max().unwrap();
        let max_y = droplet_coords.iter().map(|p| p.1).max().unwrap();
        let max_z = droplet_coords.iter().map(|p| p.2).max().unwrap();

        let mut v = vec![vec![vec![Point::Interior; max_z + 1]; max_y + 1]; max_x + 1];
        for (x, y, z) in droplet_coords {
            v[x][y][z] = Point::Droplet;
        }

        Ok(v)
    }
}

fn neighbors<T>(
    v: &Vec3<T>,
    x: usize,
    y: usize,
    z: usize,
) -> impl Iterator<Item = ((usize, usize, usize), &T)> {
    let directions = [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    directions.into_iter().filter_map(move |(dx, dy, dz)| {
        #[allow(clippy::cast_sign_loss)]
        let add = |s, ds| Some((s as isize).checked_add(ds)? as usize);
        let x = add(x, dx)?;
        let y = add(y, dy)?;
        let z = add(z, dz)?;
        let value = v.get(x)?.get(y)?.get(z)?;
        Some(((x, y, z), value))
    })
}
