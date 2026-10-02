// SigmaOS Binder IPC Ring Buffer
// Inspired by Android/Linux in-kernel Binder driver:
// shared-memory ring buffer, transaction reference counting,
// death notifications, and object handle table.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{
    atomic::{AtomicU32, AtomicU64, Ordering},
    Arc, Mutex,
};

// ─────────────────────────────────────────────────────────────────────────────
// Object Handle Table (like Binder's proc->refs)
// ─────────────────────────────────────────────────────────────────────────────

pub type Handle = u32;
pub type BinderNodeId = u64;

#[derive(Debug, Clone)]
pub struct BinderObject {
    pub node_id: BinderNodeId,
    pub strong_refs: u32,
    pub weak_refs: u32,
    pub owner_pid: u32,
    pub cookie: u64,
    pub dead: bool,
}

pub struct HandleTable {
    handles: BTreeMap<Handle, BinderObject>,
    next_handle: AtomicU32,
}

impl HandleTable {
    pub fn new() -> Self {
        HandleTable {
            handles: BTreeMap::new(),
            next_handle: AtomicU32::new(1),
        }
    }

    pub fn insert(&mut self, obj: BinderObject) -> Handle {
        let h = self.next_handle.fetch_add(1, Ordering::SeqCst);
        self.handles.insert(h, obj);
        h
    }

    pub fn lookup(&self, handle: Handle) -> Option<&BinderObject> {
        self.handles.get(&handle)
    }

