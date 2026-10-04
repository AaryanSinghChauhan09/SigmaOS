# Grok AI Panel Agent

SigmaOS's **Omarchy Grok Panel Agent** is a multi-provider AI assistant embedded directly in the desktop panel — inspired by the Omarchy `agent-panel-grok` branch and extended with 6 AI providers, tool-calling, and live provider switching.

---

## Architecture Comparison

| Metric | Omarchy agent-panel-grok | Linux Mint (n/a) | SigmaOS Grok Panel Agent |
|---|---|---|---|
| Core language | Shell + Python | — | Safe Rust (`#![no_std]`) |
| AI providers | Grok (xAI) only | — | Grok, GPT-4o, Claude, Gemini, Ollama, SigmaLocal |
| Tool calling | No | — | Yes (structured tool dispatch) |
| History management | Unbounded | — | Rolling 50-message window |
| Live provider switch | No | — | Yes — hot-swap without restart |
| Local LLM | No | — | Ollama + SigmaOS Local (port 52000) |
| External deps | Python, API libs | — | **Zero** |
| JSON request builder | No | — | Auto-built from conversation history |

---

## Architectural Highlights

- **6 AI providers** — `Grok`, `Gpt4o`, `Claude`, `Gemini`, `Ollama`, `SigmaLocal`; each with correct API base URL and default model
- **SigmaLocal** — first-class provider pointing to SigmaOS's own local inference server (`sigma-7b` model, port 52000)
- **Rolling history** — keeps the last 50 messages; oldest pruned automatically to control token costs
- **System prompt** — configurable per-session; defaults to SigmaOS-aware system assistant
- **JSON request builder** — `build_api_request_json()` serializes the full conversation + model into a standards-compliant request body
- **Hot-swap providers** — `switch_provider()` changes provider + model atomically; history is preserved
- **Tool-calling foundation** — `tool_calls_enabled` flag gates structured function dispatch for agentic workflows
- **Panel state machine** — `Idle` → `Thinking` → `Responding` → `Idle` with `Error` terminal state

---

## API & Usage

```rust
use sigmaos::ai::omarchy_grok_panel_agent::{
    OmarchyGrokPanelAgent, PanelAiProvider
};

// Initialize with Grok
let mut agent = OmarchyGrokPanelAgent::new(PanelAiProvider::Grok);

// Custom system prompt
agent.set_system_prompt(
    "You are SigmaOS, an AI-native OS assistant. Help with code, terminal, and system tasks."
);

// Chat
let response = agent.send_user_message("What's my CPU temperature?");

// Switch to local Ollama for privacy
agent.switch_provider(PanelAiProvider::Ollama);
let resp2 = agent.send_user_message("Summarize my clipboard");

// Export request for debugging
let json = agent.build_api_request_json();
// {"model":"llama3.2","messages":[{"role":"system",...},...],"stream":false}

// Switch to SigmaOS Local LLM
agent.switch_provider(PanelAiProvider::SigmaLocal);
// → model = "sigma-7b", api_base = "http://localhost:52000/v1"

// Clear history
agent.clear_history();
```

---

## Supported Providers

| Provider | Model | API Base |
|---|---|---|
| `Grok` | `grok-3` | `https://api.x.ai/v1` |
| `Gpt4o` | `gpt-4o` | `https://api.openai.com/v1` |
| `Claude` | `claude-3-5-sonnet-20241022` | `https://api.anthropic.com/v1` |
| `Gemini` | `gemini-2.0-flash` | `https://generativelanguage.googleapis.com/v1` |
| `Ollama` | `llama3.2` | `http://localhost:11434/v1` |
| `SigmaLocal` | `sigma-7b` | `http://localhost:52000/v1` |

---

## Testing

```bash
rustc --test src/ai/omarchy_grok_panel_agent.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_grok && ./build/test_grok
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [Multi-Agent Provider Orchestration](Multi-Agent-Provider-Orchestration.md) — agent dispatch
- [Cloud Agent Environment](Cloud-Agent-Environment.md) — cloud deployment
- [AI and ML Engine](AI-and-ML-Engine.md) — local inference
