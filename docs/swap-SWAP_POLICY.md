# Swap Subsystem Policy (SWAP_POLICY.md)

## Requirements & Standards

### 1. Race-Free Swap Operations
- Swap-out and swap-in operations operate under fine-grained swap slot locks or atomic slot state bits (`FREE`, `ALLOCATED`, `IO_PENDING`).
- Concurrent faulting on a swapping page is synchronized via page locks to prevent duplicate swap reads or race conditions.

### 2. Bounded Metadata & Slot Allocator
- Total swap slots are bounded by the backing block device capacity or zswap compressed pool limits.
- Swap map metadata uses a compact byte/bit array with O(1) allocation queues.

### 3. Lost-Swap & Double-Swap Prevention
- Freeing a swapped page releases its backing swap slot exactly once.
- Unmapping or exit cleanup verifies that associated swap slots are reclaimed without orphan slots.

### 4. Observable OOM & Pressure Metrics
- Swap usage metrics (`swap_total`, `swap_used`, `swap_free`, `swap_in_ops`, `swap_out_ops`) are exposed via `/proc/meminfo` and kernel telemetry.
- When swap utilization exceeds 95% and reclaim fails, OOM enforcement is triggered and logged with full process diagnostics.
