use std::time::{Duration, Instant};

/// Recorded result of a single solve attempt.
#[derive(Debug, Clone)]
pub struct SolveRecord {
    pub steps: usize,
    pub duration: Duration,
}

impl SolveRecord {
    /// Steps per millisecond (throughput).
    pub fn steps_per_ms(&self) -> f64 {
        let ms = self.duration.as_secs_f64() * 1000.0;
        if ms > 0.0 {
            self.steps as f64 / ms
        } else {
            f64::INFINITY
        }
    }
}

/// Accumulates solve statistics across multiple solves.
#[derive(Debug, Default, Clone)]
pub struct SolveStats {
    pub records: Vec<SolveRecord>,
}

impl SolveStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, steps: usize, duration: Duration) {
        self.records.push(SolveRecord { steps, duration });
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn avg_steps(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records.iter().map(|r| r.steps as f64).sum::<f64>() / self.records.len() as f64
    }

    pub fn avg_duration_ms(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records
            .iter()
            .map(|r| r.duration.as_secs_f64() * 1000.0)
            .sum::<f64>()
            / self.records.len() as f64
    }

    /// Average steps per millisecond across all recorded solves.
    pub fn avg_steps_per_ms(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records.iter().map(|r| r.steps_per_ms()).sum::<f64>() / self.records.len() as f64
    }

    pub fn last(&self) -> Option<&SolveRecord> {
        self.records.last()
    }
}
