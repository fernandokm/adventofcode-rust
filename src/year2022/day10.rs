use anyhow::Context;
use aoc::ProblemOutput;

aoc::register!(solve, 2022, 10);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut cpu = Cpu::new(input.trim().lines());

    let mut acc = 0;
    cpu.draw_crt();
    while cpu.run_cycle()? {
        cpu.draw_crt();

        if (cpu.cycle + 20) % 40 == 0 {
            acc += cpu.signal_strength();
        }
    }

    out.set_part1(acc);
    out.set_part2(cpu.screen);

    Ok(())
}

struct Cpu<'a> {
    instructions: Box<dyn 'a + Iterator<Item = &'a str>>,
    x: i64,
    value_to_add: Option<i64>,
    ip: usize,
    cycle: i64,
    screen: String,
}

impl<'a> Cpu<'a> {
    fn new(instructions: impl 'a + IntoIterator<Item = &'a str>) -> Self {
        Self {
            instructions: Box::new(instructions.into_iter()),
            x: 1,
            value_to_add: None,
            ip: 0,
            cycle: 1,
            screen: String::new(),
        }
    }

    fn draw_crt(&mut self) {
        let crt_row = (self.cycle - 1) / 40;
        let crt_col = (self.cycle - 1) % 40;
        if crt_row >= 6 {
            return;
        }
        if crt_row > 0 && crt_col == 0 {
            self.screen.push('\n');
        }

        if (self.x - 1..=self.x + 1).contains(&crt_col) {
            self.screen.push('#');
        } else {
            self.screen.push(' ');
        }
    }

    fn signal_strength(&self) -> i64 {
        self.cycle * self.x
    }

    fn run_cycle(&mut self) -> anyhow::Result<bool> {
        if let Some(v) = self.value_to_add {
            self.x += v;
            self.value_to_add = None;
        } else if let Some(line) = self.instructions.next() {
            if let Some(val) = line.strip_prefix("addx ") {
                self.value_to_add = Some(val.parse().context("Error parsing instruction")?);
            }
            self.ip += 1;
        } else {
            return Ok(false);
        }

        self.cycle += 1;
        Ok(true)
    }
}
