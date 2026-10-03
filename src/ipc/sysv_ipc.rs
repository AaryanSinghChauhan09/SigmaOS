//! # System V IPC
//!
//! System V Inter-Process Communication: shared memory, semaphores, and message queues.
//! Inspired by Linux ipc/ and FreeBSD kern/sysv_*.c.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};

/// IPC permissions structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IpcPerm {
    pub uid: u32,      // Owner's user ID
    pub gid: u32,      // Owner's group ID
    pub cuid: u32,     // Creator's user ID
    pub cgid: u32,     // Creator's group ID
    pub mode: u16,     // Permission bits
    pub seq: u16,      // Sequence number
    pub key: i32,      // IPC key
}

impl IpcPerm {
    pub fn new(key: i32, mode: u16, uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            cuid: uid,
            cgid: gid,
            mode,
            seq: 0,
            key,
        }
    }
}

/// IPC commands
pub const IPC_CREAT: i32 = 0o1000;   // Create if key doesn't exist
pub const IPC_EXCL: i32 = 0o2000;    // Fail if key exists
pub const IPC_NOWAIT: i32 = 0o4000;  // Return error on wait

pub const IPC_RMID: i32 = 0;         // Remove identifier
pub const IPC_SET: i32 = 1;          // Set options
pub const IPC_STAT: i32 = 2;         // Get options

pub const IPC_PRIVATE: i32 = 0;      // Private key

// ==================== System V Shared Memory ====================

/// Shared memory segment descriptor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ShmidDs {
    pub shm_perm: IpcPerm,      // Permissions
    pub shm_segsz: usize,       // Size of segment
    pub shm_atime: u64,         // Last attach time
    pub shm_dtime: u64,         // Last detach time
    pub shm_ctime: u64,         // Last change time
    pub shm_cpid: i32,          // PID of creator
    pub shm_lpid: i32,          // PID of last shmat/shmdt
    pub shm_nattch: u32,        // Number of attaches
}

/// Shared memory segment
pub struct ShmSegment {
    pub ds: ShmidDs,
    pub data: Vec<u8>,
    pub attached_pids: Vec<i32>,
}

impl ShmSegment {
    pub fn new(key: i32, size: usize, mode: u16, uid: u32, gid: u32, pid: i32) -> Self {
        Self {
            ds: ShmidDs {
                shm_perm: IpcPerm::new(key, mode, uid, gid),
                shm_segsz: size,
                shm_atime: 0,
                shm_dtime: 0,
                shm_ctime: 0,
                shm_cpid: pid,
                shm_lpid: 0,
                shm_nattch: 0,
            },
            data: alloc::vec![0u8; size],
            attached_pids: Vec::new(),
        }
    }
    
    pub fn attach(&mut self, pid: i32) {
        self.attached_pids.push(pid);
        self.ds.shm_nattch += 1;
        self.ds.shm_lpid = pid;
        self.ds.shm_atime = 0; // In production: get current time
    }
    
    pub fn detach(&mut self, pid: i32) -> bool {
        if let Some(pos) = self.attached_pids.iter().position(|&p| p == pid) {
            self.attached_pids.remove(pos);
            self.ds.shm_nattch = self.ds.shm_nattch.saturating_sub(1);
            self.ds.shm_lpid = pid;
            self.ds.shm_dtime = 0; // In production: get current time
            true
        } else {
            false
        }
    }
}

/// Shared memory registry
pub struct ShmRegistry {
    segments: BTreeMap<i32, ShmSegment>,
    next_id: AtomicI32,
}

impl ShmRegistry {
    pub fn new() -> Self {
        Self {
            segments: BTreeMap::new(),
            next_id: AtomicI32::new(1),
        }
    }
    
