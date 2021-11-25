use std::str::FromStr;

use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use super::game::{Game, Op};

aoc::register!(solve, 2020, 8);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let mut mem: Vec<_> = input.lines().map(FromStr::from_str).try_collect()?;
    let mut game = Game::new(&mem);

    let mut visited = FxHashSet::default();
    while visited.insert(game.pos()) {
        game.execute_single()?;
    }
    out.set_part1(game.acc());

    for i in 0..mem.len() {
        let old_op = mem[i].op;
        match old_op {
            Op::Jmp => mem[i].op = Op::Nop,
            Op::Nop => mem[i].op = Op::Jmp,
            _ => continue,
        };

        let mut game = Game::new(&mem);
        visited.clear();
        while visited.insert(game.pos()) && game.pos() < mem.len() {
            game.execute_single()?;
        }
        if game.pos() == mem.len() {
            out.set_part2(game.acc());
            return Ok(());
        }

        mem[i].op = old_op;
    }

    Ok(())
}
