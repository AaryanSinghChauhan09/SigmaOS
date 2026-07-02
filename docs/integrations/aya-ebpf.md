# aya Rust eBPF Integration

## Overview

SigmaOS uses [aya](https://github.com/aya-rs/aya) (Apache-2.0 / MIT) for safe, pure-Rust eBPF programs. aya allows writing both the eBPF bytecode (runs in kernel context) and the userspace loader in Rust, eliminating the clang/libbpf toolchain dependency.

---

## Use Cases

| Tool | eBPF Use |
|---|---|
| `sigma-trace` | Syscall tracing: audit per-process syscall usage |
| `sigma-net-monitor` | Network monitoring: per-flow byte counters |
| `sigma-ids` | Intrusion detection: block known-malicious packet patterns via XDP |
| `sigma-perf` | CPU profiling: sample kernel stack traces via `perf_event` |

---

## File Layout

```
tools/tracing/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # userspace loader (aya)
    ├── ebpf/
    │   ├── syscall_tracer.rs   # eBPF program (aya-bpf)
    │   └── xdp_filter.rs       # XDP packet filter
    └── events.rs        # shared event types (kernel ↔ userspace)
```

---

## Cargo.toml

```toml
[package]
name    = "sigma-trace"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "sigma-trace"
path = "src/main.rs"

[dependencies]
aya           = { version = "=0.12.0", features = ["async_tokio"] }
aya-log       = "=0.2.0"
tokio         = { version = "=1.38", features = ["full"] }
log           = "0.4"
env_logger    = "0.11"
anyhow        = "1.0"

[build-dependencies]
aya-build     = "=0.1.0"
```

---

## Syscall Tracer eBPF Program

`tools/tracing/src/ebpf/syscall_tracer.rs` (runs in kernel, compiled to BPF bytecode):

```rust
#![no_std]
#![no_main]

use aya_bpf::{
    macros::{tracepoint, map},
    maps::PerfEventArray,
    programs::TracePointContext,
    BpfContext,
};
use aya_log_ebpf::info;

// Shared event type: must match events.rs in userspace
#[repr(C)]
pub struct SyscallEvent {
    pub pid:     u32,
    pub syscall: u32,
    pub comm:    [u8; 16],
}

#[map]
static EVENTS: PerfEventArray<SyscallEvent> = PerfEventArray::new(0);

#[tracepoint(name = "sys_enter")]
pub fn trace_sys_enter(ctx: TracePointContext) -> u32 {
    let pid     = ctx.pid();
    let syscall = unsafe { ctx.read_at::<u32>(8).unwrap_or(0) };
    let mut comm = [0u8; 16];
    unsafe { ctx.read_comm(&mut comm) };

    let event = SyscallEvent { pid, syscall, comm };
    EVENTS.output(&ctx, &event, 0);
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
```

---

## Userspace Loader (src/main.rs)

```rust
use aya::{Bpf, programs::TracePoint, maps::perf::AsyncPerfEventArray};
use aya_log::BpfLogger;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Load compiled BPF object
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/sigma-trace"
    ))?;

    BpfLogger::init(&mut bpf)?;

    // Attach to sys_enter tracepoint
    let prog: &mut TracePoint = bpf.program_mut("trace_sys_enter").unwrap().try_into()?;
    prog.load()?;
    prog.attach("syscalls", "sys_enter")?;

    // Read events from perf buffer
    let mut events: AsyncPerfEventArray<_> =
        bpf.take_map("EVENTS").unwrap().try_into()?;

    println!("sigma-trace running. Press Ctrl+C to stop.");
    signal::ctrl_c().await?;
    Ok(())
}
```

---

## XDP Packet Filter Example

```rust
// tools/tracing/src/ebpf/xdp_filter.rs
#![no_std]
#![no_main]

use aya_bpf::{macros::xdp, programs::XdpContext, bindings::xdp_action};
use network_types::{eth::EthHdr, ip::Ipv4Hdr};

#[xdp(name = "sigma_xdp_filter")]
pub fn sigma_xdp_filter(ctx: XdpContext) -> u32 {
    // Block packets from a known-bad IP (e.g., 203.0.113.0/24)
    if let Ok(ip) = parse_ipv4(&ctx) {
        if ip & 0xFFFFFF00 == 0xCB007100 {  // 203.0.113.0/24
            return xdp_action::XDP_DROP;
        }
    }
    xdp_action::XDP_PASS
}

fn parse_ipv4(ctx: &XdpContext) -> Result<u32, ()> {
    let eth = unsafe { ctx.ptr_at::<EthHdr>(0).ok_or(())? };
    let ip  = unsafe { ctx.ptr_at::<Ipv4Hdr>(EthHdr::LEN).ok_or(())? };
    Ok(unsafe { (*ip).src_addr })
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
```

---

## Exit Criteria

- `sudo sigma-trace` attaches to `sys_enter` tracepoint and prints pid + syscall number for each syscall.
- `sigma-ids --iface eth0` loads the XDP filter on eth0; `ping 203.0.113.1` times out (dropped).
- `cargo build -p sigma-trace --target bpfel-unknown-none` succeeds.
