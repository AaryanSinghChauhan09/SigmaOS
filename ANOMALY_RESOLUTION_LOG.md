# ANOMALY RESOLUTION LOG

> **Component**: `kernel/self_healing/` | **Format**: Chronological | **Auto-updated**: Via sigma-automation

This log records all detected system anomalies, their root cause analysis, and the resolutions applied by the SigmaOS Self-Healing subsystem or maintainers. It serves as a living post-mortem database.

---

## Log Format

Each entry follows this structure:

```yaml
id: ANOMALY-XXXX
date: YYYY-MM-DD
severity: CRITICAL | HIGH | MEDIUM | LOW
component: kernel subsystem or shard name
symptom: Observable behavior
root_cause: Analyzed cause
resolution: Action taken
pr: GitHub PR number (if applicable)
status: RESOLVED | MITIGATED | MONITORING | OPEN
```

---

## Anomaly Entries

---

### ANOMALY-0001

```yaml
id: ANOMALY-0001
date: 2025-01-15
severity: HIGH
component: kernel/net/firewall/sigma_shield
symptom: >
  SigmaShield packet filter applying incorrect rules for IPv6 fragmented
  packets, causing legitimate ICMPv6 neighbor discovery to be silently
  dropped, breaking IPv6 connectivity on dual-stack interfaces.
root_cause: >
  Fragment offset calculation in sigma_shield.rs was using u16 arithmetic
  that overflowed for packets with offset > 32767. The mask was applied
  before the boundary check, yielding incorrect fragment chain tracking.
resolution: >
  Fixed arithmetic to use u32 for intermediate calculations.
  Added dedicated IPv6 fragment reassembly state machine.
  Added regression test: tests/net/ipv6_fragment_ndp.rs
pr: 142
status: RESOLVED
```

---

### ANOMALY-0002

```yaml
id: ANOMALY-0002
date: 2025-02-03
severity: MEDIUM
component: kernel/hal/thermal/mod.rs
symptom: >
  Thermal daemon reporting incorrect temperatures on AMD Zen 4 CPUs.
  'k10temp' sensor readings 15°C lower than actual, causing autotuner
  to allow sustained boost clocks beyond safe limits.
root_cause: >
  HAL thermal driver was reading Tctl (control temperature) instead of
  Tdie (die temperature). On Zen 4, Tctl includes a +15°C offset that
  is not reflected in actual die temperature.
resolution: >
  Added CPU family/model detection to select correct thermal sensor.
  For AMD Family 19h (Zen 4): read Tdie directly, not Tctl.
  Added unit test with mock sensor registry.
pr: 178
status: RESOLVED
```

---

### ANOMALY-0003

```yaml
id: ANOMALY-0003
date: 2025-03-12
severity: CRITICAL
component: kernel/security/cgroups/mod.rs
symptom: >
  Memory accounting in cgroup namespace was producing negative values
  after container restart, causing OOM killer to incorrectly target
  host processes instead of containerized workloads.
root_cause: >
  AtomicI64 counter for `rss_bytes` was not reset to zero on cgroup
  teardown/reinit cycle. Subsequent container's memory was subtracted
  from a nonzero (potentially negative) base.
resolution: >
  Added explicit counter reset in cgroup_ns_destroy().
  Added assertion in cgroup_ns_init() that all counters are zero.
  Added integration test: tests/security/cgroup_restart_cycle.rs
pr: 201
status: RESOLVED
```

---

### ANOMALY-0004

```yaml
id: ANOMALY-0004
date: 2025-04-20
severity: LOW
component: sigpkg (package manager)
symptom: >
  sigpkg install --dry-run modifying the on-disk package database
  despite the dry-run flag, causing inconsistent state on next real install.
root_cause: >
  The transaction commit function checked the dry-run flag AFTER writing
  to the database, not before. The flag was tested for external output
  only (no package binaries were extracted), but the DB update path
  was unconditional.
resolution: >
  Moved dry-run check to before any mutation in Transaction::commit().
  All write operations gated behind a single DryRun::is_live() guard.
  Added test: sigpkg/tests/dry_run_no_db_mutation.rs
pr: 234
status: RESOLVED
```

---

### ANOMALY-0005

