// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sigma_blockchain_audit.rs — Blockchain-style tamper-proof audit log
// Novel: Kernel events form a cryptographic hash chain. Any tampering breaks
// the chain and is immediately detectable.
//
// Architecture:
//   Each log entry: timestamp | event | prev_hash | hash(all)
//   Chain: genesis → block_1 → block_2 → ... → block_N
//   Verify: recompute hashes, check chain integrity
//
// Use cases: compliance audit trail, kernel security events, AI-generated
//   command logging (required by audit regulations)
//
// Language: Rust (std)

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::VecDeque;

// ── Audit event types ─────────────────────────────────────────────────────
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum AuditEventType {
    KernelBoot          = 1,
    KernelShutdown      = 2,
    ProcessStart        = 10,
    ProcessExit         = 11,
    ProcessKill         = 12,
    SyscallDenied       = 20,   // pledge violation
    FileAccess          = 30,
    FileWrite           = 31,
    FileDeletion        = 32,
    NetworkConnect      = 40,
    NetworkListen       = 41,
    PackageInstall      = 50,
    PackageRemove       = 51,
    SecurityPolicyChange= 60,
    UserLogin           = 70,
    UserLogout          = 71,
    SudoCommand         = 72,
    AiCommandExecuted   = 80,   // sigma-agent generated command
    AiCommandDenied     = 81,
    WorkflowStarted     = 82,
    WorkflowCompleted   = 83,
    ConfigChanged       = 90,
    KernelParamChanged  = 91,
    Custom              = 255,
}

// ── Audit block ───────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct AuditBlock {
    pub index:      u64,
    pub timestamp:  u64,        // Unix timestamp nanoseconds
    pub event_type: u8,
    pub pid:        u32,
    pub uid:        u32,
    pub data:       String,     // event-specific data (JSON or text)
    pub prev_hash:  [u8; 32],
    pub hash:       [u8; 32],
    pub nonce:      u64,        // for future proof-of-work extension
}

impl AuditBlock {
    /// Compute hash of this block's content (excluding hash field)
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut input = Vec::new();
        input.extend_from_slice(&self.index.to_le_bytes());
        input.extend_from_slice(&self.timestamp.to_le_bytes());
        input.push(self.event_type);
        input.extend_from_slice(&self.pid.to_le_bytes());
        input.extend_from_slice(&self.uid.to_le_bytes());
        input.extend_from_slice(self.data.as_bytes());
        input.extend_from_slice(&self.prev_hash);
        input.extend_from_slice(&self.nonce.to_le_bytes());
        sha256_chain(&input)
    }

    pub fn is_valid(&self) -> bool {
        self.hash == self.compute_hash()
    }

    pub fn to_json(&self) -> String {
        let hash_hex: String = self.hash.iter().map(|b| format!("{:02x}", b)).collect();
        let prev_hex: String = self.prev_hash.iter().map(|b| format!("{:02x}", b)).collect();
        format!(
            r#"{{"index":{},"timestamp":{},"event_type":{},"pid":{},"uid":{},"data":"{}","prev_hash":"{}","hash":"{}"}}"#,
            self.index, self.timestamp, self.event_type, self.pid, self.uid,
            self.data.replace('"', "\\\""), prev_hex, hash_hex
        )
    }
}

// ── Blockchain audit log ───────────────────────────────────────────────────
pub struct AuditChain {
    pub chain:       VecDeque<AuditBlock>,
    pub max_blocks:  usize,
    pub persist_path: Option<String>,
}

impl AuditChain {
    pub fn new(max_blocks: usize, persist_path: Option<String>) -> Self {
        let mut chain = Self {
            chain: VecDeque::new(),
            max_blocks,
            persist_path,
        };
        // Genesis block
        chain.chain.push_back(AuditBlock {
            index: 0,
            timestamp: now_ns(),
            event_type: AuditEventType::KernelBoot as u8,
            pid: 0, uid: 0,
            data: "SigmaOS audit chain genesis".to_owned(),
            prev_hash: [0u8; 32],
            hash: [0u8; 32],
            nonce: 0,
        });
        if let Some(last) = chain.chain.back_mut() {
            last.hash = last.compute_hash();
        }
        chain
    }

    /// Append a new audit event to the chain
    pub fn append(&mut self, event_type: AuditEventType, pid: u32, uid: u32, data: &str) -> &AuditBlock {
        let prev_hash = self.chain.back().map(|b| b.hash).unwrap_or([0u8; 32]);
        let index     = self.chain.len() as u64;
        let mut block = AuditBlock {
            index,
            timestamp: now_ns(),
            event_type: event_type as u8,
            pid, uid,
            data: data.to_owned(),
            prev_hash,
            hash: [0u8; 32],
            nonce: 0,
        };
        block.hash = block.compute_hash();

        if self.chain.len() >= self.max_blocks {
            // Persist oldest block before removing (if path configured)
            if let (Some(path), Some(old)) = (&self.persist_path, self.chain.front()) {
                let line = old.to_json() + "\n";
                let _ = std::fs::OpenOptions::new()
                    .create(true).append(true).open(path)
                    .and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) });
            }
            self.chain.pop_front();
        }
        self.chain.push_back(block);
        self.chain.back().unwrap()
    }

    /// Verify entire chain integrity — O(n)
    pub fn verify(&self) -> ChainVerifyResult {
        let mut prev_hash = [0u8; 32];
        for (i, block) in self.chain.iter().enumerate() {
            // Check computed hash matches stored hash
            if block.hash != block.compute_hash() {
                return ChainVerifyResult::TamperedBlock {
                    index: block.index, reason: "hash mismatch".to_owned() };
            }
            // Check prev_hash chain linkage (skip genesis)
            if i > 0 && block.prev_hash != prev_hash {
                return ChainVerifyResult::BrokenChain {
                    index: block.index, reason: "prev_hash mismatch".to_owned() };
            }
            prev_hash = block.hash;
        }
        ChainVerifyResult::Valid { blocks: self.chain.len() }
    }

    /// Get last N events as JSON
    pub fn tail(&self, n: usize) -> Vec<String> {
        self.chain.iter().rev().take(n).rev().map(|b| b.to_json()).collect()
    }

    /// Search events by type
    pub fn filter(&self, event_type: u8) -> Vec<&AuditBlock> {
        self.chain.iter().filter(|b| b.event_type == event_type).collect()
    }

    /// Export full chain as JSONL
    pub fn export_jsonl(&self) -> String {
        self.chain.iter().map(|b| b.to_json()).collect::<Vec<_>>().join("\n")
    }

    /// Get chain root hash (last block's hash = commitment to full history)
    pub fn root_hash(&self) -> [u8; 32] {
        self.chain.back().map(|b| b.hash).unwrap_or([0u8; 32])
    }
}

