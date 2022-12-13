use std::time::{Duration, Instant};

use crate::Part;

#[derive(Default)]
pub struct Monitor {
    exec_times: [Vec<Duration>; 2],
    current: Option<Instant>,
    total_time: Duration,
    dropped_time: Duration,
}

impl Monitor {
    #[must_use]
    pub fn new_at_current_instant() -> Self {
        Self {
            current: Some(Instant::now()),
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        if let Some(c) = self.current.replace(now) {
            self.dropped_time += now.duration_since(c);
        }
    }

    pub fn finish(&mut self, part: Part) {
        if let Some(t) = self.current.take() {
            let elapsed = t.elapsed();
            self.total_time += elapsed;
            self.exec_times[part.to_index()].push(elapsed);
        } else {
            panic!("Nothing to finish");
        }
    }

    #[must_use]
    pub fn stats(&self, part: Part) -> Stats {
        Stats::new(&self.exec_times[part.to_index()])
    }

    #[must_use]
    pub fn dropped_time(&self) -> Duration {
        self.dropped_time
    }

    #[must_use]
    pub fn total_time(&self) -> Duration {
        self.total_time
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub exec_count: usize,
    pub exec_time_total: Duration,
    pub exec_time_mean: Duration,
    pub exec_time_std: Option<Duration>,
}

impl Stats {
    #[must_use]
    pub fn new(exec_times: &[Duration]) -> Self {
        let exec_time_total = exec_times.iter().sum::<Duration>();
        #[allow(clippy::cast_possible_truncation)]
        let exec_time_mean: Duration = exec_time_total / exec_times.len() as u32;
        let exec_time_err2_secs: f64 = exec_times
            .iter()
            .map(|d| (d.as_secs_f64() - exec_time_mean.as_secs_f64()).powi(2))
            .sum();
        let exec_time_std = if exec_times.len() <= 1 {
            None
        } else {
            #[allow(clippy::cast_precision_loss)]
            let secs = (exec_time_err2_secs / (exec_times.len() - 1) as f64).sqrt();
            Some(Duration::from_secs_f64(secs))
        };
        Self {
            exec_count: exec_times.len(),
            exec_time_total,
            exec_time_mean,
            exec_time_std,
        }
    }

    #[must_use]
    pub fn total_time(&self) -> Duration {
        #[allow(clippy::cast_possible_truncation)]
        let total_time = self.exec_time_mean * (self.exec_count as u32);
        total_time
    }
}
