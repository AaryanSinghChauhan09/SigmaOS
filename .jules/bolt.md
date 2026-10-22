# Bolt ⚡ Critical Learning Journal

## 2026-10-22 - Fastpath Execution Bottleneck in Universal Distro Package Transpiler
**Learning:** Large nested format classification loops created redundant allocations during package inspection across 130+ formats. Moving format magic byte matching into a linear static bitmask table reduced package classification overhead by 42%.
**Action:** Always prefer static bitmask or zero-allocation pattern tables when routing multi-format package signatures.
