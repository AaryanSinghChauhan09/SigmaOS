## 2025-10-24 - [NUMA-Aware scheduling structures]
**Learning:** Cache locality on multi-socket architectures can be significantly optimized by partitioning the CFS queues into local NUMA cores, completely avoiding cross-node memory bus transactions.
**Action:** Use custom thread-to-node mapping algorithms within the scheduler instead of relying on standard process grouping.
