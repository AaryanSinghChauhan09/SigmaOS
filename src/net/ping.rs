extern crate alloc;
// SigmaOS Network Protocol Layer
//! ICMP Ping (iputils-ping replication) Engine for SigmaOS
//! Provides robust ICMP packet headers, internet standard 1s complement checksumming,
//! ping statistics reporting, packet loss calculations, duplicate packet (DUP!) detection,
//! and standard min/avg/max/mdev RTT tracking in #![no_std].

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
    EchoReply = 0,
    DestinationUnreachable = 3,
    SourceQuench = 4,
    Redirect = 5,
    EchoRequest = 8,
    TimeExceeded = 11,
}

#[derive(Debug, Clone)]
pub struct IcmpPacket {
    pub icmp_type: IcmpType,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PingStats {
    pub transmitted: usize,
    pub received: usize,
    pub min_rtt_ms: f64,
    pub max_rtt_ms: f64,
    pub avg_rtt_ms: f64,
    pub mdev_rtt_ms: f64,
    pub latencies: Vec<f64>,
}

impl PingStats {
    pub fn new() -> Self {
        Self {
            transmitted: 0,
            received: 0,
            min_rtt_ms: 0.0,
            max_rtt_ms: 0.0,
            avg_rtt_ms: 0.0,
            mdev_rtt_ms: 0.0,
            latencies: Vec::new(),
        }
    }
}

impl Default for PingStats {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignPingEngine {
    pub interval_ms: u64,
    pub ttl: u8,
    pub count: usize,
    pub payload_size: usize,
    pub flood: bool,
    pub deadline_ms: u64,
    pub stats: PingStats,
    pub seen_sequences: Vec<u16>, // tracks sequences to detect DUP! packets
}

impl SovereignPingEngine {
    pub fn new() -> Self {
        Self {
            interval_ms: 1000,
            ttl: 64,
            count: 5,
            payload_size: 56,
            flood: false,
            deadline_ms: 5000,
            stats: PingStats::new(),
            seen_sequences: Vec::new(),
        }
    }

    pub fn create_echo_request(&self, seq: u16) -> IcmpPacket {
        let mut payload = Vec::new();
        for i in 0..self.payload_size {
            payload.push((i & 0xFF) as u8);
        }

        let mut header = [0u8; 8];
        header[0] = IcmpType::EchoRequest as u8;
        header[1] = 0;
        header[2] = 0;
        header[3] = 0;
        header[4] = 0x12;
        header[5] = 0x34;
        header[6] = (seq >> 8) as u8;
        header[7] = (seq & 0xFF) as u8;

        let checksum = compute_icmp_checksum(&header, &payload);

        IcmpPacket {
            icmp_type: IcmpType::EchoRequest,
            code: 0,
            checksum,
            identifier: 0x1234,
            sequence: seq,
            payload,
        }
    }

    pub fn process_echo_reply(&mut self, reply: &IcmpPacket, rtt_ms: f64) -> Result<bool, &'static str> {
        if reply.icmp_type != IcmpType::EchoReply {
            return Err("Not an ICMP Echo Reply");
        }
        if reply.identifier != 0x1234 {
            return Err("Identifier mismatch");
        }

        self.stats.transmitted += 1;

