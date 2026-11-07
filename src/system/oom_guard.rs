//! Memory-pressure policy planning based on Linux PSI.
//!
//! This module parses the kernel's `/proc/pressure/memory` format and turns
//! the ten-second stall averages into advisory actions. It deliberately does
//! not signal processes or write to cgroups; a privileged service can consume
//! the recommendation and apply its own explicit policy.

/// Ten-second memory pressure averages from a PSI `some` and optional `full`
/// record. Values are percentages in the inclusive range 0 through 100.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryPressure {
    pub some_avg10: f32,
    pub full_avg10: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureParseError {
    MissingSomeRecord,
    MissingAverage,
    InvalidAverage,
    AverageOutOfRange,
}

impl MemoryPressure {
    /// Parse the `some` and `full` lines exposed by `/proc/pressure/memory`.
    /// Unknown fields and records are ignored for forward compatibility.
    pub fn parse(input: &str) -> Result<Self, PressureParseError> {
        let mut some = None;
        let mut full = None;

        for line in input.lines() {
            let mut fields = line.split_whitespace();
            let record = fields.next();
            if record != Some("some") && record != Some("full") {
                continue;
            }

            let average = fields
                .filter_map(|field| field.strip_prefix("avg10="))
                .next()
                .ok_or(PressureParseError::MissingAverage)?
                .parse::<f32>()
                .map_err(|_| PressureParseError::InvalidAverage)?;
            if !average.is_finite() || !(0.0..=100.0).contains(&average) {
                return Err(PressureParseError::AverageOutOfRange);
            }

            if record == Some("some") {
                some = Some(average);
            } else {
                full = Some(average);
            }
        }

        Ok(Self {
            some_avg10: some.ok_or(PressureParseError::MissingSomeRecord)?,
            full_avg10: full.unwrap_or(0.0),
        })
    }
}

/// Advisory response to memory pressure. `PrepareEviction` asks the caller to
/// review its configured process policy; it never authorizes a kill itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureRecommendation {
    Normal,
    Notify,
    RequestReclaim,
    PrepareEviction,
}

/// Thresholds apply to PSI `avg10` percentages. The policy uses `some` to
/// detect contention and `full` to detect stalls affecting all non-idle work.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryPressurePolicy {
    pub notify_some_percent: f32,
    pub reclaim_some_percent: f32,
    pub prepare_eviction_full_percent: f32,
}

impl Default for MemoryPressurePolicy {
    fn default() -> Self {
        Self {
            notify_some_percent: 5.0,
            reclaim_some_percent: 10.0,
            prepare_eviction_full_percent: 2.0,
        }
    }
}

impl MemoryPressurePolicy {
    pub fn recommendation(self, pressure: MemoryPressure) -> PressureRecommendation {
        if pressure.full_avg10 >= self.prepare_eviction_full_percent {
            PressureRecommendation::PrepareEviction
        } else if pressure.some_avg10 >= self.reclaim_some_percent {
            PressureRecommendation::RequestReclaim
        } else if pressure.some_avg10 >= self.notify_some_percent {
            PressureRecommendation::Notify
        } else {
            PressureRecommendation::Normal
        }
    }
}
