# TCP/IP Networking Stack Implementation Plan

## Executive Summary

This plan addresses the complete TCP/IP networking stack implementation for SigmaOS, a `#![no_std]` capability-based microkernel OS in Rust. The existing network infrastructure at `src/network/` contains 67 files with partial implementations:

**Current State:**
- ✅ TCP basics exist (tcp.rs: states, TcpSocket, TcpSegment, buffers) but incomplete state machine
- ✅ ARP packet structures exist (arp.rs: 416 lines, ArpPacket, ArpCache) 
- ✅ ICMP basics exist (icmp.rs: 422 lines)
- ✅ Routing table exists (routing.rs: 237 lines)
- ❌ **CRITICAL:** 40+ files use `std::` imports violating `no_std` requirement
- ❌ TCP state machine incomplete (no retransmit timers, no out-of-order handling, no proper congestion control)
- ❌ UDP minimal (only header parsing in src/net/udp.rs)
- ❌ No proper socket API layer

**Architecture Decision:** Keep existing flat file structure in `src/network/` for backward compatibility with mod.rs re-exports. Add new subdirectories only when strictly necessary. Migrate `std::` violations to `alloc::`/`core::` incrementally.

**Design Principles Applied:**
- LAYERED DESIGN: Link → Network → Transport → Socket layers
- ZERO-COPY: In-place packet processing, scatter-gather I/O
- MEMORY SAFETY: Safe Rust by default, `unsafe` only with full safety comments
- SEPARATION OF CONCERNS: Protocol logic separate from device I/O
- RFC COMPLIANCE: RFC 793 (TCP), RFC 5681 (congestion), RFC 768 (UDP), RFC 826 (ARP), RFC 4861 (IPv6 NDP)

---

## Implementation Phases

### Phase 1: Fix `no_std` Violations (CRITICAL — BLOCKING ALL OTHER WORK)

**Rationale:** The kernel cannot compile with `std::` imports. This is a prerequisite for all subsequent work.

- [ ] 1. **Audit and document all `std::` violations**
      Create a comprehensive list of all files using `std::` imports and categorize by type (String, Vec, collections, etc.).
      
      **Files to audit:**
      - src/network/routing.rs (uses `std::string::String`, `std::vec::Vec`)
      - src/network/zenithnet.rs (uses `std::collections::BTreeMap`, `std::string::String`, `std::vec::Vec`)
      - src/network/config.rs (uses `std::vec`, `std::collections::BTreeMap`, `std::string::String`, `std::vec::Vec`)
      - src/network/zero_copy_networking.rs (uses `std::string::{String, ToString}`, `std::vec::Vec`)
      - src/network/distro_net.rs (uses `std::format`, `std::string::{String, ToString}`, `std::vec`, `std::vec::Vec`)
      - src/network/stack.rs (uses `std::boxed::Box`, `std::vec::Vec`)
      - src/network/tcp_udp.rs (uses `std::boxed::Box`, `std::vec::Vec`)
      - src/network/security.rs (uses `std::string::String`, `std::vec::Vec`)
      - src/network/wireguard.rs (uses `std::collections::HashMap`, `std::net::SocketAddr`)
      - src/network/tc_qdisc_sovereign.rs (uses `std::collections::VecDeque`, `std::string::{String, ToString}`, `std::vec::Vec`)
      - 30+ additional files identified in grep search
      
      **Actions:**
      - Create .agents/tasks/tcpip-nostd-audit.md documenting each file
      - For each file: list `std::` imports, map to `alloc::`/`core::` replacements
      - Identify any truly platform-specific dependencies (e.g., `std::net::SocketAddr` needs custom implementation)
      
      **Verification:** `grep -r "use std::" src/network/ | wc -l` reports 0 lines after completion

- [ ] 2. **Fix routing.rs `no_std` violations**
      Migrate routing.rs to use `alloc::string::String`, `alloc::vec::Vec`.
      
      **Files:** src/network/routing.rs
      
      **Changes:**
      ```rust
      // Replace:
      use std::string::String;
      use std::vec::Vec;
      
      // With:
      extern crate alloc;
      use alloc::string::String;
      use alloc::vec::Vec;
      ```
      
      **Preserve:** Existing RouteEntry, RoutingTable, RoutingEngine, ForwardingDecision logic
      
      **Verification:** `cargo check --lib` passes, `cargo test --lib routing` passes

- [ ] 3. **Fix zenithnet.rs `no_std` violations**
      Migrate zenithnet.rs to use `alloc::collections::BTreeMap`, `alloc::string::String`, `alloc::vec::Vec`.
      
      **Files:** src/network/zenithnet.rs
      
      **Changes:**
      ```rust
      extern crate alloc;
      use alloc::collections::BTreeMap;
      use alloc::string::String;
      use alloc::vec::Vec;
      ```
      
      **Preserve:** Existing EthernetFrame, Ipv4Header, TcpHeader, UdpHeader, ZenithNet stack logic
      
      **Verification:** `cargo check --lib` passes

- [ ] 4. **Fix security.rs `no_std` violations**
      Migrate security.rs firewall rules to `alloc::`.
      
      **Files:** src/network/security.rs
      
      **Changes:** Replace `std::string::String` → `alloc::string::String`, `std::vec::Vec` → `alloc::vec::Vec`
      
      **Verification:** `cargo check --lib` passes

- [ ] 5. **Fix remaining high-priority network files (batch)**
      Apply `no_std` migration to: config.rs, stack.rs, tcp_udp.rs, distro_net.rs, tc_qdisc_sovereign.rs, zero_copy_networking.rs.
      
      **Files:** 
      - src/network/config.rs
      - src/network/stack.rs
      - src/network/tcp_udp.rs
      - src/network/distro_net.rs
      - src/network/tc_qdisc_sovereign.rs
      - src/network/zero_copy_networking.rs
      
      **Pattern:** For each file:
      1. Add `#![no_std]` at top if missing
      2. Add `extern crate alloc;`
      3. Replace all `std::` imports with `alloc::`/`core::` equivalents
      4. For `std::collections::HashMap`: use `alloc::collections::BTreeMap` (no HashMap in alloc)
      5. For `std::net::SocketAddr`: will need custom implementation or feature-gate
      
      **Verification:** `cargo check --lib` passes, no `std::` imports in these files

- [ ] 6. **Fix remaining 25+ network files (batch cleanup)**
      Apply `no_std` migration to all remaining network files with `std::` imports identified in audit.
      
      **Files:** All remaining files from audit in step 1 (npf.rs, wireguard.rs, bridge.rs, mptcp.rs, etc.)
      
      **Strategy:** Apply same pattern as step 5 to each file in batch
      
      **Verification:** `grep -r "use std::" src/network/ | wc -l` returns 0

---

### Phase 2: Complete TCP State Machine (RFC 793)

**Rationale:** Existing tcp.rs has all 11 states defined but state transitions are incomplete. Need full RFC 793 compliance with three-way handshake, simultaneous open, four-way close, and RST handling.

**Dependencies:** Phase 1 must be complete (no_std violations fixed)