    pub fn acquire(&mut self, handle: Handle) -> Result<(), &'static str> {
        match self.handles.get_mut(&handle) {
            Some(obj) if !obj.dead => {
                obj.strong_refs += 1;
                Ok(())
            }
            Some(_) => Err("Object is dead"),
            None => Err("Invalid handle"),
        }
    }

    pub fn release(&mut self, handle: Handle) -> Result<bool, &'static str> {
        match self.handles.get_mut(&handle) {
            Some(obj) => {
                if obj.strong_refs == 0 {
                    return Err("Reference underflow");
                }
                obj.strong_refs -= 1;
                Ok(obj.strong_refs == 0) // true = can be freed
            }
            None => Err("Invalid handle"),
        }
    }

    pub fn mark_dead(&mut self, node_id: BinderNodeId) -> Vec<Handle> {
        let mut dead_handles = Vec::new();
        for (&h, obj) in self.handles.iter_mut() {
            if obj.node_id == node_id {
                obj.dead = true;
                dead_handles.push(h);
            }
        }
        dead_handles
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Binder Transaction
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionCode {
    Transaction = 1,
    Reply = 2,
    AcquireResult = 4,
    DeadReply = 5,
    TransactionComplete = 6,
    IncRefs = 7,
    Acquire = 8,
    Release = 9,
    DecRefs = 10,
    AttemptAcquire = 11,
    Noop = 12,
    SpawnLooper = 13,
    FinishedLooper = 14,
    RegisterLooper = 15,
}

#[derive(Debug, Clone)]
pub struct BinderTransaction {
    pub debug_id: u64,
    pub from_pid: u32,
    pub to_pid: u32,
    pub to_handle: Handle,
    pub code: u32, // AIDL interface method code
    pub flags: u32,
    pub data: Vec<u8>,     // flat parcel data
    pub offsets: Vec<u32>, // byte offsets of embedded binder objects
    pub reply: bool,
    pub one_way: bool, // async (no reply expected)
}

impl BinderTransaction {
    pub fn new(from: u32, to: u32, handle: Handle, code: u32, data: Vec<u8>) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        BinderTransaction {
            debug_id: COUNTER.fetch_add(1, Ordering::Relaxed),
            from_pid: from,
            to_pid: to,
            to_handle: handle,
            code,
            flags: 0,
            data,
            offsets: Vec::new(),
            reply: false,
            one_way: false,
        }
    }

    pub fn one_way(mut self) -> Self {
        self.one_way = true;
        self
    }
    pub fn with_reply(mut self) -> Self {
        self.reply = true;
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared Memory Ring Buffer (Binder mmap'd region)
// ─────────────────────────────────────────────────────────────────────────────

pub struct BinderRingBuffer {
    /// Simulated mmap'd shared region (in real kernel: get_vm_area)
    buffer: Arc<Mutex<Vec<u8>>>,
    pub capacity: usize,
    write_pos: AtomicU64,
    read_pos: AtomicU64,
    pub alloc_count: AtomicU64,
    pub free_count: AtomicU64,
}

impl BinderRingBuffer {
    pub fn new(capacity: usize) -> Self {
        BinderRingBuffer {
            buffer: Arc::new(Mutex::new(vec![0u8; capacity])),
            capacity,
            write_pos: AtomicU64::new(0),
            read_pos: AtomicU64::new(0),
            alloc_count: AtomicU64::new(0),
            free_count: AtomicU64::new(0),
        }
    }

    /// Allocate space for a transaction in the ring buffer
    pub fn alloc_transaction(&self, size: usize) -> Option<u64> {
        let write = self.write_pos.load(Ordering::Acquire);
        let read = self.read_pos.load(Ordering::Acquire);
        let used = if write >= read {
            write - read
        } else {
            self.capacity as u64 - read + write
        };
        let free = self.capacity as u64 - used;

        if size as u64 > free {
            return None;
        }

        let offset = write % self.capacity as u64;
        self.write_pos.fetch_add(size as u64, Ordering::Release);
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        Some(offset)
    }

    /// Write transaction data to the buffer
    pub fn write_at(&self, offset: u64, data: &[u8]) {
        let mut buf = self.buffer.lock().unwrap();
        let start = offset as usize % self.capacity;
        let end = (start + data.len()).min(self.capacity);
        buf[start..end].copy_from_slice(&data[..end - start]);
    }

    /// Free allocated transaction buffer
    pub fn free_transaction(&self, size: usize) {
        self.read_pos.fetch_add(size as u64, Ordering::Release);
        self.free_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn used_bytes(&self) -> u64 {
        let w = self.write_pos.load(Ordering::Relaxed);
        let r = self.read_pos.load(Ordering::Relaxed);
        w.saturating_sub(r)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Death Notification
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DeathRecipient {
    pub handle: Handle,
    pub cookie: u64,
    pub registered_by_pid: u32,
}

pub struct DeathNotifier {
    recipients: Vec<DeathRecipient>,
}

impl DeathNotifier {
    pub fn new() -> Self {
        DeathNotifier {
            recipients: Vec::new(),
        }
    }

    pub fn register(&mut self, r: DeathRecipient) {
        self.recipients.push(r);
    }

    pub fn unregister(&mut self, handle: Handle, cookie: u64) {
        self.recipients
            .retain(|r| !(r.handle == handle && r.cookie == cookie));
    }

    /// Called when a service process dies; returns list of notifications to send
    pub fn notify_dead(&self, node_id: BinderNodeId, dead_pid: u32) -> Vec<&DeathRecipient> {
        self.recipients
            .iter()
            .filter(|r| r.registered_by_pid != dead_pid)
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Binder Driver (per-process context)
// ─────────────────────────────────────────────────────────────────────────────

pub struct BinderProcess {
    pub pid: u32,
    pub handles: HandleTable,
    pub ring: Arc<BinderRingBuffer>,
    pub incoming: Arc<Mutex<VecDeque<BinderTransaction>>>,
    pub death: DeathNotifier,
    pub is_context_manager: bool, // true for servicemanager
}

impl BinderProcess {
    pub fn new(pid: u32, ring_size: usize) -> Self {
        BinderProcess {
            pid,
            handles: HandleTable::new(),
            ring: Arc::new(BinderRingBuffer::new(ring_size)),
            incoming: Arc::new(Mutex::new(VecDeque::new())),
            death: DeathNotifier::new(),
            is_context_manager: false,
        }
    }

    /// Send a transaction to another process
    pub fn transact(
        &self,
        mut txn: BinderTransaction,
        target: &BinderProcess,
    ) -> Result<(), &'static str> {
        // Validate handle
        if !self.handles.lookup(txn.to_handle).is_some() && txn.to_handle != 0 {
            return Err("Invalid handle");
        }
        // Allocate buffer in target's ring
        let size = txn.data.len() + 64; // header overhead
        let offset = target
            .ring
            .alloc_transaction(size)
            .ok_or("Target buffer full")?;
        target.ring.write_at(offset, &txn.data);
        // Queue to target's incoming
        target.incoming.lock().unwrap().push_back(txn);
        Ok(())
    }

    /// Receive a pending transaction (blocking in real driver via wait_event)
    pub fn receive(&self) -> Option<BinderTransaction> {
        self.incoming.lock().unwrap().pop_front()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Binder Context Manager (ServiceManager equivalent)
// ─────────────────────────────────────────────────────────────────────────────

pub struct BinderContextManager {
    service_registry: HashMap<String, BinderNodeId>,
    next_node: AtomicU64,
}

impl BinderContextManager {
    pub fn new() -> Self {
        BinderContextManager {
            service_registry: HashMap::new(),
            next_node: AtomicU64::new(1),
        }
    }

    pub fn add_service(&mut self, name: &str) -> BinderNodeId {
        let id = self.next_node.fetch_add(1, Ordering::SeqCst);
        self.service_registry.insert(name.into(), id);
        id
    }

    pub fn check_service(&self, name: &str) -> Option<BinderNodeId> {
        self.service_registry.get(name).copied()
    }

    pub fn list_services(&self) -> Vec<&String> {
        self.service_registry.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_table_acquire_release() {
        let mut table = HandleTable::new();
        let obj = BinderObject {
            node_id: 1,
            strong_refs: 0,
            weak_refs: 0,
            owner_pid: 100,
            cookie: 0xAB,
            dead: false,
        };
        let h = table.insert(obj);
        assert!(table.acquire(h).is_ok());
        assert_eq!(table.lookup(h).unwrap().strong_refs, 1);
        let freed = table.release(h).unwrap();
        assert!(freed); // strong_refs == 0
    }

    #[test]
    fn test_ring_buffer_alloc_write_free() {
        let ring = BinderRingBuffer::new(4096);
        let offset = ring.alloc_transaction(256).unwrap();
        ring.write_at(offset, &[0xAB; 256]);
        assert!(ring.used_bytes() >= 256);
        ring.free_transaction(256);
    }

    #[test]
    fn test_ring_buffer_full() {
        let ring = BinderRingBuffer::new(512);
        ring.alloc_transaction(500).unwrap();
        // Only 12 bytes left
        assert!(ring.alloc_transaction(256).is_none());
    }

    #[test]
    fn test_binder_transact_receive() {
        let sender = BinderProcess::new(100, 65536);
        let receiver = BinderProcess::new(200, 65536);

        // Sender registers a handle to receiver's service
        let mut sender_mut = BinderProcess::new(100, 65536);
        let node = BinderObject {
            node_id: 1,
            strong_refs: 1,
            weak_refs: 0,
            owner_pid: 200,
            cookie: 0,
            dead: false,
        };
        let handle = sender_mut.handles.insert(node);

        let txn = BinderTransaction::new(100, 200, handle, 1, b"ping".to_vec());
        assert!(sender_mut.transact(txn, &receiver).is_ok());

        let received = receiver.receive();
        assert!(received.is_some());
        assert_eq!(received.unwrap().data, b"ping");
    }

    #[test]
    fn test_death_notification() {
        let mut notifier = DeathNotifier::new();
        notifier.register(DeathRecipient {
            handle: 1,
            cookie: 0xFF,
            registered_by_pid: 300,
        });
        notifier.register(DeathRecipient {
            handle: 1,
            cookie: 0xFE,
            registered_by_pid: 400,
        });
        // Notify with dead_pid=300: only pid 400 should be notified
        let notifications = notifier.notify_dead(1, 300);
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].registered_by_pid, 400);
    }

    #[test]
    fn test_context_manager() {
        let mut mgr = BinderContextManager::new();
        let id = mgr.add_service("android.hardware.camera.provider");
        assert_eq!(
            mgr.check_service("android.hardware.camera.provider"),
            Some(id)
        );
        assert!(mgr.check_service("nonexistent").is_none());
        assert_eq!(mgr.list_services().len(), 1);
    }
}
