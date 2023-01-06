use std::{iter::zip, str::FromStr};

use anyhow::anyhow;
use aoc::ProblemOutput;
use itertools::{izip, Itertools};

aoc::register!(solve, 2022, 19);

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let blueprints: Vec<Blueprint> = input.trim().lines().map(str::parse).try_collect()?;

    out.set_part1(
        blueprints
            .iter()
            .map(|bp| {
                let mut opt = Optimizer::new(24);
                opt.optimize(&Simulation::new(bp));
                opt.max_geode * bp.id
            })
            .sum::<u32>(),
    );

    out.set_part2(
        blueprints
            .iter()
            .take(3)
            .map(|bp| {
                let mut opt = Optimizer::new(32);
                opt.optimize(&Simulation::new(bp));
                opt.max_geode
            })
            .product::<u32>(),
    );

    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
struct Blueprint {
    id: u32,
    robot_costs: [[u32; 3]; 4],
    max_resource_usage: [u32; 3],
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    #[allow(clippy::field_reassign_with_default)]
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut it = s
            .trim()
            .split_ascii_whitespace()
            .filter(|x| (b'0'..=b'9').contains(&x.bytes().next().unwrap()))
            .map(|x| x.trim_end_matches(':').parse());
        let mut get = || -> anyhow::Result<u32> {
            Ok(it
                .next()
                .ok_or_else(|| anyhow!("Unexpected end of input in line: {s}"))??)
        };
        let mut bp = Blueprint::default();
        bp.id = get()?;
        bp.robot_costs[ORE][ORE] = get()?;
        bp.robot_costs[CLAY][ORE] = get()?;
        bp.robot_costs[OBSIDIAN][ORE] = get()?;
        bp.robot_costs[OBSIDIAN][CLAY] = get()?;
        bp.robot_costs[GEODE][ORE] = get()?;
        bp.robot_costs[GEODE][OBSIDIAN] = get()?;

        for i in 0..3 {
            bp.max_resource_usage[i] = bp.robot_costs.iter().map(|costs| costs[i]).max().unwrap();
        }
        Ok(bp)
    }
}

#[derive(Debug, Clone, Copy)]
struct Optimizer {
    max_geode: u32,
    time_limit: u32,
}

impl Optimizer {
    fn new(time_limit: u32) -> Self {
        Self {
            max_geode: 0,
            time_limit,
        }
    }

    fn compute_upper_bound(self, sim: &Simulation<'_>) -> u32 {
        let mut sims = [*sim; 4];
        for _ in sim.time..self.time_limit {
            let mut new_robots = sims[0].robots;
            for (i, sim) in sims.iter_mut().enumerate() {
                if sim.can_build(i) {
                    sim.build(i);
                    sim.robots[i] -= 1;
                    new_robots[i] += 1;
                }
                sim.step_n(1);
            }
            for sim in &mut sims {
                sim.robots = new_robots;
            }
        }
        sims[GEODE].resources[GEODE]
    }

    fn optimize(&mut self, sim: &Simulation<'_>) {
        if sim.time >= self.time_limit || self.compute_upper_bound(sim) <= self.max_geode {
            self.max_geode = self.max_geode.max(sim.resources[GEODE]);
            return;
        }

        let num_branches = (0..4)
            .rev()
            .filter_map(|robot| self.build_and_optimize(sim, robot).ok())
            .count();

        if num_branches == 0 {
            let mut sim2 = *sim;
            sim2.step_n(self.time_limit - sim2.time);
            self.max_geode = self.max_geode.max(sim2.resources[GEODE]);
        }
    }

    fn build_and_optimize(&mut self, sim: &Simulation<'_>, robot: usize) -> Result<(), ()> {
        let n = sim.time_until_resources(robot);
        if n >= self.time_limit - sim.time - 1 {
            return Err(());
        }

        let mut sim2 = *sim;
        sim2.step_n(n + 1);
        sim2.build(robot);
        self.optimize(&sim2);

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Simulation<'a> {
    resources: [u32; 4],
    robots: [u32; 4],
    time: u32,
    blueprint: &'a Blueprint,
}

impl<'a> Simulation<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        let mut sim = Self {
            resources: [0; 4],
            robots: [0; 4],
            time: 0,
            blueprint,
        };
        sim.robots[ORE] = 1;
        sim
    }

    fn can_build(&self, robot: usize) -> bool {
        let cost = &self.blueprint.robot_costs[robot];
        let cond_ore = self.resources[ORE] >= cost[ORE];
        match robot {
            OBSIDIAN => cond_ore && self.resources[CLAY] >= cost[CLAY],
            GEODE => cond_ore && self.resources[OBSIDIAN] >= cost[OBSIDIAN],
            _ => cond_ore,
        }
    }

    fn time_until_resources(&self, robot: usize) -> u32 {
        let wanted_resources = &self.blueprint.robot_costs[robot];
        izip!(&self.resources, &self.robots, wanted_resources)
            .map(|(&available, &robots, &wanted)| {
                if available >= wanted {
                    0
                } else if robots == 0 {
                    u32::MAX
                } else {
                    let missing = wanted - available;
                    // stable version of missing.div_ceil(robots)
                    (missing + robots - 1) / robots
                }
            })
            .max()
            .unwrap()
    }

    fn build(&mut self, robot: usize) {
        for (r, c) in zip(&mut self.resources, &self.blueprint.robot_costs[robot]) {
            *r -= c;
        }
        self.robots[robot] += 1;
    }

    fn step_n(&mut self, n: u32) {
        for (resource, robots) in zip(&mut self.resources, self.robots) {
            *resource += robots * n;
        }
        self.time += n;
    }
}