- [ ] 7. **Implement complete TCP state transitions**
      Extend TcpSocket::process_segment() in tcp.rs to handle all RFC 793 state transitions.
      
      **Files:** src/network/tcp.rs
      
      **RFC 793 States to implement:**
      - CLOSED → LISTEN (passive open)
      - CLOSED → SYN-SENT (active open)
      - LISTEN → SYN-RECEIVED (SYN received)
      - SYN-SENT → SYN-RECEIVED (simultaneous open: SYN received in SYN-SENT)
      - SYN-SENT → ESTABLISHED (SYN-ACK received)
      - SYN-RECEIVED → ESTABLISHED (ACK of SYN received)
      - ESTABLISHED → FIN-WAIT-1 (close initiated)
      - ESTABLISHED → CLOSE-WAIT (FIN received)
      - FIN-WAIT-1 → FIN-WAIT-2 (ACK of FIN received)
      - FIN-WAIT-1 → CLOSING (FIN received in FIN-WAIT-1: simultaneous close)
      - FIN-WAIT-2 → TIME-WAIT (FIN received)
      - CLOSE-WAIT → LAST-ACK (close after FIN received)
      - CLOSING → TIME-WAIT (ACK of FIN received in CLOSING)
      - LAST-ACK → CLOSED (ACK of FIN received)
      - TIME-WAIT → CLOSED (2MSL timer expires)
      - Any state → CLOSED (RST received or sent)
      
      **Add RST handling:**
      - Generate RST for invalid segments
      - Process RST in any state → transition to CLOSED
      
      **Test cases to add:**
      - test_three_way_handshake_active_open
      - test_three_way_handshake_passive_open
      - test_simultaneous_open
      - test_four_way_close_active
      - test_four_way_close_passive
      - test_simultaneous_close
      - test_rst_in_established
      - test_rst_generation_invalid_segment
      
      **Verification:** `cargo test --lib tcp::tests` passes all new state machine tests

- [ ] 8. **Add TCP retransmission timer infrastructure**
      Implement retransmission timeout (RTO) calculation and timer management.
      
      **Files:** src/network/tcp.rs
      
      **Add to TcpSocket:**
      ```rust
      pub struct TcpRetransmitTimer {
          rto: AtomicU64,           // Current RTO (microseconds)
          srtt: AtomicU64,          // Smoothed RTT (microseconds)
          rttvar: AtomicU64,        // RTT variance (microseconds)
          retransmit_count: AtomicU32,  // Number of retransmissions
          last_send_time: AtomicU64,    // Timestamp of last send
      }
      ```
      
      **Implement RFC 6298 RTO calculation:**
      - Initial RTO = 1 second
      - SRTT = (7/8) * SRTT + (1/8) * RTT_measured
      - RTTVAR = (3/4) * RTTVAR + (1/4) * |SRTT - RTT_measured|
      - RTO = SRTT + max(G, 4 * RTTVAR) where G = clock granularity
      - RTO bounds: [1s, 60s]
      - Exponential backoff on timeout: RTO = 2 * RTO
      
      **Add methods:**
      - `update_rtt(measured_rtt: u64)` — update SRTT/RTTVAR
      - `get_rto() -> u64` — return current RTO
      - `on_timeout()` — handle retransmit timeout (backoff, retransmit)
      - `reset_retransmit_count()` — on successful ACK
      
      **Verification:** Unit tests for RTO calculation edge cases

- [ ] 9. **Add unacknowledged segments queue**
      Track sent-but-unacknowledged segments for retransmission.
      
      **Files:** src/network/tcp.rs
      
      **Add to TcpSocket:**
      ```rust
      pub struct UnackedSegment {
          seq_num: u32,
          data: Vec<u8>,
          send_time: u64,  // Timestamp for RTT measurement
          retransmit_count: u32,
      }
      
      pub struct TcpSendBuffer {
          // ... existing fields ...
          unacked: VecDeque<UnackedSegment>,  // Sent but not ACKed
      }
      ```
      
      **Methods:**
      - `add_unacked(seq: u32, data: Vec<u8>, time: u64)` — add to queue on send
      - `remove_acked(ack_num: u32)` — remove segments ACKed by ack_num
      - `get_retransmit_segment() -> Option<UnackedSegment>` — get oldest unACKed for retransmit
      
      **Verification:** Unit tests for queue operations, ACK processing

---

### Phase 3: TCP Data Transfer & Flow Control

**Rationale:** Implement sliding window protocol with sequence number management, out-of-order segment reordering, and flow control.

**Dependencies:** Phase 2 complete (state machine working)

- [ ] 10. **Implement TCP sliding window**
      Add send window (SND.WND) and receive window (RCV.WND) tracking per RFC 793 Section 3.2.
      
      **Files:** src/network/tcp.rs
      
      **Add window tracking to TcpSocket:**
      ```rust
      pub struct TcpSendWindow {
          una: AtomicU32,      // Send unacknowledged (SND.UNA)
          nxt: AtomicU32,      // Send next (SND.NXT)
          wnd: AtomicU16,      // Send window (SND.WND) — peer's receive window
          up: AtomicU32,       // Send urgent pointer (SND.UP)
          wl1: AtomicU32,      // Segment seq for last window update (SND.WL1)
          wl2: AtomicU32,      // Segment ack for last window update (SND.WL2)
          iss: u32,            // Initial send sequence number (immutable)
      }
      
      pub struct TcpRecvWindow {
          nxt: AtomicU32,      // Receive next (RCV.NXT)
          wnd: AtomicU16,      // Receive window (RCV.WND) — our window
          up: AtomicU32,       // Receive urgent pointer (RCV.UP)
          irs: u32,            // Initial receive sequence number (immutable)
      }
      ```
      
      **Implement window update logic (RFC 793 page 72):**
      - Update SND.WND only if segment is valid and more recent than last update
      - Condition: SEG.SEQ =< SND.WL1 AND SEG.ACK >= SND.WL2 OR (SEG.SEQ = SND.WL1 AND SEG.ACK > SND.WL2)
      
      **Implement send window check:**
      - Before sending: check SND.NXT < SND.UNA + SND.WND
      - Implement silly window syndrome avoidance (RFC 1122 Section 4.2.3.3)
      
      **Test cases:**
      - test_send_window_enforcement
      - test_receive_window_advertisement
      - test_window_update_validation
      
      **Verification:** `cargo test --lib tcp::tests::test_sliding_window` passes

- [ ] 11. **Implement out-of-order segment handling**
      Add reorder buffer to TcpRecvBuffer for segments arriving out of sequence.
      
      **Files:** src/network/tcp.rs
      
      **Enhance TcpRecvBuffer:**
      ```rust
      pub struct TcpRecvBuffer {
          // ... existing fields ...
          reorder_buffer: BTreeMap<u32, Vec<u8>>,  // seq_num -> data
          holes: Vec<(u32, u32)>,  // List of sequence number holes (start, end)
      }
      ```
      
      **Implement RFC 793 segment reassembly (page 70):**
      - On receive: if seq == RCV.NXT, deliver immediately
      - If seq > RCV.NXT, store in reorder_buffer
      - After in-order data delivered, check reorder_buffer for contiguous segments
      - Deliver all newly-contiguous segments
      
      **Add duplicate ACK generation:**
      - If out-of-order segment received, send duplicate ACK with current RCV.NXT
      - This enables fast retransmit at sender (Phase 4)
      
      **Test cases:**
      - test_out_of_order_single_segment
      - test_out_of_order_multiple_segments
      - test_out_of_order_fills_hole
      - test_duplicate_segment_handling
      
      **Verification:** `cargo test --lib tcp::tests::test_reordering` passes

- [ ] 12. **Implement TCP flow control**
      Proper receive window management and zero-window probing.
      
      **Files:** src/network/tcp.rs
      
      **Implement receive window calculation:**
      - RCV.WND = buffer_capacity - buffered_data_length
      - Update on every segment sent (in ACK)
      - Minimum window advertisement: max(buffer_capacity/2, MSS)
      
      **Implement zero-window probing (RFC 1122 Section 4.2.2.17):**
      - When SND.WND = 0, sender enters persist state
      - Send 1-byte probe segments at persist timeout intervals
      - Exponential backoff: 5s, 10s, 20s, ... up to 60s
      
      **Add to TcpSocket:**
      ```rust
      pub struct TcpPersistTimer {
          enabled: AtomicBool,
          interval: AtomicU64,  // Current persist interval (microseconds)
      }
      ```
      
      **Test cases:**
      - test_flow_control_window_updates
      - test_zero_window_probe
      - test_window_reopening
      
      **Verification:** `cargo test --lib tcp::tests::test_flow_control` passes

---

