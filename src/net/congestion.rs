// Congestion Control Algorithms
// Inspired by Linux TCP congestion control (BBR, CUBIC, Reno, etc.)

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Congestion control algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionControlType {
    Reno,
    Cubic,
    Bbr,
    Htcp,
    Vegas,
}

/// Congestion state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionState {
    Open,
    Disorder,
    Cwr,
    Recovery,
    Loss,
}

/// Congestion window
#[derive(Debug, Clone)]
pub struct CongestionWindow {
    pub cwnd: u32,     // Congestion window (bytes)
    pub ssthresh: u32, // Slow start threshold
    pub min_cwnd: u32, // Minimum congestion window
    pub max_cwnd: u32, // Maximum congestion window
}

impl CongestionWindow {
    pub fn new() -> Self {
        Self {
            cwnd: 10 * 1460, // Initial cwnd (10 MSS)
            ssthresh: u32::MAX,
            min_cwnd: 2 * 1460,
            max_cwnd: u32::MAX,
        }
    }

    /// Increase congestion window
    pub fn increase(&mut self, acked_bytes: u32) {
        self.cwnd = (self.cwnd + acked_bytes).min(self.max_cwnd);
    }

    /// Decrease congestion window on loss
    pub fn decrease(&mut self) {
        self.ssthresh = self.cwnd / 2;
        self.cwnd = self.ssthresh.max(self.min_cwnd);
    }

    /// Slow start
    pub fn slow_start(&mut self, acked_bytes: u32) {
        if self.cwnd < self.ssthresh {
            self.cwnd += acked_bytes;
        }
    }
}

/// Reno congestion control
pub struct RenoCongestionControl {
    pub cwnd: CongestionWindow,
    pub state: CongestionState,
    pub dup_acks: u32,
    pub next_seq: u32,
}

impl RenoCongestionControl {
    pub fn new() -> Self {
        Self {
            cwnd: CongestionWindow::new(),
            state: CongestionState::Open,
            dup_acks: 0,
            next_seq: 0,
        }
    }

    /// On ACK received
    pub fn on_ack(&mut self, ack_seq: u32, acked_bytes: u32) {
        if ack_seq >= self.next_seq {
            // New ACK
            self.next_seq = ack_seq + 1;
            self.dup_acks = 0;

            match self.state {
                CongestionState::Open => {
                    if self.cwnd.cwnd < self.cwnd.ssthresh {
                        self.cwnd.slow_start(acked_bytes);
                    } else {
                        self.cwnd
                            .increase(acked_bytes * acked_bytes / self.cwnd.cwnd);
                    }
                }
                CongestionState::Recovery => {
                    self.state = CongestionState::Open;
                }
                _ => {}
            }
        } else {
            // Duplicate ACK
            self.dup_acks += 1;

            if self.dup_acks == 3 {
                // Triple duplicate ACK
                self.cwnd.decrease();
                self.state = CongestionState::Recovery;
            }
        }
    }

    /// On timeout
    pub fn on_timeout(&mut self) {
        self.cwnd.decrease();
        self.state = CongestionState::Loss;
        self.dup_acks = 0;
    }

    /// Get current cwnd
    pub fn cwnd(&self) -> u32 {
        self.cwnd.cwnd
    }
}

/// CUBIC congestion control
pub struct CubicCongestionControl {
    pub cwnd: CongestionWindow,
    pub state: CongestionState,
    pub w_last_max: u32,
    pub epoch_start: u64,
    pub origin_point: u32,
    pub c: f64, // CUBIC parameter
}

impl CubicCongestionControl {
    pub fn new() -> Self {
        Self {
            cwnd: CongestionWindow::new(),
            state: CongestionState::Open,
            w_last_max: 0,
            epoch_start: 0,
            origin_point: 0,
            c: 0.4,
        }
    }

    /// Calculate CUBIC window
    fn cubic_cwnd(&self, time_since_epoch: u64) -> u32 {
        let t = time_since_epoch as f64 / 1000.0; // Convert to seconds
        let delta = (self.c * t.powi(3)).powf(1.0 / 3.0);
        (self.origin_point as f64 + delta) as u32
    }

    /// On ACK received
    pub fn on_ack(&mut self, ack_seq: u32, acked_bytes: u32, current_time: u64) {
        if self.state == CongestionState::Open {
            let time_since_epoch = current_time - self.epoch_start;
            let target_cwnd = self.cubic_cwnd(time_since_epoch);

            if self.cwnd.cwnd < target_cwnd {
                self.cwnd.increase(acked_bytes);
            }
        }
    }

    /// On congestion event
    pub fn on_congestion(&mut self, current_time: u64) {
        self.w_last_max = self.cwnd.cwnd;
        self.origin_point = self.cwnd.cwnd;
        self.epoch_start = current_time;
        self.cwnd.decrease();
        self.state = CongestionState::Recovery;
    }

    /// Get current cwnd
    pub fn cwnd(&self) -> u32 {
        self.cwnd.cwnd
    }
}

