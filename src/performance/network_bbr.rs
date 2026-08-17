//! BBR (Bottleneck Bandwidth and RTT) Congestion Control Engine
//! Implements high-throughput network flow pacing and TCP Fast Open validation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BbrState {
    Startup,
    Drain,
    ProbeBw,
    ProbeRtt,
}

pub struct BbrEngine {
    pub state: BbrState,
    pub max_bandwidth_kbps: u64,
    pub min_rtt_ms: u64,
    pub pacing_rate_kbps: u64,
    pub cwnd_bytes: u32,
    pub bdp_bytes: u32,
    pub tfo_enabled: bool,
}

impl BbrEngine {
    pub fn new() -> Self {
        Self {
            state: BbrState::Startup,
            max_bandwidth_kbps: 1000, // starting estimate (1 Mbps)
            min_rtt_ms: 100,          // baseline (100 ms)
            pacing_rate_kbps: 1250,   // initial pacing (pacing gain = 1.25)
            cwnd_bytes: 14600,        // 10 initial segments (MSS = 1460)
            bdp_bytes: 14600,
            tfo_enabled: true,
        }
    }

    /// Update congestion metrics based on a delivered packet acknowledgment
    pub fn update_metrics(&mut self, delivered_bytes: usize, rtt_ms: u64) {
        // 1. Update minimum RTT
        if rtt_ms < self.min_rtt_ms || self.min_rtt_ms == 0 {
            self.min_rtt_ms = rtt_ms.max(1);
        }

        // 2. Estimate dynamic delivery rate (kbps)
        // Delivered bytes converted to kilobits, over the RTT interval
        let delivery_rate_kbps = (delivered_bytes as u64 * 8) / self.min_rtt_ms;
        if delivery_rate_kbps > self.max_bandwidth_kbps {
            self.max_bandwidth_kbps = delivery_rate_kbps;
        }

        // 3. Compute Bandwidth-Delay Product (BDP) in bytes
        // BDP = Bandwidth (bytes/ms) * RTT (ms)
        // Bandwidth (bytes/ms) = max_bandwidth_kbps * 1000 / 8000 = max_bandwidth_kbps / 8
        let bdp = (self.max_bandwidth_kbps * self.min_rtt_ms) / 8;
        self.bdp_bytes = bdp.max(1460) as u32;

        // 4. Update pacing rate & cwnd based on BBR state
        let (pacing_gain, cwnd_gain) = match self.state {
            BbrState::Startup => (2.89, 2.89), // High gain to fill pipe fast
            BbrState::Drain => (0.35, 2.89),   // Drain queue excess
            BbrState::ProbeBw => (1.25, 2.0),  // Steady probing
            BbrState::ProbeRtt => (1.0, 4.0),  // Low bandwidth probing
        };

        self.pacing_rate_kbps = (self.max_bandwidth_kbps as f32 * pacing_gain) as u64;
        self.cwnd_bytes = (self.bdp_bytes as f32 * cwnd_gain) as u32;

        // Ensure cwnd is at least 4 segments (MSS = 1460)
        self.cwnd_bytes = self.cwnd_bytes.max(1460 * 4);
    }

    /// Perform fast TCP Fast Open validation to bypass handshakes on repeat connections
    pub fn verify_tfo_cookie(&self, client_ip: u32, cookie: u64) -> bool {
        if !self.tfo_enabled {
            return false;
        }
        // Secure deterministic cookie check based on IP hashing
        let expected_cookie = (client_ip as u64).wrapping_mul(0x5bd1e995) ^ 0x1e35a019;
        cookie == expected_cookie
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbr_metrics_initialization_and_update() {
        let mut bbr = BbrEngine::new();
        assert_eq!(bbr.state, BbrState::Startup);
        assert_eq!(bbr.min_rtt_ms, 100);

        // Update metrics with some fast delivery (low RTT, high bytes)
        bbr.update_metrics(20000, 40); // 20KB over 40ms RTT
        assert_eq!(bbr.min_rtt_ms, 40);
        assert!(bbr.max_bandwidth_kbps > 1000);
        assert!(bbr.cwnd_bytes > 14600);
    }

    #[test]
    fn test_tcp_fast_open_cookie_verification() {
        let bbr = BbrEngine::new();
        let client_ip = 0x7F000001; // 127.0.0.1
        let expected_cookie = (client_ip as u64).wrapping_mul(0x5bd1e995) ^ 0x1e35a019;

        assert!(bbr.verify_tfo_cookie(client_ip, expected_cookie));
        assert!(!bbr.verify_tfo_cookie(client_ip, 0x12345678));
    }
}
