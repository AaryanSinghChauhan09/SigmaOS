//! SigmaOS Unix Kernel Primitives
//!
//! Production-oriented reimplementations of foundational Linux and BSD kernel
//! mechanisms. Concepts are absorbed; code is original and capability-aware.
//!
//! ## Linux heritage
//! - Red-black trees (CFS runqueues, page cache indexes)
//! - Seqlocks (lock-free readers, rare writers)
//! - Wait queues (sleep / wakeup)
//! - Futexes (fast userspace locking)
//! - Softirq-style deferred work hooks
//! - Radix trees / XArray-style sparse maps
//! - Per-CPU variables
//! - Netlink-style kernel↔userspace messaging
//! - RCU epochs (safe memory reclamation for readers)
//!
//! ## BSD heritage
//! - kqueue / kevent (scalable event notification)
//! - sysctl OID tree (runtime tunables)
//! - UMA zones (typed slab allocation)
//! - Callouts (one-shot / periodic timers)
//! - sbuf (safe kernel string buffers)
//! - WITNESS (lock-order validation)
//! - Turnstiles (priority inheritance)
//! - Capsicum capability rights on FDs
//! - GEOM-style I/O transform chains
//! - SMR (safe memory reclamation epochs)

#![allow(dead_code)]
#![allow(unused_imports)]
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

use core::cmp::Ordering as CmpOrdering;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};

// ═══════════════════════════════════════════════════════════════════════════
// 1. RED-BLACK TREE  (Linux: lib/rbtree.c — CFS, timers, page cache)
// ═══════════════════════════════════════════════════════════════════════════

/// Color of an RB-tree node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbColor {
    Red,
    Black,
}

/// Intrusive-style red-black tree keyed by `K`, storing values `V`.
/// Used as the backbone for CFS vruntime ordering and sorted kernel indexes.
#[derive(Debug, Clone)]
pub struct RbNode<K, V> {
    pub key: K,
    pub value: V,
    pub color: RbColor,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Option<usize>,
}

/// Arena-backed red-black tree (no recursive allocation; suitable for kernels).
#[derive(Debug)]
pub struct RbTree<K, V> {
    nodes: Vec<RbNode<K, V>>,
    root: Option<usize>,
    free: Vec<usize>,
    len: usize,
}

impl<K: Ord + Clone, V: Clone> RbTree<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            free: Vec::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn alloc_node(&mut self, key: K, value: V) -> usize {
        if let Some(idx) = self.free.pop() {
            self.nodes[idx] = RbNode {
                key,
                value,
                color: RbColor::Red,
                left: None,
                right: None,
                parent: None,
            };
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(RbNode {
                key,
                value,
                color: RbColor::Red,
                left: None,
                right: None,
                parent: None,
            });
            idx
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            let idx = self.alloc_node(key, value);
            self.nodes[idx].color = RbColor::Black;
            self.root = Some(idx);
            self.len = 1;
            return;
        }

        let mut cur = self.root.unwrap();
        loop {
            match key.cmp(&self.nodes[cur].key) {
                CmpOrdering::Equal | CmpOrdering::Less => {
                    if let Some(left) = self.nodes[cur].left {
                        cur = left;
                    } else {
                        let idx = self.alloc_node(key, value);
                        self.nodes[cur].left = Some(idx);
                        self.nodes[idx].parent = Some(cur);
                        self.len += 1;
                        self.insert_fixup(idx);
                        return;
                    }
                }
                CmpOrdering::Greater => {
                    if let Some(right) = self.nodes[cur].right {
                        cur = right;
                    } else {
                        let idx = self.alloc_node(key, value);
                        self.nodes[cur].right = Some(idx);
                        self.nodes[idx].parent = Some(cur);
                        self.len += 1;
                        self.insert_fixup(idx);
                        return;
                    }
                }
            }
        }
    }

    fn insert_fixup(&mut self, mut node: usize) {
        while let Some(parent) = self.nodes[node].parent {
            if self.nodes[parent].color == RbColor::Black {
                break;
            }
            let grand = match self.nodes[parent].parent {
                Some(g) => g,
                None => break,
            };
            let parent_is_left = self.nodes[grand].left == Some(parent);
            let uncle = if parent_is_left {
                self.nodes[grand].right
            } else {
                self.nodes[grand].left
            };

            if let Some(u) = uncle {
                if self.nodes[u].color == RbColor::Red {
                    self.nodes[parent].color = RbColor::Black;
                    self.nodes[u].color = RbColor::Black;
                    self.nodes[grand].color = RbColor::Red;
                    node = grand;
                    continue;
                }
            }

            if parent_is_left {
                if self.nodes[parent].right == Some(node) {
                    self.rotate_left(parent);
                    node = parent;
                    // parent is now the child
                    let p = self.nodes[node].parent.unwrap();
                    self.nodes[p].color = RbColor::Black;
                    self.nodes[grand].color = RbColor::Red;
                    self.rotate_right(grand);
                } else {
                    self.nodes[parent].color = RbColor::Black;
                    self.nodes[grand].color = RbColor::Red;
                    self.rotate_right(grand);
                }
            } else if self.nodes[parent].left == Some(node) {
                self.rotate_right(parent);
                node = parent;
                let p = self.nodes[node].parent.unwrap();
                self.nodes[p].color = RbColor::Black;
                self.nodes[grand].color = RbColor::Red;
                self.rotate_left(grand);
            } else {
                self.nodes[parent].color = RbColor::Black;
                self.nodes[grand].color = RbColor::Red;
                self.rotate_left(grand);
            }
            break;
        }
        if let Some(r) = self.root {
            self.nodes[r].color = RbColor::Black;
        }
    }

    fn rotate_left(&mut self, x: usize) {
        let y = match self.nodes[x].right {
            Some(y) => y,
            None => return,
        };
        self.nodes[x].right = self.nodes[y].left;
        if let Some(yl) = self.nodes[y].left {
            self.nodes[yl].parent = Some(x);
        }
        self.nodes[y].parent = self.nodes[x].parent;
        match self.nodes[x].parent {
            None => self.root = Some(y),
            Some(p) if self.nodes[p].left == Some(x) => self.nodes[p].left = Some(y),
            Some(p) => self.nodes[p].right = Some(y),
        }
        self.nodes[y].left = Some(x);
        self.nodes[x].parent = Some(y);
    }

    fn rotate_right(&mut self, y: usize) {
        let x = match self.nodes[y].left {
            Some(x) => x,
            None => return,
        };
        self.nodes[y].left = self.nodes[x].right;
        if let Some(xr) = self.nodes[x].right {
            self.nodes[xr].parent = Some(y);
        }
        self.nodes[x].parent = self.nodes[y].parent;
        match self.nodes[y].parent {
            None => self.root = Some(x),
            Some(p) if self.nodes[p].right == Some(y) => self.nodes[p].right = Some(x),
            Some(p) => self.nodes[p].left = Some(x),
        }
        self.nodes[x].right = Some(y);
        self.nodes[y].parent = Some(x);
    }

    /// Return the minimum key/value (leftmost) — CFS uses this for earliest vruntime.
    pub fn min(&self) -> Option<(&K, &V)> {
        let mut cur = self.root?;
        while let Some(l) = self.nodes[cur].left {
            cur = l;
        }
        Some((&self.nodes[cur].key, &self.nodes[cur].value))
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut cur = self.root?;
        loop {
            match key.cmp(&self.nodes[cur].key) {
                CmpOrdering::Equal => return Some(&self.nodes[cur].value),
                CmpOrdering::Less => cur = self.nodes[cur].left?,
                CmpOrdering::Greater => cur = self.nodes[cur].right?,
            }
        }
    }

    /// In-order walk collecting up to `limit` keys (for diagnostics / /proc).
    pub fn keys_inorder(&self, limit: usize) -> Vec<K> {
        let mut out = Vec::new();
        if let Some(r) = self.root {
            self.walk_inorder(r, &mut out, limit);
        }
        out
    }

    fn walk_inorder(&self, idx: usize, out: &mut Vec<K>, limit: usize) {
        if out.len() >= limit {
            return;
        }
        if let Some(l) = self.nodes[idx].left {
            self.walk_inorder(l, out, limit);
        }
        if out.len() < limit {
            out.push(self.nodes[idx].key.clone());
        }
        if let Some(r) = self.nodes[idx].right {
            self.walk_inorder(r, out, limit);
        }
    }
}

