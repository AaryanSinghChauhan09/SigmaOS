# SigmaOS Sovereign Kernel Samepage Merging (KSM)

## Overview

SigmaOS implements a **pure-Rust sovereign KSM (Kernel Samepage Merging) memory deduplication engine** (`src/kernel/ksm_sovereign.rs`), absorbing the anonymous memory deduplication system merged in Linux 2.6.32.

KSM scans memory regions registered by processes via `madvise(addr, len, MADV_MERGEABLE)` (e.g. virtual machines, container runtimes, or language interpreters), identifies pages with identical contents, and merges them into a single read-only Copy-on-Write (CoW) page, recovering physical memory.

## Dual-Tree Architecture

1. **Unstable Tree**:
   - Contains candidate pages whose contents may still be changing.
   - When a page is scanned, KSM computes a 64-bit FNV-1a content hash and looks for a duplicate in the unstable tree.
2. **Stable Tree**:
   - When a match is found in the unstable tree, both pages are merged into a single read-only CoW page and moved to the stable tree.
   - Future identical pages increment the existing stable page's `shared_count`.
3. **Copy-on-Write Break**:
   - When any process attempts to write to a merged page, a CoW page fault breaks sharing, allocates a private physical copy, and updates memory savings counters.

## Test Verification

6 standalone unit tests verified in test runner suite `[22]`:
- `test_ksm_initial_state`: Initial zeroed metrics.
- `test_ksm_unstable_candidate_insertion`: New candidate pages held in unstable tree.
- `test_ksm_duplicate_promoted_to_stable`: Duplicate detected, merged into CoW page in stable tree.
- `test_ksm_multi_sharing_deduplication`: Multiple processes sharing same memory page.
- `test_ksm_break_cow_on_write`: Correct CoW break and memory accounting decrement.
- `test_ksm_break_cow_nonexistent`: Safe handling of non-existent page hashes.
