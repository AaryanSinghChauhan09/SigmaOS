// SPDX-License-Identifier: MIT
// SigmaOS - POSIX Message Queue Filesystem (mqueue)
// Inspired by Linux mqueue filesystem (fs/mqueue/mqueue.c)
// Provides VFS mount at /dev/mqueue for POSIX message queue operations

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::kernel::msgqueue::{Message, MessageQueue, MessageQueueManager, MsgQPerm};

/// mqueue filesystem inode
#[derive(Debug, Clone)]
pub struct MqueueInode {
    pub name: String,
    pub queue_id: Option<u64>,
    pub file_type: MqueueFileType,
    pub mode: u16,
    pub uid: u32,
    pub gid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MqueueFileType {
    Directory,
    QueueFile,
}

/// mqueue superblock representing mounted /dev/mqueue
pub struct MqueueSuperblock {
    pub queues: MessageQueueManager,
    pub inodes: BTreeMap<String, MqueueInode>,
    pub next_inode_id: u64,
}

impl MqueueSuperblock {
    pub fn new() -> Self {
        let mut inodes = BTreeMap::new();

        // Create root directory
        inodes.insert(
            ".".to_string(),
            MqueueInode {
                name: ".".to_string(),
                queue_id: None,
                file_type: MqueueFileType::Directory,
                mode: 0o755,
                uid: 0,
                gid: 0,
            },
        );

        Self {
            queues: MessageQueueManager::new(),
            inodes,
            next_inode_id: 1,
        }
    }

    /// Create a new message queue (mknod /dev/mqueue/<name>)
    pub fn create_queue(
        &mut self,
        name: &str,
        key: i32,
        perm: MsgQPerm,
        max_size: usize,
    ) -> Result<u64, String> {
        if self.inodes.contains_key(name) {
            return Err(format!("Queue {} already exists", name));
        }

        let queue_id = self.queues.create_queue(key, perm, max_size);

        let inode = MqueueInode {
            name: name.to_string(),
            queue_id: Some(queue_id),
            file_type: MqueueFileType::QueueFile,
            mode: 0o600,
            uid: 0,
            gid: 0,
        };

        self.inodes.insert(name.to_string(), inode);
        Ok(queue_id)
    }

    /// Open a message queue by name
    pub fn open_queue(&self, name: &str) -> Result<u64, String> {
        let inode = self
            .inodes
            .get(name)
            .ok_or_else(|| format!("Queue {} not found", name))?;

        inode
            .queue_id
            .ok_or_else(|| format!("{} is not a queue", name))
    }

    /// Send a message to a named queue
    pub fn send(&mut self, name: &str, message: Message) -> Result<(), String> {
        let queue_id = self.open_queue(name)?;
        self.queues.send(queue_id, message)
    }

    /// Receive a message from a named queue
    pub fn receive(&mut self, name: &str, mtype: i64) -> Result<Message, String> {
        let queue_id = self.open_queue(name)?;
        self.queues.receive(queue_id, mtype)
    }

    /// Unlink/remove a message queue
    pub fn unlink_queue(&mut self, name: &str) -> Result<(), String> {
        let inode = self
            .inodes
            .remove(name)
            .ok_or_else(|| format!("Queue {} not found", name))?;

        if let Some(queue_id) = inode.queue_id {
            self.queues.remove_queue(queue_id)?;
        }

        Ok(())
    }

    /// List all queue names in /dev/mqueue
    pub fn list_queues(&self) -> Vec<String> {
        self.inodes
            .keys()
            .filter(|name| *name != ".")
            .cloned()
            .collect()
    }

    /// Get queue statistics
    pub fn get_queue_stats(&self, name: &str) -> Result<MqueueStats, String> {
        let queue_id = self.open_queue(name)?;
        let queue = self
            .queues
            .get_queue(queue_id)
            .ok_or_else(|| format!("Queue {} not found", name))?;

        Ok(MqueueStats {
            message_count: queue.message_count(),
            max_size: queue.max_size,
            is_empty: queue.is_empty(),
            is_full: queue.is_full(),
        })
    }
}

/// Queue statistics
#[derive(Debug, Clone)]
pub struct MqueueStats {
    pub message_count: usize,
    pub max_size: usize,
    pub is_empty: bool,
    pub is_full: bool,
}

impl Default for MqueueSuperblock {
    fn default() -> Self {
        Self::new()
    }
}

/// Global mqueue filesystem instance (must be initialized externally)
pub static mut MQUEUE_FS: Option<MqueueSuperblock> = None;

/// Initialize the mqueue filesystem
pub fn init_mqueue_fs() {
    unsafe {
        MQUEUE_FS = Some(MqueueSuperblock::new());
    }
}

/// Get the global mqueue filesystem
pub fn get_mqueue_fs() -> Option<&'static mut MqueueSuperblock> {
    unsafe { MQUEUE_FS.as_mut() }
}