impl<K: Ord + Clone, V: Clone> Default for RbTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. SEQLOCK  (Linux: include/linux/seqlock.h)
// ═══════════════════════════════════════════════════════════════════════════

/// Sequence lock: readers never block; writers take exclusive access.
/// Readers retry if a write occurred during the critical section.
#[derive(Debug)]
pub struct SeqLock {
    sequence: AtomicUsize,
    writing: AtomicBool,
}

impl SeqLock {
    pub const fn new() -> Self {
        Self {
            sequence: AtomicUsize::new(0),
            writing: AtomicBool::new(false),
        }
    }

    pub fn write_begin(&self) {
        while self
            .writing
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        self.sequence.fetch_add(1, Ordering::Release); // odd = write in progress
    }

    pub fn write_end(&self) {
        self.sequence.fetch_add(1, Ordering::Release); // even again
        self.writing.store(false, Ordering::Release);
    }

    /// Start a read-side critical section; returns sequence to validate later.
    pub fn read_begin(&self) -> usize {
        loop {
            let s = self.sequence.load(Ordering::Acquire);
            if s & 1 == 0 {
                return s;
            }
            core::hint::spin_loop();
        }
    }

    /// Returns true if the read-side section was consistent (no concurrent write).
    pub fn read_retry(&self, start: usize) -> bool {
        core::sync::atomic::fence(Ordering::Acquire);
        self.sequence.load(Ordering::Relaxed) != start
    }
}

impl Default for SeqLock {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. WAIT QUEUES  (Linux: kernel/sched/wait.c)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitResult {
    Woken,
    Timeout,
    Interrupted,
}

/// A single waiter entry on a wait queue.
#[derive(Debug, Clone)]
pub struct WaitEntry {
    pub pid: u64,
    pub exclusive: bool,
    pub woken: bool,
}

/// Classic Linux wait_queue_head for sleep/wakeup.
#[derive(Debug, Default)]
pub struct WaitQueue {
    waiters: Vec<WaitEntry>,
    generation: u64,
}

impl WaitQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_waiter(&mut self, pid: u64, exclusive: bool) {
        self.waiters.push(WaitEntry {
            pid,
            exclusive,
            woken: false,
        });
    }

    pub fn remove_waiter(&mut self, pid: u64) {
        self.waiters.retain(|w| w.pid != pid);
    }

    /// Wake up to `nr` waiters (nr == usize::MAX → all non-exclusive + one exclusive).
    pub fn wake_up(&mut self, nr: usize) -> Vec<u64> {
        let mut woken = Vec::new();
        let mut remaining = nr;
        let mut i = 0;
        while i < self.waiters.len() && remaining > 0 {
            if !self.waiters[i].woken {
                self.waiters[i].woken = true;
                woken.push(self.waiters[i].pid);
                remaining -= 1;
                if self.waiters[i].exclusive {
                    break;
                }
            }
            i += 1;
        }
        self.waiters.retain(|w| !w.woken);
        self.generation = self.generation.wrapping_add(1);
        woken
    }

    pub fn wake_up_all(&mut self) -> Vec<u64> {
        self.wake_up(usize::MAX)
    }

