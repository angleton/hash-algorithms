use cpu_time::ProcessTime;
use std::fmt;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct StepTiming {
    pub name: &'static str,
    pub duration: Duration,
    pub cpu_duration: Duration,
    pub memory_bytes: usize,
}

#[derive(Debug, Default, Clone)]
pub struct HashTelemetry {
    steps: Vec<StepTiming>,
}

impl HashTelemetry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn time<T>(&mut self, name: &'static str, memory_bytes: usize, f: impl FnOnce() -> T) -> T {
        let wall_start = Instant::now();
        let cpu_start = ProcessTime::now();
        let result = f();
        self.steps.push(StepTiming {
            name,
            duration: wall_start.elapsed(),
            cpu_duration: cpu_start.elapsed(),
            memory_bytes,
        });
        result
    }

    pub fn steps(&self) -> &[StepTiming] {
        &self.steps
    }

    pub fn total(&self) -> Duration {
        self.steps.iter().map(|step| step.duration).sum()
    }

    pub fn total_cpu(&self) -> Duration {
        self.steps.iter().map(|step| step.cpu_duration).sum()
    }

    pub fn peak_memory_bytes(&self) -> usize {
        self.steps.iter().map(|step| step.memory_bytes).max().unwrap_or(0)
    }
}

impl fmt::Display for HashTelemetry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{:<24} {:>12} {:>12} {:>8} {:>12}", "stage", "wall ms", "cpu ms", "cpu %", "memory KiB")?;
        let total_cpu = self.total_cpu().as_secs_f64();
        for step in &self.steps {
            let cpu_percent = if total_cpu == 0.0 {
                0.0
            } else {
                step.cpu_duration.as_secs_f64() / total_cpu * 100.0
            };
            writeln!(
                formatter,
                "{:<24} {:>12.3} {:>12.3} {:>7.1}% {:>12.1}",
                step.name,
                step.duration.as_secs_f64() * 1e3,
                step.cpu_duration.as_secs_f64() * 1e3,
                cpu_percent,
                step.memory_bytes as f64 / 1024.0,
            )?;
        }
        write!(
            formatter,
            "{:<24} {:>12.3} {:>12.3} {:>8} {:>12.1}",
            "total / peak",
            self.total().as_secs_f64() * 1e3,
            self.total_cpu().as_secs_f64() * 1e3,
            "100.0%",
            self.peak_memory_bytes() as f64 / 1024.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_07_telemetry_records_order_and_memory() {
        let mut telemetry = HashTelemetry::new();
        telemetry.time("stage_01", 64, || ());
        telemetry.time("stage_02", 128, || ());
        assert_eq!(telemetry.steps().iter().map(|step| step.name).collect::<Vec<_>>(), ["stage_01", "stage_02"]);
        assert_eq!(telemetry.peak_memory_bytes(), 128);
    }
}
