# AI Subsystem

SigmaOS integrates a local AI subsystem for OS-level intelligence — no cloud dependency.

## Architecture

```
User / Application
        ↓
  S-AI Orchestrator  (src/ai/orchestrator.rs)
        ↓
  ┌─────┴──────┬──────────────┐
  ▼            ▼              ▼
Agent Pool    LLM Engine    APM Monitor
(Bolt/Palette (src/ai/llm.rs) (src/ai/apm.rs)
 /Sentinel)
  ↓
Memory Search (LiftEngine)
  ↓
Task Queue → Execution → Result
```

## Agents

SigmaOS has three specialized AI agents:

### Bolt ⚡
- Performance optimizer
- Analyzes hotpaths and suggests micro-optimizations
- Manages circular IPC queue tuning
- File: `src/ai/agent.rs` + `.jules/bolt_journal.md`

### Palette 🎨
- UI/UX specialist
- Manages Zenith desktop theming and animations
- Spring physics engine for UI motion
- File: `src/ai/agent.rs` + `.jules/palette_journal.md`

### Sentinel 🛡️
- Security auditor
- Monitors capability token usage
- Detects privilege escalation attempts
- Fixed the bitmask overlap bug in `CapabilityToken`

## Local LLM Engine (`src/ai/llm.rs`)

- Transformer-based inference running entirely on-device
- No internet required
- Quantized model support (4-bit, 8-bit)
- Used by the AI scheduler for process behavior prediction

## Multi-Agent Orchestration (`src/ai/orchestrator.rs`)

```rust
// Route a task to the appropriate agent
let result = orchestrator.route_task(Task {
    kind: TaskKind::SecurityAudit,
    payload: "check capability tokens".into(),
})?;
```

Supports:
- Model routing (pick LLM based on task type)
- Task negotiation between agents
- Memory search (LiftEngine context retrieval)
- Parallel agent execution

## WANDR Research Integration (`src/ai/wandr.rs`, `src/compatibility/relay_nexus.rs`)

Inspired by Perplexity AI's WANDR benchmark:
- **WandrEvent**: tracks research trajectory
- **AtifTrajectoryMonitor**: monitors search path quality
- **VerifierConsensus**: multi-agent result verification
- **RelayNexus**: multi-hop research relay

## AutoGen (`src/ai/autogen.rs`)

Multi-agent task generation framework — agents can spawn sub-agents for complex tasks.

## APM — AI Performance Monitor (`src/ai/apm.rs`)

Tracks AI subsystem metrics:
- Inference latency per query
- Token throughput
- Memory usage per model
- Agent task success/failure rates

## Voice (`src/ai/voice.rs`)

Local speech synthesis and recognition:
- Text-to-speech for screen reader integration
- Speech-to-text for voice commands
- No cloud API dependency
