# AI Agent Context Data Operation Management Guide

## Overview
This wiki guide details Context Data Operation Management protocols for AI coding agents operating on SigmaOS. It covers Context Virtual MMU page allocation (`allocate_context_page`), `PawThreeLayerMemory` live context pruning (`live_context`, `max_live_turns`), short-term conversation context buffers (`short_term_context`), context key-value maps (`get_context`/`set_context`), and zero-allocation context token budgeting.

## Key Principles
1. **Virtual Context Allocation**: Context tokens and vector embeddings are managed as virtual memory pages in a 128k context window pool.
2. **Three-Layer Memory**: Active turns in `live_context` are capped at `max_live_turns`; older turns are evicted to verbatim/distilled storage.
3. **Short-Term Context Pruning**: Conversation context buffers maintain a strict 10-turn FIFO limit.

## Context Allocation (`src/ai/agentic_os_runtime.rs`)
```rust
let mut mmu = ContextVirtualMMU::default();
mmu.allocate_context_page("agent_session", 16_000, false, false);
```

## Related Documents
- `docs/AI_AGENT_CONTEXT_DATA_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_CONTEXT_DATA_OPERATION_MANAGEMENT_GUIDELINES.md`
- `docs/AI_AGENT_TOKEN_MANAGEMENT_ARCHITECTURE.md`