/// POSIX mq_open - open or create a message queue
pub fn mq_open(
    name: &str,
    oflag: u32,
    mode: u16,
    attr: Option<&MqueueAttr>,
) -> Result<i32, String> {
    let fs = get_mqueue_fs().ok_or_else(|| "mqueue filesystem not initialized".to_string())?;

    if oflag & 0o0100 != 0 {
        // O_CREAT
        let key = name.len() as i32;
        let perm = MsgQPerm::new().with_read().with_write();
        let max_size = attr.map(|a| a.max_msg).unwrap_or(10);

        let queue_id = fs.create_queue(name, key, perm, max_size)?;
        Ok(queue_id as i32)
    } else {
        let queue_id = fs.open_queue(name)?;
        Ok(queue_id as i32)
    }
}

/// POSIX mq_close - close a message queue descriptor
pub fn mq_close(mqdes: i32) -> Result<(), String> {
    // In this implementation, mqdes is the queue_id
    // Close is a no-op as queues persist until mq_unlink
    Ok(())
}

/// POSIX mq_unlink - remove a message queue
pub fn mq_unlink(name: &str) -> Result<(), String> {
    let fs = get_mqueue_fs().ok_or_else(|| "mqueue filesystem not initialized".to_string())?;
    fs.unlink_queue(name)
}

/// POSIX mq_send - send a message
pub fn mq_send(mqdes: i32, msg_ptr: &[u8], msg_prio: u32) -> Result<(), String> {
    let fs = get_mqueue_fs().ok_or_else(|| "mqueue filesystem not initialized".to_string())?;

    // Find queue by ID
    let queue_name = fs
        .inodes
        .values()
        .find(|inode| inode.queue_id == Some(mqdes as u64))
        .map(|inode| inode.name.clone())
        .ok_or_else(|| format!("Queue descriptor {} invalid", mqdes))?;

    let message = Message::new(msg_prio as i64, msg_ptr.to_vec());
    fs.send(&queue_name, message)
}

/// POSIX mq_receive - receive a message
pub fn mq_receive(mqdes: i32, msg_prio: i64) -> Result<Vec<u8>, String> {
    let fs = get_mqueue_fs().ok_or_else(|| "mqueue filesystem not initialized".to_string())?;

    // Find queue by ID
    let queue_name = fs
        .inodes
        .values()
        .find(|inode| inode.queue_id == Some(mqdes as u64))
        .map(|inode| inode.name.clone())
        .ok_or_else(|| format!("Queue descriptor {} invalid", mqdes))?;

    let message = fs.receive(&queue_name, msg_prio)?;
    Ok(message.data)
}

/// Message queue attributes
#[derive(Debug, Clone)]
pub struct MqueueAttr {
    pub max_msg: usize,
    pub msg_size: usize,
    pub cur_msgs: usize,
    pub flags: i32,
}

impl Default for MqueueAttr {
    fn default() -> Self {
        Self {
            max_msg: 10,
            msg_size: 8192,
            cur_msgs: 0,
            flags: 0,
        }
    }
}

/// POSIX mq_getattr - get queue attributes
pub fn mq_getattr(mqdes: i32) -> Result<MqueueAttr, String> {
    let fs = get_mqueue_fs().ok_or_else(|| "mqueue filesystem not initialized".to_string())?;

    // Find queue by ID
    let queue_name = fs
        .inodes
        .values()
        .find(|inode| inode.queue_id == Some(mqdes as u64))
        .map(|inode| inode.name.clone())
        .ok_or_else(|| format!("Queue descriptor {} invalid", mqdes))?;

    let stats = fs.get_queue_stats(&queue_name)?;

    Ok(MqueueAttr {
        max_msg: stats.max_size,
        msg_size: 8192,
        cur_msgs: stats.message_count,
        flags: 0,
    })
}