    pub fn pending(&self) -> usize {
        self.waiters.len()
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. FUTEX  (Linux: kernel/futex.c)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutexOp {
    Wait,
    Wake,
    Requeue,
    CmpRequeue,
    WakeOp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutexError {
    WouldBlock,
    TimedOut,
    Invalid,
    Again,
}

/// Kernel-side futex hash bucket entry.
#[derive(Debug, Clone)]
pub struct FutexWaiter {
    pub pid: u64,
    pub uaddr: u64,
    pub bitset: u32,
}

/// Minimal futex subsystem: WAIT / WAKE on a userspace address key.
#[derive(Debug, Default)]
pub struct FutexTable {
    /// Keyed conceptually by (mm, uaddr); we use uaddr alone for host simulation.
    buckets: Vec<(u64, Vec<FutexWaiter>)>,
    wake_count: AtomicU64,
}

impl FutexTable {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            wake_count: AtomicU64::new(0),
        }
    }

    fn bucket_mut(&mut self, uaddr: u64) -> &mut Vec<FutexWaiter> {
        if let Some(pos) = self.buckets.iter().position(|(a, _)| *a == uaddr) {
            return &mut self.buckets[pos].1;
        }
        self.buckets.push((uaddr, Vec::new()));
        &mut self.buckets.last_mut().unwrap().1
    }

    /// FUTEX_WAIT: park if `*uaddr` still equals `expected`.
    pub fn wait(
        &mut self,
        pid: u64,
        uaddr: u64,
        current_val: u32,
        expected: u32,
        bitset: u32,
    ) -> Result<(), FutexError> {
        if current_val != expected {
            return Err(FutexError::WouldBlock);
        }
        self.bucket_mut(uaddr).push(FutexWaiter { pid, uaddr, bitset });
        Ok(())
    }

    /// FUTEX_WAKE: wake up to `nr` waiters matching bitset.
    pub fn wake(&mut self, uaddr: u64, nr: usize, bitset: u32) -> usize {
        let Some(pos) = self.buckets.iter().position(|(a, _)| *a == uaddr) else {
            return 0;
        };
        let waiters = &mut self.buckets[pos].1;
        let mut woken = 0usize;
        let mut i = 0;
        while i < waiters.len() && woken < nr {
            if waiters[i].bitset & bitset != 0 {
                waiters.remove(i);
                woken += 1;
            } else {
                i += 1;
            }
        }
        if waiters.is_empty() {
            self.buckets.remove(pos);
        }
        self.wake_count.fetch_add(woken as u64, Ordering::Relaxed);
        woken
    }

    pub fn waiter_count(&self, uaddr: u64) -> usize {
        self.buckets
            .iter()
            .find(|(a, _)| *a == uaddr)
            .map(|(_, w)| w.len())
            .unwrap_or(0)
    }

    pub fn total_wakes(&self) -> u64 {
        self.wake_count.load(Ordering::Relaxed)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. RCU EPOCH  (Linux: kernel/rcu — simplified QSBR/epoch form)
// ═══════════════════════════════════════════════════════════════════════════

/// Quiescent-state based RCU using global epochs.
#[derive(Debug)]
pub struct RcuEpoch {
    global: AtomicUsize,
    /// Per-CPU/reader nested read depth (host simulation: flat map by reader id).
    readers: Vec<(u64, usize)>, // (reader_id, nested_epoch or 0 = inactive)
}

impl RcuEpoch {
    pub fn new() -> Self {
        Self {
            global: AtomicUsize::new(1),
            readers: Vec::new(),
        }
    }

    pub fn read_lock(&mut self, reader_id: u64) {
        let epoch = self.global.load(Ordering::Acquire);
        if let Some(slot) = self.readers.iter_mut().find(|(id, _)| *id == reader_id) {
            slot.1 = epoch;
        } else {
            self.readers.push((reader_id, epoch));
        }
    }

    pub fn read_unlock(&mut self, reader_id: u64) {
        if let Some(slot) = self.readers.iter_mut().find(|(id, _)| *id == reader_id) {
            slot.1 = 0;
        }
    }

    /// Advance epoch; returns when no reader still holds the old epoch.
    /// Host simulation: non-blocking check (caller may retry).
    pub fn synchronize(&mut self) -> bool {
        let old = self.global.fetch_add(1, Ordering::AcqRel);
        !self.readers.iter().any(|(_, e)| *e == old)
    }

    pub fn current_epoch(&self) -> usize {
        self.global.load(Ordering::Relaxed)
    }
}

impl Default for RcuEpoch {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. RADIX / XARRAY  (Linux: lib/radix-tree.c, lib/xarray.c)
// ═══════════════════════════════════════════════════════════════════════════

/// Sparse key→value map inspired by Linux radix-tree / xarray.
/// Sorted vector of slots: O(log n) lookup, host-safe, no_std-friendly shape.
#[derive(Debug, Clone)]
pub struct SparseMap<T> {
    entries: Vec<(u64, T)>,
}

impl<T: Clone> SparseMap<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn insert(&mut self, index: u64, value: T) {
        match self.entries.binary_search_by_key(&index, |(k, _)| *k) {
            Ok(i) => self.entries[i].1 = value,
            Err(i) => self.entries.insert(i, (index, value)),
        }
    }

    pub fn get(&self, index: u64) -> Option<&T> {
        self.entries
            .binary_search_by_key(&index, |(k, _)| *k)
            .ok()
            .map(|i| &self.entries[i].1)
    }

    pub fn remove(&mut self, index: u64) -> Option<T> {
        if let Ok(i) = self.entries.binary_search_by_key(&index, |(k, _)| *k) {
            Some(self.entries.remove(i).1)
        } else {
            None
        }
    }

    pub fn range(&self, start: u64, end: u64) -> Vec<(u64, T)> {
        self.entries
            .iter()
            .filter(|(k, _)| *k >= start && *k < end)
            .cloned()
            .collect()
    }
}

impl<T: Clone> Default for SparseMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. PER-CPU VARIABLES  (Linux: include/linux/percpu.h)
// ═══════════════════════════════════════════════════════════════════════════

/// Per-CPU array of values — avoids cache-line bouncing on counters.
#[derive(Debug, Clone)]
pub struct PerCpu<T> {
    cpus: Vec<T>,
}

impl<T: Clone + Default> PerCpu<T> {
    pub fn new(ncpus: usize) -> Self {
        Self {
            cpus: vec![T::default(); ncpus.max(1)],
        }
    }

    pub fn get(&self, cpu: usize) -> Option<&T> {
        self.cpus.get(cpu)
    }

    pub fn get_mut(&mut self, cpu: usize) -> Option<&mut T> {
        self.cpus.get_mut(cpu)
    }

    pub fn ncpus(&self) -> usize {
        self.cpus.len()
    }

    /// Sum reduction for numeric counters (caller provides fold).
    pub fn fold<R, F>(&self, init: R, mut f: F) -> R
    where
        F: FnMut(R, &T) -> R,
    {
        self.cpus.iter().fold(init, |acc, v| f(acc, v))
    }
}

impl PerCpu<u64> {
    pub fn add(&mut self, cpu: usize, delta: u64) {
        if let Some(v) = self.cpus.get_mut(cpu) {
            *v = v.wrapping_add(delta);
        }
    }

    pub fn sum(&self) -> u64 {
        self.cpus.iter().copied().sum()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. KQUEUE / KEVENT  (FreeBSD: sys/kern/kern_event.c)
// ═══════════════════════════════════════════════════════════════════════════

pub const EV_ADD: u16 = 0x0001;
pub const EV_DELETE: u16 = 0x0002;
pub const EV_ENABLE: u16 = 0x0004;
pub const EV_DISABLE: u16 = 0x0008;
pub const EV_ONESHOT: u16 = 0x0010;
pub const EV_CLEAR: u16 = 0x0020;
pub const EV_EOF: u16 = 0x8000;
pub const EV_ERROR: u16 = 0x4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum KqFilter {
    Read = -1,
    Write = -2,
    Aio = -3,
    Vnode = -4,
    Proc = -5,
    Signal = -6,
    Timer = -7,
    User = -8,
}

#[derive(Debug, Clone)]
pub struct Kevent {
    pub ident: u64,
    pub filter: KqFilter,
    pub flags: u16,
    pub fflags: u32,
    pub data: i64,
    pub udata: u64,
}

#[derive(Debug, Clone)]
struct KqRegistration {
    event: Kevent,
    enabled: bool,
    triggered: bool,
    pending_data: i64,
}

/// FreeBSD-style kqueue multiplexer.
#[derive(Debug, Default)]
pub struct Kqueue {
    regs: Vec<KqRegistration>,
    id: u64,
}

impl Kqueue {
    pub fn new(id: u64) -> Self {
        Self {
            regs: Vec::new(),
            id,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    /// Apply changelist (EV_ADD / EV_DELETE / EV_ENABLE / EV_DISABLE).
    pub fn change(&mut self, changes: &[Kevent]) -> Result<(), &'static str> {
        for ch in changes {
            if ch.flags & EV_DELETE != 0 {
                self.regs
                    .retain(|r| !(r.event.ident == ch.ident && r.event.filter == ch.filter));
                continue;
            }
            if ch.flags & EV_ADD != 0 {
                if let Some(r) = self
                    .regs
                    .iter_mut()
                    .find(|r| r.event.ident == ch.ident && r.event.filter == ch.filter)
                {
                    r.event = ch.clone();
                    r.enabled = ch.flags & EV_DISABLE == 0;
                } else {
                    self.regs.push(KqRegistration {
                        event: ch.clone(),
                        enabled: ch.flags & EV_DISABLE == 0,
                        triggered: false,
                        pending_data: 0,
                    });
                }
                continue;
            }
            if let Some(r) = self
                .regs
                .iter_mut()
                .find(|r| r.event.ident == ch.ident && r.event.filter == ch.filter)
            {
                if ch.flags & EV_ENABLE != 0 {
                    r.enabled = true;
                }
                if ch.flags & EV_DISABLE != 0 {
                    r.enabled = false;
                }
            }
        }
        Ok(())
    }

    /// Note that an event has fired (from driver / VFS / signal path).
    pub fn note(&mut self, ident: u64, filter: KqFilter, data: i64) {
        for r in &mut self.regs {
            if r.event.ident == ident && r.event.filter == filter && r.enabled {
                r.triggered = true;
                r.pending_data = data;
            }
        }
    }

    /// Collect ready events (up to `maxevents`).
    pub fn poll(&mut self, maxevents: usize) -> Vec<Kevent> {
        let mut out = Vec::new();
        let mut remove = Vec::new();
        for (i, r) in self.regs.iter_mut().enumerate() {
            if out.len() >= maxevents {
                break;
            }
            if r.triggered && r.enabled {
                let mut ev = r.event.clone();
                ev.data = r.pending_data;
                out.push(ev);
                r.triggered = false;
                if r.event.flags & EV_ONESHOT != 0 {
                    remove.push(i);
                } else if r.event.flags & EV_CLEAR != 0 {
                    r.pending_data = 0;
                }
            }
        }
        for i in remove.into_iter().rev() {
            self.regs.remove(i);
        }
        out
    }

    pub fn registered_count(&self) -> usize {
        self.regs.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 9. SYSCTL  (BSD/Linux: hierarchical kernel tunables)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysctlType {
    Int,
    Long,
    String,
    Node,
    Proc, // computed on read
}

#[derive(Debug, Clone)]
pub enum SysctlValue {
    Int(i64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct SysctlOid {
    pub name: String,
    pub path: String,
    pub ty: SysctlType,
    pub value: SysctlValue,
    pub writable: bool,
    pub description: String,
}

/// Hierarchical sysctl registry (kern.*, vm.*, net.*, security.*).
#[derive(Debug)]
pub struct SysctlTree {
    oids: Vec<SysctlOid>,
    securelevel: i32,
}

impl SysctlTree {
    pub fn new() -> Self {
        let mut t = Self {
            oids: Vec::new(),
            securelevel: 1,
        };
        t.seed_defaults();
        t
    }

    fn seed_defaults(&mut self) {
        let defaults: &[(&str, SysctlType, SysctlValue, bool, &str)] = &[
            (
                "kern.ostype",
                SysctlType::String,
                SysctlValue::String("SigmaOS".into()),
                false,
                "Operating system type",
            ),
            (
                "kern.osrelease",
                SysctlType::String,
                SysctlValue::String("0.1.0".into()),
                false,
                "OS release",
            ),
            (
                "kern.hostname",
                SysctlType::String,
                SysctlValue::String("sigma".into()),
                true,
                "Hostname",
            ),
            (
                "kern.maxproc",
                SysctlType::Int,
                SysctlValue::Int(4096),
                true,
                "Maximum processes",
            ),
            (
                "kern.maxfiles",
                SysctlType::Int,
                SysctlValue::Int(65536),
                true,
                "Maximum open files",
            ),
            (
                "kern.securelevel",
                SysctlType::Int,
                SysctlValue::Int(1),
                false,
                "Securelevel (OpenBSD-style)",
            ),
            (
                "kern.ipc.somaxconn",
                SysctlType::Int,
                SysctlValue::Int(128),
                true,
                "Listen backlog",
            ),
            (
                "vm.overcommit",
                SysctlType::Int,
                SysctlValue::Int(0),
                true,
                "Memory overcommit mode",
            ),
            (
                "vm.swappiness",
                SysctlType::Int,
                SysctlValue::Int(60),
                true,
                "Swap tendency (Linux-inspired)",
            ),
            (
                "vm.max_map_count",
                SysctlType::Int,
                SysctlValue::Int(65530),
                true,
                "Max memory map areas",
            ),
            (
                "net.inet.ip.forwarding",
                SysctlType::Int,
                SysctlValue::Int(0),
                true,
                "IP forwarding",
            ),
            (
                "net.inet.tcp.keepidle",
                SysctlType::Int,
                SysctlValue::Int(7200),
                true,
                "TCP keepalive idle (s)",
            ),
            (
                "net.core.rmem_max",
                SysctlType::Int,
                SysctlValue::Int(212992),
                true,
                "Max socket receive buffer",
            ),
            (
                "security.bsd.see_other_uids",
                SysctlType::Int,
                SysctlValue::Int(1),
                true,
                "Allow seeing other UIDs' procs",
            ),
            (
                "security.sigma.pledge_enforce",
                SysctlType::Int,
                SysctlValue::Int(1),
                true,
                "Enforce OpenBSD-style pledge",
            ),
            (
                "hw.ncpu",
                SysctlType::Int,
                SysctlValue::Int(1),
                false,
                "Number of CPUs",
            ),
            (
                "hw.physmem",
                SysctlType::Int,
                SysctlValue::Int(0),
                false,
                "Physical memory bytes",
            ),
        ];
        for (path, ty, value, writable, desc) in defaults {
            let name = path.rsplit('.').next().unwrap_or(path).to_string();
            self.oids.push(SysctlOid {
                name,
                path: (*path).to_string(),
                ty: *ty,
                value: value.clone(),
                writable: *writable,
                description: (*desc).to_string(),
            });
        }
    }

    pub fn get(&self, path: &str) -> Option<&SysctlValue> {
        self.oids.iter().find(|o| o.path == path).map(|o| &o.value)
    }

    pub fn get_int(&self, path: &str) -> Option<i64> {
        match self.get(path)? {
            SysctlValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_int(&mut self, path: &str, value: i64) -> Result<(), &'static str> {
        if self.securelevel >= 2 && path.starts_with("security.") {
            return Err("sysctl locked by securelevel");
        }
        let oid = self
            .oids
            .iter_mut()
            .find(|o| o.path == path)
            .ok_or("sysctl not found")?;
        if !oid.writable {
            return Err("sysctl is read-only");
        }
        if path == "kern.securelevel" {
            if value < self.securelevel as i64 {
                return Err("securelevel can only be raised");
            }
            self.securelevel = value as i32;
        }
        oid.value = SysctlValue::Int(value);
        Ok(())
    }

    pub fn set_string(&mut self, path: &str, value: &str) -> Result<(), &'static str> {
        let oid = self
            .oids
            .iter_mut()
            .find(|o| o.path == path)
            .ok_or("sysctl not found")?;
        if !oid.writable {
            return Err("sysctl is read-only");
        }
        oid.value = SysctlValue::String(value.to_string());
        Ok(())
    }

    pub fn register(
        &mut self,
        path: &str,
        ty: SysctlType,
        value: SysctlValue,
        writable: bool,
        description: &str,
    ) {
        if self.oids.iter().any(|o| o.path == path) {
            return;
        }
        let name = path.rsplit('.').next().unwrap_or(path).to_string();
        self.oids.push(SysctlOid {
            name,
            path: path.to_string(),
            ty,
            value,
            writable,
            description: description.to_string(),
        });
    }

    pub fn list_prefix(&self, prefix: &str) -> Vec<&SysctlOid> {
        self.oids
            .iter()
            .filter(|o| o.path.starts_with(prefix))
            .collect()
    }

    pub fn securelevel(&self) -> i32 {
        self.securelevel
    }

    pub fn raise_securelevel(&mut self, level: i32) -> Result<(), &'static str> {
        if level < self.securelevel {
            return Err("securelevel can only be raised");
        }
        self.securelevel = level;
        if let Some(oid) = self.oids.iter_mut().find(|o| o.path == "kern.securelevel") {
            oid.value = SysctlValue::Int(level as i64);
        }
        Ok(())
    }
}

impl Default for SysctlTree {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 10. UMA ZONES  (FreeBSD: vm/uma_*)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct UmaItem {
    pub addr: usize,
    pub in_use: bool,
}

/// Universal Memory Allocator zone — typed fixed-size object cache.
#[derive(Debug)]
pub struct UmaZone {
    pub name: String,
    pub size: usize,
    pub align: usize,
    items: Vec<UmaItem>,
    freelist: Vec<usize>,
    allocated: usize,
    peak: usize,
    next_addr: usize,
}

impl UmaZone {
    pub fn create(name: &str, size: usize, align: usize, capacity: usize) -> Self {
        let align = align.max(1);
        let size = size.max(1);
        let mut zone = Self {
            name: name.to_string(),
            size,
            align,
            items: Vec::with_capacity(capacity),
            freelist: Vec::with_capacity(capacity),
            allocated: 0,
            peak: 0,
            next_addr: 0xA000_0000, // simulated kernel VA base
        };
        for _ in 0..capacity {
            let addr = zone.next_addr;
            zone.next_addr = zone.next_addr.wrapping_add(size + align);
            let idx = zone.items.len();
            zone.items.push(UmaItem {
                addr,
                in_use: false,
            });
            zone.freelist.push(idx);
        }
        zone
    }

    pub fn alloc(&mut self) -> Option<usize> {
        let idx = self.freelist.pop()?;
        self.items[idx].in_use = true;
        self.allocated += 1;
        if self.allocated > self.peak {
            self.peak = self.allocated;
        }
        Some(self.items[idx].addr)
    }

    pub fn free(&mut self, addr: usize) -> Result<(), &'static str> {
        let idx = self
            .items
            .iter()
            .position(|i| i.addr == addr && i.in_use)
            .ok_or("invalid uma free")?;
        self.items[idx].in_use = false;
        self.freelist.push(idx);
        self.allocated = self.allocated.saturating_sub(1);
        Ok(())
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.allocated, self.freelist.len(), self.peak)
    }
}

/// Registry of UMA zones (mbuf, socket, vnode, thread, …).
#[derive(Debug, Default)]
pub struct UmaCache {
    zones: Vec<UmaZone>,
}

impl UmaCache {
    pub fn new() -> Self {
        let mut c = Self { zones: Vec::new() };
        c.create_zone("mbuf", 256, 64, 1024);
        c.create_zone("socket", 512, 64, 256);
        c.create_zone("vnode", 384, 64, 512);
        c.create_zone("thread", 1024, 64, 256);
        c.create_zone("filedesc", 128, 16, 1024);
        c
    }

    pub fn create_zone(&mut self, name: &str, size: usize, align: usize, cap: usize) -> usize {
        let id = self.zones.len();
        self.zones.push(UmaZone::create(name, size, align, cap));
        id
    }

    pub fn zone_mut(&mut self, name: &str) -> Option<&mut UmaZone> {
        self.zones.iter_mut().find(|z| z.name == name)
    }

    pub fn zone(&self, name: &str) -> Option<&UmaZone> {
        self.zones.iter().find(|z| z.name == name)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 11. CALLOUT / TIMER  (BSD: sys/kern/kern_timeout.c)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutState {
    Idle,
    Pending,
    Active,
    Executing,
}

pub type CalloutFn = fn(arg: u64);

#[derive(Debug, Clone)]
pub struct Callout {
    pub id: u64,
    pub expires_at: u64, // ticks
    pub period: Option<u64>,
    pub func: CalloutFn,
    pub arg: u64,
    pub state: CalloutState,
}

fn noop_callout(_arg: u64) {}

/// Wheel-like callout manager (simplified single list sorted by expiry).
#[derive(Debug)]
pub struct CalloutWheel {
    callouts: Vec<Callout>,
    ticks: u64,
    next_id: u64,
}

impl CalloutWheel {
    pub fn new() -> Self {
        Self {
            callouts: Vec::new(),
            ticks: 0,
            next_id: 1,
        }
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn reset(&mut self, ticks_from_now: u64, period: Option<u64>, func: CalloutFn, arg: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.callouts.push(Callout {
            id,
            expires_at: self.ticks.saturating_add(ticks_from_now),
            period,
            func,
            arg,
            state: CalloutState::Pending,
        });
        self.callouts.sort_by_key(|c| c.expires_at);
        id
    }

    pub fn stop(&mut self, id: u64) -> bool {
        if let Some(c) = self.callouts.iter_mut().find(|c| c.id == id) {
            c.state = CalloutState::Idle;
            true
        } else {
            false
        }
    }

    /// Advance time by one tick; run due callouts. Returns number executed.
    pub fn softclock(&mut self) -> usize {
        self.ticks = self.ticks.wrapping_add(1);
        let now = self.ticks;
        let mut ran = 0;
        let mut requeue = Vec::new();
        self.callouts.retain(|c| {
            if c.state == CalloutState::Pending && c.expires_at <= now {
                (c.func)(c.arg);
                ran += 1;
                if let Some(period) = c.period {
                    let mut again = c.clone();
                    again.expires_at = now.saturating_add(period);
                    again.state = CalloutState::Pending;
                    requeue.push(again);
                }
                false
            } else {
                c.state != CalloutState::Idle
            }
        });
        self.callouts.extend(requeue);
        self.callouts.sort_by_key(|c| c.expires_at);
        ran
    }

    pub fn pending(&self) -> usize {
        self.callouts
            .iter()
            .filter(|c| c.state == CalloutState::Pending)
            .count()
    }
}

impl Default for CalloutWheel {
    fn default() -> Self {
        Self::new()
    }
}

// Silence unused warning for noop
#[allow(dead_code)]
fn _use_noop() {
    let _ = noop_callout as CalloutFn;
}

// ═══════════════════════════════════════════════════════════════════════════
// 12. SBUF  (FreeBSD: sys/kern/subr_sbuf.c)
// ═══════════════════════════════════════════════════════════════════════════

/// Safe growable string buffer for kernel diagnostics (/proc, sysctl, dmesg).
#[derive(Debug, Clone)]
pub struct Sbuf {
    buf: Vec<u8>,
    capacity: usize,
    overflow: bool,
    finished: bool,
}

impl Sbuf {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            capacity,
            overflow: false,
            finished: false,
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.overflow = false;
        self.finished = false;
    }

    pub fn cat(&mut self, s: &str) -> &mut Self {
        if self.finished || self.overflow {
            return self;
        }
        for b in s.bytes() {
            if self.buf.len() >= self.capacity {
                self.overflow = true;
                break;
            }
            self.buf.push(b);
        }
        self
    }

    pub fn cat_int(&mut self, v: i64) -> &mut Self {
        let mut tmp = [0u8; 32];
        let s = int_to_str(v, &mut tmp);
        self.cat(s)
    }

    pub fn putc(&mut self, c: u8) -> &mut Self {
        if !self.finished && !self.overflow && self.buf.len() < self.capacity {
            self.buf.push(c);
        } else if self.buf.len() >= self.capacity {
            self.overflow = true;
        }
        self
    }

    pub fn finish(&mut self) -> Result<&str, &'static str> {
        if self.overflow {
            return Err("sbuf overflow");
        }
        self.finished = true;
        core::str::from_utf8(&self.buf).map_err(|_| "sbuf invalid utf8")
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}

fn int_to_str(mut v: i64, buf: &mut [u8; 32]) -> &str {
    if v == 0 {
        buf[0] = b'0';
        return core::str::from_utf8(&buf[..1]).unwrap();
    }
    let neg = v < 0;
    if neg {
        v = -v;
    }
    let mut i = 32;
    while v > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    if neg {
        i -= 1;
        buf[i] = b'-';
    }
    core::str::from_utf8(&buf[i..]).unwrap()
}

// ═══════════════════════════════════════════════════════════════════════════
// 13. WITNESS  (FreeBSD: sys/kern/subr_witness.c)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessError {
    Recursion,
    LockOrderReversal,
    Uninitialized,
    NotHeld,
}

#[derive(Debug, Clone)]
struct LockClass {
    name: String,
    /// Partial order rank (lower acquired first).
    rank: u32,
}

/// Runtime lock-order checker inspired by FreeBSD WITNESS.
#[derive(Debug, Default)]
pub struct Witness {
    classes: Vec<LockClass>,
    /// Per-thread held lock stack (thread_id → class indices, bottom→top).
    held: Vec<(u64, Vec<usize>)>,
    violations: u64,
}

impl Witness {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_class(&mut self, name: &str, rank: u32) -> usize {
        let id = self.classes.len();
        self.classes.push(LockClass {
            name: name.to_string(),
            rank,
        });
        id
    }

    pub fn check_lock(&mut self, thread: u64, class: usize) -> Result<(), WitnessError> {
        if class >= self.classes.len() {
            return Err(WitnessError::Uninitialized);
        }
        let top = {
            let stack = self.thread_stack(thread);
            if stack.contains(&class) {
                self.violations += 1;
                return Err(WitnessError::Recursion);
            }
            stack.last().copied()
        };

        if let Some(top_class) = top {
            if self.classes[class].rank < self.classes[top_class].rank {
                self.violations += 1;
                return Err(WitnessError::LockOrderReversal);
            }
        }

        self.thread_stack(thread).push(class);
        Ok(())
    }

    pub fn check_unlock(&mut self, thread: u64, class: usize) -> Result<(), WitnessError> {
        let stack = self.thread_stack(thread);
        if stack.last().copied() != Some(class) {
            // Allow non-LIFO only as error for strict WITNESS
            if let Some(pos) = stack.iter().rposition(|&c| c == class) {
                stack.remove(pos);
                return Ok(());
            }
            return Err(WitnessError::NotHeld);
        }
        stack.pop();
        Ok(())
    }

    fn thread_stack(&mut self, thread: u64) -> &mut Vec<usize> {
        if let Some(pos) = self.held.iter().position(|(t, _)| *t == thread) {
            return &mut self.held[pos].1;
        }
        self.held.push((thread, Vec::new()));
        &mut self.held.last_mut().unwrap().1
    }

    pub fn violations(&self) -> u64 {
        self.violations
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 14. TURNSTILES  (FreeBSD: sys/kern/subr_turnstile.c — priority inheritance)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct TurnstileWaiter {
    pub tid: u64,
    pub priority: u8, // 0 = highest
}

/// Priority-inheritance turnstile for a single lock.
#[derive(Debug, Default)]
pub struct Turnstile {
    pub owner: Option<u64>,
    pub owner_base_prio: u8,
    pub owner_eff_prio: u8,
    waiters: Vec<TurnstileWaiter>,
}

impl Turnstile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lock(&mut self, tid: u64, prio: u8) -> Result<(), u64> {
        if self.owner.is_none() {
            self.owner = Some(tid);
            self.owner_base_prio = prio;
            self.owner_eff_prio = prio;
            return Ok(());
        }
        // Contended: enqueue and propagate priority
        self.waiters.push(TurnstileWaiter {
            tid,
            priority: prio,
        });
        self.propagate();
        Err(self.owner.unwrap())
    }

    pub fn unlock(&mut self, tid: u64) -> Option<u64> {
        if self.owner != Some(tid) {
            return None;
        }
        // Wake highest-priority waiter
        if self.waiters.is_empty() {
            self.owner = None;
            return None;
        }
        self.waiters.sort_by_key(|w| w.priority);
        let next = self.waiters.remove(0);
        self.owner = Some(next.tid);
        self.owner_base_prio = next.priority;
        self.owner_eff_prio = next.priority;
        self.propagate();
        Some(next.tid)
    }

    fn propagate(&mut self) {
        if let Some(best) = self.waiters.iter().map(|w| w.priority).min() {
            if best < self.owner_eff_prio {
                self.owner_eff_prio = best; // inherit
            }
        } else {
            self.owner_eff_prio = self.owner_base_prio;
        }
    }

    pub fn effective_priority(&self) -> Option<u8> {
        self.owner.map(|_| self.owner_eff_prio)
    }

    pub fn waiter_count(&self) -> usize {
        self.waiters.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 15. CAPSICUM RIGHTS  (FreeBSD: sys/kern/sys_capability.c)
// ═══════════════════════════════════════════════════════════════════════════

bitflags_capsicum! {
    // Manual bitflags without external crate
}

/// Capability rights that may be attached to a file descriptor (Capsicum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapRights {
    bits: u64,
}

impl CapRights {
    pub const READ: u64 = 1 << 0;
    pub const WRITE: u64 = 1 << 1;
    pub const SEEK: u64 = 1 << 2;
    pub const FSTAT: u64 = 1 << 3;
    pub const FCHMOD: u64 = 1 << 4;
    pub const FCHFLAGS: u64 = 1 << 5;
    pub const EVENT: u64 = 1 << 6; // kqueue
    pub const IOCTL: u64 = 1 << 7;
    pub const MMAP: u64 = 1 << 8;
    pub const CREATE: u64 = 1 << 9;
    pub const FTRUNCATE: u64 = 1 << 10;
    pub const LINKAT: u64 = 1 << 11;
    pub const CONNECT: u64 = 1 << 12;
    pub const BIND: u64 = 1 << 13;
    pub const LISTEN: u64 = 1 << 14;
    pub const ACCEPT: u64 = 1 << 15;
    pub const ALL: u64 = 0xFFFF;

    pub const fn all() -> Self {
        Self { bits: Self::ALL }
    }

    pub const fn new(bits: u64) -> Self {
        Self { bits }
    }

    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn contains(self, right: u64) -> bool {
        self.bits & right == right
    }

    /// Rights may only be narrowed (never expanded) — Capsicum invariant.
    pub fn limit(self, new_rights: CapRights) -> Result<CapRights, &'static str> {
        if new_rights.bits & !self.bits != 0 {
            return Err("capsicum: cannot expand rights");
        }
        Ok(CapRights {
            bits: self.bits & new_rights.bits,
        })
    }

    pub fn bits(self) -> u64 {
        self.bits
    }
}

/// Process capability mode flag (after cap_enter).
#[derive(Debug, Default)]
pub struct CapMode {
    entered: AtomicBool,
}

impl CapMode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter(&self) {
        self.entered.store(true, Ordering::SeqCst);
    }

    pub fn is_entered(&self) -> bool {
        self.entered.load(Ordering::SeqCst)
    }
}

// Macro stub to document intent without depending on bitflags crate
macro_rules! bitflags_capsicum {
    ($($t:tt)*) => {};
}
use bitflags_capsicum;

// ═══════════════════════════════════════════════════════════════════════════
// 16. NETLINK-STYLE MESSAGING  (Linux: net/netlink)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum NlFamily {
    Route = 0,
    Audit = 9,
    Generic = 16,
    Sigma = 32, // SigmaOS private family
}

#[derive(Debug, Clone)]
pub struct NlMessage {
    pub family: NlFamily,
    pub msg_type: u16,
    pub flags: u16,
    pub seq: u32,
    pub pid: u32,
    pub payload: Vec<u8>,
}

/// Simple netlink-like bus for kernel↔userspace control plane.
#[derive(Debug, Default)]
pub struct NetlinkBus {
    sockets: Vec<(u32, NlFamily, Vec<NlMessage>)>, // (portid, family, inbox)
    seq: AtomicU32,
}

impl NetlinkBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_socket(&mut self, portid: u32, family: NlFamily) {
        if !self.sockets.iter().any(|(p, _, _)| *p == portid) {
            self.sockets.push((portid, family, Vec::new()));
        }
    }

    pub fn send_to_kernel(&mut self, from: u32, mut msg: NlMessage) -> Result<(), &'static str> {
        msg.pid = from;
        msg.seq = self.seq.fetch_add(1, Ordering::Relaxed);
        // Kernel port is 0
        if let Some((_, _, inbox)) = self.sockets.iter_mut().find(|(p, f, _)| *p == 0 && *f == msg.family)
        {
            inbox.push(msg);
            Ok(())
        } else {
            // Auto-create kernel socket
            self.sockets.push((0, msg.family, vec![msg]));
            Ok(())
        }
    }

    pub fn multicast(&mut self, family: NlFamily, msg: NlMessage) -> usize {
        let mut n = 0;
        for (port, fam, inbox) in &mut self.sockets {
            if *fam == family && *port != 0 {
                inbox.push(msg.clone());
                n += 1;
            }
        }
        n
    }

    pub fn recv(&mut self, portid: u32) -> Option<NlMessage> {
        self.sockets
            .iter_mut()
            .find(|(p, _, _)| *p == portid)
            .and_then(|(_, _, inbox)| {
                if inbox.is_empty() {
                    None
                } else {
                    Some(inbox.remove(0))
                }
            })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 17. GEOM TRANSFORM CHAIN  (FreeBSD: sys/geom)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomClass {
    Disk,
    Part,
    Label,
    Mirror,
    Raid3,
    Eli, // encryption
    Journal,
    Concat,
    Stripe,
    SigmaSnap, // SigmaOS snapshot provider
}

#[derive(Debug, Clone)]
pub struct GeomProvider {
    pub name: String,
    pub class: GeomClass,
    pub mediasize: u64,
    pub sectorsize: u32,
    pub parent: Option<String>,
}

/// GEOM-inspired storage transformation graph.
#[derive(Debug, Default)]
pub struct GeomMesh {
    providers: Vec<GeomProvider>,
}

impl GeomMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_provider(&mut self, p: GeomProvider) -> Result<(), &'static str> {
        if self.providers.iter().any(|x| x.name == p.name) {
            return Err("geom provider exists");
        }
        if let Some(ref parent) = p.parent {
            if !self.providers.iter().any(|x| x.name == *parent) {
                return Err("geom parent missing");
            }
        }
        self.providers.push(p);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&GeomProvider> {
        self.providers.iter().find(|p| p.name == name)
    }

    /// Walk from leaf to root disk (for bio path resolution).
    pub fn path_to_root(&self, name: &str) -> Vec<String> {
        let mut path = Vec::new();
        let mut cur = name.to_string();
        for _ in 0..32 {
            path.push(cur.clone());
            if let Some(p) = self.lookup(&cur) {
                if let Some(ref parent) = p.parent {
                    cur = parent.clone();
                    continue;
                }
            }
            break;
        }
        path
    }

    pub fn providers(&self) -> &[GeomProvider] {
        &self.providers
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 18. UNIFIED KERNEL PRIMITIVES HUB
// ═══════════════════════════════════════════════════════════════════════════

/// Aggregate handle for all absorbed Linux/BSD primitives.
#[derive(Debug)]
pub struct UnixKernelHub {
    pub sysctl: SysctlTree,
    pub uma: UmaCache,
    pub callouts: CalloutWheel,
    pub futex: FutexTable,
    pub rcu: RcuEpoch,
    pub witness: Witness,
    pub netlink: NetlinkBus,
    pub geom: GeomMesh,
    pub cap_mode: CapMode,
    pub kqueues: Vec<Kqueue>,
    next_kq: u64,
}

impl UnixKernelHub {
    pub fn new() -> Self {
        let mut witness = Witness::new();
        // Canonical FreeBSD-ish lock ranks
        witness.register_class("sleep", 10);
        witness.register_class("turnstile", 20);
        witness.register_class("vm", 30);
        witness.register_class("vfs", 40);
        witness.register_class("dev", 50);
        witness.register_class("net", 60);

        let mut geom = GeomMesh::new();
        let _ = geom.add_provider(GeomProvider {
            name: "ada0".into(),
            class: GeomClass::Disk,
            mediasize: 0,
            sectorsize: 512,
            parent: None,
        });

        Self {
            sysctl: SysctlTree::new(),
            uma: UmaCache::new(),
            callouts: CalloutWheel::new(),
            futex: FutexTable::new(),
            rcu: RcuEpoch::new(),
            witness,
            netlink: NetlinkBus::new(),
            geom,
            cap_mode: CapMode::new(),
            kqueues: Vec::new(),
            next_kq: 1,
        }
    }

    pub fn create_kqueue(&mut self) -> u64 {
        let id = self.next_kq;
        self.next_kq += 1;
        self.kqueues.push(Kqueue::new(id));
        id
    }

    pub fn kqueue_mut(&mut self, id: u64) -> Option<&mut Kqueue> {
        self.kqueues.iter_mut().find(|k| k.id() == id)
    }

    /// One softclock + housekeeping tick.
    pub fn tick(&mut self) -> usize {
        self.callouts.softclock()
    }
}

impl Default for UnixKernelHub {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rbtree_insert_and_min() {
        let mut t = RbTree::new();
        t.insert(50u64, "c");
        t.insert(20u64, "a");
        t.insert(80u64, "d");
        t.insert(10u64, "z");
        assert_eq!(t.len(), 4);
        let (k, v) = t.min().unwrap();
        assert_eq!(*k, 10);
        assert_eq!(*v, "z");
        assert_eq!(t.get(&80), Some(&"d"));
    }

    #[test]
    fn seqlock_consistent_read() {
        let lock = SeqLock::new();
        let data = 42u64;
        lock.write_begin();
        lock.write_end();
        loop {
            let s = lock.read_begin();
            let v = data;
            if !lock.read_retry(s) {
                assert_eq!(v, 42);
                break;
            }
        }
    }

    #[test]
    fn wait_queue_wake() {
        let mut wq = WaitQueue::new();
        wq.add_waiter(1, false);
        wq.add_waiter(2, false);
        wq.add_waiter(3, true);
        let woken = wq.wake_up(2);
        assert_eq!(woken, vec![1, 2]);
        assert_eq!(wq.pending(), 1);
    }

    #[test]
    fn futex_wait_wake() {
        let mut ft = FutexTable::new();
        assert!(ft.wait(10, 0x1000, 0, 0, !0).is_ok());
        assert!(ft.wait(11, 0x1000, 0, 0, !0).is_ok());
        assert_eq!(ft.wake(0x1000, 1, !0), 1);
        assert_eq!(ft.waiter_count(0x1000), 1);
    }

    #[test]
    fn kqueue_oneshot() {
        let mut kq = Kqueue::new(1);
        kq.change(&[Kevent {
            ident: 5,
            filter: KqFilter::Read,
            flags: EV_ADD | EV_ONESHOT,
            fflags: 0,
            data: 0,
            udata: 0,
        }])
        .unwrap();
        kq.note(5, KqFilter::Read, 128);
        let ev = kq.poll(8);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].data, 128);
        assert_eq!(kq.registered_count(), 0); // oneshot removed
    }

    #[test]
    fn sysctl_securelevel() {
        let mut t = SysctlTree::new();
        assert_eq!(t.get_int("kern.maxproc"), Some(4096));
        t.set_int("kern.maxproc", 8192).unwrap();
        assert_eq!(t.get_int("kern.maxproc"), Some(8192));
        t.raise_securelevel(2).unwrap();
        assert!(t.raise_securelevel(1).is_err());
    }

    #[test]
    fn uma_alloc_free() {
        let mut cache = UmaCache::new();
        let zone = cache.zone_mut("mbuf").unwrap();
        let a = zone.alloc().unwrap();
        let b = zone.alloc().unwrap();
        assert_ne!(a, b);
        zone.free(a).unwrap();
        let (alloc, free, _) = zone.stats();
        assert_eq!(alloc, 1);
        assert!(free > 0);
    }

    #[test]
    fn callout_periodic() {
        static mut HITS: u32 = 0;
        fn hit(_: u64) {
            unsafe { HITS += 1 }
        }
        let mut w = CalloutWheel::new();
        w.reset(2, Some(3), hit, 0);
        assert_eq!(w.softclock(), 0);
        assert_eq!(w.softclock(), 1); // expires at tick 2
        unsafe { assert_eq!(HITS, 1) };
        // period 3 → next at 5
        assert_eq!(w.softclock(), 0);
        assert_eq!(w.softclock(), 0);
        assert_eq!(w.softclock(), 1);
        unsafe { assert_eq!(HITS, 2) };
    }

    #[test]
    fn sbuf_build() {
        let mut s = Sbuf::new(64);
        s.cat("pid=").cat_int(42).putc(b'\n');
        let out = s.finish().unwrap();
        assert_eq!(out, "pid=42\n");
    }

    #[test]
    fn witness_order_violation() {
        let mut w = Witness::new();
        let a = w.register_class("A", 10);
        let b = w.register_class("B", 20);
        w.check_lock(1, b).unwrap();
        assert_eq!(w.check_lock(1, a), Err(WitnessError::LockOrderReversal));
    }

    #[test]
    fn turnstile_priority_inherit() {
        let mut ts = Turnstile::new();
        assert!(ts.lock(1, 50).is_ok());
        assert!(ts.lock(2, 10).is_err()); // blocks, inherits
        assert_eq!(ts.effective_priority(), Some(10));
        assert_eq!(ts.unlock(1), Some(2));
    }

    #[test]
    fn capsicum_narrow_only() {
        let r = CapRights::new(CapRights::READ | CapRights::WRITE);
        let limited = r.limit(CapRights::new(CapRights::READ)).unwrap();
        assert!(limited.contains(CapRights::READ));
        assert!(!limited.contains(CapRights::WRITE));
        assert!(limited
            .limit(CapRights::new(CapRights::WRITE))
            .is_err());
    }

    #[test]
    fn geom_path() {
        let mut g = GeomMesh::new();
        g.add_provider(GeomProvider {
            name: "ada0".into(),
            class: GeomClass::Disk,
            mediasize: 1 << 30,
            sectorsize: 512,
            parent: None,
        })
        .unwrap();
        g.add_provider(GeomProvider {
            name: "ada0p1".into(),
            class: GeomClass::Part,
            mediasize: 1 << 29,
            sectorsize: 512,
            parent: Some("ada0".into()),
        })
        .unwrap();
        let path = g.path_to_root("ada0p1");
        assert_eq!(path, vec!["ada0p1".to_string(), "ada0".to_string()]);
    }

    #[test]
    fn hub_smoke() {
        let mut hub = UnixKernelHub::new();
        let kq = hub.create_kqueue();
        assert!(hub.kqueue_mut(kq).is_some());
        hub.sysctl.set_int("vm.swappiness", 10).unwrap();
        assert_eq!(hub.tick(), 0);
    }

    #[test]
    fn sparse_map() {
        let mut m = SparseMap::new();
        m.insert(100, "a");
        m.insert(50, "b");
        m.insert(200, "c");
        assert_eq!(m.get(50), Some(&"b"));
        assert_eq!(m.range(40, 150), vec![(50, "b"), (100, "a")]);
    }

    #[test]
    fn percpu_sum() {
        let mut p = PerCpu::<u64>::new(4);
        p.add(0, 10);
        p.add(3, 5);
        assert_eq!(p.sum(), 15);
    }
}
