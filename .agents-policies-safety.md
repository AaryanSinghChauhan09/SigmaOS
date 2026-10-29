# Policy: Agent Safety & Human Gate Guardrails (.agents/policies/safety.md)

1. **Human Approval Mandate**:
   Autonomous agents must obtain explicit human review and approval prior to merging changes that:
   - Modify kernel security boundaries or capability checkers.
   - Edit bootloaders, installers, or system update/recovery routines.
   - Execute package maintainer scripts or external post-install hooks.
   - Download or execute binary blobs from external network locations.
   - Alter release channel definitions or target distribution outputs.
   - Force-push to protected repository branches or delete system modules.

2. **Least Privilege & Isolation**:
   - Subsystem agents operate strictly within declared ownership paths.
   - External package parsing and script evaluation must occur inside isolated sandbox containers without host privilege escalation.
