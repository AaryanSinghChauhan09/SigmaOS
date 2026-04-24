
# Policy Module System


SigmaOS enables **behavioral sovereignty** through hot-swappable policy modules. Instead of baking scheduling or security rules into the kernel, they are registered as loadable modules that can be swapped at runtime.

Located in `modules/ext/plugins/policy_modules.c`.


## Built-in Policies



### Security Policies

| Policy | Behavior |
| :--- | :--- |
| `strict_security` | **Default.** Denies all access unless explicitly granted via capability. Terminates violating processes. |
| `permissive_security` | Allows everything. For developer/debug mode only. |


### Scheduling Policies

| Policy | Behavior |
| :--- | :--- |
| `round_robin_scheduler` | Each process gets an equal 10ms time slice in rotation. |
| `priority_scheduler` | Higher-priority processes get longer slices (`5 + priority * 2` ms). |


## Hot-Swapping a Policy


```c
// Activate the priority scheduler at runtime — no reboot required
policy_activate(policy_id_of("priority_scheduler"));
```


## Custom Policy Registration


Any module can define and register a new policy:
```c
policy_register("ai_scheduler_policy", POLICY_SCHEDULING,
    NULL, NULL, ai_pick_next, ai_timeslice_ms, NULL);
```

This means SigmaOS's behavior (security model, scheduler algorithm, memory quota rules) can be changed purely by loading a new policy capsule.

---


# Self-Optimising Filesystem


Located in `modules/core/fs/self_opt_fs.c`.

The Self-Optimising FS (SOFS) monitors file access patterns and reorganises storage automatically, without any administrator intervention.


## File Tier Classification


| Tier | Access Pattern | Action |
| :--- | :--- | :--- |
| **HOT** | Accessed ≥ 10 times recently | Defragged to **contiguous** blocks for fastest reads |
| **WARM** | Moderate access | No change |
| **COLD** | Rarely accessed | **Compressed** (LZ4/zstd) to reclaim physical storage |


## How It Works

- `sofs_record_access()` — Called on every file read/write; increments access counter.
- `sofs_optimize()` — Called periodically by the kernel timer tick; re-classifies all files and triggers compression or defragmentation as needed.
- All classification events are **audited** in the Tamper-Proof Audit Chain.