### Phase 4: TCP Congestion Control (RFC 5681 + CUBIC)

**Rationale:** Existing TcpCongestion struct is a stub. Need full slow start, congestion avoidance, fast retransmit, fast recovery, and CUBIC algorithm (Linux default).

**Dependencies:** Phase 3 complete (data transfer working)

- [ ] 13. **Implement TCP Reno congestion control baseline**
      Complete implementation of classic Reno algorithm per RFC 5681.
      
      **Files:** src/network/tcp.rs
      
      **Enhance TcpCongestion:**
      ```rust
      pub struct TcpCongestion {
          cwnd: AtomicU32,         // Congestion window (bytes)
          ssthresh: AtomicU32,     // Slow start threshold (bytes)
          algorithm: CongestionAlgorithm,
          
          // Fast retransmit/recovery state
          duplicate_acks: AtomicU32,
          recover: AtomicU32,      // Highest seq sent when entering recovery
          in_recovery: AtomicBool,
      }
      ```
      
      **Implement RFC 5681 state machine:**
      
      1. **Slow Start (cwnd < ssthresh):**
         - On ACK: cwnd += min(N, MSS) where N = bytes ACKed
         - Exponential growth: cwnd doubles per RTT
      
      2. **Congestion Avoidance (cwnd >= ssthresh):**
         - On ACK: cwnd += MSS * MSS / cwnd
         - Linear growth: cwnd increases by 1 MSS per RTT
      
      3. **Fast Retransmit (3 duplicate ACKs):**
         - Retransmit missing segment immediately
         - ssthresh = max(FlightSize/2, 2*MSS)
         - cwnd = ssthresh + 3*MSS
         - Enter fast recovery
      
      4. **Fast Recovery:**
         - On duplicate ACK: cwnd += MSS (inflate window)
         - On new ACK: cwnd = ssthresh, exit recovery
      
      5. **Timeout:**
         - ssthresh = max(FlightSize/2, 2*MSS)
         - cwnd = 1 MSS
         - Enter slow start
      
      **Test cases:**
      - test_slow_start
      - test_congestion_avoidance
      - test_fast_retransmit
      - test_fast_recovery
      - test_timeout_recovery
      
      **Verification:** `cargo test --lib tcp::tests::test_congestion_reno` passes

- [ ] 14. **Implement CUBIC congestion control**
      Add CUBIC algorithm (Linux default, more aggressive than Reno on high-BDP paths).
      
      **Files:** src/network/tcp.rs (or new src/network/tcp_cubic.rs if file gets too large)
      
      **Add CUBIC state:**
      ```rust
      pub struct CubicState {
          w_max: AtomicU32,        // Window size before last reduction
          w_last_max: AtomicU32,   // w_max from previous congestion event
          epoch_start: AtomicU64,  // Time of last congestion event (microseconds)
          origin_point: AtomicU32, // Origin point for cubic function
          k: AtomicU32,            // Time to increase cwnd to w_max (in RTTs)
          tcp_friendly: bool,      // Enable TCP-friendly mode
      }
      ```
      
      **Implement CUBIC function (RFC 8312):**
      - W(t) = C * (t - K)^3 + W_max
      - C = 0.4 (CUBIC scaling constant)
      - K = ∛(W_max * β / C) where β = 0.7 (multiplicative decrease)
      
      **CUBIC phases:**
      1. **Slow Start:** Same as Reno (cwnd < ssthresh)
      2. **Concave Region (W(t) < W_max):** Fast window growth approaching previous max
      3. **Convex Region (W(t) > W_max):** Probing for more bandwidth
      4. **TCP-Friendly Region:** Fallback to Reno-like behavior if CUBIC is slower
      
      **On ACK (congestion avoidance):**
      ```
      t = now - epoch_start
      target = cubic_function(t)
      if target > cwnd:
          cwnd += (target - cwnd) / cwnd  // Additive increase toward target
      ```
      
      **On loss:**
      ```
      w_last_max = w_max
      w_max = cwnd
      ssthresh = cwnd * β  // β = 0.7 for CUBIC (vs 0.5 for Reno)
      cwnd = ssthresh
      epoch_start = now
      ```
      
      **Test cases:**
      - test_cubic_slow_start
      - test_cubic_concave_region
      - test_cubic_convex_region
      - test_cubic_tcp_friendly_mode
      - test_cubic_vs_reno_performance
      
      **Verification:** `cargo test --lib tcp::tests::test_congestion_cubic` passes

- [ ] 15. **Add congestion control algorithm selection**
      Allow runtime selection between Reno, CUBIC, Vegas (if implemented).
      
      **Files:** src/network/tcp.rs
      
      **Add to TcpSocket:**
      ```rust
      pub fn set_congestion_algorithm(&mut self, algo: CongestionAlgorithm) {
          self.congestion.algorithm = algo;
          // Reset algorithm-specific state
      }
      
      pub fn get_congestion_algorithm(&self) -> CongestionAlgorithm {
          self.congestion.algorithm
      }
      ```
      
      **Dispatch in congestion control:**
      ```rust
      impl TcpCongestion {
          pub fn on_ack(&mut self, acked_bytes: u32) {
              match self.algorithm {
                  CongestionAlgorithm::Reno => self.reno_on_ack(acked_bytes),
                  CongestionAlgorithm::Cubic => self.cubic_on_ack(acked_bytes),
                  _ => {} // Other algorithms
              }
          }
      }
      ```
      
      **Verification:** Unit tests verify algorithm switching

---

### Phase 5: UDP Implementation (RFC 768)

**Rationale:** Existing UDP implementation is minimal (only header parsing). Need full datagram send/receive, checksum calculation, port binding, and multicast support.

**Dependencies:** Phase 1 complete (no_std fixed)

- [ ] 16. **Expand UDP implementation with send/receive**
      Build out UDP socket with datagram operations.
      
      **Files:** Create src/network/udp.rs (migrate from src/net/udp.rs if needed, or enhance existing)
      
      **Add UDP socket structure:**
      ```rust
      #![no_std]
      
      extern crate alloc;
      use alloc::collections::VecDeque;
      use alloc::vec::Vec;
      
      pub struct UdpSocket {
          local_port: u16,
          local_addr: [u8; 4],  // IPv4 for now
          recv_queue: VecDeque<UdpDatagram>,
          recv_buffer_size: usize,
      }
      
      pub struct UdpDatagram {
          src_addr: [u8; 4],
          src_port: u16,
          dst_addr: [u8; 4],
          dst_port: u16,
          data: Vec<u8>,
      }
      ```
      
      **Methods:**
      - `new(local_addr: [u8; 4], local_port: u16) -> Self`
      - `bind(addr: [u8; 4], port: u16) -> Result<Self, UdpError>`
      - `send_to(data: &[u8], dst_addr: [u8; 4], dst_port: u16) -> Result<(), UdpError>`
      - `recv_from(buffer: &mut [u8]) -> Result<(usize, [u8; 4], u16), UdpError>`
      - `set_recv_buffer_size(size: usize)`
      
      **Error types:**
      ```rust
      pub enum UdpError {
          PortInUse,
          BufferFull,
          InvalidAddress,
          Checksum,
      }
      ```
      
      **Verification:** Unit tests for bind, send_to, recv_from

