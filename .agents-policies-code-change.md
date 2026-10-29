# Policy: Code Change & PR Scope Limits (.agents/policies/code-change.md)

1. **One PR, One Concern**:
   - Each Pull Request must focus on a single, narrowly defined task (one bug fix, one component test, one documentation update, or one subsystem integration).
   - Large horizontal refactors are prohibited unless split into atomic sequential PRs.

2. **Boundary Protection**:
   - Subsystem agents are prohibited from modifying another subsystem's public interface without cross-agent review and updated conformance tests.