```yaml
id: ANOMALY-0005
date: 2025-05-08
severity: MEDIUM
component: kernel/ipc/sigma_bus
symptom: >
  sigma-bus ring-buffer producer stalling intermittently under high IPC
  load (>100K msg/s). Shard-to-shard latency spikes from ~250ns to >10ms
  for ~50ms intervals.
root_cause: >
  Ring-buffer consumer was using a spin-wait with exponential backoff that
  backed off too aggressively. Under burst load, the backoff reached 512μs
  while the producer had already enqueued 4096 messages, causing head-of-
  line blocking across all waiting producers.
resolution: >
  Replaced exponential backoff with adaptive backoff capped at 16μs.
  Added producer-side condition variable notification when ring >75% full.
  Benchmark: IPC throughput stable at 280K msg/s under sustained load.
pr: 267
status: RESOLVED
```

---

### ANOMALY-0006

```yaml
id: ANOMALY-0006
date: 2025-06-15
severity: HIGH
component: kernel/crypto/pqc_dilithium
symptom: >
  Package signature verification failing on ARM64 with Dilithium5 for
  packages larger than 16MB. Verification returns INVALID_SIGNATURE
  for valid packages.
root_cause: >
  SHA3-256 intermediate state was stored in a stack buffer sized for
  32-bit alignment. On ARM64 with strict alignment requirements, the
  NEON SIMD instructions require 16-byte alignment. Buffer was 8-byte
  aligned → undefined behavior → incorrect hash output.
resolution: >
  Changed stack buffer declaration to `#[repr(align(16))]`.
  Added compile-time assertion: assert_eq!(align_of::<Sha3State>(), 16).
  All ARM64 CI jobs now include large-package signature tests.
pr: 299
status: RESOLVED
```

---

### ANOMALY-0007

```yaml
id: ANOMALY-0007
date: 2025-07-01
severity: MEDIUM
component: wiki/AUTO_SOVEREIGN_WIKI.md
symptom: >
  Wiki automation generating duplicate entries for absorption roadmap items,
  creating 200+ near-duplicate pages with names like "Absorption-Roadmap-v2"
  through "Absorption-Roadmap-v47".
root_cause: >
  sigma_automation.sh wiki-sync was not deduplicating existing page titles
  before creating new pages. Each run of the absorption pipeline appended
  a versioned copy instead of updating in-place.
resolution: >
  Added --dedup flag to wiki-sync: checks existing titles before creating.
  Added cleanup script: scripts/wiki_dedup.sh
  Ran deduplication pass: removed 187 duplicate pages.
pr: 312
status: RESOLVED
```

---

### ANOMALY-0008

```yaml
id: ANOMALY-0008
date: 2025-07-10
severity: LOW
component: kernel/sched/eevdf
symptom: >
  EEVDF scheduler producing lower throughput than CFS for CPU-bound
  workloads with many short-lived tasks (compile jobs, shell scripts).
root_cause: >
  EEVDF's virtual deadline calculation was penalizing tasks with high
  context-switch frequency even when they were I/O-bound in nature.
  CPU-bound compile tasks were being deprioritized due to incorrect
  eligibility classification.
resolution: >
  Added I/O-wait fraction tracking per task to classify CPU vs I/O bound.
  Adjusted eligibility function: CPU-bound tasks get shorter virtual
  deadline to maintain throughput.
  Benchmark: compile throughput improved 18% vs previous EEVDF impl.
pr: 331
status: RESOLVED
```

---

## Self-Healing Automation

The `kernel/self_healing/` subsystem automatically detects and resolves a subset of anomalies:

```rust
// Anomaly categories handled automatically:
pub enum AutoHealAction {
    RestartShard(ShardId),       // shard crash/panic → restart
    ExpandCgroup(CgroupId, u64), // OOM → expand memory limit
    ThrottleNetwork(ShardId),    // DDoS pattern → throttle
    ReloadConfig(ShardId),       // Config corruption → reload last good
    QuarantineShard(ShardId),    // Persistent failure → quarantine + alert
}
```

**Automated detection rate**: ~73% of LOW/MEDIUM anomalies resolved without human intervention.

---

## Reporting New Anomalies

To report a new anomaly:

1. Open a GitHub Issue with label `anomaly` and `severity:X`
2. Fill in the anomaly template (see `.github/ISSUE_TEMPLATE/`)
3. Attach kernel panic logs from `/var/log/sigma/kernel.log`
4. Self-healing system will attempt auto-diagnosis and comment on the issue

---

*Log maintained by the SigmaOS Self-Healing subsystem and human maintainers.*
*Total anomalies logged: 8 | Resolved: 8 | Open: 0*
