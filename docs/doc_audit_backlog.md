# SigmaOS Documentation Audit and Backlog (Full Audit - Session 2)

> Last Updated: 2026-07-13

## 1. Repository Root Markdown Files

| File Path | Status | Priority | Language | Est. Effort | First Step |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `README.md` | Implemented (Partial) | High | — | 2h | Expand quickstart, architecture diagram, and CI badges |
| `CODE_OF_CONDUCT.md` | Implemented | Low | — | — | Keep as-is; sync to Wiki |
| `COMMUNITY.md` | Implemented | Medium | — | 1h | Consolidate to Wiki; keep repo pointer |
| `SUPPORT.md` | Implemented | Medium | — | 1h | Consolidate to Wiki |
| `LICENSE.md` / `LICENSES.md` | Implemented | Low | — | — | Keep as legal canonical |
| `THIRD-PARTY-NOTICES.md` | Implemented | Low | — | — | Keep as legal canonical |
| `sigma-build/Readme.md` | Partial | Medium | — | 2h | Add full build chain docs and troubleshooting |

---

## 2. Docs / Design Markdown Files

| File Path | Status | Priority | Language | Est. Effort | First Step |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `docs/design/sigmapkg.md` | Implemented | High | Rust | — | Design spec complete; prototype exists |
| `docs/doc_audit_backlog.md` | Partial | High | — | 1h | Expand this file (in progress) |

---

## 3. Kiro Specs

| File Path | Status | Priority | Language | Est. Effort | First Step |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `.kiro/specs/sigmaos-roadmap/design.md` | Partial | High | — | 3h | Migrate conceptual content to Wiki; link under Master Roadmap |
| `.kiro/specs/sigmaos-roadmap/requirements.md` | Partial | High | — | 2h | Migrate to Wiki requirements page |
| `.kiro/specs/sigmaos-roadmap/tasks.md` | Partial (Empty) | High | — | 1h | Populate from TODO.md then migrate |

---

## 4. Wiki Pages (key unimplemented / placeholder entries)

| Wiki Page | Status | Priority | Language | Est. Effort | First Step |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `AI_SovereignGGML_Inference.md` | Implemented (spec) | High | Rust | 4h | Prototype `kernel/ai/inference/` module |
| `AI_SchedulerNet_Autotune.md` | Implemented | High | Rust | — | `kernel/src/ai/scheduler/autotune.rs` done ✅ |
| `OSS_Absorption_PipeWire.md` | Implemented | High | Nim | 3h | `userland/audio/sigma_audiod.rs` exists |
| `Wine-Windows-Compat.md` | Implemented | High | Rust | — | `compat/wine/vsock.rs` done ✅ |
| `gVisor-compat.md` | Implemented | High | Rust | — | `linux_compat/` done ✅ |
| `smoltcp.md` | Implemented | High | Rust | — | `network/smoltcp_adapter.rs` done ✅ |
| `CLOUD_NATIVE.md` | Empty | High | Nim | 3h | Implement `sigma-nebula` cloud orchestration spec |
| `Desktop-UX.md` | Empty | High | Nim | 3h | Implement Zenith Desktop control center spec |
| `sigma-agent.md` | Placeholder | Medium | Rust | 2h | Implement `sigma-agent` IPC spec |
| `RFC-0002-kernel-stable-abi.md` | Implemented | Medium | Rust | — | kABI stability spec complete |
| `Kernel_Boot_Sequence.md` | Implemented | High | Rust/Zig | — | Boot sequence documented |
| `Driver-Development-Guide.md` | Implemented | High | Zig | — | Full driver framework documented |
| `Building-from-Source.md` | Partial | High | — | 2h | Complete with Rust/Zig/Nim toolchain steps |
| `Contributor-Guidelines.md` | Partial | High | — | 1h | Expand OOP/Trait pattern requirements |
| `Coding-Standards.md` | Partial | High | — | 1h | Expand no-std / no-extern guidance |

---

## 5. Prioritized Implementation Backlog (New)

### 🔴 High Priority — Unimplemented
1. **`CLOUD_NATIVE.md`** → Nim prototype: `userland/nebula/sigma_nebula.nim`
2. **`Desktop-UX.md`** → Nim prototype: `userland/gui/sigma_control_center.nim`
3. **`sigma-agent.md`** → Rust prototype: `userland/agent/src/ipc_agent.rs`
4. **`Building-from-Source.md`** → Full Rust/Zig/Nim build instructions with `USAGE.md`

### 🟡 Medium Priority — Partial
5. **`Coding-Standards.md`** → Add no-std / no-predefined-libs guidance sections
6. **`Contributor-Guidelines.md`** → Add OOP/Trait design pattern requirements

### 🟢 Low Priority — Cleanup
7. **Merge duplicate wiki roadmap pages** (`Roadmap.md` vs `Master_Strategic_Roadmap.md`)
8. **Deduplicate CLOUD_NATIVE variants**: `CLOUD_NATIVE.md` vs `CLOUD_ABSORPTION_ROADMAP.md`

---

## 6. Migration Log

| Date | File | Action | Wiki Target | PR |
| :--- | :--- | :--- | :--- | :--- |
| 2026-07-03 | `Driver-Framework.md` | Migrated | `Driver-Development-Guide` | b8746d7c34 |
| 2026-07-13 | `Wine-Windows-Compat.md` | Implemented | `Wine-Windows-Compat` | 03288e3 |
| 2026-07-13 | `gVisor-compat.md` | Implemented | `gVisor-compat` | 9897dcb799 |
| 2026-07-13 | `AI_SchedulerNet_Autotune.md` | Implemented | `AI_SchedulerNet_Autotune` | 7dd70c2059 |