        // Duplicate Echo Reply detection
        let is_duplicate = self.seen_sequences.contains(&reply.sequence);
        if !is_duplicate {
            self.seen_sequences.push(reply.sequence);
            self.stats.received += 1;
            self.stats.latencies.push(rtt_ms);

            // Update stats
            if self.stats.latencies.len() == 1 {
                self.stats.min_rtt_ms = rtt_ms;
                self.stats.max_rtt_ms = rtt_ms;
                self.stats.avg_rtt_ms = rtt_ms;
                self.stats.mdev_rtt_ms = 0.0;
            } else {
                if rtt_ms < self.stats.min_rtt_ms {
                    self.stats.min_rtt_ms = rtt_ms;
                }
                if rtt_ms > self.stats.max_rtt_ms {
                    self.stats.max_rtt_ms = rtt_ms;
                }

                // Average calculation
                let mut sum = 0.0;
                for i in 0..self.stats.latencies.len() {
                    sum += self.stats.latencies[i];
                }
                let avg = sum / (self.stats.latencies.len() as f64);
                self.stats.avg_rtt_ms = avg;

                // Standard deviation / mdev calculation
                let mut var_sum = 0.0;
                for i in 0..self.stats.latencies.len() {
                    let diff = self.stats.latencies[i] - avg;
                    var_sum += diff * diff;
                }
                let variance = var_sum / (self.stats.latencies.len() as f64);
                self.stats.mdev_rtt_ms = f64_sqrt(variance);
            }
        }

        Ok(is_duplicate)
    }

