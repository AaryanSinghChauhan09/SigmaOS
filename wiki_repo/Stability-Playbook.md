# Stability Playbook (Kernel Networking + Safe Boot)

This page tracks the **minimum wiring needed for “it boots and stays up”** while SigmaOS continues to replace external abstractions with sovereign subsystems.

## Networking: make the stack actually send/receive

### TX: `kernel/net/sigma_net.c` → driver hook

- The kernel network stack emits packets via:
  - `nic_tx_packet(sigma_u8* frame, sigma_u32 len)`
- The current implementation routes TX to the compiled e1000 driver:
  - `kernel/core/drivers/SovereignE1000.cpp` implements `nic_tx_packet(...)` and forwards to `SovereignE1000::transmit(...)`.
- ICMP echo replies are enabled in the stack:
  - `sigma_net_receive_frame(...)` replies to ICMP Echo Request and calls `nic_tx_packet(...)`.

### RX: NIC driver → `sigma_net_receive_frame(...)`

- RX delivery is unified via:
  - `nic_rx_deliver(sigma_u8* frame, sigma_u32 len)` in `kernel/core/network/SovereignNICDriver.cpp`
- `nic_rx_deliver(...)` calls:
  - `sigma_net_receive_frame(frame, len)` (Ethernet frame parse/dispatch)

**Driver rule**: any NIC implementation (VirtIO/RTL8139/e1000/etc.) should call `nic_rx_deliver(...)` when it has a complete Ethernet frame.

## Syscalls: a single kernel socket allocation path

- The syscall gate supports `SIGMA_SYS_SOCKET (0x05)` in:
  - `kernel/core/syscall/SovereignSyscall.cpp`
- Behavior:
  - `SIGMA_SYS_SOCKET(arg1=protocol)` returns a kernel socket handle by calling `sigma_net_socket_create(protocol)`.

Protocol values live in `kernel/net/sigma_net_socket.cpp`:

- `SIGMA_PROTO_TCP`
- `SIGMA_PROTO_UDP`
- `SIGMA_PROTO_RAW`

## Container orchestration: native namespace/cgroup intent

- `userland/tools/sigma_pod_cli.cpp` now supports:
  - `sigma-pod run-native <pkg> [--all-ns|--net|--ipc] [--cpu=<ms>] [--mem=<mb>] [--io=<w>]`
- The CLI builds a `SigmaPodNativeSpec` and sends it to orchestrator shard IPC as:
  - `SIGMA_MSG_SPAWN_NATIVE_CONTAINER`
- Intent:
  - direct kernel namespace selection (`mnt/pid/uts/net/ipc`)
  - explicit cgroup limits (CPU, memory, I/O weight)
  - no Docker/Podman dependency path in SigmaOS runtime

## Boot resilience: rollback gate → resilient safe mode

Early boot now checks whether repeated boots have failed:

- `sigma_rollback_check_fallback()` (from `kernel/resilience/sigma_rollback.cpp`)
- If fallback is requested, the kernel enters resilient mode:
  - `sigma_resilient_fallback_entry("...")` (from `kernel/resilience/sigma_micro_fallback.cpp`)

After reaching a “stable enough” point, the boot is marked known-good:

- `sigma_rollback_mark_boot_successful()`

## Minimal / Safe build profile

To boot with fewer moving parts, the kernel supports:

- `SIGMA_MINIMAL_MODE=1`

In minimal mode, `kernel/core/sigma_kernel_main.c` skips scheduler/task bring-up and marks the boot successful after basic HAL init.