/// BBR congestion control
pub struct BbrCongestionControl {
    pub cwnd: CongestionWindow,
    pub state: CongestionState,
    pub min_rtt: u32,
    pub max_bw: u64,
    pub bw: u64,
    pub rtt: u32,
    pub pacing_rate: u64,
    pub send_quantum: u32,
}

impl BbrCongestionControl {
    pub fn new() -> Self {
        Self {
            cwnd: CongestionWindow::new(),
            state: CongestionState::Open,
            min_rtt: u32::MAX,
            max_bw: 0,
            bw: 0,
            rtt: 0,
            pacing_rate: 0,
            send_quantum: 10 * 1460,
        }
    }

    /// Update bandwidth estimate
    pub fn update_bw(&mut self, bytes_acked: u64, elapsed_ms: u32) {
        if elapsed_ms > 0 {
            self.bw = bytes_acked * 1000 / elapsed_ms as u64;
            self.max_bw = self.max_bw.max(self.bw);
        }
    }

    /// Update RTT estimate
    pub fn update_rtt(&mut self, rtt_sample: u32) {
        self.rtt = rtt_sample;
        self.min_rtt = self.min_rtt.min(rtt_sample);
    }

    /// Calculate pacing rate
    pub fn update_pacing_rate(&mut self) {
        // BBR uses BDP * gain
        let bdp = (self.max_bw * self.min_rtt as u64) / 1000;
        self.pacing_rate = bdp * 3 / 2; // 1.5x gain
    }

    /// On ACK received
    pub fn on_ack(&mut self, bytes_acked: u64, rtt_sample: u32) {
        self.update_rtt(rtt_sample);
        self.update_pacing_rate();

        // BBR adjusts cwnd based on BDP
        let bdp = (self.max_bw * self.min_rtt as u64) / 1000;
        self.cwnd.cwnd = bdp as u32 * 2; // 2x BDP
    }

    /// Get current cwnd
    pub fn cwnd(&self) -> u32 {
        self.cwnd.cwnd
    }

    /// Get pacing rate
    pub fn pacing_rate(&self) -> u64 {
        self.pacing_rate
    }
}

/// Congestion control manager
pub struct CongestionControlManager {
    pub algorithm: CongestionControlType,
    pub reno: Option<RenoCongestionControl>,
    pub cubic: Option<CubicCongestionControl>,
    pub bbr: Option<BbrCongestionControl>,
}

impl CongestionControlManager {
    pub fn new(algorithm: CongestionControlType) -> Self {
        Self {
            algorithm,
            reno: None,
            cubic: None,
            bbr: None,
        }
    }

    /// Initialize the selected algorithm
    pub fn init(&mut self) {
        match self.algorithm {
            CongestionControlType::Reno => {
                self.reno = Some(RenoCongestionControl::new());
            }
            CongestionControlType::Cubic => {
                self.cubic = Some(CubicCongestionControl::new());
            }
            CongestionControlType::Bbr => {
                self.bbr = Some(BbrCongestionControl::new());
            }
            _ => {
                self.reno = Some(RenoCongestionControl::new());
            }
        }
    }

    /// Get current cwnd
    pub fn cwnd(&self) -> u32 {
        match self.algorithm {
            CongestionControlType::Reno => self.reno.as_ref().map(|r| r.cwnd()).unwrap_or(0),
            CongestionControlType::Cubic => self.cubic.as_ref().map(|c| c.cwnd()).unwrap_or(0),
            CongestionControlType::Bbr => self.bbr.as_ref().map(|b| b.cwnd()).unwrap_or(0),
            _ => self.reno.as_ref().map(|r| r.cwnd()).unwrap_or(0),
        }
    }

    /// Switch algorithm
    pub fn switch_algorithm(&mut self, new_algorithm: CongestionControlType) {
        self.algorithm = new_algorithm;
        self.init();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_congestion_window() {
        let mut cwnd = CongestionWindow::new();

        cwnd.increase(1460);
        assert!(cwnd.cwnd > 10 * 1460);

        cwnd.decrease();
        assert!(cwnd.cwnd < cwnd.ssthresh);
    }

    #[test]
    fn test_reno_congestion() {
        let mut reno = RenoCongestionControl::new();

        reno.on_ack(1, 1460);
        assert_eq!(reno.state, CongestionState::Open);

        reno.on_timeout();
        assert_eq!(reno.state, CongestionState::Loss);
    }

    #[test]
    fn test_cubic_congestion() {
        let mut cubic = CubicCongestionControl::new();

        cubic.on_congestion(0);
        assert_eq!(cubic.state, CongestionState::Recovery);
    }

    #[test]
    fn test_bbr_congestion() {
        let mut bbr = BbrCongestionControl::new();

        bbr.update_bw(10000, 100);
        bbr.update_rtt(50000);
        bbr.update_pacing_rate();

        assert!(bbr.pacing_rate > 0);
    }

    #[test]
    fn test_congestion_manager() {
        let mut manager = CongestionControlManager::new(CongestionControlType::Reno);
        manager.init();

        assert!(manager.cwnd() > 0);

        manager.switch_algorithm(CongestionControlType::Bbr);
        assert_eq!(manager.algorithm, CongestionControlType::Bbr);
    }
}
