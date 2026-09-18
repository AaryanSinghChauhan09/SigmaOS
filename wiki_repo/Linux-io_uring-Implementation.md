# Linux io_uring Implementation

SigmaOS implements Linux io_uring (I/O URing) for high-performance asynchronous I/O operations with zero-copy support and minimal syscall overhead.

## Overview

io_uring provides:
- Zero-copy I/O operations
- Batched submission and completion
- Polling mode for ultra-low latency
- Fixed ring buffers for lock-free operation
- Support for various I/O types (file, network, etc.)

## Architecture

### io_uring Components
- **Submission Queue (SQ)**: User submits I/O operations
- **Completion Queue (CQ)**: Kernel reports completion status
- **Shared Memory**: Mapped between user and kernel space
- **Submission Queue Entries (SQE)**: I/O operation descriptors
- **Completion Queue Entries (CQE)**: I/O operation results

### Operation Flow
1. **Setup**: Create io_uring instance
2. **Submit**: Add I/O operations to SQ
3. **Wait**: Wait for completions or poll
4. **Complete**: Process CQE results
5. **Cleanup**: Destroy io_uring instance

## Implementation

### io_uring Instance
```rust
// src/kernel/io_uring.rs
pub struct IoUring {
    sq: SubmissionQueue,
    cq: CompletionQueue,
    flags: IoUringFlags,
    ring_fd: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct IoUringFlags {
    pub sq_poll: bool,
    pub sq_affinity: Option<u32>,
    pub sq_cpu: Option<u32>,
    pub sq_thread_idle: Option<u32>,
}

impl IoUring {
    pub fn new(entries: u32, flags: IoUringFlags) -> Result<Self, IoUringError> {
        // Create io_uring via syscalls
        let ring_fd = unsafe {
            io_uring_setup(entries, &mut setup)
        };
        
        if ring_fd < 0 {
            return Err(IoUringError::SetupFailed);
        }
        
        // Map submission and completion queues
        let sq = SubmissionQueue::new(ring_fd, entries)?;
        let cq = CompletionQueue::new(ring_fd, entries)?;
        
        Ok(IoUring {
            sq,
            cq,
            flags,
            ring_fd,
        })
    }

    pub fn submit(&mut self) -> Result<u32, IoUringError> {
        let submitted = unsafe {
            io_uring_enter(
                self.ring_fd,
                self.sq.ready,
                0,
                IORING_ENTER_GETEVENTS,
                std::ptr::null_mut(),
            )
        };
        
        if submitted < 0 {
            return Err(IoUringError::SubmitFailed);
        }
        
        self.sq.advance(submitted as u32);
        Ok(submitted as u32)
    }

    pub fn wait(&mut self, min_complete: u32) -> Result<u32, IoUringError> {
        let completed = unsafe {
            io_uring_enter(
                self.ring_fd,
                0,
                min_complete,
                IORING_ENTER_GETEVENTS,
                std::ptr::null_mut(),
            )
        };
        
        if completed < 0 {
            return Err(IoUringError::WaitFailed);
        }
        
        Ok(completed as u32)
    }
}
```

### Submission Queue
```rust
// src/kernel/io_uring/sq.rs
pub struct SubmissionQueue {
    ring: *mut io_uring_sq,
    entries: u32,
    sqes: *mut io_uring_sqe,
    mask: u32,
    head: AtomicU32,
    tail: AtomicU32,
}

impl SubmissionQueue {
    pub fn get_sqe(&mut self) -> Option<&mut io_uring_sqe> {
        let tail = self.tail.load(Ordering::Acquire);
        let next = (tail + 1) & self.mask;
        let head = self.head.load(Ordering::Acquire);
        
        if next == head {
            return None; // Queue full
        }
        
        let index = tail as usize;
        let sqe = unsafe { &mut *self.sqes.add(index) };
        self.tail.store(next, Ordering::Release);
        
        Some(sqe)
    }

    pub fn submit_read(&mut self, fd: i32, buf: &mut [u8], offset: u64) -> Result<(), IoUringError> {
        let sqe = self.get_sqe().ok_or(IoUringError::QueueFull)?;
        
        unsafe {
            io_uring_prep_read(sqe, fd, buf.as_mut_ptr(), buf.len(), offset);
        }
        
        Ok(())
    }

    pub fn submit_write(&mut self, fd: i32, buf: &[u8], offset: u64) -> Result<(), IoUringError> {
        let sqe = self.get_sqe().ok_or(IoUringError::QueueFull)?;
        
        unsafe {
            io_uring_prep_write(sqe, fd, buf.as_ptr(), buf.len(), offset);
        }
        
        Ok(())
    }
}
```

