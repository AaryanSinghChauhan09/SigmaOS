# 🛡️ SIGMAOS AI-GENERATED CODE GOVERNANCE & TRANSPARENCY GUIDELINES

> **Adapted from the Linux Kernel Tool-Generated Content Guidance for Sovereign Bare-Metal Engineering**

As AI-assisted "vibe coding" and autonomous code generation become prevalent, maintaining kernel correctness, memory safety (`#![no_std]`), and zero-trust security boundaries in **SigmaOS** requires rigorous transparency. Polished, compiler-clean AI code can confidently introduce silent failures, subtle timing bugs, or unexpected capability leaks.

SigmaOS establishes five mandatory rules for AI-generated and AI-assisted contributions, inspired by Linux kernel governance.

---

## 📜 THE 5 SIGMAOS TRANSPARENCY RULES

### 1. Name the AI Tool
* **Rule:** Clearly identify the primary AI model, assistant, or toolchain used to generate significant portions of code, specifications, or patches.
* **Scope:** Applies when AI generates functions, driver modules, system tests, documentation, or changelogs. (Does not apply to standard IDE syntax formatting or basic spellchecking).
* **Git Attribution:** Include the `Assisted-by: <Tool Name>` trailer in git commit messages and Pull Request descriptions.
  ```text
  feat(kernel): implement O(1) lock-free ring buffer queue

  Assisted-by: OpenAI Codex / Claude 3.5 Sonnet
  ```

---

### 2. Preserve the Inputs Behind the Result
* **Rule:** Document and preserve the original specifications, requirements, and technical constraints (`PROJECT_SPEC.md` or PR description) provided to the AI.
* **Rationale:** AI-generated code only makes sense within the context and bounds provided. Without recorded limits, reproducing success or diagnosing edge-case failures turns into guessing.
* **Requirement:** Maintain independent acceptance test fixtures (e.g. sample WAV files, binary packets, register traces) that the AI tool did not directly optimize against.

---

### 3. Keep the Prompt Trail
* **Rule:** Preserve a log or summary of the prompt sequence (`AI_ASSISTANCE.md` or PR notes) that guided the implementation.
* **Scope:** For short sessions, include the exact prompts. For longer autonomous sessions, summarize the core constraints, security boundaries, and architectural guidelines given to the agent.
* **Key Focus:** Ensure safety boundaries are explicit (e.g. "Strict `#![no_std]`, no dynamic allocation in fast paths, zero network calls without capability tokens").

---

### 4. Record Exactly What the AI Changed
* **Rule:** Provide a granular breakdown of which files, modules, and tests were AI-generated versus human-authored.
* **Audit Trail:**
  * **Human Input:** Architectural specification, threat model, acceptance tests, hardware fixtures.
  * **AI Output:** Initial implementation, boilerplate scaffolding, unit tests, refactoring.
  * **Human/Agent Verification:** Code review findings, memory bounds checks, performance profiling results.

---

### 5. Make the Generated Code Prove Itself
* **Rule:** AI-generated code must prove its correctness through rigorous automated testing, boundary checks, and edge-case validation.
* **The Danger:** A confident, error-free execution that silently produces incorrect results (e.g., miscalculating timing thresholds, signal decoding errors, or silent memory corruption).
* **Mandatory Process:**
  1. Every AI contribution must pass 100% of unit and integration tests via `./run_sigma_tests.sh`.
  2. Edge cases (dash-heavy signals, high white noise, zero-length buffers, out-of-bounds register reads) must be turned into repeatable `pytest` or `cargo test` cases.
  3. No PR or commit shall be accepted based solely on a single successful happy-path run.

---

## 🛠️ INTEGRATION WITH SIGMAOS WORKFLOWS

When committing changes or opening Pull Requests for `github.com/AaryanSinghChauhan09/SigmaOS`:
- **Changelogs:** Log AI assistance notes in `CHANGELOG.md` when adding major subsystems.
- **Wiki Synchronization:** Ensure all architectural specs and guidelines are synchronized across `WIKI/`, `wiki/`, and `wiki_repo/` via `./scripts/sync_wiki.sh`.
