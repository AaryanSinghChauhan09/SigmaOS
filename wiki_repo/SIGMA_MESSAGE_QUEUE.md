# SigmaOS Message Queue

## Overview

`SigmaMessageQueue` (`src/ipc/sigma_message_queue.rs`) is a sovereign POSIX-compatible, priority-ordered message queue for IPC. It provides POSIX `mq_open` / `mq_send` / `mq_receive` parity without any standard-library dependency beyond `alloc`.

---

## POSIX `mq_open` Parity

| POSIX API | SigmaOS equivalent |
|-----------|-------------------|
| `mq_open(name, O_CREAT, …)` | `SigmaMessageQueue::open(name, attrs)` |
| `mq_send(mqd, buf, len, prio)` | `mq.send(SigmaMessage::new(prio, data, ts))` |
| `mq_receive(mqd, buf, len, &prio)` | `mq.receive()` |
| `mq_getattr(mqd, &attr)` | `mq.attrs()` |
| `mq_close(mqd)` | `mq.close()` |
| `struct mq_attr { mq_maxmsg, mq_msgsize, mq_curmsgs }` | `MessageQueueAttributes { max_msgs, max_msg_size, cur_msgs }` |

### Notable deviations

- Notifications (`mq_notify`) are not yet implemented.
- Non-blocking flag is modelled via immediate errors (`QueueFull` / `QueueEmpty`) rather than `O_NONBLOCK`.
- Time-based blocking (`mq_timedreceive`) is future work.

---

## Priority Ordering

Messages are delivered **highest priority first**. Within the same priority level, FIFO ordering (by `timestamp`) is preserved.

```
Queue state (after 3 sends):

  [priority=255, ts=3] ◄── receive() returns this first
  [priority=128, ts=2]
  [priority=0,   ts=1] ◄── receive() returns this last
```

The internal storage uses a sorted `Vec<SigmaMessage>` with `partition_point` insertion (O(n) send, O(1) receive).

### Priority values

| Value | Symbolic name | Method |
|-------|--------------|--------|
| 0 | Lowest | `SigmaMessage::low(data, ts)` |
| 128 | Normal | `SigmaMessage::normal(data, ts)` |
| 255 | Highest | `SigmaMessage::high(data, ts)` |
| 1–254 | Custom | `SigmaMessage::new(prio, data, ts)` |

---

## API Reference

### `SigmaMessageQueue::open(name, attrs)`

```rust
let attrs = MessageQueueAttributes {
    max_msgs: 64,
    max_msg_size: 8192,
    cur_msgs: 0,
};
let mut mq = SigmaMessageQueue::open("/sigma/events".to_string(), attrs)?;
```

### `SigmaMessageQueue::open_default(name)`

Uses default attributes: 32 messages × 4 KiB.

### `send(msg: SigmaMessage) → Result<(), MessageQueueError>`

| Error | Meaning |
|-------|---------|
| `QueueFull` | `cur_msgs == max_msgs` |
| `MessageTooLarge` | `msg.data.len() > max_msg_size` |
| `Closed` | Queue has been closed |

### `receive() → Result<SigmaMessage, MessageQueueError>`

Returns the highest-priority, oldest message.

| Error | Meaning |
|-------|---------|
| `QueueEmpty` | No messages available |
| `Closed` | Queue has been closed |

### `peek() → Option<&SigmaMessage>`

Non-destructive look at the next message.

### `attrs() → MessageQueueAttributes`

Returns a snapshot of current attributes including live `cur_msgs`.

---

## `SigmaMessage` Structure

```rust
pub struct SigmaMessage {
    pub priority: u8,       // 0 = lowest, 255 = highest
    pub data: Vec<u8>,      // raw payload
    pub timestamp: u64,     // nanoseconds since boot (or logical clock)
}
```

---

## Kernel Integration

### Scheduler wake-up

When a receiver blocks on an empty queue (gets `QueueEmpty`), the scheduler parks the thread on the queue's wait-list. When a sender calls `send()`, the kernel wakes the highest-priority waiter.

### File descriptor table

Each open message queue is assigned an `mqd_t` (message-queue descriptor) in the process's file-descriptor table, pointing to a `SigmaMessageQueue` instance.

### Namespace

Queue names beginning with `/` are registered in a kernel-global IPC namespace (`IpcNamespace`) scoped to the process's IPC namespace, enabling cross-process discovery.

```
/sigma/events   → SigmaMessageQueue { max_msgs: 64, … }
/sigma/logs     → SigmaMessageQueue { max_msgs: 128, … }
```

---

## Example: Producer / Consumer

```rust
// Producer
let mut mq = SigmaMessageQueue::open_default("/work_queue")?;
mq.send(SigmaMessage::high(b"urgent task".to_vec(), now()))?;
mq.send(SigmaMessage::low(b"background task".to_vec(), now()))?;

// Consumer
let msg = mq.receive()?; // always gets "urgent task" first
process(msg.data);
```

---

## See Also

- [`SIGMA_IPC_PIPES.md`](SIGMA_IPC_PIPES.md) — byte-stream IPC
- [`SIGMA_CONCURRENCY_PRIMITIVES.md`](SIGMA_CONCURRENCY_PRIMITIVES.md) — spinlocks, semaphores
- [`SIGMA_RCU_SYNCHRONIZATION.md`](SIGMA_RCU_SYNCHRONIZATION.md) — lockless reads
