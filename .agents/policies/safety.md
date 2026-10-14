# Agent Safety Policy

## Principle of Least Privilege

1. **Sandboxed Execution**: All agent operations must run within designated filesystem boundaries and containerized environments.
2. **No Blanket Credentials**: Agents must never receive force-push rights, blanket API tokens, or direct main-branch commit access.
3. **No Direct Execution of Untrusted Code**: Package scripts, downloaded binaries, and external network payloads must be executed inside isolated sandbox containers with explicit filesystem, network, and process restrictions.
4. **Network Access**: Agents operate with default-deny network rules unless explicitly allowed for metadata indexing.
5. **Human Gate**: Critical subsystem merges (kernel entry, security boundary, package installation, destructive storage, installer/update) require explicit human code review.
## 1. Grounding Principles
- **Safety First**: External repositories, downloaded binaries, third-party packages, and agent scripts are untrusted by default.
- **Least Privilege**: Agents and CI processes execute with only the explicitly granted filesystem, network, and execution permissions declared in `.agents/registry.toml`.
- **No Broad Host Execution**: Package scripts, downloaded binaries, and external code must run inside isolated sandboxes (containers, chroots, microVMs, or pledge/unveil restricted processes).
## 2. Mandatory Human Approval Triggers
Agents may work autonomously to perform builds, run unit/integration tests, generate reports, open documentation PRs, and triage bugs. Human approval is strictly required before:
1. Modifying kernel security boundaries, MAC policies, pledge promises, or capability checks.
2. Modifying bootloaders, kernel entry logic, installers, or system update/rollback mechanisms.
3. Executing third-party package scripts with host privileges.
4. Downloading or executing unverified binary artifacts.
5. Promoting a component status to `Supported` or `Production-Ready`.
6. Force-pushing or deleting protected git branches.
7. Modifying release channels or publishing stable channel release artifacts.
## 3. Sandboxing Controls
- All package execution tests must use `dry-run` or isolated overlay filesystems.
- Network access during automated agent runs is denied by default unless explicitly granted for specific metadata fetches.
- Processes running in test harnesses must enforce process resource limits (CPU time, memory footprint, stack size).
