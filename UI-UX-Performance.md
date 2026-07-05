# SigmaOS UI/UX & Performance Improvements

> This page documents all UI, UX, and performance subsystems added in batch 6+.

---

## New Desktop Subsystems

### Animation Engine (`userland/desktop/sigma_animation.rs`)
- **Spring physics**: tunable stiffness + damping, settles naturally
- **Easing functions**: linear, quad, cubic, sine, elastic, bounce (7 built-in)
- **Animation timelines**: from/to/duration, delay, loop, auto-reverse
- **Reduce Motion**: system preference disables all animations instantly
- **Convenience**: `window_open()`, `window_close()`, `workspace_switch()` pre-built

### Software Renderer (`userland/desktop/sigma_renderer.rs`)
- **DrawCmd list**: fill_rect, stroke_rect, circle, line, glass_panel, image, text, clip
- **Rounded corners**: anti-aliased corner radius on any rect
- **Glassmorphism**: background blur (3×3 box blur), semi-transparent panels
- **Alpha compositing**: per-pixel alpha blend in all draw operations
- **Image blit**: bilinear-scaled image rendering

### Input Manager (`userland/desktop/sigma_input.rs`)
- **Unified event queue**: keyboard, mouse, touch, gestures in one pipe
- **Gesture recogniser**: tap, double-tap, long-press, swipe (4 dirs), pinch, pan
- **Double-click detection**: < 300ms + position threshold
- **Keyboard shortcuts**: register action → triggered on matching key+mods
- **Modifier tracking**: Ctrl/Alt/Shift/Super state always consistent
- **Touch events**: multi-touch with pressure, swipe velocity calculation

### Panel / Dynamic Island (`userland/desktop/sigma_panel.rs`)
- **Modular items**: PanelItem trait — clock, CPU, memory, battery, network, window title
- **Adaptive colour**: items change colour based on thresholds (CPU red > 80%)
- **Left/Center/Right** layout groups
- **Live updates**: tick() called every frame, items refresh independently

### Settings Hub (`userland/desktop/sigma_settings.rs`)
- **SettingsPanel trait**: pluggable panels (Appearance, Network, Privacy)
- **SettingValue enum**: Bool/Int/Float/String/Enum/Color
- **Appearance**: theme, corner radius, gap, scale, animations, blur, auto-theme
- **Network**: auto-connect, IPv6, DoH, DNS servers, firewall, VPN autostart
- **Privacy**: telemetry (off), crash reports (off), clipboard guard, indicators

### Accessibility (`userland/desktop/sigma_accessibility.rs`)
- **High contrast**: luminance threshold → pure black/white
- **Colour blind modes**: Deuteranopia, Protanopia, Tritanopia (matrix remap)
- **Screen reader**: announcements via eprintln → future TTS integration
- **Large text**: configurable scale multiplier
- **Reduce motion**: disables all animations system-wide
- **Sticky keys**: modifier latch with per-keypress consume

---

## New Tools

### sigma-perf (`userland/tools/sigma_perf.rs`)
- Call tree profiler with enter/exit instrumentation
- Folded stacks output (compatible with FlameGraph tool)
- ASCII flamegraph tree to stdout
- Top-N functions by self time
- RAII `ProfGuard` for automatic scope timing

### sigma-strace (`userland/tools/sigma_strace.nim`)
- Linux ptrace-based syscall tracing
- Syscall name lookup table (all 33 SigmaOS syscalls)
- Per-syscall statistics: count, total time, average time
- Filter by syscall name or min duration

### sigma-top (`userland/tools/sigma_top.nim`)
- Real-time process monitor (no ncurses dependency)
- Sort by CPU/MEM/PID/name
- Colour-coded process state (R=green, S=cyan, D=red, Z=magenta)
- System header: uptime, load avg, CPU%, mem usage

---

## New Kernel / Runtime

### QUIC transport (`kernel/net/sigma_quic.rs`)
- RFC 9000 packet header builder (Initial/Handshake/Short)
- Stream management (open/close/state machine)
- Flow control (max_data local/remote)
- Connection lifecycle (Initial→Handshake→Connected→Closing)

### eBPF VM (`kernel/core/sigma_ebpf.rs`)
- ALU64/ALU opcodes: add/sub/mul/div/and/or/xor/mov/lsh/rsh
- JMP opcodes: ja/jeq/jne/jgt/jge/jlt/jle/exit
- Bytecode verifier: register init tracking, bounds check, no backward jumps
- Interpreter: 11 registers + 512-byte stack

### Capability Tokens (`kernel/security/sigma_capability.rs`)
- seL4-inspired capability derivation (can only restrict, never expand)
- 14 rights bits: READ/WRITE/EXEC/SEND/RECV/GRANT/REVOKE/IO/MMAP/FORK/NET/DEVICE/SETUID/KILL
- Revocation cascade: revoke a cap → all derived caps revoked
- CapResource: File/Fd/Ipc/Memory/Device/Network/Process

### ELF Loader (`runtime/elf/sigma_elf_loader.rs`)
- Validates ELF64 magic, class, data encoding, machine type
- Parses PT_LOAD and PT_INTERP segments
- Maps segments with caller-provided page allocator
- BSS zeroing, ASLR offset support
- `no_std` — works in kernel context

### CRDT Sync (`userland/net/sigma_crdt.rs`)
- Vector clocks: increment, merge, happens-before, concurrent
- LWW (Last-Write-Wins) register with conflict resolution
- PN Counter (Positive-Negative) with per-node tracking
- Operation log for peer sync (`ops_since(seq)`)
- `merge_from(ops)` for offline-first reconciliation

---

## Performance Targets

| Metric | Target | Achieved |
|---|---|---|
| Animation frame time | < 16ms (60fps) | Spring physics O(n) |
| Input latency | < 5ms | Lock-free queue |
| Renderer fill_rect | < 2ms for 1920×1080 | Direct pixel write |
| eBPF program verify | < 1ms for 4096 insns | Linear scan |
| CRDT merge (1000 ops) | < 10ms | BTreeMap O(n log n) |