### Completion Queue
```rust
// src/kernel/io_uring/cq.rs
pub struct CompletionQueue {
    ring: *mut io_uring_cq,
    entries: u32,
    cqes: *mut io_uring_cqe,
    mask: u32,
    head: AtomicU32,
    tail: AtomicU32,
}

impl CompletionQueue {
    pub fn peek(&self) -> Option<&io_uring_cqe> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        if head == tail {
            return None; // Queue empty
        }
        
        let index = head as usize;
        let cqe = unsafe { &*self.cqes.add(index) };
        Some(cqe)
    }

    pub fn advance(&mut self, count: u32) {
        let head = self.head.load(Ordering::Acquire);
        let new_head = (head + count) & self.mask;
        self.head.store(new_head, Ordering::Release);
        
        unsafe {
            (*self.ring).head.store(new_head, Ordering::Release);
        }
    }

    pub fn get_result(&self, cqe: &io_uring_cqe) -> i32 {
        cqe.res
    }
}
```

### I/O Operations
```rust
// src/kernel/io_uring/ops.rs
impl IoUring {
    pub fn read(&mut self, fd: i32, buf: &mut [u8], offset: u64) -> Result<i32, IoUringError> {
        self.submit_read(fd, buf, offset)?;
        self.submit()?;
        self.wait(1)?;
        
        if let Some(cqe) = self.cq.peek() {
            let result = self.cq.get_result(cqe);
            self.cq.advance(1);
            return Ok(result);
        }
        
        Err(IoUringError::NoCompletion)
    }

    pub fn write(&mut self, fd: i32, buf: &[u8], offset: u64) -> Result<i32, IoUringError> {
        self.submit_write(fd, buf, offset)?;
        self.submit()?;
        self.wait(1)?;
        
        if let Some(cqe) = self.cq.peek() {
            let result = self.cq.get_result(cqe);
            self.cq.advance(1);
            return Ok(result);
        }
        
        Err(IoUringError::NoCompletion)
    }

    pub fn batch_reads(&mut self, ops: Vec<(i32, &mut [u8], u64)>) -> Result<Vec<i32>, IoUringError> {
        for (fd, buf, offset) in ops.iter() {
            self.submit_read(*fd, buf, *offset)?;
        }
        
        let submitted = self.submit()?;
        self.wait(submitted)?;
        
        let mut results = Vec::new();
        for _ in 0..submitted {
            if let Some(cqe) = self.cq.peek() {
                results.push(self.cq.get_result(cqe));
                self.cq.advance(1);
            }
        }
        
        Ok(results)
    }
}
```

## Configuration

### io_uring Configuration
```toml
# /etc/sigmaos/io_uring.toml
[defaults]
entries = 128
sq_thread_idle = 1000

[sq_poll]
enabled = false
cpu = 0
idle_ms = 1000

[limits]
max_entries = 4096
max_sqpoll_workers = 4
```

### Runtime Control
```bash
# View io_uring statistics
siguring stats

# Set default queue size
siguring set entries 256

# Enable SQ polling
siguring sq-poll enable --cpu 0

# Disable SQ polling
siguring sq-poll disable

# View active instances
siguring list
```

## Performance Optimization

### Zero-Copy I/O
Use registered buffers for zero-copy operations:
```rust
let mut ring = IoUring::new(128, flags)?;
ring.register_buffers(&buffers)?;

// Zero-copy read
ring.read_fixed(fd, buf_index, offset)?;
```

### Polling Mode
For ultra-low latency:
```rust
let flags = IoUringFlags {
    sq_poll: true,
    sq_cpu: Some(0),
    sq_thread_idle: Some(100),
    ..Default::default()
};

let mut ring = IoUring::new(128, flags)?;
```

### Batched Operations
Batch multiple I/O operations for efficiency:
```rust
ring.submit_read(fd1, buf1, offset1)?;
ring.submit_read(fd2, buf2, offset2)?;
ring.submit_write(fd3, buf3, offset3)?;
ring.submit()?;

ring.wait(3)?;
```

## Troubleshooting

### Poor Performance
If io_uring performance is poor:
1. Check queue size: `siguring stats`
2. Enable SQ polling for high I/O rates
3. Use batched operations
4. Consider zero-copy with registered buffers
5. Check CPU affinity

### High CPU Usage
If CPU usage is high:
1. Disable SQ polling: `siguring sq-poll disable`
2. Increase idle timeout
3. Reduce queue size
4. Use event-driven mode instead of polling

### Setup Failures
If io_uring setup fails:
1. Check kernel version (requires 5.1+)
2. Verify permissions
3. Check memory limits
4. Review kernel config: `grep IO_URING /boot/config`

---

**[Performance & Kernel](Category-Performance)** | **[I/O Subsystem](I-O-Subsystem)** | **[Linux Compatibility](Linux-Compatibility)**