- [ ] 17. **Implement UDP checksum calculation**
      Add proper UDP checksum per RFC 768.
      
      **Files:** src/network/udp.rs
      
      **UDP checksum pseudo-header:**
      ```
      0               16              32
      +---------------+---------------+
      |    Source IP Address          |
      +---------------+---------------+
      |  Destination IP Address       |
      +---------------+---------------+
      | zero | proto  |  UDP length   |
      +---------------+---------------+
      ```
      
      **Implement:**
      ```rust
      pub fn compute_udp_checksum(
          src_ip: [u8; 4],
          dst_ip: [u8; 4],
          udp_header: &[u8; 8],
          data: &[u8],
      ) -> u16 {
          // 1. Build pseudo-header
          // 2. Compute 16-bit one's complement sum of pseudo-header + header + data
          // 3. Return one's complement of result
      }
      
      pub fn verify_udp_checksum(...) -> bool { ... }
      ```
      
      **ZERO-COPY consideration:** Checksum should operate on scattered buffers without copying:
      ```rust
      pub fn compute_udp_checksum_scatter(
          src_ip: [u8; 4],
          dst_ip: [u8; 4],
          header: &[u8],
          data_chunks: &[&[u8]],
      ) -> u16
      ```
      
      **Test cases:**
      - test_udp_checksum_zero_data
      - test_udp_checksum_odd_length
      - test_udp_checksum_even_length
      - test_udp_checksum_verification
      
      **Verification:** `cargo test --lib udp::tests::test_checksum` passes

- [ ] 18. **Add UDP port binding and multiplexing**
      Port table to map incoming datagrams to correct socket.
      
      **Files:** src/network/udp.rs
      
      **Add port table:**
      ```rust
      use alloc::collections::BTreeMap;
      
      pub struct UdpPortTable {
          bindings: BTreeMap<u16, UdpSocket>,  // port -> socket
      }
      
      impl UdpPortTable {
          pub fn bind(&mut self, port: u16, socket: UdpSocket) -> Result<(), UdpError> {
              if self.bindings.contains_key(&port) {
                  return Err(UdpError::PortInUse);
              }
              self.bindings.insert(port, socket);
              Ok(())
          }
          
          pub fn unbind(&mut self, port: u16) -> Option<UdpSocket> {
              self.bindings.remove(&port)
          }
          
          pub fn deliver(&mut self, dst_port: u16, datagram: UdpDatagram) -> Result<(), UdpError> {
              if let Some(socket) = self.bindings.get_mut(&dst_port) {
                  socket.enqueue(datagram)
              } else {
                  Err(UdpError::PortNotBound)
              }
          }
      }
      ```
      
      **Test cases:**
      - test_port_binding
      - test_port_already_bound
      - test_datagram_delivery_to_socket
      - test_unbind_port
      
      **Verification:** `cargo test --lib udp::tests::test_port_table` passes

- [ ] 19. **Add UDP multicast support (optional enhancement)**
      Support for joining/leaving multicast groups.
      
      **Files:** src/network/udp.rs
      
      **Add to UdpSocket:**
      ```rust
      pub struct UdpSocket {
          // ... existing fields ...
          multicast_groups: Vec<[u8; 4]>,  // List of joined groups
      }
      
      impl UdpSocket {
          pub fn join_multicast(&mut self, group: [u8; 4]) -> Result<(), UdpError> {
              // Validate group is in 224.0.0.0/4 range
              if group[0] < 224 || group[0] > 239 {
                  return Err(UdpError::InvalidMulticastGroup);
              }
              self.multicast_groups.push(group);
              Ok(())
          }
          
          pub fn leave_multicast(&mut self, group: [u8; 4]) -> Result<(), UdpError> {
              if let Some(pos) = self.multicast_groups.iter().position(|&g| g == group) {
                  self.multicast_groups.remove(pos);
                  Ok(())
              } else {
                  Err(UdpError::NotInGroup)
              }
          }
      }
      ```
      
      **Test cases:**
      - test_join_multicast_group
      - test_leave_multicast_group
      - test_invalid_multicast_address
      
      **Verification:** `cargo test --lib udp::tests::test_multicast` passes

---

### Phase 6: ARP Cache & IPv6 NDP

**Rationale:** Existing arp.rs has packet structures and basic cache. Need TTL-based eviction, proper request/reply handling, and IPv6 Neighbor Discovery Protocol support.

**Dependencies:** Phase 1 complete (no_std fixed)

- [ ] 20. **Enhance ARP cache with TTL and eviction**
      Add time-to-live tracking and automatic cache entry eviction.
      
      **Files:** src/network/arp.rs
      
      **Examine existing ArpCache structure** (line 100+), enhance with:
      ```rust
      pub struct ArpCacheEntry {
          mac_addr: [u8; 6],
          ip_addr: [u8; 4],
          ttl: AtomicU64,       // Expiration time (microseconds since boot)
          state: ArpEntryState,
      }
      
      #[derive(Debug, Clone, Copy, PartialEq, Eq)]
      pub enum ArpEntryState {
          Incomplete,  // Request sent, awaiting reply
          Reachable,   // Entry is valid
          Stale,       // Entry expired but not evicted
      }
      
      pub struct ArpCache {
          entries: BTreeMap<[u8; 4], ArpCacheEntry>,
          timeout_ms: u64,  // Default TTL: 300,000ms (5 minutes)
      }
      ```
      
      **Add methods:**
      ```rust
      impl ArpCache {
          pub fn insert(&mut self, ip: [u8; 4], mac: [u8; 6], current_time: u64) {
              let entry = ArpCacheEntry {
                  mac_addr: mac,
                  ip_addr: ip,
                  ttl: AtomicU64::new(current_time + self.timeout_ms * 1000),
                  state: ArpEntryState::Reachable,
              };
              self.entries.insert(ip, entry);
          }
          
          pub fn lookup(&self, ip: [u8; 4], current_time: u64) -> Option<[u8; 6]> {
              if let Some(entry) = self.entries.get(&ip) {
                  if entry.ttl.load(Ordering::Acquire) > current_time {
                      return Some(entry.mac_addr);
                  }
              }
              None
          }
          
          pub fn evict_expired(&mut self, current_time: u64) {
              self.entries.retain(|_, entry| {
                  entry.ttl.load(Ordering::Acquire) > current_time
              });
          }
      }
      ```
      
      **Test cases:**
      - test_arp_cache_insert_lookup
      - test_arp_cache_expiration
      - test_arp_cache_eviction
      - test_arp_cache_update_existing
      
      **Verification:** `cargo test --lib arp::tests::test_cache` passes

- [ ] 21. **Implement ARP request/reply handling**
      Complete ARP protocol handler for request generation and reply processing.
      
      **Files:** src/network/arp.rs
      
      **Enhance ArpHandler** (examine existing structure around line 200+):
      ```rust
      pub struct ArpHandler {
          cache: ArpCache,
          local_mac: [u8; 6],
          local_ip: [u8; 4],
          pending_requests: BTreeMap<[u8; 4], u64>,  // ip -> request_time
      }
      
      impl ArpHandler {
          pub fn send_request(&mut self, target_ip: [u8; 4]) -> ArpPacket {
              ArpPacket::request(self.local_mac, self.local_ip, target_ip)
          }
          
          pub fn handle_request(&mut self, packet: &ArpPacket) -> Option<ArpPacket> {
              // If request is for our IP, send reply
              if packet.target_proto_addr == self.local_ip {
                  Some(ArpPacket::reply(
                      self.local_mac,
                      self.local_ip,
                      packet.sender_hw_addr,
                      packet.sender_proto_addr,
                  ))
              } else {
                  None
              }
          }
          
          pub fn handle_reply(&mut self, packet: &ArpPacket, current_time: u64) {
              // Update cache with sender's mapping
              self.cache.insert(
                  packet.sender_proto_addr,
                  packet.sender_hw_addr,
                  current_time,
              );
              // Remove from pending requests
              self.pending_requests.remove(&packet.sender_proto_addr);
          }
          
          pub fn process_packet(&mut self, packet: &ArpPacket, current_time: u64) -> Option<ArpPacket> {
              match u16::from_be(packet.operation) {
                  1 => self.handle_request(packet),  // ARP Request
                  2 => { self.handle_reply(packet, current_time); None },  // ARP Reply
                  _ => None,
              }
          }
      }
      ```
      
      **Test cases:**
      - test_arp_request_generation
      - test_arp_request_for_local_ip
      - test_arp_reply_updates_cache
      - test_gratuitous_arp
      
      **Verification:** `cargo test --lib arp::tests::test_protocol` passes

