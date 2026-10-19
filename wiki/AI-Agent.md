# SigmaOS AI Agent Integration Components

## Overview
SigmaOS AI agent integration providing configurable default coding agent with instant launch shortcuts and crash diagnosis, competitive with Omarchy's AI agent features.

## Features
- **Multiple AI providers**: Claude Code, OpenAI Codex, Fireworks AI, Local LLM, Custom
- **Instant launch shortcut** (configurable, default: SUPER+ALT+A)
- **Crash diagnosis** with suggested fixes
- **API key management**
- **Model configuration**
- **Temperature and max tokens**
- **Enable/disable toggle**

## Structures

| Structure | Description |
|-----------|-------------|
| **AIManager** | AI agent manager |
| **AIConfig** | AI agent configuration |
| **AIStatus** | AI status report |
| **AIProvider** | ClaudeCode, Codex, Fireworks, LocalLLM, Custom |
| **CrashReport** | Crash diagnosis report |

## Usage Examples

```rust
// Create AI manager
let mut mgr = AIManager::new();

// Configure AI agent
let config = AIConfig {
    provider: AIProvider::ClaudeCode,
    model: "claude-3-sonnet".to_string(),
    api_key: Some("sk-...".to_string()),
    instant_launch_shortcut: "SUPER+ALT+A".to_string(),
    crash_diagnosis: true,
    ..Default::default()
};
mgr.configure(config);

// Diagnose a crash
let report = mgr.diagnose_crash("panic: null pointer".to_string(), Some("at main.rs:42".to_string()));

// Get status
let status = mgr.status();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: AI & Productivity (P1)

## Related Files
- `src/ai/agent.rs` - AI agent implementation
- `wiki/AI-Agent.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, SEPARATION_OF_CONCERNS,
INVERSION_OF_CONTROL, INFORMATION_HIDING, INTERFACE_SEGREGATION, PLUG_IN_DRIVER_MODEL,
POLYMORPHISM, COMPOSITION_OVER_INHERITANCE, USER_FREEDOM_AND_CONTROL, CONFIGURABILITY,
COMMUNITY_ORIENTED_DEVELOPMENT