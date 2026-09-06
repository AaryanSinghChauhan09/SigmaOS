# AI Agent Context Data Operation Management Guidelines

## Purpose
These guidelines define operational protocols, implementation patterns, and safety guardrails for AI coding agents allocating, storing, or pruning context data in SigmaOS.

---

## Directives for AI Agents

1. **Context Window Token Budgeting**:
   - Always allocate context memory using `allocate_context_page` prior to large LLM prompt construction.
   - Enforce `max_live_turns` bounds in `PawThreeLayerMemory` to prevent context window overflow.

2. **Context Key-Value Management**:
   - Clear ephemeral context data using `clear_context()` upon completing multi-turn conversational tasks.

3. **Code Pattern: Allocating and Managing Context**:
```rust
let mut mmu = ContextVirtualMMU::default();
mmu.allocate_context_page("agent_convo_01", 16_000, false, false);

let mut memory = PawThreeLayerMemory::new(5); // max 5 live turns
memory.add_turn("User: Run build".to_string());
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm AI runtime context unit tests pass.

---

## Related Files
- `src/ai/agentic_os_runtime.rs`
- `src/ai/qwenpaw.rs`
- `docs/AI_AGENT_CONTEXT_DATA_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_CONTEXT_DATA_OPERATION_MANAGEMENT.md`
