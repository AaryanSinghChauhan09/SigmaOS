# Sigma Copilot CLI (sigma-agent)

Sovereign AI agent for SigmaOS — perform GUI tasks via CLI, inspired by Claude Code, openclaw, and hermes-ide.

## Quick start

```bash
# Natural language
sigma-agent chat "switch to tile layout and open settings"

# Direct GUI control (maps 1:1 to Zenith Desktop)
sigma-agent gui status
sigma-agent gui layout tile
sigma-agent gui theme cyber
sigma-agent gui launch browser
sigma-agent gui settings set ui.theme obsidian
sigma-agent gui files search "architecture docs"
sigma-agent gui dashboard query "why is CPU high"

# Agent tools
sigma-agent tools
sigma-agent tool ZenithGUI '{"command":"launch settings"}'

# Skills
sigma-agent skill list
sigma-agent skill run zenith-gui '{"command":"theme cyber"}'
```

## Architecture

| Layer | Component |
|-------|-----------|
| CLI | `userland/agent/sigma_agent_cli.cpp` |
| Agent runtime | `kernel/core/ai/SovereignCopilot.cpp` |
| GUI bridge | `kernel/core/ai/SovereignZenithCLI.cpp` |
| Skills | `skills/sigma-copilot/`, `skills/zenith-gui/` |
| Training | `profiles/ai_agent/system_prompt.md`, `tools.json` |

## GUI coverage

Every major Zenith GUI app has CLI equivalents:

- **Compositor** — start, stop, status, layout, theme, workspace
- **zenith-settings** — settings get/set/list
- **zenith-files** — search, tree, open
- **zenith-browser** — navigate URL
- **zenith-panel** — apps list, workspace switch
- **sigma-dashboard** — metrics, AI diagnosis
- **Window manager** — tiling, focus

Fallback: `computer-use` skill (accessibility tree) when CLI mapping is insufficient.

## Host development

```bash
./scripts/sigma-agent.sh validate
./scripts/sigma-agent.sh test
cmake -S tests/cpp_host -B build/cpp_host && ctest --test-dir build/cpp_host
```

## Related

- [Sigma Agent Platform](Sigma-Agent-Platform)
- [Shell Reference (sigma-sh)](Shell-Reference)
