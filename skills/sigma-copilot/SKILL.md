# SKILL

---
name: sigma-copilot
description: Sovereign Copilot agent — Claude Code / openclaw / hermes-ide inspired OS automation.
---

# sigma-copilot

The primary SigmaOS agent persona. Runs tool-use loops to accomplish OS tasks via CLI instead of GUI clicks.

## Architecture (inspired by Claude Code)

1. **Read persona** — `profiles/ai_agent/system_prompt.md`

2. **Plan** — decompose user intent into tool calls

3. **Execute** — dispatch tools (Read, Bash, ZenithGUI, Skill, Memory, …)

4. **Observe** — collect tool results

5. **Respond** — summarize outcome for the user

6. **Persist** — JSONL session log under `.sigma/logs/agent/`

## Entry points

```bash
sigma-agent chat "install kyber package and switch theme to cyber"
sigma-agent repl
sigma-agent tool ZenithGUI '{"command":"launch browser"}'
sigma-agent skill run zenith-gui '{"command":"layout tile"}'
```

## Tool catalog

| Tool | Purpose |
|------|---------|
| Read / Write / Edit | SemanticFS file operations |
| Bash | sigma-sh commands |
| Grep / Glob | Codebase search |
| ZenithGUI | All GUI tasks via CLI bridge |
| Skill | Dispatch skills/ catalog |
| Memory | L0-L3 layered recall |
| ComputerUse | a11y snapshot/click/type fallback |
| Pkg / Net / Sec | sigma-pkg, sigma-net, sigma-sec |
| LLM | xLLM direct inference |
| FastContext | Repo exploration with citations |

## Kernel API

```c
sigma_copilot_init();
sigma_copilot_session_create("sovereign-copilot", session, sizeof(session));
sigma_copilot_run("open settings and set dark theme", response, sizeof(response));
sigma_copilot_tool_dispatch(SIGMA_TOOL_GUI, "theme obsidian", result, sizeof(result));
```

## Training config

- Persona: `profiles/ai_agent/system_prompt.md`

- Tools schema: `profiles/ai_agent/tools.json`

- Active skills: `profiles/ai_agent/config.json`

## Related skills

- `zenith-gui` — GUI-specific commands

- `computer-use` — accessibility automation fallback

- `claude-skills` / `copilot-patterns` — IDE workflow patterns

- `session-viewer` — review agent JSONL trajectories