    /// Get shared memory segment (shmget)
    pub fn get(&mut self, key: i32, size: usize, flags: i32, uid: u32, gid: u32, pid: i32) -> Result<i32, IpcError> {
        let mode = (flags & 0o777) as u16;
        
        if key == IPC_PRIVATE {
            // Always create new private segment
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let segment = ShmSegment::new(key, size, mode, uid, gid, pid);
            self.segments.insert(id, segment);
            return Ok(id);
        }
        
        // Search for existing segment with this key
        if let Some((&id, segment)) = self.segments.iter().find(|(_, seg)| seg.ds.shm_perm.key == key) {
            if flags & IPC_CREAT != 0 && flags & IPC_EXCL != 0 {
                return Err(IpcError::AlreadyExists);
            }
            
            if size > 0 && size > segment.ds.shm_segsz {
                return Err(IpcError::InvalidSize);
            }
            
            return Ok(id);
        }
        
        // Not found - create if IPC_CREAT
        if flags & IPC_CREAT != 0 {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let segment = ShmSegment::new(key, size, mode, uid, gid, pid);
            self.segments.insert(id, segment);
            Ok(id)
        } else {
            Err(IpcError::NotFound)
        }
    }
    
    /// Attach shared memory (shmat)
    pub fn attach(&mut self, id: i32, pid: i32) -> Result<*mut u8, IpcError> {
        let segment = self.segments.get_mut(&id).ok_or(IpcError::NotFound)?;
        segment.attach(pid);
        Ok(segment.data.as_mut_ptr())
    }
    
    /// Detach shared memory (shmdt)
    pub fn detach(&mut self, id: i32, pid: i32) -> Result<(), IpcError> {
        let segment = self.segments.get_mut(&id).ok_or(IpcError::NotFound)?;
        if segment.detach(pid) {
            Ok(())
        } else {
            Err(IpcError::NotAttached)
        }
    }
    
    /// Control operations (shmctl)
    pub fn control(&mut self, id: i32, cmd: i32) -> Result<ShmidDs, IpcError> {
        match cmd {
            IPC_STAT => {
                let segment = self.segments.get(&id).ok_or(IpcError::NotFound)?;
                Ok(segment.ds)
            }
            IPC_RMID => {
                self.segments.remove(&id).ok_or(IpcError::NotFound)?;
                Ok(unsafe { core::mem::zeroed() })
            }
            _ => Err(IpcError::InvalidCommand)
        }
    }
}

// ==================== System V Semaphores ====================

/// Semaphore operation
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SemBuf {
    pub sem_num: u16,   // Semaphore number
    pub sem_op: i16,    // Operation (-1: wait, 0: wait for zero, +n: signal)
    pub sem_flg: i16,   // Flags (IPC_NOWAIT, SEM_UNDO)
}

pub const SEM_UNDO: i16 = 0o10000;

/// Semaphore descriptor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SemidDs {
    pub sem_perm: IpcPerm,
    pub sem_otime: u64,     // Last semop time
    pub sem_ctime: u64,     // Last change time
    pub sem_nsems: u16,     // Number of semaphores
}

/// Semaphore set
pub struct SemSet {
    pub ds: SemidDs,
    pub values: Vec<AtomicI32>,
    pub undo_values: BTreeMap<i32, Vec<i16>>, // PID -> undo values
}

impl SemSet {
    pub fn new(key: i32, nsems: u16, mode: u16, uid: u32, gid: u32) -> Self {
        let mut values = Vec::with_capacity(nsems as usize);
        for _ in 0..nsems {
            values.push(AtomicI32::new(0));
        }
        
        Self {
            ds: SemidDs {
                sem_perm: IpcPerm::new(key, mode, uid, gid),
                sem_otime: 0,
                sem_ctime: 0,
                sem_nsems: nsems,
            },
            values,
            undo_values: BTreeMap::new(),
        }
    }
    
