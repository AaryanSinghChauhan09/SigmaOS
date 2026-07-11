// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_bbr.rs — BBR Congestion Control Implementation
//
// BBR (Bottleneck Bandwidth and RTT) is a modern congestion control algorithm
// that uses measurements of bottleneck bandwidth and RTT to control the sending rate,
// rather than using packet loss as a signal like traditional TCP congestion control.
// This implementation follows the Linux BBRv2 design with OOP principles.
//
// Key features:
// - Model-based congestion control using bandwidth and RTT measurements
// - Four states: Startup, Drain, ProbeBW, ProbeRTT
// - Pacing to control sending rate
// - No external dependencies, pure Rust implementation
// - OOP-style traits for extensibility

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Constants (Linux BBR values)
// ─────────────────────────────────────────────────────────────────────────────

pub const BBR_INIT_CWND: u32 = 10; // Initial congestion window (packets)
pub const BBR_MIN_RTT_FILTER_LEN: u32 = 200; // Min RTT filter window (ms)
pub const BBR_PROBE_RTT_DURATION: u32 = 200; // ProbeRTT state duration (ms)
pub const BBR_LOSS_THRESH: u32 = 2; // Loss threshold (packets)
pub const BBR_GAIN_CYCLE_LEN: usize = 8; // Gain cycle length
pub const BBR_PACING_RATE: u64 = 1_000_000; // Default pacing rate (bytes/sec)
pub const BBR_HIGH_GAIN: u64 = 285; // 2.85x gain (285/100)
pub const BBR_DRAIN_GAIN: u64 = 71; // 0.71x gain (71/100)
pub const BBR_UNIT_GAIN: u64 = 100; // 1.0x gain (100/100)
pub const BBR_BETA: u64 = 70; // 0.7x for drain (70/100)

// ─────────────────────────────────────────────────────────────────────────────
// BBR States
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BbrState {
    Startup,    // Rapidly increase sending rate to find pipe capacity
    Drain,      // Drain any queue created during Startup
    ProbeBW,    // Probe bandwidth by cycling gains
    ProbeRTT,   // Probe min RTT
}

// ─────────────────────────────────────────────────────────────────────────────
// BBR Delivery Rate Sample
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct BbrRateSample {
    pub delivered: u64,     // Total delivered bytes
    pub delivered_time: u64, // Time when delivered
    pub prior_delivered: u64, // Delivered bytes at prior sample
    pub prior_time: u64,    // Time at prior sample
    pub is_app_limited: bool, // Is application limited
    pub send_elapsed: u64,  // Time elapsed since send
    pub ack_elapsed: u64,   // Time elapsed since ack
    pub losses: u32,        // Number of losses
}

impl BbrRateSample {
    pub const fn empty() -> Self {
        Self {
            delivered: 0,
            delivered_time: 0,
            prior_delivered: 0,
            prior_time: 0,
            is_app_limited: false,
            send_elapsed: 0,
            ack_elapsed: 0,
            losses: 0,
        }
    }

