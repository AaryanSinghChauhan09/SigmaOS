#![no_std]
#![allow(dead_code)]

/// SigmaOS ML Anomaly Detector
/// Implements a lightweight Isolation Forest algorithm stub to detect
/// anomalous behavior in system telemetry.

use core::sync::atomic::{AtomicU32, Ordering};

/// Telemetry vector for an observation
#[derive(Copy, Clone, Default)]
pub struct TelemetryVector {
    pub cpu_usage: u32,
    pub memory_usage: u32,
    pub io_reads: u32,
    pub io_writes: u32,
    pub network_tx: u32,
    pub network_rx: u32,
}

pub struct IsolationForestDetector {
    enabled: AtomicU32,
    anomaly_threshold: u32, // Lower score = more anomalous in this simplified stub
}

impl IsolationForestDetector {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicU32::new(1),
            anomaly_threshold: 30, // 0-100 normalized score
        }
    }

    /// Evaluates a telemetry vector. Returns true if it represents an anomaly.
    /// In a real isolation forest, this calculates the path length down multiple random trees.
    /// Here, we use a heuristic stub optimized for no_std integers.
    pub fn is_anomalous(&self, vec: &TelemetryVector) -> bool {
        if self.enabled.load(Ordering::Relaxed) == 0 {
            return false;
        }

        // Extremely simplified heuristic for demonstration:
        // Anomalies usually involve sudden, massive spikes in specific metrics
        // while others remain zero, or extreme values across the board.
        
        let mut score = 100; // 100 is perfectly normal
        
        // Spike in CPU without IO or Network might be a tight infinite loop
        if vec.cpu_usage > 95 && vec.io_reads == 0 && vec.network_tx == 0 {
            score -= 40;
        }
        
        // Massive IO writes with low CPU might be ransomware encrypting files
        if vec.io_writes > 10000 && vec.cpu_usage < 10 {
            score -= 50;
        }
        
        // Huge network TX without matching RX could be data exfiltration
        if vec.network_tx > 50000 && vec.network_rx < 100 {
            score -= 60;
        }
        
        // A single process maxing out memory
        if vec.memory_usage > 90 {
            score -= 30;
        }
        
        // Bound the score
        if score < 0 { score = 0; }

        score <= self.anomaly_threshold
    }
    
    pub fn set_threshold(&mut self, threshold: u32) {
        self.anomaly_threshold = threshold;
    }
}

static mut G_ANOMALY_DETECTOR: IsolationForestDetector = IsolationForestDetector::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_anomaly_detector_init() {
    G_ANOMALY_DETECTOR.enabled.store(1, Ordering::Relaxed);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_anomaly_detector_check(vec_ptr: *const TelemetryVector) -> u32 {
    if vec_ptr.is_null() {
        return 0;
    }
    
    if G_ANOMALY_DETECTOR.is_anomalous(&*vec_ptr) {
        1
    } else {
        0
    }
}
