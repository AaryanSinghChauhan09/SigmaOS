# AI Agent Context Data Operation Management Architecture

## Executive Overview

Context Data Operation Management in SigmaOS governs the allocation, retention, pruning, and indexing of LLM context windows, conversation turns, vector embeddings, and agent intent metadata. Implemented across `src/ai/agentic_os_runtime.rs`, `src/ai/qwenpaw.rs`, `src/ai/open_computer.rs`, `src/ai/voice.rs`, and `src/ai/autonomous_agents.rs`, SigmaOS uses a Context Virtual MMU (`allocate_context_page`), a Three-Layer Memory system (`PawThreeLayerMemory`), short-term conversation context buffers, and zero-allocation token budgeting to optimize AI agent context windows.

This document serves as the architectural reference for AI coding agents inspecting, storing, or pruning context data in SigmaOS.

---

## Subsystem Integration & Context Memory Pipeline

```
                                +-----------------------------------+
                                |    AI Agent / Pipeline Input      |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |     Context Virtual MMU           |
                                |  allocate_context_page(128k)      |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | PawThreeLayerMemory   |   | Short-Term Context|   | Voice Assistant Map   |
            | live_context (turns)  |   | short_term_context|   | set_context/get_ctx   |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                | Pruning & Context Window Budgeting|
                                | max_live_turns / FIFO eviction    |
                                +-----------------------------------+
```

### Core Context Management Components

1. **Context Virtual MMU (`src/ai/agentic_os_runtime.rs`)**:
   - `allocate_context_page(owner, page_tokens, is_embedding, is_persistent)`: Virtual MMU allocator mapping LLM context tokens and vector embeddings into virtual pages (default 128k context window pool).

2. **Three-Layer Context Memory (`src/ai/qwenpaw.rs`)**:
   - `PawThreeLayerMemory`: Manages `live_context` (active turns), verbatim history, and distilled knowledge. When `live_context.len() > max_live_turns`, the oldest turn is evicted to preserve prompt budget.

3. **Short-Term Conversation Context (`src/ai/open_computer.rs`)**:
   - `short_term_context: Vec<String>`: Retains up to 10 recent interaction turns with automated FIFO pruning.

4. **Voice Assistant Context Map (`src/ai/voice.rs`)**:
   - `set_context(key, val)` / `get_context(key)`: Key-value context dictionary for real-time speech and multimodal state tracking.

---

## Zero-Allocation Guardrails

AI agents manipulating context data must observe these constraints:
- Context page token calculations operate on stack primitives (`usize`).
- Eviction of stale conversation turns uses slice removal without allocating new vector backing buffers.

---

## Related Architectural References
- `src/ai/agentic_os_runtime.rs` - Virtual Context MMU.
- `src/ai/qwenpaw.rs` - Three-layer context memory engine.
- `src/ai/open_computer.rs` - Computer control context buffer.
- `src/ai/voice.rs` - Voice assistant context map.
- `docs/AI_AGENT_TOKEN_MANAGEMENT_ARCHITECTURE.md` - Neural token management.