    // Calculate bandwidth from rate sample
    pub fn bandwidth(&self) -> u64 {
        if self.send_elapsed == 0 { return 0; }
        let delivered = self.delivered.saturating_sub(self.prior_delivered);
        let time = self.send_elapsed;
        delivered * 1_000_000_000 / time // bytes per second
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BBR Round Trip Time Sample
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct BbrRttSample {
    pub rtt: u64,           // Measured RTT (nanoseconds)
    pub rtt_var: u64,      // RTT variance
    pub min_rtt: u64,      // Minimum RTT observed
    pub min_rtt_stamp: u64, // Timestamp of min RTT
}

impl BbrRttSample {
    pub const fn empty() -> Self {
        Self {
            rtt: 0,
            rtt_var: 0,
            min_rtt: u64::MAX,
            min_rtt_stamp: 0,
        }
    }

    // Update RTT sample with new measurement
    pub fn update(&mut self, rtt: u64, now: u64) {
        self.rtt = rtt;
        
        // Update min RTT
        if rtt < self.min_rtt {
            self.min_rtt = rtt;
            self.min_rtt_stamp = now;
        }
        
        // Update RTT variance (exponential moving average)
        if self.rtt_var == 0 {
            self.rtt_var = rtt / 2;
        } else {
            self.rtt_var = (3 * self.rtt_var + rtt) / 4;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BBR Bandwidth Sample
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct BbrBandwidthSample {
    pub bandwidth: u64,     // Current bandwidth (bytes/sec)
    pub max_bandwidth: u64, // Maximum bandwidth observed
    pub bw_stamp: u64,      // Timestamp of max bandwidth
}

impl BbrBandwidthSample {
    pub const fn empty() -> Self {
        Self {
            bandwidth: 0,
            max_bandwidth: 0,
            bw_stamp: 0,
        }
    }

    // Update bandwidth sample with new measurement
    pub fn update(&mut self, bandwidth: u64, now: u64) {
        self.bandwidth = bandwidth;
        
        // Update max bandwidth
        if bandwidth > self.max_bandwidth {
            self.max_bandwidth = bandwidth;
            self.bw_stamp = now;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BBR Congestion Control with OOP principles
// ─────────────────────────────────────────────────────────────────────────────

pub struct BbrCongestionControl {
    state: BbrState,
    cwnd: u32,              // Congestion window (packets)
    min_rtt: u64,           // Minimum RTT (nanoseconds)
    bw: u64,                // Current bandwidth (bytes/sec)
    max_bw: u64,            // Maximum bandwidth (bytes/sec)
    pacing_rate: u64,       // Pacing rate (bytes/sec)
    send_quantum: u32,      // Send quantum (bytes)
    round_count: u32,       // Round count
    next_round_delivered: u64, // Delivered bytes for next round
    cycle_idx: usize,       // Current cycle index
    cycle_gain: u64,        // Current cycle gain
    has_seen_rtt: bool,     // Has seen valid RTT
    probe_rtt_done: bool,   // ProbeRTT state done
    packet_conservation: bool, // Packet conservation mode
    loss_in_round: u32,     // Losses in current round
    delivered: u64,        // Total delivered bytes
    delivered_time: u64,   // Time of last delivery
    prior_delivered: u64,   // Prior delivered bytes
    prior_time: u64,        // Prior time
    app_limited: bool,     // Application limited
    app_limited_start: u64, // Start of app-limited period
}

impl BbrCongestionControl {
    pub const fn new() -> Self {
        Self {
            state: BbrState::Startup,
            cwnd: BBR_INIT_CWND,
            min_rtt: u64::MAX,
            bw: 0,
            max_bw: 0,
            pacing_rate: BBR_PACING_RATE,
            send_quantum: 0,
            round_count: 0,
            next_round_delivered: 0,
            cycle_idx: 0,
            cycle_gain: BBR_HIGH_GAIN,
            has_seen_rtt: false,
            probe_rtt_done: false,
            packet_conservation: false,
            loss_in_round: 0,
            delivered: 0,
            delivered_time: 0,
            prior_delivered: 0,
            prior_time: 0,
            app_limited: false,
            app_limited_start: 0,
        }
    }

    // Initialize BBR
    pub fn init(&mut self) {
        self.state = BbrState::Startup;
        self.cwnd = BBR_INIT_CWND;
        self.min_rtt = u64::MAX;
        self.bw = 0;
        self.max_bw = 0;
        self.pacing_rate = BBR_PACING_RATE;
        self.round_count = 0;
        self.next_round_delivered = 0;
        self.cycle_idx = 0;
        self.cycle_gain = BBR_HIGH_GAIN;
        self.has_seen_rtt = false;
        self.probe_rtt_done = false;
        self.packet_conservation = false;
        self.loss_in_round = 0;
        self.delivered = 0;
        self.delivered_time = 0;
        self.prior_delivered = 0;
        self.prior_time = 0;
        self.app_limited = false;
        self.app_limited_start = 0;
    }

    // Update BBR state on ACK
    pub fn on_ack(&mut self, sample: BbrRateSample, rtt: u64, now: u64) {
        // Update delivery rate
        self.delivered = sample.delivered;
        self.delivered_time = sample.delivered_time;
        
        // Check for round completion
        if sample.delivered >= self.next_round_delivered {
            self.next_round_delivered = self.delivered;
            self.round_count += 1;
            self.loss_in_round = 0;
        }
        
        // Track losses
        self.loss_in_round += sample.losses;
        
        // Update RTT
        if rtt > 0 {
            if !self.has_seen_rtt {
                self.has_seen_rtt = true;
            }
            if rtt < self.min_rtt {
                self.min_rtt = rtt;
            }
        }
        
        // Update bandwidth
        let bandwidth = sample.bandwidth();
        if bandwidth > 0 {
            self.bw = bandwidth;
            if bandwidth > self.max_bw {
                self.max_bw = bandwidth;
            }
        }
        
        // Update state machine
        self.update_state(now);
        
        // Update pacing rate
        self.update_pacing();
        
        // Update congestion window
        self.update_cwnd();
    }

    // Update BBR state machine
    fn update_state(&mut self, now: u64) {
        match self.state {
            BbrState::Startup => {
                // Check if we should exit Startup
                if self.round_count >= 3 && self.loss_in_round >= BBR_LOSS_THRESH {
                    self.state = BbrState::Drain;
                    self.cycle_gain = BBR_DRAIN_GAIN;
                }
            }
            BbrState::Drain => {
                // Check if we should exit Drain
                if self.delivered >= self.prior_delivered + self.cwnd as u64 {
                    self.state = BbrState::ProbeBW;
                    self.cycle_idx = 0;
                    self.cycle_gain = BBR_UNIT_GAIN;
                }
            }
            BbrState::ProbeBW => {
                // Cycle through gains
                self.cycle_idx = (self.cycle_idx + 1) % BBR_GAIN_CYCLE_LEN;
                
                // Gain cycle: [1.25, 0.75, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
                let gains = [125, 75, 100, 100, 100, 100, 100, 100];
                self.cycle_gain = gains[self.cycle_idx] as u64;
                
                // Check if we should enter ProbeRTT
                if now - self.delivered_time > BBR_MIN_RTT_FILTER_LEN as u64 * 1_000_000 {
                    self.state = BbrState::ProbeRTT;
                    self.probe_rtt_done = false;
                }
            }
            BbrState::ProbeRTT => {
                // Stay in ProbeRTT for at least BBR_PROBE_RTT_DURATION
                if !self.probe_rtt_done {
                    if now - self.delivered_time >= BBR_PROBE_RTT_DURATION as u64 * 1_000_000 {
                        self.probe_rtt_done = true;
                    }
                } else {
                    // Exit ProbeRTT
                    self.state = BbrState::ProbeBW;
                    self.cycle_idx = 0;
                    self.cycle_gain = BBR_UNIT_GAIN;
                    self.min_rtt = u64::MAX; // Reset min RTT
                }
            }
        }
    }

    // Update pacing rate
    fn update_pacing(&mut self) {
        let gain = match self.state {
            BbrState::Startup => BBR_HIGH_GAIN,
            BbrState::Drain => BBR_DRAIN_GAIN,
            BbrState::ProbeBW => self.cycle_gain,
            BbrState::ProbeRTT => BBR_UNIT_GAIN,
        };
        
        let target_rate = if self.max_bw > 0 {
            self.max_bw * gain / 100
        } else {
            BBR_PACING_RATE
        };
        
        self.pacing_rate = target_rate;
        
        // Update send quantum (typically 1 MSS or pacing_rate/100)
        self.send_quantum = (self.pacing_rate / 100) as u32;
        self.send_quantum = self.send_quantum.max(1448); // Minimum 1 MSS
    }

    // Update congestion window
    fn update_cwnd(&mut self) {
        match self.state {
            BbrState::Startup => {
                // Grow cwnd exponentially in Startup
                self.cwnd = self.cwnd.saturating_add(self.cwnd);
            }
            BbrState::Drain => {
                // Drain queue
                self.cwnd = self.cwnd * BBR_BETA / 100;
            }
            BbrState::ProbeBW => {
                // Use BDP-based cwnd
                if self.max_bw > 0 && self.min_rtt < u64::MAX {
                    let bdp = self.max_bw * self.min_rtt / 1_000_000_000;
                    let bdp_packets = (bdp / 1448) as u32; // Assume 1448-byte MSS
                    self.cwnd = bdp_packets.max(BBR_INIT_CWND);
                }
            }
            BbrState::ProbeRTT => {
                // Reduce cwnd to 4 packets in ProbeRTT
                self.cwnd = 4;
            }
        }
    }

    // Get current congestion window
    pub fn get_cwnd(&self) -> u32 {
        self.cwnd
    }

    // Get current pacing rate
    pub fn get_pacing_rate(&self) -> u64 {
        self.pacing_rate
    }

    // Get current state
    pub fn get_state(&self) -> BbrState {
        self.state
    }

    // Get minimum RTT
    pub fn get_min_rtt(&self) -> u64 {
        self.min_rtt
    }

    // Get maximum bandwidth
    pub fn get_max_bandwidth(&self) -> u64 {
        self.max_bw
    }

    // Set application limited
    pub fn set_app_limited(&mut self, limited: bool, now: u64) {
        self.app_limited = limited;
        if limited {
            self.app_limited_start = now;
        }
    }

    // Check if application limited
    pub fn is_app_limited(&self) -> bool {
        self.app_limited
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton per connection (simplified - in real impl, per-connection)
// ─────────────────────────────────────────────────────────────────────────────

static mut BBR_CC: BbrCongestionControl = BbrCongestionControl::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_init() {
    BBR_CC.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_on_ack(
    delivered: u64,
    delivered_time: u64,
    prior_delivered: u64,
    prior_time: u64,
    send_elapsed: u64,
    ack_elapsed: u64,
    losses: u32,
    rtt: u64,
    now: u64,
) {
    let sample = BbrRateSample {
        delivered,
        delivered_time,
        prior_delivered,
        prior_time,
        is_app_limited: BBR_CC.is_app_limited(),
        send_elapsed,
        ack_elapsed,
        losses,
    };
    BBR_CC.on_ack(sample, rtt, now);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_get_cwnd() -> u32 {
    BBR_CC.get_cwnd()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_get_pacing_rate() -> u64 {
    BBR_CC.get_pacing_rate()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_get_state() -> u8 {
    match BBR_CC.get_state() {
        BbrState::Startup => 0,
        BbrState::Drain => 1,
        BbrState::ProbeBW => 2,
        BbrState::ProbeRTT => 3,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_get_min_rtt() -> u64 {
    BBR_CC.get_min_rtt()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_get_max_bandwidth() -> u64 {
    BBR_CC.get_max_bandwidth()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_set_app_limited(limited: bool, now: u64) {
    BBR_CC.set_app_limited(limited, now);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bbr_is_app_limited() -> bool {
    BBR_CC.is_app_limited()
}
