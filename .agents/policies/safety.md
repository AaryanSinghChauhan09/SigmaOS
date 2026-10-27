# Agent Safety Policy

## Operational Boundaries

1. **Least Privilege**: Agents must operate strictly within their declared owner paths and permission class.
2. **No Blanket Credentials**: Direct push or force push access to main/release branches is prohibited.
3. **Sandboxed Execution**: Untrusted package scripts and external binaries must run in isolated container/sandbox environments.
4. **Human Escalation**: Security risks, cryptographic implementation changes, and architectural redesigns require human review.
