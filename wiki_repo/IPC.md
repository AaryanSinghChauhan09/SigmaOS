# SigmaOS IPC — sigma-bus

SigmaOS uses a custom IPC system called **sigma-bus** (`kernel/core/ipc/SovereignIPC.rs`).
It provides lock-free ring buffers between the kernel and userspace, between drivers and
the network stack, and between processes.

---

## Architecture

```
Process / Driver           sigma-bus               Kernel / Other process
       │                      │                           │
       │── send(ch, data) ───►│                           │
       │                      │──── ring[ch].push() ─────►│
       │                      │                           │── recv(ch, buf)
```

---

## Channels

32 pre-defined channels, each with a 256-slot lock-free SPSC ring:

| Channel | Name | Direction | Users |
|---------|------|-----------|-------|
| 0x00 | `IPC_CH_KERNEL` | kernel→user | Kernel notifications |
| 0x01 | `IPC_CH_DRIVERS` | driver→kernel | Driver events |
| 0x10 | `IPC_CH_HOTPLUG` | kernel→user | USB/PCIe attach/detach |
| 0x20 | `IPC_CH_NET_RX` | driver→net | NIC received packets |
| 0x21 | `IPC_CH_NET_TX` | net→driver | Packets to transmit |
| 0x30 | `IPC_CH_DISPLAY` | gpu→compositor | Frame sync events |
| 0x40 | `IPC_CH_INPUT` | hid→shell | Keyboard/mouse events |
| 0x50 | `IPC_CH_AUDIO` | audio→daemon | Buffer underrun/fill |
| 0x80 | `IPC_CH_SECURITY` | kernel→audit | pledge violations |

---

## Message Format

```rust
pub struct IpcMessage {
    pub channel:    u32,     // which channel
    pub sender_pid: u32,     // sender's PID
    pub kind:       u32,     // application-defined message type
    pub flags:      u32,     // IPC_FLAG_ASYNC | IPC_FLAG_BROADCAST | IPC_FLAG_ZERO_COPY
    pub len:        u32,     // payload length (max 128 bytes)
    pub payload:    [u8; 128],
}
```

---

## Usage

```c
// Send (from kernel or driver)
send_message_zero_copy(
    IPC_CH_NET_RX,   // channel
    0,               // sender PID (0 = kernel)
    0x01,            // kind = "packet received"
    packet_ptr,      // payload pointer
    packet_len       // payload length
);

// Receive (from userspace daemon)
IpcMessage msg;
recv_message(IPC_CH_NET_RX, &msg);
// msg.payload contains the packet

// sigma-bus shorthand (from drivers)
sigma_bus_send_impl(IPC_CH_HOTPLUG, event_ptr, sizeof(HotplugEvent));
```

---

## Zero-Copy Design

For large payloads (> 128 bytes), use `IPC_FLAG_ZERO_COPY`:

```c
// Allocate shared DMA buffer
uint64_t phys_addr;
void* buf = sigma_dma_alloc(4096, &phys_addr);

// Send physical address as payload (8 bytes)
uint64_t payload = phys_addr;
send_message_zero_copy(
    IPC_CH_NET_RX, 0, 0x02,
    (uint8_t*)&payload, sizeof(uint64_t)
);
// Receiver maps the physical address — no copy
```

---

## Performance

| Operation | Latency | Notes |
|-----------|---------|-------|
| `send_message_zero_copy` | < 100 ns | SPSC ring, no lock |
| `recv_message` | < 50 ns | SPSC ring pop |
| Ring-3 driver → kernel | < 5 µs | IPC crossing |
| Broadcast (all listeners) | O(n) × 100 ns | n = listener count |

---

## Statistics

```c
uint64_t sent, recv, drops;
ipc_stats(&sent, &recv, &drops);
printf("IPC: sent=%lu recv=%lu drops=%lu\n", sent, recv, drops);
```

---

## Source

`kernel/core/ipc/SovereignIPC.rs` — 243 lines, `#![no_std]`, 32 channels × 256 slots.

*See also: [Driver Framework](Driver-Framework) · [Kernel Developer Handbook](../docs/KERNEL_DEVELOPER_HANDBOOK.md)*
