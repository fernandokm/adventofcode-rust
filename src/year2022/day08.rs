use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2022, 8);

type P2 = (usize, usize);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let grid_size = (grid.len(), grid[0].len());
    let grid_positions = (0..grid_size.0).cartesian_product(0..grid_size.1);

    out.set_part1(
        grid_positions
            .clone()
            .filter(|&pos| {
                Direction::variants()
                    .any(|d| matches!(d.viewing_distance(pos, &grid), ViewingDistance::Visible(_)))
            })
            .count(),
    );

    out.set_part2(
        grid_positions
            .map(|pos| {
                Direction::variants()
                    .map(|d| d.viewing_distance(pos, &grid).as_u64())
                    .product::<u64>()
            })
            .max()
            .unwrap(),
    );

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn variants() -> impl Iterator<Item = Direction> {
        [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
    }

    fn viewing_distance(self, (i, j): P2, grid: &[&[u8]]) -> ViewingDistance {
        let mut count = 0;
        for (ii, jj) in self.iter_to_end((i, j), (grid.len(), grid[0].len())) {
            if grid[ii][jj] >= grid[i][j] {
                return ViewingDistance::Hidden(count + 1);
            }
            count += 1;
        }
        ViewingDistance::Visible(count)
    }

    fn iter_to_end(self, pos: P2, grid_size: P2) -> impl Iterator<Item = P2> {
        let (mut i, mut j) = pos;
        std::iter::from_fn(move || {
            (i, j) = match self {
                Direction::Up => (i.checked_sub(1)?, j),
                Direction::Left => (i, j.checked_sub(1)?),
                Direction::Down if i + 1 < grid_size.0 => (i + 1, j),
                Direction::Right if j + 1 < grid_size.1 => (i, j + 1),
                _ => return None,
            };
            Some((i, j))
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum ViewingDistance {
    Visible(u64),
    Hidden(u64),
}

impl ViewingDistance {
    fn as_u64(self) -> u64 {
        match self {
            ViewingDistance::Visible(d) | ViewingDistance::Hidden(d) => d,
        }
    }
}
