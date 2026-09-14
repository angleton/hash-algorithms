//! Lightweight, high-resolution timing telemetry for the RandomX hash
//! pipeline.
//!
//! Every stage described in the [`crate`] module docs (Cache init,
//! SuperscalarHash generation, entropy seeding, the 8 chained
//! program-generate/AES-fill/VM-execute/hash-and-fill rounds, and final
//! Blake2b) can be wrapped in [`HashTelemetry::time`] to record how long it
//! took, at the resolution of the platform's monotonic clock
//! (`std::time::Instant`, sub-microsecond on most systems). This mirrors
//! the same "one entry per pipeline step" shape the BDD features already
//! use to describe the algorithm, just for timing instead of correctness.

use std::fmt;
use std::time::{Duration, Instant};

use cpu_time::ProcessTime;

/// Timing for a single named pipeline step.
#[derive(Debug, Clone, Copy)]
pub struct StepTiming {
    pub name: &'static str,
    pub duration: Duration,
    pub cpu_duration: Duration,
    pub memory_bytes: usize,
}

/// Ordered collection of [`StepTiming`]s gathered while computing one hash.
#[derive(Debug, Default, Clone)]
pub struct HashTelemetry {
    steps: Vec<StepTiming>,
}

impl HashTelemetry {
    /// Create an empty telemetry collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Run `f`, recording its wall-clock duration under `name`, and return
    /// `f`'s result. Steps are recorded in the order they're timed, so
    /// nested/sequential calls naturally produce a pipeline trace.
    pub fn time<T>(&mut self, name: &'static str, f: impl FnOnce() -> T) -> T {
        self.time_with_memory(name, 0, f)
    }

    /// Run `f`, recording wall time, process CPU time, and the amount of
    /// algorithm memory attributed to the stage.
    pub fn time_with_memory<T>(
        &mut self,
        name: &'static str,
        memory_bytes: usize,
        f: impl FnOnce() -> T,
    ) -> T {
        let start = Instant::now();
        let cpu_start = ProcessTime::now();
        let result = f();
        self.steps.push(StepTiming {
            name,
            duration: start.elapsed(),
            cpu_duration: cpu_start.elapsed(),
            memory_bytes,
        });
        result
    }

    /// All recorded step timings, in the order they were recorded.
    pub fn steps(&self) -> &[StepTiming] {
        &self.steps
    }

    /// Sum of all recorded step durations.
    pub fn total(&self) -> Duration {
        self.steps.iter().map(|step| step.duration).sum()
    }

    /// Sum of process CPU time consumed by all recorded stages.
    pub fn total_cpu(&self) -> Duration {
        self.steps.iter().map(|step| step.cpu_duration).sum()
    }

    /// Largest attributed working set among the recorded stages.
    pub fn peak_memory_bytes(&self) -> usize {
        self.steps
            .iter()
            .map(|step| step.memory_bytes)
            .max()
            .unwrap_or(0)
    }
}

impl fmt::Display for HashTelemetry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{:<24} {:>12} {:>12} {:>8} {:>12}",
            "stage", "wall ms", "cpu ms", "cpu %", "memory MiB"
        )?;
        let total_cpu = self.total_cpu().as_secs_f64();
        for step in &self.steps {
            let cpu_percent = if total_cpu == 0.0 {
                0.0
            } else {
                step.cpu_duration.as_secs_f64() / total_cpu * 100.0
            };
            writeln!(
                f,
                "{:<24} {:>12.3} {:>12.3} {:>7.1}% {:>12.1}",
                step.name,
                step.duration.as_secs_f64() * 1e3,
                step.cpu_duration.as_secs_f64() * 1e3,
                cpu_percent,
                step.memory_bytes as f64 / (1024.0 * 1024.0),
            )?;
        }
        write!(
            f,
            "{:<24} {:>12.3} {:>12.3} {:>8} {:>12.1}",
            "total / peak",
            self.total().as_secs_f64() * 1e3,
            self.total_cpu().as_secs_f64() * 1e3,
            "100.0%",
            self.peak_memory_bytes() as f64 / (1024.0 * 1024.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn time_records_a_step_with_a_positive_duration() {
        let mut telemetry = HashTelemetry::new();
        telemetry.time("sleep", || thread::sleep(Duration::from_millis(1)));

        assert_eq!(telemetry.steps().len(), 1);
        assert_eq!(telemetry.steps()[0].name, "sleep");
        assert!(telemetry.steps()[0].duration >= Duration::from_millis(1));
    }

    #[test]
    fn time_returns_the_closures_value() {
        let mut telemetry = HashTelemetry::new();
        let value = telemetry.time("compute", || 2 + 2);

        assert_eq!(value, 4);
    }

    #[test]
    fn total_sums_every_recorded_step() {
        let mut telemetry = HashTelemetry::new();
        telemetry.time("a", || thread::sleep(Duration::from_millis(1)));
        telemetry.time("b", || thread::sleep(Duration::from_millis(1)));

        assert!(telemetry.total() >= Duration::from_millis(2));
        assert_eq!(telemetry.steps().len(), 2);
    }

    #[test]
    fn new_telemetry_has_no_steps_and_zero_total() {
        let telemetry = HashTelemetry::new();

        assert!(telemetry.steps().is_empty());
        assert_eq!(telemetry.total(), Duration::ZERO);
        assert_eq!(telemetry.total_cpu(), Duration::ZERO);
        assert_eq!(telemetry.peak_memory_bytes(), 0);
    }

    #[test]
    fn steps_are_recorded_in_the_order_they_were_timed() {
        let mut telemetry = HashTelemetry::new();
        telemetry.time("first", || ());
        telemetry.time("second", || ());
        telemetry.time("third", || ());

        let names: Vec<&str> = telemetry.steps().iter().map(|step| step.name).collect();
        assert_eq!(names, ["first", "second", "third"]);
    }

    #[test]
    fn display_lists_each_step_and_a_trailing_total() {
        let mut telemetry = HashTelemetry::new();
        telemetry.time("stage_one", || ());
        telemetry.time("stage_two", || ());

        let rendered = telemetry.to_string();
        let lines: Vec<&str> = rendered.lines().collect();

        assert_eq!(lines.len(), 4);
        assert!(lines[0].starts_with("stage"));
        assert!(lines[1].starts_with("stage_one"));
        assert!(lines[2].starts_with("stage_two"));
        assert!(lines[3].starts_with("total / peak"));
    }

    #[test]
    fn time_with_memory_tracks_attributed_memory() {
        let mut telemetry = HashTelemetry::new();
        telemetry.time_with_memory("small", 1024, || ());
        telemetry.time_with_memory("large", 4096, || ());

        assert_eq!(telemetry.steps()[0].memory_bytes, 1024);
        assert_eq!(telemetry.peak_memory_bytes(), 4096);
    }
}
