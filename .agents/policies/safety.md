# Agent Safety Policy

## Principle of Least Privilege

1. **Sandboxed Execution**: All agent operations must run within designated filesystem boundaries and containerized environments.
2. **No Blanket Credentials**: Agents must never receive force-push rights, blanket API tokens, or direct main-branch commit access.
3. **No Direct Execution of Untrusted Code**: Package scripts, downloaded binaries, and external network payloads must be executed inside isolated sandbox containers with explicit filesystem, network, and process restrictions.
4. **Network Access**: Agents operate with default-deny network rules unless explicitly allowed for metadata indexing.
5. **Human Gate**: Critical subsystem merges (kernel entry, security boundary, package installation, destructive storage, installer/update) require explicit human code review.