#[derive(Debug)]
pub enum ChainVerifyResult {
    Valid        { blocks: usize },
    TamperedBlock{ index: u64, reason: String },
    BrokenChain  { index: u64, reason: String },
}

// ── Global audit chain (singleton, thread-safe in production) ─────────────
use std::sync::Mutex;

lazy_static::lazy_static! {
    // In no_std kernel: use a spinlock instead
    // In userland daemon: Mutex is fine
    pub static ref AUDIT_CHAIN: Mutex<AuditChain> = Mutex::new(
        AuditChain::new(10_000, Some("/var/log/sigma/audit.jsonl".to_owned()))
    );
}

pub fn audit_log(event_type: AuditEventType, pid: u32, uid: u32, data: &str) {
    if let Ok(mut chain) = AUDIT_CHAIN.lock() {
        chain.append(event_type, pid, uid, data);
    }
}

// ── SHA-256 chain hash (single-block, optimised for audit log) ────────────
fn sha256_chain(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
        0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
        0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
        0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
        0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
        0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
        0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
    ];
    let mut padded = data.to_vec();
    let tl = data.len() as u64;
    padded.push(0x80);
    while padded.len() % 64 != 56 { padded.push(0); }
    padded.extend_from_slice(&(tl*8).to_be_bytes());
    for chunk in padded.chunks(64) {
        let mut w=[0u32;64];
        for i in 0..16{w[i]=u32::from_be_bytes([chunk[i*4],chunk[i*4+1],chunk[i*4+2],chunk[i*4+3]]);}
        for i in 16..64{
            let s0=w[i-15].rotate_right(7)^w[i-15].rotate_right(18)^(w[i-15]>>3);
            let s1=w[i-2].rotate_right(17)^w[i-2].rotate_right(19)^(w[i-2]>>10);
            w[i]=w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }
        let(mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut hh)=(h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7]);
        for i in 0..64{
            let s1=(e.rotate_right(6))^(e.rotate_right(11))^(e.rotate_right(25));
            let ch=(e&f)^((!e)&g);
            let t1=hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0=(a.rotate_right(2))^(a.rotate_right(13))^(a.rotate_right(22));
            let maj=(a&b)^(a&c)^(b&c);
            let t2=s0.wrapping_add(maj);
            hh=g;g=f;f=e;e=d.wrapping_add(t1);d=c;c=b;b=a;a=t1.wrapping_add(t2);
        }
        let add=[a,b,c,d,e,f,g,hh];
        for i in 0..8{h[i]=h[i].wrapping_add(add[i]);}
    }
    let mut out=[0u8;32];
    for(i,&v)in h.iter().enumerate(){out[i*4..(i+1)*4].copy_from_slice(&v.to_be_bytes());}
    out
}

fn now_ns() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}

// ── CLI ───────────────────────────────────────────────────────────────────
pub fn audit_cmd(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("verify") => {
            let chain = AUDIT_CHAIN.lock().unwrap();
            match chain.verify() {
                ChainVerifyResult::Valid { blocks } =>
                    println!("✓ Audit chain valid ({} blocks)", blocks),
                ChainVerifyResult::TamperedBlock { index, reason } =>
                    println!("✗ TAMPERED BLOCK at index {}: {}", index, reason),
                ChainVerifyResult::BrokenChain { index, reason } =>
                    println!("✗ BROKEN CHAIN at index {}: {}", index, reason),
            }
        }
        Some("tail") => {
            let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);
            let chain = AUDIT_CHAIN.lock().unwrap();
            for line in chain.tail(n) { println!("{}", line); }
        }
        Some("root") => {
            let chain = AUDIT_CHAIN.lock().unwrap();
            let root: String = chain.root_hash().iter().map(|b| format!("{:02x}", b)).collect();
            println!("Chain root hash: {}", root);
        }
        Some("log") if args.len() > 2 => {
            audit_log(AuditEventType::Custom, 0, 0, &args[2..].join(" "));
            println!("✓ Logged");
        }
        Some("export") => {
            let chain = AUDIT_CHAIN.lock().unwrap();
            println!("{}", chain.export_jsonl());
        }
        _ => println!("sigma-audit — Blockchain-style tamper-proof audit log\n\
            Usage:\n\
            sigma-audit verify         Verify chain integrity\n\
            sigma-audit tail [n]       Show last N events\n\
            sigma-audit root           Show chain root hash (commitment)\n\
            sigma-audit log <message>  Log a custom event\n\
            sigma-audit export         Export full chain as JSONL\n\
            \nEvery event is hashed and chained. Tampering breaks the chain\n\
            and is immediately detected by sigma-audit verify."),
    }
}
