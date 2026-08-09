// SigmaOS Custom Time Implementation
// Reduces dependency on std::time by providing custom implementations

use core::cell::Cell;

/// Custom timestamp for OS timekeeping
#[derive(Debug, Clone, Copy)]
pub struct SigmaTimestamp {
    pub seconds: u64,
    pub nanoseconds: u32,
}

impl SigmaTimestamp {
    pub fn new(seconds: u64, nanoseconds: u32) -> Self {
        SigmaTimestamp {
            seconds,
            nanoseconds,
        }
    }

    /// Get current time (simulated)
    pub fn now() -> Self {
        // In real implementation, this would get actual system time
        SigmaTimestamp {
            seconds: 0,
            nanoseconds: 0,
        }
    }

    /// Add duration to timestamp
    pub fn add(&self, duration: SigmaDuration) -> SigmaTimestamp {
        let new_seconds = self.seconds + duration.seconds;
        let new_nanos = self.nanoseconds + duration.nanoseconds;
        let carry = if new_nanos >= 1_000_000_000 {
            1
        } else {
            0
        };
        SigmaTimestamp {
            seconds: new_seconds + carry as u64,
            nanoseconds: new_nanos % 1_000_000_000,
        }
    }

    /// Compare timestamps
    pub fn cmp(&self, other: &SigmaTimestamp) -> core::cmp::Ordering {
        if self.seconds != other.seconds {
            self.seconds.cmp(&other.seconds)
        } else {
            self.nanoseconds.cmp(&other.nanoseconds)
        }
    }
}

/// Custom duration for time intervals
#[derive(Debug, Clone, Copy)]
pub struct SigmaDuration {
    pub seconds: u64,
    pub nanoseconds: u32,
}

impl SigmaDuration {
    pub fn new(seconds: u64, nanoseconds: u32) -> Self {
        SigmaDuration {
            seconds,
            nanoseconds,
        }
    }

    /// Duration from seconds
    pub fn from_secs(seconds: u64) -> Self {
        SigmaDuration {
            seconds,
            nanoseconds: 0,
        }
    }

    /// Duration from milliseconds
    pub fn from_millis(millis: u64) -> Self {
        SigmaDuration {
            seconds: millis / 1000,
            nanoseconds: ((millis % 1000) * 1_000_000) as u32,
        }
    }

    /// Duration from microseconds
    pub fn from_micros(micros: u64) -> Self {
        SigmaDuration {
            seconds: micros / 1_000_000,
            nanoseconds: ((micros % 1_000_000) * 1000) as u32,
        }
    }

    /// Add durations
    pub fn add(&self, other: SigmaDuration) -> SigmaDuration {
        let new_seconds = self.seconds + other.seconds;
        let new_nanos = self.nanoseconds + other.nanoseconds;
        let carry = if new_nanos >= 1_000_000_000 {
            1
        } else {
            0
        };
        SigmaDuration {
            seconds: new_seconds + carry as u64,
            nanoseconds: new_nanos % 1_000_000_000,
        }
    }

    /// Convert to milliseconds
    pub fn as_millis(&self) -> u64 {
        self.seconds * 1000 + (self.nanoseconds / 1_000_000) as u64
    }

    /// Convert to microseconds
    pub fn as_micros(&self) -> u64 {
        self.seconds * 1_000_000 + (self.nanoseconds / 1000) as u64
    }
}

/// Timer for measuring time intervals
pub struct SigmaTimer {
    pub start_time: Cell<SigmaTimestamp>,
    pub elapsed: Cell<SigmaDuration>,
}

impl SigmaTimer {
    pub fn new() -> Self {
        SigmaTimer {
            start_time: Cell::new(SigmaTimestamp::now()),
            elapsed: Cell::new(SigmaDuration::new(0, 0)),
        }
    }

    /// Start the timer
    pub fn start(&self) {
        self.start_time.set(SigmaTimestamp::now());
    }

    /// Stop the timer and get elapsed time
    pub fn stop(&self) -> SigmaDuration {
        let now = SigmaTimestamp::now();
        let start = self.start_time.get();
        
        // Calculate elapsed time
        let mut elapsed_seconds = now.seconds - start.seconds;
        let mut elapsed_nanos = now.nanoseconds as i64 - start.nanoseconds as i64;
        
        if elapsed_nanos < 0 {
            elapsed_seconds -= 1;
            elapsed_nanos += 1_000_000_000;
        }
        
        let elapsed = SigmaDuration {
            seconds: elapsed_seconds,
            nanoseconds: elapsed_nanos as u32,
        };
        
        self.elapsed.set(elapsed);
        elapsed
    }

    /// Get elapsed time without stopping
    pub fn elapsed(&self) -> SigmaDuration {
        let now = SigmaTimestamp::now();
        let start = self.start_time.get();
        
        // Calculate elapsed time
        let mut elapsed_seconds = now.seconds - start.seconds;
        let mut elapsed_nanos = now.nanoseconds as i64 - start.nanoseconds as i64;
        
        if elapsed_nanos < 0 {
            elapsed_seconds -= 1;
            elapsed_nanos += 1_000_000_000;
        }
        
        SigmaDuration {
            seconds: elapsed_seconds,
            nanoseconds: elapsed_nanos as u32,
        }
    }

    /// Reset the timer
    pub fn reset(&self) {
        self.start_time.set(SigmaTimestamp::now());
        self.elapsed.set(SigmaDuration::new(0, 0));
    }
}

impl Default for SigmaTimestamp {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Default for SigmaDuration {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Default for SigmaTimer {
    fn default() -> Self {
        Self::new()
    }
}
