# ARCHITECTURE DECISIONS (ADR)

## ADR-0001: Zero-External-Download Safe-Rust Self-Sufficiency Architecture
- **Status**: Accepted
- **Context**: Operating systems traditionally require users to download third-party software packages (e.g. VLC, LibreOffice, GIMP, PyTorch, Ollama, MySQL).
- **Decision**: Embed zero-dependency Safe-Rust (`klib`) native engines directly into the 12 Core System Shards of SigmaOS to satisfy all media, office, AI/LLM, ML, DB, security, scientific, and robotics requirements without external application downloads.
- **Consequences**: Memory safety, reduced supply-chain attack surface, instant startup performance, and complete OS sovereignty.