/// POSIX mq_setattr - set queue attributes
pub fn mq_setattr(
    mqdes: i32,
    newattr: &MqueueAttr,
    oldattr: Option<&mut MqueueAttr>,
) -> Result<(), String> {
    // For now, only allow setting flags (non-blocking mode)
    // max_msg and msg_size are set at creation time in this implementation
    if let Some(old) = oldattr {
        let current = mq_getattr(mqdes)?;
        *old = current;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mqueue_superblock_init() {
        let sb = MqueueSuperblock::new();
        assert!(sb.inodes.contains_key("."));
        assert_eq!(sb.list_queues().len(), 0);
    }

    #[test]
    fn test_create_queue() {
        let mut sb = MqueueSuperblock::new();
        let perm = MsgQPerm::new().with_read().with_write();

        let queue_id = sb.create_queue("test_queue", 12345, perm, 10).unwrap();
        assert!(queue_id > 0);
        assert!(sb.inodes.contains_key("test_queue"));
    }

    #[test]
    fn test_create_duplicate_queue() {
        let mut sb = MqueueSuperblock::new();
        let perm = MsgQPerm::new().with_read().with_write();

        sb.create_queue("test_queue", 12345, perm, 10).unwrap();
        assert!(sb.create_queue("test_queue", 12346, perm, 10).is_err());
    }

    #[test]
    fn test_send_receive() {
        let mut sb = MqueueSuperblock::new();
        let perm = MsgQPerm::new().with_read().with_write();

        sb.create_queue("test_queue", 12345, perm, 10).unwrap();

        let message = Message::new(1, b"Hello".to_vec());
        sb.send("test_queue", message).unwrap();

        let received = sb.receive("test_queue", 0).unwrap();
        assert_eq!(received.data, b"Hello");
    }

    #[test]
    fn test_unlink_queue() {
        let mut sb = MqueueSuperblock::new();
        let perm = MsgQPerm::new().with_read().with_write();

        sb.create_queue("test_queue", 12345, perm, 10).unwrap();
        sb.unlink_queue("test_queue").unwrap();

        assert!(!sb.inodes.contains_key("test_queue"));
    }

    #[test]
    fn test_list_queues() {
        let mut sb = MqueueSuperblock::new();
        let perm = MsgQPerm::new().with_read().with_write();

        sb.create_queue("queue1", 12345, perm, 10).unwrap();
        sb.create_queue("queue2", 12346, perm, 10).unwrap();

        let queues = sb.list_queues();
        assert_eq!(queues.len(), 2);
        assert!(queues.contains(&"queue1".to_string()));
        assert!(queues.contains(&"queue2".to_string()));
    }

    #[test]
    fn test_get_queue_stats() {
        let mut sb = MqueueSuperblock::new();
        let perm = MsgQPerm::new().with_read().with_write();

        sb.create_queue("test_queue", 12345, perm, 10).unwrap();

        let message = Message::new(1, b"Hello".to_vec());
        sb.send("test_queue", message).unwrap();

        let stats = sb.get_queue_stats("test_queue").unwrap();
        assert_eq!(stats.message_count, 1);
        assert!(!stats.is_empty);
    }

    #[test]
    fn test_mq_open_create() {
        init_mqueue_fs();

        let mqdes = mq_open("test_queue", 0o0100, 0o600, None).unwrap();
        assert!(mqdes > 0);
    }

    #[test]
    fn test_mq_open_existing() {
        init_mqueue_fs();

        mq_open("test_queue", 0o0100, 0o600, None).unwrap();
        let mqdes = mq_open("test_queue", 0, 0o600, None).unwrap();
        assert!(mqdes > 0);
    }

    #[test]
    fn test_mq_send_receive() {
        init_mqueue_fs();

        let mqdes = mq_open("test_queue", 0o0100, 0o600, None).unwrap();
        mq_send(mqdes, b"Hello", 1).unwrap();

        let received = mq_receive(mqdes, 0).unwrap();
        assert_eq!(received, b"Hello");
    }

    #[test]
    fn test_mq_unlink() {
        init_mqueue_fs();

        mq_open("test_queue", 0o0100, 0o600, None).unwrap();
        mq_unlink("test_queue").unwrap();

        assert!(mq_open("test_queue", 0, 0o600, None).is_err());
    }

    #[test]
    fn test_mq_getattr() {
        init_mqueue_fs();

        let mqdes = mq_open("test_queue", 0o0100, 0o600, None).unwrap();
        let attr = mq_getattr(mqdes).unwrap();

        assert_eq!(attr.max_msg, 10);
        assert_eq!(attr.msg_size, 8192);
    }
}
