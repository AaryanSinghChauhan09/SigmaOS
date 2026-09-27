# Sentinel's Journal

## 2026-07-20 - [Stack Variable Alignment Checks & Packed Struct Memory Safety]
**Learning:** Taking direct references (`&tss.rsp0`) to fields within `#[repr(packed)]` structs (such as `TaskStateSegment64`) triggers Rust compiler warning E0793 and can lead to unaligned memory accesses and kernel panic / undefined behavior on strict hardware architectures.
**Action:** Always copy fields of packed structs into local stack variables before passing them to assertion macros or function calls requiring aligned references.
