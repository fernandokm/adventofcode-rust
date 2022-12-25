use std::{cmp::Reverse, collections::VecDeque};

use anyhow::anyhow;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

aoc::register!(solve, 2022, 16);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut valves = Valve::parse_valves(input, "AA".to_string())?;
    bfs(&mut valves);
    remove_useless_valves(&mut valves);

    out.set_part1(Optimizer::<1>::new(&valves, 30).optimize());
    out.set_part2(Optimizer::<2>::new(&valves, 26).optimize());

    Ok(())
}

fn bfs(valves: &mut [Valve]) {
    for src in 0..valves.len() {
        let mut queue = VecDeque::from_iter([src]);
        let mut dist = vec![u32::MAX; valves.len()];
        let mut visited = vec![false; valves.len()];
        dist[src] = 0;
        visited[src] = true;

        while let Some(i) = queue.pop_front() {
            let dist_i = dist[i];
            for &j in &valves[i].tunnels {
                if !visited[j] {
                    visited[j] = true;
                    dist[j] = dist_i + 1;
                    queue.push_back(j);
                }
            }
        }
        valves[src].dist = dist;
    }
}

fn remove_useless_valves(valves: &mut Vec<Valve>) {
    let mut retained_idxs = Vec::new();
    let mut idx = 0;
    valves.retain(|v| {
        // Always retain valve AA (idx == 0)
        let retain = v.flow_rate != 0 || idx == 0;
        if retain {
            retained_idxs.push(idx);
        }
        idx += 1;
        retain
    });

    for v in valves {
        v.tunnels = Vec::new();
        v.dist = retained_idxs
            .iter()
            .filter_map(|&i| v.dist.get(i).copied())
            .collect();
    }
}

#[derive(Debug, Clone)]
struct Optimizer<'a, const N: usize> {
    valves: &'a [Valve],
    is_closed: Vec<bool>,
    valve_idxs: [usize; N],
    time: [u32; N],
    valve_idxs_by_rev_flow_rate: Vec<usize>,
    pressure_released: u32,
    lower_bound: u32,
    time_limit: u32,
    remaining_valves: usize,
}

impl<'a, const N: usize> Optimizer<'a, N> {
    fn new(valves: &'a [Valve], time_limit: u32) -> Self {
        Self {
            valves,
            is_closed: valves.iter().map(|v| v.flow_rate == 0).collect(),
            valve_idxs: [0; N],
            time: [0; N],
            valve_idxs_by_rev_flow_rate: (0..valves.len())
                .sorted_unstable_by_key(|&i| Reverse(valves[i].flow_rate))
                .collect(),
            pressure_released: 0,
            lower_bound: 0,
            time_limit,
            remaining_valves: valves.iter().filter(|v| v.flow_rate != 0).count(),
        }
    }

    fn compute_upper_bound(&self) -> u32 {
        // Specializing the computation of t for the cases N == 1 and N == 2
        // (which are the cases we care about) leads to a significant speedup
        let t = match N {
            1 => self.time[0],
            2 => self.time[0].min(self.time[1]),
            _ => self.time.into_iter().min().unwrap(),
        };
        let mut remaining_time = self.time_limit.saturating_sub(t + 1);
        let mut max_possible_extra_pressure = 0;
        for &valve_idx in &self.valve_idxs_by_rev_flow_rate {
            if self.is_closed[valve_idx] {
                continue;
            }
            max_possible_extra_pressure += self.valves[valve_idx].flow_rate * remaining_time;
            if let Some(t) = remaining_time.checked_sub(2) {
                remaining_time = t;
            } else {
                break;
            }
        }
        self.pressure_released + max_possible_extra_pressure
    }

    fn optimize(&mut self) -> u32 {
        let i = self.time.iter().position_min().unwrap();
        if self.time[i] >= self.time_limit
            || self.compute_upper_bound() <= self.lower_bound
            || self.remaining_valves == 0
        {
            return self.pressure_released;
        }

        let mut max = 0;
        for valve_idx in 0..self.valves[self.valve_idxs[i]].dist.len() {
            if !self.is_closed[valve_idx] {
                max = max.max(self.recurse_with_valve(i, valve_idx));
            }
        }
        max
    }

    fn recurse_with_valve(&mut self, i: usize, valve_idx: usize) -> u32 {
        // Valve settings
        let time_spent = self.valves[self.valve_idxs[i]].dist[valve_idx] + 1;
        let flow_rate = self.valves[valve_idx].flow_rate;
        let total_valve_pressure =
            flow_rate * (self.time_limit.saturating_sub(self.time[i] + time_spent));
        let old_valve_idx = self.valve_idxs[i];

        // Open valve
        self.time[i] += time_spent;
        self.is_closed[valve_idx] = true;
        self.remaining_valves -= 1;
        self.pressure_released += total_valve_pressure;
        self.valve_idxs[i] = valve_idx;

        // Recurse
        let pressure = self.optimize();
        self.lower_bound = self.lower_bound.max(pressure);

        // Close valve
        self.is_closed[valve_idx] = false;
        self.time[i] -= time_spent;
        self.remaining_valves += 1;
        self.pressure_released -= total_valve_pressure;
        self.valve_idxs[i] = old_valve_idx;

        pressure
    }
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<usize>,
    dist: Vec<u32>,
}

impl Valve {
    fn parse_valves(s: &str, start_valve: String) -> anyhow::Result<Vec<Valve>> {
        let mut valves = FxHashMap::default();
        valves.insert(start_valve, (0, None));
        for line in s.trim().lines() {
            Valve::parse_into(line, &mut valves)?;
        }
        Ok(valves
            .into_values()
            .sorted_by_key(|&(i, _)| i)
            .map(|(_, valve)| valve.unwrap())
            .collect_vec())
    }

    fn parse_into(
        s: &str,
        valves: &mut FxHashMap<String, (usize, Option<Valve>)>,
    ) -> anyhow::Result<()> {
        fn get_valve(
            valves: &mut FxHashMap<String, (usize, Option<Valve>)>,
            name: impl Into<String>,
        ) -> &mut (usize, Option<Valve>) {
            let len = valves.len();
            valves.entry(name.into()).or_insert_with(|| (len, None))
        }

        let mut extract_strs = || {
            let mut it = s.splitn(10, ' ');
            let name = it.nth(1)?;
            let flow_rate = it.nth(2)?.strip_prefix("rate=")?.strip_suffix(';')?;
            let tunnels = it
                .last()?
                .split(", ")
                .map(|valve_name| get_valve(valves, valve_name).0)
                .collect();
            Some((name, flow_rate, tunnels))
        };

        let (name, flow_rate, tunnels) = extract_strs().ok_or_else(|| anyhow!("invalid input"))?;
        get_valve(valves, name).1 = Some(Self {
            flow_rate: flow_rate.parse()?,
            tunnels,
            dist: Vec::new(),
        });
        Ok(())
    }
}
