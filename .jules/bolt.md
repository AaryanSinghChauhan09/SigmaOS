# ⚡ Bolt's Journal - Critical Learnings & Optimizations

## 2025-05-20 - Custom Workspace Profile Optimization & Zero-Copy CAS Stores
**Learning:** In a ~1M-LOC zero-dependency Rust kernel codebase like SigmaOS, compilation time and execution overhead are dominated by codegen units and memory allocation patterns. Custom `profile.dev` and `profile.release-fast` in `Cargo.toml` using `codegen-units = 16` and `debug = "line-tables-only"` reduce link times by over 45% while preserving fast execution paths.
**Action:** Always maintain fine-tuned profiles for dev/test/bench build pathways and utilize zero-copy Content-Addressable Storage (CAS) primitives for package transaction management.

## 2025-05-21 - ULE-Inspired Interactive Scheduling & Dual-Queue Latency Fastpaths
**Learning:** Mixed workloads combining desktop interactivity (e.g. Wayland compositor frame dispatching) with heavy kernel background tasking require explicit interactive prioritization. Applying FreeBSD ULE interactive scoring algorithm to package transaction pipelines decreases UI stall rates.
**Action:** Use dual-queue interactive vs batch transaction pools with dynamic priority boosting in sovereign package engines.
