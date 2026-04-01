# 🛠️ Sovereign Tools Reference

All tools in `sovereign_tools/` are pure C11, zero-dependency binaries that compile with `-nostdlib -ffreestanding`.

---

## Tool Index

| Tool | File | Category | Description |
|------|------|----------|-------------|
| **Build Master** | `SovereignBuildMaster.c` | DevOps | Audits codebase for standard library violations (`stdio.h`, `windows.h` etc.) |
| **AI Distributor** | `sigma_ai_distribute.c` | AI | Routes prompts to multiple local LLMs concurrently via IPC sockets |
| **Auto Optimizer** | `sigma_auto_optimizer.c` | System | Autonomous RAM balancer with OOM sacrifice logic via `SYS_MADVISE` |
| **System Cleaner** | `system_cleaner.c` | Security | Multi-pass DOD 5220.22-M forensic memory and disk wiper |
| **XClicker** | `xclicker.c` | Automation | Native auto-click event injector |
| **Studio** | `studio.c` | Media | Creative media file automation shard |
| **Remote Bot** | `remote_bot.c` | Automation | Remote session instruction daemon |
| **Academy** | `academy.c` | Education | Educational content delivery shard |
| **Gaming** | `gaming.c` | Gaming | Low-latency gaming session orchestrator |
| **Backup Manager** | `backup_manager.c` | Storage | Incremental backup scheduling daemon |

---

## `sigma_ai_distribute.c` — AI Distributor

**Mission**: Distribute prompt payloads to multiple local AI models simultaneously using IPC socket routing.

**OOP Class**: `AIModel_t` (inherits `SigmaObject_t`)

```c
// Create model instances
AIModel_t local_llm  = create_ai_model("Sigma_QWen_local",    "/var/ipc/sigma_llm.sock",      100);
AIModel_t code_model = create_ai_model("Sigma_StarCoder",      "/var/ipc/sigma_code.sock",      80);
AIModel_t forensic   = create_ai_model("Sigma_Forensic_Analyst","/var/ipc/sigma_forensic.sock", 95);

// Polymorphic dispatch
local_llm.dispatch(&local_llm, "Analyze kernel memory for unauthorized hooks.");
```

**Key Syscalls**: `SYS_EXIT (60)` for graceful kernel-level termination.

---

## `sigma_auto_optimizer.c` — Auto Optimizer

**Mission**: Monitor resource domains and apply autonomous rebalancing using native memory syscalls.

**OOP Class**: `NodeResource_t` (inherits `SigmaObject_t`)

**Virtual Methods**:
- `balance()` — Halves current usage, calls `SYS_MADVISE (28)` to release pages
- `scale_up(extra_kb)` — Dynamically grows the shard quota
- `evict()` — OOM killer: zeros usage and logs sacrifice pattern  

```c
NodeResource_t ai_shard = create_resource("Matrix_Compute_Ring", 4096);
ai_shard.scale_up(&ai_shard, 8192);  // Heavy compute requested
ai_shard.balance(&ai_shard);
```

---

## `system_cleaner.c` — Amnesic Scrubber

**Mission**: Perform forensic-grade zero-trust memory wiping.

**OOP Class**: `MemoryScrubber_t` (inherits `SigmaObject_t`)

**Standard**: DOD 5220.22-M (multi-pass zero-write + sync)

```c
MemoryScrubber_t ram  = create_scrubber("Kernel_Memory_Pages", 3);
MemoryScrubber_t disk = create_scrubber("VFS_Temporary_Blocks", 7);

ram.scrub(&ram);
ram.report(&ram);
```

Each pass triggers `SYS_SYNC (162)` to flush caches to block devices.

---

## `SovereignBuildMaster.c` — Sovereignty Auditor

**Mission**: Scan all kernel source files and flag any use of forbidden headers.

**Forbidden Headers**:
```
stdio.h  stdlib.h  string.h  windows.h  unistd.h
```

Run before every build to guarantee zero-dependency compliance.

---

## Build All Tools

```powershell
gcc -nostdlib -ffreestanding -std=c11 -o sigma_ai_distribute sovereign_tools/sigma_ai_distribute.c libc/sigma_libc.c
gcc -nostdlib -ffreestanding -std=c11 -o sigma_auto_optimizer sovereign_tools/sigma_auto_optimizer.c libc/sigma_libc.c
gcc -nostdlib -ffreestanding -std=c11 -o system_cleaner sovereign_tools/system_cleaner.c libc/sigma_libc.c
```
