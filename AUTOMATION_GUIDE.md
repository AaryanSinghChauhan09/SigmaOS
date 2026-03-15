# 🤖 SigmaOS: Automation & Integration Guide (v4.9)

This guide explains how to interface with SigmaOS using automation tools, including **Google Stitch**, **Antigravity**, and custom scripts.

## 🔗 The Apex Shim Interface
Every component in SigmaOS is accessed through a **Lazy Shim**. This design is 100% automation-friendly as it allows for:
1. **Late-Binding**: You can replace a shard's implementation at runtime without restarting the OS.
2. **Interceptable Calls**: Every function call to a shim can be logged or redirected for debugging.

### Integration Hooks
- **Standard Library**: `sigma_core.interfaces.base_sovereign.SovereignModule` - Use this as the base for any new automation agents.
- **AI Sync**: `userland.system_api.ai_integration.omni_prompt_distributor` - Connect your LLM pipelines here.

## 🛠️ Customization & Personalization
SigmaOS is 100% data-driven.
- **Theme Shards**: Located in `userland/apps/theme_engine/themes_shards/`. You can automate UI changes by writing new JSON shards here.
- **Task Orchestration**: Use the `startup_orchestrator` shards to define personalized boot sequences for different user profiles (e.g., NCERT Teacher vs. AI Researcher).

## 🚀 Performance for Stitch
When using Google Stitch to generate UI:
1. **Reference DESIGN.md**: Ensure all generated CSS variables match the `Sovereign Cyberpunk` palette.
2. **Component Sharding**: Tell Stitch to generate components as standalone shards.
3. **Event Bus**: Use the `sigma_core.unified_api` to broadcast events across the system without tight coupling.

## 🛡️ Future-Proofing
The **Atomic Sharding Paradigm** ensures that as you add new features, you never break existing ones. Each shard is a sandbox of logic. 

---
*Automation Principle: "Shards are cheap. Logic is immutable. The Agent is Sovereign."*
