# AI Agent Directives & Memory Management Guide for SigmaOS

Welcome, AI Engineer / Agent! This document specifies core operational guidelines and memory management procedures for working with the SigmaOS codebase.

---

## 🧠 Memory Management Principles in SigmaOS

1. **Zero-Allocation Primitives (`klib`)**
   - Core kernel and low-level subsystem primitives (`src/klib/`) operate in `#![no_std]` environment.
   - Prefer stack-based formatting (`format_u64_stack` in `src/klib/conversion.rs`) and FNV-1a zero-allocation hashing (`fnv1a_hash_64`) over dynamic heap allocations.

2. **Single-Buffer In-Place Formatting**
   - When serializing structured data (e.g. JSON in `src/klib/json.rs`), use `append_json_string` or single-buffer mutators rather than returning newly allocated `String` objects per field/key.

3. **Safe Memory Allocation Layering**
   - Userland components (`src/desktop/`, `src/sigpkg/`) use standard `alloc::vec::Vec` and `alloc::boxed::Box`.
   - Kernel memory management (`src/memory/`) usesBuddy Allocator (`sigma_buddy.rs`), Slab Cache (`slab.rs`), and Physical Memory Manager (`pmm_vmm.rs`). Never create custom unsafe `Vec<T>` structs.

4. **Virtual Memory Paging Indexing**
   - In 4-level paging (`SimpleVMM` in `src/klib/paging.rs`), maintain unique table indexing:
     - `pd_table_idx = pml4_idx * 512 + pdpt_idx`
     - `pt_table_idx = (pml4_idx * 512 + pdpt_idx) * 512 + pd_idx`

---

## 🎨 UI Management & Accessibility Principles

1. **Native WASM / Rust UI Engine First**
   - Implement UI event handlers, keyboard focus, and ARIA attributes in native Rust/WASM (`NativeWasmDesktopEngine` in `src/desktop/web_wasm_bridge.rs` & `zenith_desktop/src/lib.rs`). Reduce or eliminate JavaScript runtime dependencies.

2. **Accessibility Standards (Palette Persona)**
   - Icon-only buttons **must** include an `aria-label`.
   - Support keyboard navigation (`Enter` and `Space` key activation) and focus states (`:focus-visible` / `.keyboard-focus`).
   - Use `set_secure_text_content` (`textContent`) to prevent innerHTML XSS vulnerabilities when rendering dynamic titles or strings.

3. **CSS Design Tokens**
   - Leverage theme custom properties defined in `zenith_desktop.css` (`--accent-gold`, `--accent-blue`, `--accent-cyan`).

---

## 🌐 Network Management & Security Validation Principles

1. **IPv4 & IPv6 Address Parsing Security (`src/security/input_validation.rs`)**
   - In `validate_ipv4`: Reject leading zeros in multi-digit octets (e.g. `010.0.0.1`) to prevent octal parser differential SSRF vulnerabilities.
   - In `validate_ipv6`: Track explicit block count alongside `double_colon` compressed blocks. Reject addresses with 8 or more explicit blocks (`blocks >= 8`) when compressed notation is used.

2. **Cross-Platform Firewall Translation (`src/network/`)**
   - Provide interoperable rule translation across OpenBSD `pf` (`pf_firewall.rs`), Linux `nftables` (`nftables.rs`), and NetBSD `npf` (`npf_firewall.rs`).
   - Enforce FreeBSD VNET per-jail network stack isolation (`distro_net.rs`).

---

## 🔄 System State Management Principles

1. **Declarative State Graph (`src/system/state.rs`)**
   - Use NixOS-inspired `DeclarativeStateGraph` to manage system configuration nodes (`StateNode`).
   - Call `validate()` before committing state transitions to verify dependency IDs exist.

2. **Atomic Generation Rollback**
   - Trigger `create_generation` before performing major configuration updates, enabling instant atomic rollback (`rollback()`) on failure.

3. **MVI Reactive Store (`src/klib/store.rs`)**
   - For UI and userland reactivity, dispatch immutable actions through `StateStore<S, A>` and pure `Reducer<S, A>` functions.

---

## 🔒 Spinlock System Synchronization Principles

1. **Ticket Spinlock Fairness (`TicketSpinlock` in `src/kernel/classic_os.rs`)**
   - Use atomic ticket/now_serving counters with exponential backoff (`core::hint::spin_loop()`) to ensure SMP lock fairness.

2. **Fine-Grained Contention Tracking (`FineGrainedSpinlock` in `src/kernel/core/sovereign_scheduler.rs`)**
   - Track `acquire_count` and `contention_count` for latency diagnostics (FreeBSD `mtx` & Linux `spinlock_t` parity).

3. **Deadlock & Interrupt Safety**
   - Never perform dynamic memory allocation or blocking operations while holding a spinlock.
   - Acquire multiple spinlocks in strict ascending hierarchical order.

---

## 💾 Block-Oriented Device Management Principles

1. **Unified Device Abstraction (`src/storage/block.rs`)**
   - Implement `BlockOrientedDevice` for block drivers (`SsdBlockDevice`, `NvmeBlockDevice`).
   - Check `dev.is_write_blocked()` before all destructive operations (`Write`, `DiscardTrim`, `SecureErase`).

2. **Buffer Alignment & Bound Checks**
   - Ensure read/write buffers match `dev.block_size()` and verify `block_num < dev.total_blocks()` to prevent out-of-bounds access (`BlockError::OutOfBounds`).

3. **Cache & Partition Synchronization**
   - Invalidate matching blocks in `SimpleBlockCache` when performing discard or secure erase ops.

---

## ⌨️ Character Device Driver Management Principles

1. **Stream-Oriented Line Disciplines (`src/kernel/tty.rs`)**
   - Support canonical mode editing (`ICANON`), signal interjection (`ISIG`), and software flow control (`IXON`/`IXOFF`).

2. **Termios Signal & Echo Flushing**
   - Flush input buffers (`flush_input()`) upon processing signal bytes (`VINTR`, `VQUIT`, `VSUSP`) when `ISIG` is active.

3. **Lock-Free Hardware FIFO Operations (`src/kernel/drivers/legacy/uart_8250.rs`)**
   - Maintain zero heap allocations during high-frequency character transfer interrupt routines.

---

## 🔌 Device Classes & Operation Management Principles

1. **Unified Device Traits (`src/driver/device.rs`)**
   - Implement `Device` and `UnifiedPeripheral` traits for driver classes (`Block`, `Character`, `Network`, `Graphics`, `Input`, `Audio`).

2. **MMIO Volatile Register Safety**
   - Access memory-mapped channel registers (`PortAddress::MemoryMapped`) exclusively via volatile primitives (`read_volatile`/`write_volatile`).

3. **Driver Object & Extension Lifecycles (`IoManager`)**
   - Track device reference counts (`increment_ref`/`decrement_ref`) and release all `DeviceExtension` context buffers upon driver unload (`io_unload_driver`).

---

## ⚙️ Testing & Verification Procedures

- **Kernel Primitives (`klib`):**
  `cargo test --lib -- klib::json` or `./run_sigma_tests.sh`
- **Linux/BSD System Gap Engines:**
  `rustc --test src/distro/linux_bsd_distro_gaps.rs --edition=2021 -o build/distro_gaps_test && ./build/distro_gaps_test`
- **Python Integration Suite:**
  `pytest tests/test_unit_core.py tests/test_integration_system.py tests/test_stress_fuzz_bench.py`