- [ ] 22. **Add IPv6 Neighbor Discovery Protocol (NDP) basics**
      Implement NDP for IPv6 address resolution (RFC 4861).
      
      **Files:** Create src/network/ndp.rs or add to src/network/arp.rs as `pub mod ndp`
      
      **NDP replaces ARP for IPv6. Key messages:**
      - Neighbor Solicitation (NS) — like ARP request
      - Neighbor Advertisement (NA) — like ARP reply
      - Router Solicitation (RS)
      - Router Advertisement (RA)
      - Redirect
      
      **Implement NS/NA for address resolution:**
      ```rust
      #![no_std]
      
      extern crate alloc;
      use alloc::collections::BTreeMap;
      
      pub struct NdpCache {
          entries: BTreeMap<[u8; 16], [u8; 6]>,  // IPv6 addr -> MAC
      }
      
      pub struct NdpNeighborSolicitation {
          target_addr: [u8; 16],  // IPv6 address
      }
      
      pub struct NdpNeighborAdvertisement {
          target_addr: [u8; 16],
          target_link_addr: [u8; 6],  // MAC address
      }
      
      pub struct NdpHandler {
          cache: NdpCache,
          local_mac: [u8; 6],
          local_ipv6: [u8; 16],
      }
      
      impl NdpHandler {
          pub fn send_neighbor_solicitation(&self, target: [u8; 16]) -> NdpNeighborSolicitation {
              NdpNeighborSolicitation { target_addr: target }
          }
          
          pub fn handle_neighbor_solicitation(&self, ns: &NdpNeighborSolicitation) -> Option<NdpNeighborAdvertisement> {
              if ns.target_addr == self.local_ipv6 {
                  Some(NdpNeighborAdvertisement {
                      target_addr: self.local_ipv6,
                      target_link_addr: self.local_mac,
                  })
              } else {
                  None
              }
          }
          
          pub fn handle_neighbor_advertisement(&mut self, na: &NdpNeighborAdvertisement) {
              self.cache.entries.insert(na.target_addr, na.target_link_addr);
          }
      }
      ```
      
      **Note:** Full NDP implementation is complex (Router Discovery, SLAAC, DAD). This phase implements basic NS/NA for neighbor cache only.
      
      **Test cases:**
      - test_ndp_neighbor_solicitation
      - test_ndp_neighbor_advertisement
      - test_ndp_cache_update
      
      **Verification:** `cargo test --lib ndp::tests` passes (if separate module) or `cargo test --lib arp::ndp::tests` passes

---

### Phase 7: Routing Table & IPv4 Forwarding

**Rationale:** Existing routing.rs has basic routing table with longest-prefix match. Need to verify no_std compliance, add default gateway support, and implement basic IPv4 packet forwarding.

**Dependencies:** Phase 1 complete (no_std fixed)

- [ ] 23. **Verify and enhance routing table**
      Ensure routing.rs is no_std compliant and has complete longest-prefix match implementation.
      
      **Files:** src/network/routing.rs
      
      **Verify fixed in Phase 1 step 2**, then enhance:
      
      **Add default route support:**
      ```rust
      impl RoutingTable {
          pub fn add_default_gateway(&mut self, gateway: Ipv4Addr, interface: String, metric: u32) {
              let default_route = RouteEntry::new(
                  Ipv4Addr(0),  // 0.0.0.0
                  Ipv4Addr(0),  // 0.0.0.0 (netmask)
                  interface,
              )
              .with_gateway(gateway)
              .with_metric(metric);
              
              self.add_route(default_route);
          }
          
          pub fn get_default_gateway(&self) -> Option<&RouteEntry> {
              self.routes.iter().find(|r| r.destination.0 == 0 && r.netmask.0 == 0)
          }
      }
      ```
      
      **Verify longest-prefix match sorting** (already implemented in existing code around line 60):
      ```rust
      self.routes.sort_by_key(|r| {
          let bits = u32::BITS - r.netmask.0.leading_zeros();
          -(bits as i32)  // Most specific (longest prefix) first
      });
      ```
      
      **Test cases:**
      - test_routing_table_longest_prefix_match
      - test_default_gateway
      - test_more_specific_route_wins
      - test_metric_tiebreaker
      
      **Verification:** `cargo test --lib routing::tests` passes

- [ ] 24. **Implement IPv4 packet forwarding logic**
      Add routing engine that makes forwarding decisions.
      
      **Files:** src/network/routing.rs
      
      **Examine existing RoutingEngine and ForwardingDecision** (line 90+), enhance:
      ```rust
      pub struct RoutingEngine {
          table: RoutingTable,
          local_interfaces: Vec<(Ipv4Addr, String)>,  // Local IPs and interface names
      }
      
      impl RoutingEngine {
          pub fn forward(&self, dst_addr: Ipv4Addr) -> ForwardingDecision {
              // Check if destination is local
              if self.is_local_address(dst_addr) {
                  return ForwardingDecision::LocalDelivery;
              }
              
              // Lookup route
              if let Some(route) = self.table.lookup(dst_addr) {
                  if let Some(gateway) = route.gateway {
                      ForwardingDecision::Forward {
                          interface: route.interface.clone(),
                          next_hop: gateway,
                      }
                  } else {
                      // Directly connected network
                      ForwardingDecision::Forward {
                          interface: route.interface.clone(),
                          next_hop: dst_addr,  // Send directly to destination
                      }
                  }
              } else {
                  ForwardingDecision::Drop
              }
          }
          
          fn is_local_address(&self, addr: Ipv4Addr) -> bool {
              self.local_interfaces.iter().any(|(ip, _)| *ip == addr)
          }
      }
      ```
      
      **Test cases:**
      - test_local_delivery
      - test_forward_via_gateway
      - test_forward_directly_connected
      - test_drop_no_route
      
      **Verification:** `cargo test --lib routing::tests::test_forwarding` passes

- [ ] 25. **Add routing table CLI inspection (optional)**
      Helper functions for debugging routing table state.
      
      **Files:** src/network/routing.rs
      
      **Add debug helpers:**
      ```rust
      impl RoutingTable {
          pub fn dump(&self) -> alloc::string::String {
              use alloc::format;
              use alloc::string::String;
              
              let mut output = String::from("Destination\tNetmask\t\tGateway\t\tInterface\tMetric\n");
              for route in &self.routes {
                  output.push_str(&format!(
                      "{}.{}.{}.{}\t{}.{}.{}.{}\t",
                      route.destination.0 >> 24,
                      (route.destination.0 >> 16) & 0xFF,
                      (route.destination.0 >> 8) & 0xFF,
                      route.destination.0 & 0xFF,
                      route.netmask.0 >> 24,
                      (route.netmask.0 >> 16) & 0xFF,
                      (route.netmask.0 >> 8) & 0xFF,
                      route.netmask.0 & 0xFF,
                  ));
                  
                  if let Some(gw) = route.gateway {
                      output.push_str(&format!("{}.{}.{}.{}\t", gw.0 >> 24, (gw.0 >> 16) & 0xFF, (gw.0 >> 8) & 0xFF, gw.0 & 0xFF));
                  } else {
                      output.push_str("*\t\t");
                  }
                  
                  output.push_str(&format!("{}\t\t{}\n", route.interface, route.metric));
              }
              output
          }
      }
      ```
      
      **Verification:** Manual inspection of output format

---

### Phase 8: Integration & Socket API Layer

**Rationale:** Connect TCP/UDP implementations to a unified socket API that capability-aware syscalls can invoke. The existing socket.rs provides enums but needs connection to TCP/UDP engines.

**Dependencies:** All previous phases complete