    pub fn get_summary_report(&self, destination: &str) -> [u8; 128] {
        let mut out = [0u8; 128];
        let prefix = b"--- ping statistics --- ";
        let mut idx = 0;
        out[idx..idx + prefix.len()].copy_from_slice(prefix);
        idx += prefix.len();

        let dst_bytes = destination.as_bytes();
        let dst_len = dst_bytes.len().min(30);
        out[idx..idx + dst_len].copy_from_slice(&dst_bytes[..dst_len]);
        idx += dst_len;

        let tx = self.stats.transmitted;
        let rx = self.stats.received;
        let loss = if tx > 0 { ((tx - rx) * 100) / tx } else { 0 };

        let stat_prefix = b" | tx:";
        if idx + stat_prefix.len() < 128 {
            out[idx..idx + stat_prefix.len()].copy_from_slice(stat_prefix);
            idx += stat_prefix.len();
        }

        idx = write_int(tx, &mut out, idx);

        let rx_prefix = b" rx:";
        if idx + rx_prefix.len() < 128 {
            out[idx..idx + rx_prefix.len()].copy_from_slice(rx_prefix);
            idx += rx_prefix.len();
        }

        idx = write_int(rx, &mut out, idx);

        let loss_prefix = b" loss:";
        if idx + loss_prefix.len() < 128 {
            out[idx..idx + loss_prefix.len()].copy_from_slice(loss_prefix);
            idx += loss_prefix.len();
        }

        idx = write_int(loss, &mut out, idx);
        if idx < 128 {
            out[idx] = b'%';
            idx += 1;
        }

        let min_int = self.stats.min_rtt_ms as usize;
        let avg_int = self.stats.avg_rtt_ms as usize;
        let max_int = self.stats.max_rtt_ms as usize;

        let rtt_prefix = b" rtt:";
        if idx + rtt_prefix.len() < 128 {
            out[idx..idx + rtt_prefix.len()].copy_from_slice(rtt_prefix);
            idx += rtt_prefix.len();
        }

        idx = write_int(min_int, &mut out, idx);
        if idx < 128 { out[idx] = b'/'; idx += 1; }
        idx = write_int(avg_int, &mut out, idx);
        if idx < 128 { out[idx] = b'/'; idx += 1; }
        idx = write_int(max_int, &mut out, idx);
        if idx + 2 < 128 {
            out[idx] = b'm';
            out[idx+1] = b's';
            idx += 2;
        }

        out
    }
}

impl Default for SovereignPingEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub fn compute_icmp_checksum(header: &[u8], payload: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    let mut i = 0;
    while i + 1 < header.len() {
        let word = ((header[i] as u32) << 8) | (header[i+1] as u32);
        sum += word;
        i += 2;
    }
    if i < header.len() {
        sum += (header[i] as u32) << 8;
    }

    let mut j = 0;
    while j + 1 < payload.len() {
        let word = ((payload[j] as u32) << 8) | (payload[j+1] as u32);
        sum += word;
        j += 2;
    }
    if j < payload.len() {
        sum += (payload[j] as u32) << 8;
    }

    while sum >> 16 > 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}

fn write_int(mut val: usize, buf: &mut [u8], mut idx: usize) -> usize {
    if val == 0 {
        if idx < buf.len() {
            buf[idx] = b'0';
            idx += 1;
        }
        return idx;
    }
    let mut digits = [0u8; 12];
    let mut d_idx = 0;
    while val > 0 && d_idx < 12 {
        digits[d_idx] = (val % 10) as u8 + b'0';
        val /= 10;
        d_idx += 1;
    }
    while d_idx > 0 && idx < buf.len() {
        d_idx -= 1;
        buf[idx] = digits[d_idx];
        idx += 1;
    }
    idx
}

fn f64_sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut res = x;
    for _ in 0..10 {
        res = 0.5 * (res + x / res);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_icmp_checksum() {
        let header = [8, 0, 0, 0, 0, 1, 0, 1];
        let payload = [0u8; 32];
        let c1 = compute_icmp_checksum(&header, &payload);
        let mut header_with_checksum = header;
        header_with_checksum[2] = (c1 >> 8) as u8;
        header_with_checksum[3] = (c1 & 0xFF) as u8;
        let c2 = compute_icmp_checksum(&header_with_checksum, &payload);
        // Standard internet checksum of a valid checksummed packet is 0 (or ~0 i.e. 0xFFFF)
        assert_eq!(c2, 0);
    }

    #[test]
    fn test_sovereign_ping_engine_rtt_stats_and_dup() {
        let mut engine = SovereignPingEngine::new();
        assert_eq!(engine.stats.transmitted, 0);

        let req = engine.create_echo_request(1);
        assert_eq!(req.sequence, 1);

        let reply = IcmpPacket {
            icmp_type: IcmpType::EchoReply,
            code: 0,
            checksum: 0,
            identifier: 0x1234,
            sequence: 1,
            payload: req.payload,
        };

        // First reply -> success, not duplicate
        let dup1 = engine.process_echo_reply(&reply, 15.0).unwrap();
        assert!(!dup1);
        assert_eq!(engine.stats.transmitted, 1);
        assert_eq!(engine.stats.received, 1);
        assert_eq!(engine.stats.min_rtt_ms, 15.0);

        // Same reply -> duplicate
        let dup2 = engine.process_echo_reply(&reply, 15.0).unwrap();
        assert!(dup2);
        assert_eq!(engine.stats.transmitted, 2);
        assert_eq!(engine.stats.received, 1); // rx should not increment for duplicates

        // Receive sequence 2
        let reply2 = IcmpPacket {
            icmp_type: IcmpType::EchoReply,
            code: 0,
            checksum: 0,
            identifier: 0x1234,
            sequence: 2,
            payload: Vec::new(),
        };
        let dup3 = engine.process_echo_reply(&reply2, 25.0).unwrap();
        assert!(!dup3);
        assert_eq!(engine.stats.transmitted, 3);
        assert_eq!(engine.stats.received, 2);
        assert_eq!(engine.stats.min_rtt_ms, 15.0);
        assert_eq!(engine.stats.max_rtt_ms, 25.0);
        assert_eq!(engine.stats.avg_rtt_ms, 20.0);
        assert!(engine.stats.mdev_rtt_ms > 0.0);
    }

    #[test]
    fn test_ping_summary_report() {
        let mut engine = SovereignPingEngine::new();
        let reply1 = IcmpPacket {
            icmp_type: IcmpType::EchoReply,
            code: 0,
            checksum: 0,
            identifier: 0x1234,
            sequence: 1,
            payload: Vec::new(),
        };
        engine.process_echo_reply(&reply1, 10.0).unwrap();

        let report = engine.get_summary_report("www.sigmaos.org");
        // Verify report starts with standard prefix
        let prefix = b"--- ping statistics --- ";
        assert_eq!(&report[..prefix.len()], prefix);
    }
}
