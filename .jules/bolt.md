## 2026-03-31 - Bitmask Indexing Optimization in Hashmaps
**Learning:** `BTreeMap` capacity in `src/klib/hashmap.rs` is strictly constrained to powers of two (initialized via `next_power_of_two()` and doubled on rehash). Replacing integer modulo division (`% capacity`) with bitwise AND (`& (capacity - 1)`) eliminates ~10-15 cycles per lookup on x86_64/AArch64.
**Action:** Always verify power-of-two invariants before replacing modulo indexing with bitwise masking.