- [ ] 26. **Create unified socket API**
      Build socket layer that dispatches to TCP or UDP based on socket type.
      
      **Files:** Enhance src/network/socket.rs
      
      **Examine existing Socket struct and enhance:**
      ```rust
      #![no_std]
      
      extern crate alloc;
      use alloc::boxed::Box;
      use crate::network::tcp::TcpSocket;
      use crate::network::udp::UdpSocket;
      
      pub enum SocketImpl {
          Tcp(Box<TcpSocket>),
          Udp(Box<UdpSocket>),
          Raw,  // For future raw socket support
      }
      
      pub struct Socket {
          pub domain: SocketDomain,      // AF_INET, AF_INET6
          pub socket_type: SocketType,   // SOCK_STREAM, SOCK_DGRAM
          pub protocol: SocketProtocol,  // IPPROTO_TCP, IPPROTO_UDP
          pub state: SocketState,
          impl_socket: Option<SocketImpl>,
      }
      
      impl Socket {
          pub fn new(domain: SocketDomain, socket_type: SocketType, protocol: SocketProtocol) -> Result<Self, SocketError> {
              let impl_socket = match (socket_type, protocol) {
                  (SocketType::Stream, SocketProtocol::Tcp) => {
                      Some(SocketImpl::Tcp(Box::new(TcpSocket::new(0))))
                  }
                  (SocketType::Dgram, SocketProtocol::Udp) => {
                      Some(SocketImpl::Udp(Box::new(UdpSocket::new([0; 4], 0))))
                  }
                  _ => return Err(SocketError::UnsupportedProtocol),
              };
              
              Ok(Self {
                  domain,
                  socket_type,
                  protocol,
                  state: SocketState::Closed,
                  impl_socket,
              })
          }
          
          pub fn bind(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
              match &mut self.impl_socket {
                  Some(SocketImpl::Tcp(tcp)) => {
                      // Extract port from addr and bind
                      tcp.local_port = addr.port;
                      self.state = SocketState::Bound;
                      Ok(())
                  }
                  Some(SocketImpl::Udp(udp)) => {
                      // Bind UDP socket
                      // TODO: Hook into port table
                      self.state = SocketState::Bound;
                      Ok(())
                  }
                  _ => Err(SocketError::InvalidOperation),
              }
          }
          
          pub fn listen(&mut self, backlog: usize) -> Result<(), SocketError> {
              match &mut self.impl_socket {
                  Some(SocketImpl::Tcp(tcp)) => {
                      tcp.listen()?;
                      self.state = SocketState::Listening;
                      Ok(())
                  }
                  _ => Err(SocketError::InvalidOperation),
              }
          }
          
          pub fn connect(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
              match &mut self.impl_socket {
                  Some(SocketImpl::Tcp(tcp)) => {
                      tcp.connect(addr.port)?;
                      self.state = SocketState::Connected;
                      Ok(())
                  }
                  _ => Err(SocketError::InvalidOperation),
              }
          }
          
          pub fn send(&mut self, data: &[u8]) -> Result<usize, SocketError> {
              match &mut self.impl_socket {
                  Some(SocketImpl::Tcp(tcp)) => {
                      tcp.send(data).map_err(|_| SocketError::SendFailed)
                  }
                  Some(SocketImpl::Udp(udp)) => {
                      // TODO: Need destination address for UDP
                      Err(SocketError::InvalidOperation)
                  }
                  _ => Err(SocketError::InvalidOperation),
              }
          }
          
          pub fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, SocketError> {
              match &mut self.impl_socket {
                  Some(SocketImpl::Tcp(tcp)) => {
                      tcp.recv(buffer).map_err(|_| SocketError::ReceiveFailed)
                  }
                  Some(SocketImpl::Udp(udp)) => {
                      udp.recv_from(buffer).map(|(n, _, _)| n).map_err(|_| SocketError::ReceiveFailed)
                  }
                  _ => Err(SocketError::InvalidOperation),
              }
          }
          
          pub fn close(&mut self) -> Result<(), SocketError> {
              self.state = SocketState::Closed;
              self.impl_socket = None;
              Ok(())
          }
      }
      
      pub struct SocketAddr {
          pub ip: [u8; 4],  // IPv4 for now
          pub port: u16,
      }
      ```
      
      **Test cases:**
      - test_socket_create_tcp
      - test_socket_create_udp
      - test_socket_bind
      - test_socket_listen_tcp
      - test_socket_connect_tcp
      - test_socket_send_recv_tcp
      - test_socket_sendto_recvfrom_udp
      
      **Verification:** `cargo test --lib socket::tests` passes

- [ ] 27. **Add socket capability token verification**
      Integrate capability checks per SigmaOS security model.
      
      **Files:** src/network/socket.rs
      
      **Add capability checking:**
      ```rust
      use crate::security::CapabilityToken;
      
      pub struct Socket {
          // ... existing fields ...
          required_capability: CapabilityToken,
      }
      
      impl Socket {
          pub fn new_with_capability(
              domain: SocketDomain,
              socket_type: SocketType,
              protocol: SocketProtocol,
              capability: CapabilityToken,
          ) -> Result<Self, SocketError> {
              // Verify capability allows network operations
              if !capability.has_permission("network.socket.create") {
                  return Err(SocketError::PermissionDenied);
              }
              
              // Verify protocol-specific permissions
              match socket_type {
                  SocketType::Stream => {
                      if !capability.has_permission("network.tcp") {
                          return Err(SocketError::PermissionDenied);
                      }
                  }
                  SocketType::Dgram => {
                      if !capability.has_permission("network.udp") {
                          return Err(SocketError::PermissionDenied);
                      }
                  }
                  _ => {}
              }
              
              let mut socket = Self::new(domain, socket_type, protocol)?;
              socket.required_capability = capability;
              Ok(socket)
          }
          
          pub fn bind_with_capability(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
              // Check bind permission
              if !self.required_capability.has_permission("network.socket.bind") {
                  return Err(SocketError::PermissionDenied);
              }
              
              // Privileged ports (< 1024) require extra permission
              if addr.port < 1024 && !self.required_capability.has_permission("network.bind.privileged") {
                  return Err(SocketError::PermissionDenied);
              }
              
              self.bind(addr)
          }
      }
      ```
      
      **Test cases:**
      - test_socket_capability_create_allowed
      - test_socket_capability_create_denied
      - test_socket_bind_privileged_port_denied
      - test_socket_bind_privileged_port_allowed
      
      **Verification:** `cargo test --lib socket::tests::test_capabilities` passes

