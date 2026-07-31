# Sovereign OOM Shard

1

**Parity:**Linux OOM Killer � macOS Jetsam � Android LMKD**Location:**`kernel/modules/system/SovereignOOMShard.c`**Standard:** Zenith Industrial Sovereignty v1.0

---

1

The Sovereign OOM Shard provides native, zero-dependency memory-pressure governance for SigmaOS. It absorbs the Linux OOM Killer and macOS Jetsam USPs by providing score-based mission culling under configurable thresholds, with guaranteed protection for kernel-critical shards.

---

1

1

OOM Score Table (24 entries max)
  sigma_kernel_core   PID:1  score:-500  [PROTECTED]   ? never culled
  sigma_wm_compositor PID:2  score:+100
  citizen_browser     PID:42 score:+500  ? high consumer
  guest_sandbox       PID:99 score:+900  ? culled first

Thresholds:
  WARN     < 512 MB free � sweep with logging
  CRITICAL < 128 MB free � aggressive cull, highest-score victim selected

1

---

1

| Sub-command | Action | 
| --- | --- | 
| `sigma-oom reg <name> <pid> <mem_kb> <score> <prot>` | Register a mission in the OOM score table | 
| `sigma-oom sweep <free_kb>` | Trigger an auto-cull sweep at a simulated free memory level | 
| `sigma-oom audit` | Display full OOM table with scores, memory usage, and protection state | 

---

1

1

1

---

1

`GLOBAL MESH ACTIVE` � Synchronized with `AaryanSinghChauhan09/SigmaOS`.
