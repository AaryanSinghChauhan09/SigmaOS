# AI Agent Commands Management Guidelines

## Purpose
These guidelines define operational protocols, implementation rules, and safety constraints for AI coding agents invoking or adding command line utilities in SigmaOS.

---

## Directives for AI Agents

1. **Privilege Guardrails**:
   - Always validate user credentials or `doas` rules before elevating privileges to root.
   - Do NOT hardcode unrestricted root access for unknown users.

2. **Cross-Distro CLI Parity**:
   - When adding support for a new Linux or BSD CLI command (e.g. `sysctl`, `pacman`), place the function in `SovereignLinuxCommandSuite` or `SovereignBsdSysctl` in `src/tools/sovereign_commands.rs`.
   - Maintain consistent output formatting matching standard upstream CLI tool responses.

3. **Code Pattern: Privilege Elevation and Execution**:
```rust
let mut sudo = SovereignSudo::new();
let result = sudo.execute_as_root("sovereign", "sigma-pkg update", 1000)?;
assert!(result.contains("authenticated") || result.contains("cached auth"));
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm command suite unit test execution.

---

## Related Files
- `src/tools/sovereign_commands.rs`
- `docs/AI_AGENT_COMMANDS_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_COMMANDS_MANAGEMENT.md`
