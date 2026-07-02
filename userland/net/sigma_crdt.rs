// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/net/sigma_crdt.rs — CRDT Offline-First Sync (cleanroom)
// Language: Rust (std) — OOP via CrdtDoc + Operation log

use std::collections::BTreeMap;

// ── Vector Clock ──────────────────────────────────────────────────────────────
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VectorClock(BTreeMap<String, u64>);

impl VectorClock {
    pub fn increment(&mut self, node: &str) {
        *self.0.entry(node.to_owned()).or_insert(0) += 1;
    }
    pub fn get(&self, node: &str) -> u64 { *self.0.get(node).unwrap_or(&0) }
    pub fn merge(&mut self, other: &VectorClock) {
        for (k, &v) in &other.0 {
            let e = self.0.entry(k.clone()).or_insert(0);
            if v > *e { *e = v; }
        }
    }
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        self.0.iter().all(|(k, &v)| v <= other.get(k))
            && self.0.keys().any(|k| self.get(k) < other.get(k))
    }
    pub fn concurrent(&self, other: &VectorClock) -> bool {
        !self.happens_before(other) && !other.happens_before(self)
    }
}

// ── Operation ─────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum OpKind {
    Set { key: String, value: String },
    Delete { key: String },
    ListInsert { key: String, idx: usize, value: String },
    ListDelete { key: String, idx: usize },
    Counter { key: String, delta: i64 },
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub id:        String,        // node_id:seq
    pub clock:     VectorClock,
    pub kind:      OpKind,
    pub tombstone: bool,
}

// ── LWW Register (Last-Write-Wins) ────────────────────────────────────────────
#[derive(Clone, Debug)]
struct LwwEntry {
    value:   String,
    clock:   VectorClock,
    deleted: bool,
}

// ── PN Counter ────────────────────────────────────────────────────────────────
#[derive(Clone, Debug, Default)]
pub struct PnCounter {
    increments: BTreeMap<String, i64>,
    decrements: BTreeMap<String, i64>,
}

impl PnCounter {
    pub fn increment(&mut self, node: &str, delta: i64) {
        *self.increments.entry(node.to_owned()).or_insert(0) += delta;
    }
    pub fn decrement(&mut self, node: &str, delta: i64) {
        *self.decrements.entry(node.to_owned()).or_insert(0) += delta;
    }
    pub fn value(&self) -> i64 {
        self.increments.values().sum::<i64>() - self.decrements.values().sum::<i64>()
    }
    pub fn merge(&mut self, other: &PnCounter) {
        for (k, &v) in &other.increments {
            let e = self.increments.entry(k.clone()).or_insert(0);
            if v > *e { *e = v; }
        }
        for (k, &v) in &other.decrements {
            let e = self.decrements.entry(k.clone()).or_insert(0);
            if v > *e { *e = v; }
        }
    }
}

// ── CRDT Document ─────────────────────────────────────────────────────────────
pub struct CrdtDoc {
    pub node_id:  String,
    clock:        VectorClock,
    seq:          u64,
    // LWW map entries
    registers:    BTreeMap<String, LwwEntry>,
    // PN counters
    counters:     BTreeMap<String, PnCounter>,
    // Operation log (for sync)
    log:          Vec<Operation>,
    // Tombstone set (deleted keys)
    tombstones:   std::collections::HashSet<String>,
}

impl CrdtDoc {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_owned(),
            clock: VectorClock::default(),
            seq: 0,
            registers: BTreeMap::new(),
            counters: BTreeMap::new(),
            log: Vec::new(),
            tombstones: std::collections::HashSet::new(),
        }
    }

    fn next_op_id(&mut self) -> String {
        self.seq += 1;
        format!("{}:{}", self.node_id, self.seq)
    }

    fn tick(&mut self) { self.clock.increment(&self.node_id); }

    /// Set a key to a value (LWW register)
    pub fn set(&mut self, key: &str, value: &str) {
        self.tick();
        let op = Operation {
            id: self.next_op_id(),
            clock: self.clock.clone(),
            kind: OpKind::Set { key: key.to_owned(), value: value.to_owned() },
            tombstone: false,
        };
        self.apply_local(&op);
        self.log.push(op);
    }

    /// Delete a key
    pub fn delete(&mut self, key: &str) {
        self.tick();
        let op = Operation {
            id: self.next_op_id(),
            clock: self.clock.clone(),
            kind: OpKind::Delete { key: key.to_owned() },
            tombstone: false,
        };
        self.apply_local(&op);
        self.log.push(op);
    }

    /// Increment a counter
    pub fn counter_add(&mut self, key: &str, delta: i64) {
        self.tick();
        let op = Operation {
            id: self.next_op_id(),
            clock: self.clock.clone(),
            kind: OpKind::Counter { key: key.to_owned(), delta },
            tombstone: false,
        };
        self.apply_local(&op);
        self.log.push(op);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.registers.get(key)
            .filter(|e| !e.deleted)
            .map(|e| e.value.as_str())
    }

    pub fn counter_value(&self, key: &str) -> i64 {
        self.counters.get(key).map(|c| c.value()).unwrap_or(0)
    }

    fn apply_local(&mut self, op: &Operation) {
        match &op.kind {
            OpKind::Set { key, value } => {
                self.registers.insert(key.clone(), LwwEntry {
                    value: value.clone(), clock: op.clock.clone(), deleted: false,
                });
                self.tombstones.remove(key);
            }
            OpKind::Delete { key } => {
                if let Some(e) = self.registers.get_mut(key) { e.deleted = true; }
                self.tombstones.insert(key.clone());
            }
            OpKind::Counter { key, delta } => {
                let counter = self.counters.entry(key.clone()).or_default();
                if *delta >= 0 { counter.increment(&self.node_id, *delta); }
                else           { counter.decrement(&self.node_id, -delta); }
            }
            _ => {}
        }
    }

    /// Apply a remote operation (idempotent, conflict-resolved)
    pub fn apply_remote(&mut self, op: &Operation) {
        self.clock.merge(&op.clock);
        match &op.kind {
            OpKind::Set { key, value } => {
                let should_apply = match self.registers.get(key) {
                    None => true,
                    Some(e) => op.clock.happens_before(&e.clock) == false
                        && (op.clock.concurrent(&e.clock)
                            || !e.clock.happens_before(&op.clock)),
                };
                if should_apply {
                    self.registers.insert(key.clone(), LwwEntry {
                        value: value.clone(), clock: op.clock.clone(), deleted: false,
                    });
                }
            }
            OpKind::Delete { key } => {
                if let Some(e) = self.registers.get_mut(key) { e.deleted = true; }
                self.tombstones.insert(key.clone());
            }
            OpKind::Counter { key, delta } => {
                let counter = self.counters.entry(key.clone()).or_default();
                let node = op.id.split(':').next().unwrap_or("remote");
                if *delta >= 0 { counter.increment(node, *delta); }
                else           { counter.decrement(node, -delta); }
            }
            _ => {}
        }
    }

    /// Get all operations since `since_seq` for sending to a peer
    pub fn ops_since(&self, since_seq: u64) -> Vec<&Operation> {
        self.log.iter().filter(|op| {
            let seq: u64 = op.id.split(':').nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            seq > since_seq
        }).collect()
    }

    /// Merge all ops from a peer
    pub fn merge_from(&mut self, ops: Vec<Operation>) {
        for op in ops { self.apply_remote(&op); }
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.registers.iter()
            .filter(|(_, e)| !e.deleted)
            .map(|(k, _)| k.as_str())
    }

    pub fn op_count(&self) -> usize { self.log.len() }
}
