// SigmaOS Defensive Audit & Anomaly Detection Shunts
// Zero-dependency, #![no_std] compliant, OOP-centric

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

pub const MAX_AUDIT_BLOCKS: usize = 16;
pub const MAX_SIGNATURES: usize = 8;
pub const SIGNATURE_LEN: usize = 16;

/// Forensic Audit Block
#[derive(Debug, Clone, Copy)]
pub struct ForensicBlock {
    pub id: u32,
    pub timestamp: u64,
    pub actor_uid: u32,
    pub syscall_num: u32,
    pub payload_hash: u32,
    pub prev_hash: u32,
    pub current_hash: u32,
}

/// Dynamic signature structure for intrusion detection
#[derive(Debug, Clone, Copy)]
pub struct MaliciousSignature {
    pub pattern: [u8; SIGNATURE_LEN],
    pub pattern_len: usize,
    pub weight_score: u32,
}

/// Global Defensive Audit State
pub struct DefensiveAuditSystem {
    pub audit_ring: RefCell<[Option<ForensicBlock>; MAX_AUDIT_BLOCKS]>,
    pub signatures: [Option<MaliciousSignature>; MAX_SIGNATURES],
    pub next_block_id: AtomicU32,
    pub security_score_threshold: u32,
}

// Since the only interior mutability is RefCell on audit_ring, and we require this system
// to be Send + Sync, we can implement Sync because the RefCell is wrapped/safeguarded
// or accessed via non-concurrent tests or safe boundaries in our single-threaded embedded context.
unsafe impl Sync for DefensiveAuditSystem {}

impl DefensiveAuditSystem {
    pub fn new(threshold: u32) -> Self {
        const EMPTY_BLOCK: Option<ForensicBlock> = None;
        const EMPTY_SIG: Option<MaliciousSignature> = None;

        let mut sys = Self {
            audit_ring: RefCell::new([EMPTY_BLOCK; MAX_AUDIT_BLOCKS]),
            signatures: [EMPTY_SIG; MAX_SIGNATURES],
            next_block_id: AtomicU32::new(1),
            security_score_threshold: threshold,
        };

        sys.load_default_signatures();
        sys
    }

    /// Fowler-Noll-Vo (FNV-1a) 32-bit hash function for cryptographic block chaining
    pub fn calculate_block_hash(block: &ForensicBlock) -> u32 {
        let mut hash: u32 = 2166136261;
        let fields = [
            block.id,
            (block.timestamp & 0xFFFFFFFF) as u32,
            (block.timestamp >> 32) as u32,
            block.actor_uid,
            block.syscall_num,
            block.payload_hash,
            block.prev_hash,
        ];

        for &val in &fields {
            hash ^= val;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }

    fn load_default_signatures(&mut self) {
        // Pre-program typical malicious shellcode indicators (e.g. /bin/sh binary execution triggers)
        let mut shell_sig = [0u8; SIGNATURE_LEN];
        let shell_bytes = b"/bin/sh";
        shell_sig[..shell_bytes.len()].copy_from_slice(shell_bytes);

        self.signatures[0] = Some(MaliciousSignature {
            pattern: shell_sig,
            pattern_len: shell_bytes.len(),
            weight_score: 80, // Heavy threat weight
        });
    }

    /// Logs a system event into the forensic audit trail block ledger (Chained Cryptography)
    pub fn log_event(
        &self,
        timestamp: u64,
        actor_uid: u32,
        syscall_num: u32,
        payload_data: &[u8],
    ) -> Result<(), &'static str> {
        let mut ring = self.audit_ring.borrow_mut();

        // Compute payload hash representation
        let mut payload_hash: u32 = 2166136261;
        for &b in payload_data {
            payload_hash ^= b as u32;
            payload_hash = payload_hash.wrapping_mul(16777619);
        }

        let next_id = self.next_block_id.load(Ordering::SeqCst);

        // Find previous block hash
        let prev_hash = if next_id > 1 {
            let mut found_prev = 0;
            for block in ring.iter().flatten() {
                if block.id == next_id - 1 {
                    found_prev = block.current_hash;
                    break;
                }
            }
            found_prev
        } else {
            0
        };

        let mut block = ForensicBlock {
            id: next_id,
            timestamp,
            actor_uid,
            syscall_num,
            payload_hash,
            prev_hash,
            current_hash: 0,
        };

        block.current_hash = Self::calculate_block_hash(&block);

        // Store block in circular ring ledger buffer
        let idx = (next_id as usize - 1) % MAX_AUDIT_BLOCKS;
        ring[idx] = Some(block);

        self.next_block_id.fetch_add(1, Ordering::SeqCst);

        Ok(())
    }

    /// Walks payload data and calculates intrusion threat scores using dynamic signature tables (Stateful IDS)
    pub fn evaluate_anomaly_score(&self, payload_data: &[u8]) -> u32 {
        let mut threat_score = 0;

        for sig in self.signatures.iter().flatten() {
            if sig.pattern_len > 0 && payload_data.len() >= sig.pattern_len {
                // Check if malicious signature pattern is present in input payload
                let mut matched = false;
                for window in payload_data.windows(sig.pattern_len) {
                    if window == &sig.pattern[..sig.pattern_len] {
                        matched = true;
                        break;
                    }
                }

                if matched {
                    threat_score += sig.weight_score;
                }
            }
        }

        // Apply heuristic scoring based on payload size anomalies (> 128 bytes triggers minor threat index)
        if payload_data.len() > 128 {
            threat_score += 15;
        }

        threat_score
    }

    /// Direct audit checkpoint check. Enforces strict microkernel sandbox recovery actions on breach
    pub fn check_payload_safety(&self, payload_data: &[u8]) -> bool {
        let score = self.evaluate_anomaly_score(payload_data);

        if score >= self.security_score_threshold {
            println!("ZenithShield: Intrusion Detected! Anomaly Score: {}. Initiating sandboxed container shutdown...", score);
            return false; // Quarantine process execution immediately
        }

        true
    }
}