    /// Perform semaphore operation
    pub fn op(&mut self, ops: &[SemBuf], pid: i32) -> Result<(), IpcError> {
        // Check if all operations can be performed
        for op in ops {
            let sem_num = op.sem_num as usize;
            if sem_num >= self.values.len() {
                return Err(IpcError::InvalidSemaphore);
            }
            
            let current = self.values[sem_num].load(Ordering::SeqCst);
            
            if op.sem_op < 0 {
                // Wait operation (P)
                if current + (op.sem_op as i32) < 0 {
                    if op.sem_flg & IPC_NOWAIT as i16 != 0 {
                        return Err(IpcError::WouldBlock);
                    }
                    // In production: block process
                    return Err(IpcError::WouldBlock);
                }
            } else if op.sem_op == 0 {
                // Wait for zero
                if current != 0 {
                    if op.sem_flg & IPC_NOWAIT as i16 != 0 {
                        return Err(IpcError::WouldBlock);
                    }
                    return Err(IpcError::WouldBlock);
                }
            }
        }
        
        // Perform all operations
        for op in ops {
            let sem_num = op.sem_num as usize;
            
            if op.sem_op != 0 {
                self.values[sem_num].fetch_add(op.sem_op as i32, Ordering::SeqCst);
                
                // Track undo if SEM_UNDO flag set
                if op.sem_flg & SEM_UNDO != 0 {
                    let undo_list = self.undo_values.entry(pid).or_insert_with(|| {
                        alloc::vec![0i16; self.values.len()]
                    });
                    undo_list[sem_num] = undo_list[sem_num].saturating_sub(op.sem_op);
                }
            }
        }
        
        self.ds.sem_otime = 0; // In production: set to current time
        Ok(())
    }
    
    /// Set semaphore value
    pub fn setval(&self, sem_num: u16, val: i32) -> Result<(), IpcError> {
        if sem_num as usize >= self.values.len() {
            return Err(IpcError::InvalidSemaphore);
        }
        self.values[sem_num as usize].store(val, Ordering::SeqCst);
        Ok(())
    }
    
    /// Get semaphore value
    pub fn getval(&self, sem_num: u16) -> Result<i32, IpcError> {
        if sem_num as usize >= self.values.len() {
            return Err(IpcError::InvalidSemaphore);
        }
        Ok(self.values[sem_num as usize].load(Ordering::SeqCst))
    }
}

/// Semaphore registry
pub struct SemRegistry {
    sets: BTreeMap<i32, SemSet>,
    next_id: AtomicI32,
}

impl SemRegistry {
    pub fn new() -> Self {
        Self {
            sets: BTreeMap::new(),
            next_id: AtomicI32::new(1),
        }
    }
    
    /// Get semaphore set (semget)
    pub fn get(&mut self, key: i32, nsems: u16, flags: i32, uid: u32, gid: u32) -> Result<i32, IpcError> {
        let mode = (flags & 0o777) as u16;
        
        if key == IPC_PRIVATE {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let set = SemSet::new(key, nsems, mode, uid, gid);
            self.sets.insert(id, set);
            return Ok(id);
        }
        
        if let Some((&id, _)) = self.sets.iter().find(|(_, set)| set.ds.sem_perm.key == key) {
            if flags & IPC_CREAT != 0 && flags & IPC_EXCL != 0 {
                return Err(IpcError::AlreadyExists);
            }
            return Ok(id);
        }
        
        if flags & IPC_CREAT != 0 {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let set = SemSet::new(key, nsems, mode, uid, gid);
            self.sets.insert(id, set);
            Ok(id)
        } else {
            Err(IpcError::NotFound)
        }
    }
    
    /// Semaphore operations (semop)
    pub fn op(&mut self, id: i32, ops: &[SemBuf], pid: i32) -> Result<(), IpcError> {
        let set = self.sets.get_mut(&id).ok_or(IpcError::NotFound)?;
        set.op(ops, pid)
    }
    
    /// Control operations (semctl)
    pub fn control(&mut self, id: i32, sem_num: u16, cmd: i32, arg: i32) -> Result<i32, IpcError> {
        match cmd {
            16 => { // SETVAL
                let set = self.sets.get(&id).ok_or(IpcError::NotFound)?;
                set.setval(sem_num, arg)?;
                Ok(0)
            }
            12 => { // GETVAL
                let set = self.sets.get(&id).ok_or(IpcError::NotFound)?;
                set.getval(sem_num)
            }
            IPC_RMID => {
                self.sets.remove(&id).ok_or(IpcError::NotFound)?;
                Ok(0)
            }
            _ => Err(IpcError::InvalidCommand)
        }
    }
}

// ==================== System V Message Queues ====================

