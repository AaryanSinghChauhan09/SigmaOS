# Linux Mint Warpinator Agent

## Role
The **Linux Mint Warpinator Agent** (`SigmaWarpinatorAgent`) provides secure, zero-configuration local network file sharing and peer-to-peer discovery inspired by Linux Mint's `warpinator` (https://github.com/linuxmint/warpinator).

## Mission & Purpose
- Automatic peer discovery over local Wi-Fi and Ethernet subnets using mDNS and broadcast announcements.
- Secure, encrypted file transfer sessions using mutual authentication and optional PIN pairing.
- High-throughput asynchronous chunked streaming capable of saturating 1Gbps and 10Gbps local LAN links.
- Real-time progress monitoring, transfer pause/resume, and SHA-256 integrity checks.

## Source Implementation
- Core Engine: `src/network/warpinator_engine.rs`
- Network Subsystem: `src/network/mod.rs`

## Public API & Invariant Contract
```rust
pub struct WarpinatorPeer {
    pub uuid: String,
    pub hostname: String,
    pub display_name: String,
    pub ip_address: [u8; 4],
    pub port: u16,
}

pub struct SovereignWarpinatorEngine {
    pub fn register_peer(&mut self, peer: WarpinatorPeer);
    pub fn initiate_send(&mut self, peer_uuid: &str, file_name: &str, file_size: u64) -> Result<u64, &'static str>;
    pub fn advance_transfer(&mut self, session_id: u64, chunk_len: u64) -> Result<bool, &'static str>;
}
```

## Repeatable AI Execution Checklist
- [ ] Test standalone with `rustc --test src/network/warpinator_engine.rs --edition 2021`.
- [ ] Verify discovery handles dynamically joining and departing peers cleanly.
- [ ] Ensure transfer state machine transitions smoothly from InProgress to Completed.
- [ ] Eliminate unnecessary allocations during high-frequency chunk streaming.
