# AI Agent GitHub Actions Workflows Management Guidelines

## Purpose
These guidelines define operational rules, YAML syntax patterns, and safety guardrails for AI coding agents managing or adding GitHub Actions workflows in SigmaOS.

---

## Directives for AI Agents

1. **YAML Toolchain Configuration**:
   - Always specify `dtolnay/rust-toolchain@v1` for Rust toolchain setup steps.
   - Specify `with: toolchain: stable` explicitly.

2. **Trigger Condition Conventions**:
   - Use `on: [push, pull_request]` for general CI workflows.
   - Use `workflow_dispatch:` for manual release and deployment triggers.

3. **Workflow Code Pattern**:
```yaml
name: SigmaOS Sample CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@v1
        with:
          toolchain: stable
      - name: Run SigmaOS Native Tests
        run: ./run_sigma_tests.sh
```

4. **Testing and Verification**:
   - Validate YAML syntax and run `./run_sigma_tests.sh` locally.

---

## Related Files
- `.github/workflows/`
- `docs/AI_AGENT_GITHUB_WORKFLOWS_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_GITHUB_WORKFLOWS_MANAGEMENT.md`