/// Message buffer
#[repr(C)]
pub struct MsgBuf {
    pub mtype: i64,     // Message type (must be > 0)
    // mtext follows
}

/// Message queue descriptor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MsqidDs {
    pub msg_perm: IpcPerm,
    pub msg_stime: u64,     // Last msgsnd time
    pub msg_rtime: u64,     // Last msgrcv time
    pub msg_ctime: u64,     // Last change time
    pub msg_qnum: u64,      // Number of messages
    pub msg_qbytes: u64,    // Max bytes in queue
    pub msg_lspid: i32,     // PID of last msgsnd
    pub msg_lrpid: i32,     // PID of last msgrcv
}

/// Message in queue
#[derive(Debug, Clone)]
pub struct QueuedMsg {
    pub mtype: i64,
    pub data: Vec<u8>,
}

/// Message queue
pub struct MsgQueue {
    pub ds: MsqidDs,
    pub messages: Vec<QueuedMsg>,
}

impl MsgQueue {
    pub fn new(key: i32, mode: u16, uid: u32, gid: u32) -> Self {
        Self {
            ds: MsqidDs {
                msg_perm: IpcPerm::new(key, mode, uid, gid),
                msg_stime: 0,
                msg_rtime: 0,
                msg_ctime: 0,
                msg_qnum: 0,
                msg_qbytes: 16384, // Default 16KB
                msg_lspid: 0,
                msg_lrpid: 0,
            },
            messages: Vec::new(),
        }
    }
    
    /// Send message
    pub fn send(&mut self, mtype: i64, data: &[u8], pid: i32, flags: i32) -> Result<(), IpcError> {
        if mtype <= 0 {
            return Err(IpcError::InvalidType);
        }
        
        let total_bytes: usize = self.messages.iter().map(|m| m.data.len()).sum();
        if total_bytes + data.len() > self.ds.msg_qbytes as usize {
            if flags & IPC_NOWAIT != 0 {
                return Err(IpcError::WouldBlock);
            }
            return Err(IpcError::WouldBlock);
        }
        
        self.messages.push(QueuedMsg {
            mtype,
            data: data.to_vec(),
        });
        
        self.ds.msg_qnum += 1;
        self.ds.msg_lspid = pid;
        self.ds.msg_stime = 0; // In production: current time
        Ok(())
    }
    
    /// Receive message
    pub fn receive(&mut self, msgtyp: i64, buf: &mut [u8], pid: i32, flags: i32) -> Result<(usize, i64), IpcError> {
        let msg_idx = if msgtyp == 0 {
            // Get first message
            if self.messages.is_empty() {
                if flags & IPC_NOWAIT != 0 {
                    return Err(IpcError::WouldBlock);
                }
                return Err(IpcError::WouldBlock);
            }
            Some(0)
        } else if msgtyp > 0 {
            // Get first message of type msgtyp
            self.messages.iter().position(|m| m.mtype == msgtyp)
        } else {
            // Get first message with smallest type <= |msgtyp|
            self.messages.iter()
                .enumerate()
                .filter(|(_, m)| m.mtype <= -msgtyp)
                .min_by_key(|(_, m)| m.mtype)
                .map(|(i, _)| i)
        };
        
        if let Some(idx) = msg_idx {
            let msg = self.messages.remove(idx);
            let len = msg.data.len().min(buf.len());
            buf[..len].copy_from_slice(&msg.data[..len]);
            
            self.ds.msg_qnum = self.ds.msg_qnum.saturating_sub(1);
            self.ds.msg_lrpid = pid;
            self.ds.msg_rtime = 0; // In production: current time
            
            Ok((len, msg.mtype))
        } else {
            if flags & IPC_NOWAIT != 0 {
                Err(IpcError::WouldBlock)
            } else {
                Err(IpcError::WouldBlock)
            }
        }
    }
}

/// Message queue registry
pub struct MsgRegistry {
    queues: BTreeMap<i32, MsgQueue>,
    next_id: AtomicI32,
}

impl MsgRegistry {
    pub fn new() -> Self {
        Self {
            queues: BTreeMap::new(),
            next_id: AtomicI32::new(1),
        }
    }
    