- [ ] 28. **Write integration tests**
      End-to-end tests exercising full stack from socket API through TCP/UDP to packet generation.
      
      **Files:** Create tests/network_stack_integration.rs
      
      **Integration test scenarios:**
      ```rust
      #[cfg(test)]
      mod integration_tests {
          use sigmaos::network::*;
          
          #[test]
          fn test_tcp_loopback_connection() {
              // Create listening socket
              let mut listener = Socket::new(SocketDomain::AfInet, SocketType::Stream, SocketProtocol::Tcp).unwrap();
              listener.bind(SocketAddr { ip: [127, 0, 0, 1], port: 8080 }).unwrap();
              listener.listen(5).unwrap();
              
              // Create connecting socket
              let mut client = Socket::new(SocketDomain::AfInet, SocketType::Stream, SocketProtocol::Tcp).unwrap();
              client.connect(SocketAddr { ip: [127, 0, 0, 1], port: 8080 }).unwrap();
              
              // Send data
              let sent = client.send(b"Hello TCP").unwrap();
              assert_eq!(sent, 9);
              
              // TODO: Accept connection and receive on server side
              // let mut server = listener.accept().unwrap();
              // let mut buf = [0u8; 100];
              // let received = server.recv(&mut buf).unwrap();
              // assert_eq!(received, 9);
              // assert_eq!(&buf[..9], b"Hello TCP");
          }
          
          #[test]
          fn test_udp_sendto_recvfrom() {
              let mut socket1 = Socket::new(SocketDomain::AfInet, SocketType::Dgram, SocketProtocol::Udp).unwrap();
              socket1.bind(SocketAddr { ip: [127, 0, 0, 1], port: 9000 }).unwrap();
              
              let mut socket2 = Socket::new(SocketDomain::AfInet, SocketType::Dgram, SocketProtocol::Udp).unwrap();
              socket2.bind(SocketAddr { ip: [127, 0, 0, 1], port: 9001 }).unwrap();
              
              // Send from socket2 to socket1
              // (Need sendto API for UDP)
              
              // TODO: Complete when UDP sendto/recvfrom implemented in socket.rs
          }
          
          #[test]
          fn test_arp_resolution() {
              let mut arp_handler = ArpHandler::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55], [192, 168, 1, 10]);
              
              // Simulate ARP request for 192.168.1.1
              let request = arp_handler.send_request([192, 168, 1, 1]);
              assert_eq!(u16::from_be(request.operation), ArpOperation::Request as u16);
              
              // Simulate receiving reply
              let reply = ArpPacket::reply(
                  [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
                  [192, 168, 1, 1],
                  [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
                  [192, 168, 1, 10],
              );
              
              arp_handler.handle_reply(&reply, 1000000);
              
              // Verify cache entry
              let mac = arp_handler.cache.lookup([192, 168, 1, 1], 2000000).unwrap();
              assert_eq!(mac, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
          }
          
          #[test]
          fn test_routing_decision() {
              let mut table = RoutingTable::new();
              
              // Add routes
              table.add_route(RouteEntry::new(
                  Ipv4Addr(0xC0A80000),  // 192.168.0.0
                  Ipv4Addr(0xFFFFFF00),  // 255.255.255.0
                  String::from("eth0"),
              ));
              
              table.add_default_gateway(
                  Ipv4Addr(0xC0A80001),  // 192.168.0.1
                  String::from("eth0"),
                  100,
              );
              
              // Lookup destination in subnet
              let route = table.lookup(Ipv4Addr(0xC0A8000A)).unwrap();  // 192.168.0.10
              assert_eq!(route.interface, "eth0");
              assert!(route.gateway.is_none());  // Directly connected
              
              // Lookup destination requiring gateway
              let route = table.lookup(Ipv4Addr(0x08080808)).unwrap();  // 8.8.8.8
              assert_eq!(route.interface, "eth0");
              assert_eq!(route.gateway.unwrap().0, 0xC0A80001);
          }
      }
      ```
      
      **Verification:** `cargo test --test network_stack_integration` passes

---

### Phase 9: Testing, Documentation & RFC Compliance Verification

**Rationale:** Ensure all implementations conform to RFCs, have comprehensive test coverage, and are documented per SigmaOS standards.

**Dependencies:** All previous phases complete

