# 📜 Principles of Sovereignty

These are the **five immutable laws** that govern every decision in SigmaOS's architecture, design, and evolution.

---

## Principle 1: Absolute Discretion

> *"No data leaves the machine without explicit sovereign consent."*

- Zero cloud telemetry in the kernel
- All AI inference runs locally (no API calls)
- No analytics, no usage reports, no beacon pings
- VFS data never touches an external server by default

**Implementation**: Every network call in `SovereignNetMesh.c` requires explicit user permission via capability flag. The kernel has no background call-home mechanism.

---

## Principle 2: Zero Abstraction Lies

> *"What you see is what executes. No hidden runtime, no invisible framework."*

- Pure C11 and Assembly — no managed runtimes
- Every function is user-defined and inspectable
- No black-box library functions in the kernel path
- The build system (`build.ps1`) refuses to link external objects

**Implementation**: `SovereignBuildMaster.c` enforces this at compile time. Any forbidden header inclusion fails the build immediately.

---

## Principle 3: User Autonomy

> *"Every tool is a choice. The user is the final kernel branch."*

- All shards are opt-in, never force-loaded
- The Shard-On-Demand (SOD) system gives users full control over what runs
- No mandatory background services (unlike systemd)
- Every default can be overridden via the Personalizer shard

**Implementation**: The `shard_core.c` registry only activates shards on explicit user command or persona-profile match.

---

## Principle 4: Persona-Awareness

> *"The OS should know who you are and adapt accordingly."*

- Developer, Student, Forensic Analyst, Researcher, Gamer personas
- Each persona loads a different shard profile at login
- AI models adapt their response style to the active persona
- UI themes, terminal shortcuts, and tool visibility all shift per persona

**Implementation**: `SovereignPersonalizerZenith.c` reads the persona config from VFS on boot and configures the `SigmaSystem` orchestrator accordingly.

---

## Principle 5: AI-Native by Design

> *"Intelligence is not a plugin — it is the substrate."*

- AI is embedded at the scheduler, VFS, shell, and shard layers
- The omni shell has built-in LLM command prediction
- The forensic shard has ML-based anomaly detection
- The education shards have an AI tutor at every step

**Implementation**: `scheduler_ai.c`, `sigma_ai_distribute.c`, and `SovereignAIKernelZenith.c` form the AI spine of the OS, activated from the earliest boot stages.

---

## The Sovereign Manifesto (Summary)

```
Σ SigmaOS Zenith Supreme
    ├─ No cloud. No compromise. No abstraction lies.
    ├─ Every byte is sovereign. Every function is owned.
    ├─ The user's will is the kernel's law.
    ├─ Intelligence woven into silicon, not installed from a repo.
    └─ Absolute autonomy. Absolute performance. Absolute SigmaOS.
```
