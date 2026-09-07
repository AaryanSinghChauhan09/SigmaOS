//! Adaptive Storage I/O Scheduler Subsystem
//! Implements hardware-aware scheduler selection (Kyber, BFQ, Deadline) and dynamic read-ahead heuristics.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    RotationalHdd,
    Ssd,
    Nvme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOSchedulerPolicy {
    Deadline, // Balanced HDD scheduling
    Bfq,      // Responsive desktop storage queueing
    Kyber,    // Highly scalable multi-queue NVMe scheduling
    None,     // Zero overhead direct routing for low-latency SSDs
}

#[derive(Debug, Clone, Copy)]
pub struct IORequest {
    pub sector: u64,
    pub length_sectors: u32,
    pub is_write: bool,
    pub timestamp_ms: u64,
}

pub struct AdaptiveIOScheduler {
    pub device_type: DeviceType,
    pub active_policy: IOSchedulerPolicy,
    pub read_ahead_kb: u32,
    pub last_sector: u64,
    pub sequential_strike_count: u32,
}

impl AdaptiveIOScheduler {
    pub fn new(device_type: DeviceType) -> Self {
        let mut scheduler = Self {
            device_type,
            active_policy: IOSchedulerPolicy::None,
            read_ahead_kb: 128, // baseline read-ahead
            last_sector: 0,
            sequential_strike_count: 0,
        };
        scheduler.select_optimal_policy();
        scheduler
    }

    /// Automatically select the optimal policy based on hardware attributes
    pub fn select_optimal_policy(&mut self) {
        self.active_policy = match self.device_type {
            DeviceType::RotationalHdd => IOSchedulerPolicy::Bfq,
            DeviceType::Ssd => IOSchedulerPolicy::Deadline,
            DeviceType::Nvme => IOSchedulerPolicy::Kyber,
        };
    }

    /// Record an incoming I/O request to dynamically scale read-ahead window
    pub fn record_io_request(&mut self, req: &IORequest) {
        // Detect sequential access patterns: current sector starts where last sector ended
        let expected_next_sector = self.last_sector;

        if req.sector >= expected_next_sector && req.sector <= expected_next_sector + 32 {
            self.sequential_strike_count += 1;
            // Progressive read-ahead acceleration up to 512KB
            if self.sequential_strike_count >= 3 {
                self.read_ahead_kb = (self.read_ahead_kb * 2).min(512);
            }
        } else {
            // Random access detected: back off to minimize bus/cache pollution
            self.sequential_strike_count = 0;
            self.read_ahead_kb = 128;
        }

        self.last_sector = req.sector + req.length_sectors as u64;
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_policy_selection() {
        let hdd_sched = AdaptiveIOScheduler::new(DeviceType::RotationalHdd);
        assert_eq!(hdd_sched.active_policy, IOSchedulerPolicy::Bfq);

        let nvme_sched = AdaptiveIOScheduler::new(DeviceType::Nvme);
        assert_eq!(nvme_sched.active_policy, IOSchedulerPolicy::Kyber);
    }

    #[test]
    fn test_dynamic_read_ahead_scaling() {
        let mut sched = AdaptiveIOScheduler::new(DeviceType::Ssd);
        assert_eq!(sched.read_ahead_kb, 128);

        // Submit sequential reads
        let req1 = IORequest {
            sector: 0,
            length_sectors: 8,
            is_write: false,
            timestamp_ms: 100,
        };
        sched.record_io_request(&req1); // last_sector becomes 8

        let req2 = IORequest {
            sector: 8,
            length_sectors: 8,
            is_write: false,
            timestamp_ms: 105,
        };
        sched.record_io_request(&req2); // last_sector becomes 16

        let req3 = IORequest {
            sector: 16,
            length_sectors: 8,
            is_write: false,
            timestamp_ms: 110,
        };
        sched.record_io_request(&req3); // sequential strike!

        let req4 = IORequest {
            sector: 24,
            length_sectors: 8,
            is_write: false,
            timestamp_ms: 115,
        };
        sched.record_io_request(&req4); // sequential strike 2!

        assert!(sched.read_ahead_kb > 128);

        // Submit random read
        let random_req = IORequest {
            sector: 5000,
            length_sectors: 8,
            is_write: false,
            timestamp_ms: 120,
        };
        sched.record_io_request(&random_req);
        assert_eq!(sched.read_ahead_kb, 128); // backing off
    }
}