- [ ] 29. **Add comprehensive unit tests for all modules**
      Achieve >80% code coverage for network stack.
      
      **Files:** All src/network/*.rs files with `#[cfg(test)] mod tests { ... }`
      
      **Test coverage targets:**
      - tcp.rs: State machine tests, buffer tests, congestion control tests
      - udp.rs: Datagram tests, checksum tests, port binding tests
      - arp.rs: Cache tests, request/reply tests, expiration tests
      - routing.rs: Longest-prefix match tests, forwarding tests
      - socket.rs: API tests, capability tests
      
      **Run coverage analysis:**
      ```bash
      cargo install cargo-tarpaulin
      cargo tarpaulin --out Html --output-dir coverage --lib --exclude-files 'tests/*'
      ```
      
      **Verification:** Coverage report shows >80% for src/network/

- [ ] 30. **Write RFC compliance documentation**
      Document which RFC sections are implemented and which are deferred.
      
      **Files:** Create docs/network/rfc-compliance.md
      
      **Content structure:**
      ```markdown
      # SigmaOS Network Stack RFC Compliance
      
      ## RFC 793 - Transmission Control Protocol (TCP)
      
      ### Implemented:
      - ✅ Section 3.2: Terminology and state machine (all 11 states)
      - ✅ Section 3.4: Connection establishment (three-way handshake)
      - ✅ Section 3.5: Connection termination (four-way close)
      - ✅ Section 3.6: Precedence and security (deferred - no precedence bits used)
      - ✅ Section 3.7: Data communication (send/receive)
      
      ### Partially Implemented:
      - ⚠️ Section 3.3: Sequence numbers (implemented but no wraparound tests yet)
      - ⚠️ Section 3.8: Interfaces (socket API partial, no full POSIX compatibility)
      
      ### Not Implemented:
      - ❌ Push function (PSH flag accepted but no forced delivery)
      - ❌ Urgent data (URG flag parsed but no out-of-band delivery)
      
      ## RFC 5681 - TCP Congestion Control
      
      ### Implemented:
      - ✅ Section 3.1: Slow Start
      - ✅ Section 3.2: Congestion Avoidance
      - ✅ Section 3.3: Fast Retransmit
      - ✅ Section 3.4: Fast Recovery
      
      ## RFC 8312 - CUBIC for Fast Long-Distance Networks
      
      ### Implemented:
      - ✅ Section 4.1: CUBIC Congestion Control Function
      - ✅ Section 4.2: CUBIC Multiplicative Decrease
      - ✅ Section 4.3: TCP-Friendly Region
      
      ## RFC 768 - User Datagram Protocol (UDP)
      
      ### Implemented:
      - ✅ Complete (UDP header, checksum, send/receive)
      
      ## RFC 826 - Address Resolution Protocol (ARP)
      
      ### Implemented:
      - ✅ Request/Reply mechanism
      - ✅ Cache with TTL
      
      ### Not Implemented:
      - ❌ Proxy ARP
      - ❌ Gratuitous ARP (can be added easily)
      
      ## RFC 4861 - IPv6 Neighbor Discovery
      
      ### Implemented:
      - ✅ Neighbor Solicitation (NS)
      - ✅ Neighbor Advertisement (NA)
      
      ### Not Implemented:
      - ❌ Router Discovery (RS/RA)
      - ❌ Redirect
      - ❌ Duplicate Address Detection (DAD)
      - ❌ SLAAC (Stateless Address Autoconfiguration)
      ```
      
      **Verification:** Manual review by maintainer

- [ ] 31. **Add inline documentation for public APIs**
      Ensure all public functions, structs, and enums have rustdoc comments.
      
      **Files:** All src/network/*.rs
      
      **Documentation standards:**
      - Every public item must have `///` doc comment
      - Functions: describe parameters, return values, errors, examples
      - Structs: describe purpose, invariants, thread-safety
      - Use `# Examples` sections for complex APIs
      
      **Example:**
      ```rust
      /// Creates a new TCP socket bound to the specified local port.
      ///
      /// # Arguments
      ///
      /// * `local_port` - The port number to bind to (0 for ephemeral port allocation)
      ///
      /// # Returns
      ///
      /// A new `TcpSocket` in the `Closed` state.
      ///
      /// # Examples
      ///
      /// ```
      /// use sigmaos::network::tcp::TcpSocket;
      ///
      /// let socket = TcpSocket::new(8080);
      /// assert_eq!(socket.local_port, 8080);
      /// ```
      pub fn new(local_port: u16) -> Self { ... }
      ```
      
      **Verification:** `cargo doc --no-deps --open` generates docs without warnings

- [ ] 32. **Add performance benchmarks**
      Measure throughput and latency for critical paths.
      
      **Files:** Create benches/network_benchmarks.rs
      
      **Benchmark scenarios:**
      ```rust
      use criterion::{black_box, criterion_group, criterion_main, Criterion};
      use sigmaos::network::*;
      
      fn tcp_segment_parse_benchmark(c: &mut Criterion) {
          c.bench_function("tcp_segment_parse", |b| {
              let data = [0u8; 60];  // Minimal TCP segment
              b.iter(|| {
                  TcpSegment::parse(black_box(&data))
              });
          });
      }
      
      fn udp_checksum_benchmark(c: &mut Criterion) {
          c.bench_function("udp_checksum_1500", |b| {
              let data = [0u8; 1500];
              b.iter(|| {
                  compute_udp_checksum(
                      black_box([192, 168, 1, 1]),
                      black_box([192, 168, 1, 2]),
                      black_box(&data[..8]),
                      black_box(&data[8..]),
                  )
              });
          });
      }
      
      fn routing_table_lookup_benchmark(c: &mut Criterion) {
          let mut table = RoutingTable::new();
          // Add 100 routes
          for i in 0..100 {
              table.add_route(RouteEntry::new(
                  Ipv4Addr((192 << 24) | (168 << 16) | (i << 8)),
                  Ipv4Addr(0xFFFFFF00),
                  format!("eth{}", i),
              ));
          }
          
          c.bench_function("routing_lookup_100_routes", |b| {
              b.iter(|| {
                  table.lookup(black_box(Ipv4Addr((192 << 24) | (168 << 16) | (50 << 8) | 10)))
              });
          });
      }
      
      criterion_group!(benches, tcp_segment_parse_benchmark, udp_checksum_benchmark, routing_table_lookup_benchmark);
      criterion_main!(benches);
      ```
      
      **Add to Cargo.toml:**
      ```toml
      [dev-dependencies]
      criterion = "0.5"
      
      [[bench]]
      name = "network_benchmarks"
      harness = false
      ```
      
      **Verification:** `cargo bench` runs without errors, produces performance report

- [ ] 33. **Update FEATURE_STATUS.toml**
      Record implementation status per SigmaOS governance.
      
      **Files:** FEATURE_STATUS.toml (in repo root)
      
      **Add entries:**
      ```toml
      [[feature]]
      name = "TCP State Machine"
      status = "Integrated"
      module = "src/network/tcp.rs"
      tests = "src/network/tcp.rs::tests, tests/network_stack_integration.rs::test_tcp_loopback_connection"
      rfc = "RFC 793"
      pr = "#XXX"  # Fill in PR number after merge
      
      [[feature]]
      name = "TCP Congestion Control (CUBIC)"
      status = "Integrated"
      module = "src/network/tcp.rs"
      tests = "src/network/tcp.rs::tests::test_congestion_cubic"
      rfc = "RFC 8312"
      pr = "#XXX"
      
      [[feature]]
      name = "UDP Datagram Service"
      status = "Integrated"
      module = "src/network/udp.rs"
      tests = "src/network/udp.rs::tests, tests/network_stack_integration.rs::test_udp_sendto_recvfrom"
      rfc = "RFC 768"
      pr = "#XXX"
      
      [[feature]]
      name = "ARP Address Resolution"
      status = "Integrated"
      module = "src/network/arp.rs"
      tests = "tests/network_stack_integration.rs::test_arp_resolution"
      rfc = "RFC 826"
      pr = "#XXX"
      
      [[feature]]
      name = "IPv6 Neighbor Discovery (NDP)"
      status = "Unit-tested"
      module = "src/network/ndp.rs"
      tests = "src/network/ndp.rs::tests"
      rfc = "RFC 4861 (partial - NS/NA only)"
      pr = "#XXX"
      notes = "Router Discovery and SLAAC not implemented"
      
      [[feature]]
      name = "Routing Table & IPv4 Forwarding"
      status = "Integrated"
      module = "src/network/routing.rs"
      tests = "tests/network_stack_integration.rs::test_routing_decision"
      rfc = "RFC 1812 (partial)"
      pr = "#XXX"
      
      [[feature]]
      name = "Socket API Layer"
      status = "Integrated"
      module = "src/network/socket.rs"
      tests = "src/network/socket.rs::tests"
      rfc = "POSIX sockets (partial compatibility)"
      pr = "#XXX"
      ```
      
      **Verification:** Manual review, TOML syntax validation

---

## Summary & Verification Strategy

### Build Verification
After each phase:
```bash
cargo check --lib                      # Verify compilation
cargo clippy --all-targets -- -D warnings  # Lint check
cargo test --lib                       # Run unit tests
cargo test --test network_stack_integration  # Run integration tests (Phase 8+)
```

### Final Verification (All Phases Complete)
```bash
# 1. No std:: violations
grep -r "use std::" src/network/ | wc -l
# Expected: 0

# 2. All tests pass
cargo test --lib
cargo test --test network_stack_integration

# 3. Documentation builds
cargo doc --no-deps --open

# 4. Benchmarks run
cargo bench

# 5. Coverage analysis
cargo tarpaulin --out Html --lib --exclude-files 'tests/*'
# Expected: >80% coverage

# 6. Lock order documentation exists
grep -r "Lock order:" src/network/
# Each lock acquisition must document order
```

### Integration with Existing Kernel
- All network modules must use `#![no_std]` and only `core::`/`alloc::`
- Socket API must integrate with capability system (Phase 8 step 27)
- Network syscalls in `src/syscalls/` must dispatch to socket API
- Device drivers must call into TCP/IP stack for packet processing
- Network stack must be initialized during kernel boot sequence

### Known Limitations & Future Work
1. **IPv6 Support:** Currently minimal (NDP NS/NA only). Full IPv6 requires:
   - IPv6 header parsing and forwarding
   - ICMPv6 implementation
   - Router Discovery and SLAAC
   - DHCPv6

2. **Advanced TCP Features:** Not implemented:
   - SACK (Selective Acknowledgment)
   - TCP timestamps
   - Window scaling
   - TCP Fast Open (TFO)

3. **Hardware Offload:** No support for:
   - TSO (TCP Segmentation Offload)
   - LRO (Large Receive Offload)
   - Checksum offload

4. **Network Namespaces:** Partial support exists in tests but needs kernel integration

5. **Security:** TLS/DTLS must be implemented in userspace or separate kernel module

---

## Dependency Graph

```
Phase 1 (no_std fixes)
  ├─→ Phase 2 (TCP State Machine)
  │     └─→ Phase 3 (TCP Data Transfer)
  │           └─→ Phase 4 (TCP Congestion)
  ├─→ Phase 5 (UDP)
  ├─→ Phase 6 (ARP/NDP)
  └─→ Phase 7 (Routing)

Phase 2, 3, 4, 5, 6, 7
  └─→ Phase 8 (Socket API Integration)
        └─→ Phase 9 (Testing & Documentation)
```

**Critical Path:** Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 8 → Phase 9

**Can be parallelized:** Phase 5, 6, 7 can proceed independently after Phase 1

---

## Estimated Effort (Lines of Code)

| Phase | Estimated LOC | Complexity |
|-------|---------------|------------|
| Phase 1: no_std fixes | 200 (mostly search/replace) | Low |
| Phase 2: TCP State Machine | 400 | High |
| Phase 3: TCP Data Transfer | 500 | High |
| Phase 4: TCP Congestion | 600 | Very High |
| Phase 5: UDP | 300 | Medium |
| Phase 6: ARP/NDP | 400 | Medium |
| Phase 7: Routing | 200 | Low |
| Phase 8: Socket API | 500 | High |
| Phase 9: Testing & Docs | 800 (tests) | Medium |
| **Total** | **~3,900 LOC** | |

**Timeline Estimate:** 4-6 weeks for single developer, 2-3 weeks for team of 2-3

---

## References

- **RFC 793:** Transmission Control Protocol (TCP)
- **RFC 5681:** TCP Congestion Control
- **RFC 8312:** CUBIC for Fast Long-Distance Networks
- **RFC 768:** User Datagram Protocol (UDP)
- **RFC 826:** Address Resolution Protocol (ARP)
- **RFC 4861:** Neighbor Discovery for IPv6
- **RFC 1122:** Requirements for Internet Hosts
- **Linux Kernel:** net/ipv4/tcp*.c, net/ipv4/udp.c, net/core/
- **FreeBSD:** sys/netinet/tcp_*, sys/netinet/udp_usrreq.c

---

**END OF PLAN**
