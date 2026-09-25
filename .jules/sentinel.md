# Sentinel 🛡️ - Security Journal

## Critical Learnings & Vulnerability Prevention

### 2026-03-30 - Task State Segment Hardware Boundary Protection
**Learning:** Unaligned struct field access in packed hardware structures (`TaskStateSegment64`) can cause CPU traps and undefined behavior across target architectures.
**Action:** Always copy packed hardware struct values to stack variables prior to validation assertions and maintain strict ring boundary isolation.
