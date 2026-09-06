# AI Agent Circular SCAN (C-SCAN) Policy Management Guidelines

## Purpose
These guidelines define operational protocols, coding patterns, and verification constraints for AI agents modifying or configuring Circular SCAN (C-SCAN) elevator disk policies in SigmaOS.

---

## Directives for AI Agents

1. **Maintain Pure Ascending Sweep Semantics**:
   - Do NOT alter `DeadlineScheduler::dispatch_next` to sweep backwards. Bi-directional sweeps introduce non-uniform tail latency.
   - Always advance `head_pos` to `bio.end_sector()` after servicing a request.

2. **Order of Service**:
   - First check `dispatch` queue (flushes, syncs, high-priority overrides).
   - Second check `read_queue.range(head_pos..)`.
   - Third wrap around to `read_queue.iter()` if no sector $\ge$ `head_pos` is found.
   - Fourth check `write_queue.range(head_pos..)`.
   - Fifth wrap around to `write_queue.iter()`.

3. **Code Pattern: C-SCAN Sweep and Wrap-Around**:
```rust
// Sector range sweep >= head_pos
if let Some((&sector, _)) = self.read_queue.range(self.head_pos..).next() {
    let bio = self.read_queue.remove(&sector).unwrap();
    self.head_pos = bio.end_sector();
    return Some(bio);
}
// Sector wrap-around (C-SCAN return to sector 0)
if let Some((&sector, _)) = self.read_queue.iter().next() {
    let bio = self.read_queue.remove(&sector).unwrap();
    self.head_pos = bio.end_sector();
    return Some(bio);
}
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to ensure all block device unit tests pass.
   - Verify C-SCAN sector dispatch ordering using mock Bio requests submitted out of order.

---

## Related Files
- `src/kernel/block_dev.rs`
- `docs/AI_AGENT_CIRCULAR_SCAN_POLICY_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_CIRCULAR_SCAN_POLICY_MANAGEMENT.md`
