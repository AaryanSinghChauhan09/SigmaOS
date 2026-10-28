# Agent Code Change Policy

## Guidelines for Changes

1. **Smallest Minimal Slice**: Implement only the minimal diff required to address the assigned task.
2. **Conventional Commits**: Every commit message must adhere to conventional commit format (`feat:`, `fix:`, `docs:`, `test:`).
3. **Rollback Instructions**: Every PR must document explicit rollback steps in case of regression.
4. **No Interface Breaks**: Subsystem agents must not break public module interfaces without integration review.