    /// Get message queue (msgget)
    pub fn get(&mut self, key: i32, flags: i32, uid: u32, gid: u32) -> Result<i32, IpcError> {
        let mode = (flags & 0o777) as u16;
        
        if key == IPC_PRIVATE {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let queue = MsgQueue::new(key, mode, uid, gid);
            self.queues.insert(id, queue);
            return Ok(id);
        }
        
        if let Some((&id, _)) = self.queues.iter().find(|(_, q)| q.ds.msg_perm.key == key) {
            if flags & IPC_CREAT != 0 && flags & IPC_EXCL != 0 {
                return Err(IpcError::AlreadyExists);
            }
            return Ok(id);
        }
        
        if flags & IPC_CREAT != 0 {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let queue = MsgQueue::new(key, mode, uid, gid);
            self.queues.insert(id, queue);
            Ok(id)
        } else {
            Err(IpcError::NotFound)
        }
    }
    
    /// Send message (msgsnd)
    pub fn send(&mut self, id: i32, mtype: i64, data: &[u8], pid: i32, flags: i32) -> Result<(), IpcError> {
        let queue = self.queues.get_mut(&id).ok_or(IpcError::NotFound)?;
        queue.send(mtype, data, pid, flags)
    }
    
    /// Receive message (msgrcv)
    pub fn receive(&mut self, id: i32, msgtyp: i64, buf: &mut [u8], pid: i32, flags: i32) -> Result<(usize, i64), IpcError> {
        let queue = self.queues.get_mut(&id).ok_or(IpcError::NotFound)?;
        queue.receive(msgtyp, buf, pid, flags)
    }
    
    /// Control operations (msgctl)
    pub fn control(&mut self, id: i32, cmd: i32) -> Result<MsqidDs, IpcError> {
        match cmd {
            IPC_STAT => {
                let queue = self.queues.get(&id).ok_or(IpcError::NotFound)?;
                Ok(queue.ds)
            }
            IPC_RMID => {
                let queue = self.queues.remove(&id).ok_or(IpcError::NotFound)?;
                Ok(queue.ds)
            }
            _ => Err(IpcError::InvalidCommand)
        }
    }
}

/// IPC errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    NotFound,
    AlreadyExists,
    InvalidSize,
    InvalidCommand,
    InvalidSemaphore,
    InvalidType,
    NotAttached,
    WouldBlock,
    PermissionDenied,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shm_create() {
        let mut registry = ShmRegistry::new();
        let id = registry.get(IPC_PRIVATE, 4096, IPC_CREAT | 0o666, 1000, 1000, 1).unwrap();
        assert!(id > 0);
    }
    
    #[test]
    fn test_sem_operations() {
        let mut registry = SemRegistry::new();
        let id = registry.get(IPC_PRIVATE, 1, IPC_CREAT | 0o666, 1000, 1000).unwrap();
        
        // Set value to 1
        registry.control(id, 0, 16, 1).unwrap();
        
        // Get value
        let val = registry.control(id, 0, 12, 0).unwrap();
        assert_eq!(val, 1);
        
        // P operation (decrement)
        let ops = [SemBuf { sem_num: 0, sem_op: -1, sem_flg: 0 }];
        registry.op(id, &ops, 1).unwrap();
        
        let val = registry.control(id, 0, 12, 0).unwrap();
        assert_eq!(val, 0);
    }
    
    #[test]
    fn test_msg_send_receive() {
        let mut registry = MsgRegistry::new();
        let id = registry.get(IPC_PRIVATE, IPC_CREAT | 0o666, 1000, 1000).unwrap();
        
        // Send message
        let data = b"Hello, IPC!";
        registry.send(id, 1, data, 1, 0).unwrap();
        
        // Receive message
        let mut buf = [0u8; 1024];
        let (len, mtype) = registry.receive(id, 0, &mut buf, 2, 0).unwrap();
        
        assert_eq!(len, data.len());
        assert_eq!(mtype, 1);
        assert_eq!(&buf[..len], data);
    }
}
