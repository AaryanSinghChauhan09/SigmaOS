# ⏱ SigmaOS RTOS Branch — `release/rtos`

> **Real-Time Sovereign Computing: Determinism without compromise.**

The `release/rtos` branch specializes the SigmaOS kernel for **hard real-time** operation, absorbing algorithms from:
- **FreeRTOS** (task prioritization, tick-based preemption)
- **RTEMS** (POSIX real-time extensions)
- **Zephyr RTOS** (cooperative + preemptive hybrid scheduler)
- **VxWorks** (interrupt latency budgets)
- **QNX Neutrino** (microkernel message-passing RTOS)

---

## 🏗 Architecture Differences from `main`

| Feature | `main` (General) | `release/rtos` |
|---------|-----------------|----------------|
| Scheduler | Round-robin fair | Fixed-priority preemptive |
| Timer resolution | ~10ms tick | Configurable 1µs–1ms |
| IRQ latency | Best-effort | Bounded worst-case (< 10µs) |
| Memory | Dynamic + static | **Static-only** (no heap after boot) |
| Mutex behavior | Spinning | Priority-inheritance |
| Syscall paths | Full dispatch | Optimized fast-path stubs |

---

## 🔢 RTOS Scheduler (Absorbed from FreeRTOS + RTEMS)

### Priority Classes
```
Priority 0 (highest) — Interrupt Service Routines
Priority 1            — Hard Real-Time tasks (deadline = 1ms)
Priority 2            — Soft Real-Time tasks (deadline = 10ms)
Priority 3            — Background / idle tasks
```

### Task Control Block (`sigma_rtos_tcb.cpp`)
```cpp
struct SigmaRTOSTask {
    u64  stack_ptr;          /* Saved RSP (x86_64) */
    u8   priority;           /* 0 = highest */
    u32  deadline_us;        /* Hard deadline in microseconds */
    u32  wcet_us;            /* Worst-case execution time */
    u64  last_activation;    /* TSC at last activation */
    bool periodic;           /* Periodic or sporadic */
    void (*entry)(void*);    /* Task entry point */
    void* arg;               /* Task argument */
};
```

### Scheduling Algorithm: EDF + Fixed-Priority Hybrid
- **Tasks with deadlines** → Earliest Deadline First (EDF)
- **Tasks without deadlines** → Rate Monotonic Scheduling (RMS)
- **Idle task** → `hlt` instruction (power-saving)

---

## ⚡ Interrupt Latency Optimization

Absorbed from **QNX Neutrino** pulse-based IPC and **Zephyr ISR tables**:

1. **Static ISR Table** — all interrupt handlers registered at compile time (no dynamic dispatch overhead)
2. **Interrupt Stack Isolation** — dedicated 4KB interrupt stack per CPU core
3. **Nested Interrupt Masking** — only mask during critical section (not globally)
4. **TSC-based Timing** — `RDTSC` for sub-microsecond timing without hardware timer overhead

---

## 📡 RTOS-Specific Drivers

| Driver | Purpose |
|--------|---------|
| `sigma_hpet.cpp` | High Precision Event Timer — 1µs resolution |
| `sigma_apic_timer.cpp` | Per-CPU APIC timer for task preemption |
| `sigma_watchdog.cpp` | Hardware watchdog (reset on deadline miss) |
| `sigma_rtc.cpp` | Real-Time Clock for timestamp correlation |

---

## 🔒 Safety Constraints (Absorbed from DO-178C / IEC 61508)

- **No dynamic memory allocation** after `sigma_rtos_init()` returns
- **Stack depth pre-calculated** at link time via linker script guards
- **All task priorities static** — no runtime priority changes
- **Deadline monitoring** — missed deadlines log to `sigma_dmesg` ring buffer

---

## 🧪 Testing & Benchmarks

```bash
# Run RTOS latency benchmark (bare-metal QEMU)
qemu-system-x86_64 -kernel sigmaos-rtos.elf -serial stdio -nographic

# Expected output:
# [RTOS] Task A activated: latency = 3.2µs
# [RTOS] Task B activated: latency = 4.1µs
# [RTOS] Worst-case IRQ latency: 7.8µs
```

---

*Branch: `release/rtos` | Based on: SigmaOS v1.1.0 kernel*
