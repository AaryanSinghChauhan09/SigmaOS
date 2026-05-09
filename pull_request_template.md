# ðŸ“‹ Pull Request Summary

<!-- What does this PR do? One paragraph max. -->

## ðŸ—ï¸ Type of Change

- [ ] ðŸ› Bug fix (correctness / safety / UB)
- [ ] âœ¨ New shard / feature
- [ ] â™»ï¸ Refactor / modularization
- [ ] ðŸ“ Documentation / Wiki
- [ ] âš™ï¸ CI / Build system
- [ ] ðŸ›¡ï¸ Security hardening
- [ ] âš¡ Performance improvement

## ðŸ§ª Testing

- [ ] New unit tests added
- [ ] Existing tests pass (`ctest`)
- [ ] QEMU smoke test passes
- [ ] Tested on: x86_64 / arm64 _(circle one)_

## ðŸ” Code Quality Checklist

- [ ] `clang-format` passes with no diff
- [ ] `clang-tidy` reports no new warnings
- [ ] No raw machine opcodes or unsafe `void*` containers
- [ ] All new shards registered in `SHARDS.manifest`
- [ ] New public APIs documented with Doxygen `@brief`
- [ ] No blocking sleeps on the main thread

## ðŸ›¡ï¸ Security Checklist

- [ ] No new unsafe casts or `reinterpret_cast` without justification
- [ ] AI-executing code runs within `SovereignSandbox` (level â‰¥ `STRICT`)
- [ ] New IPC uses `SovereignEventBus` (not direct shard calls)
- [ ] MAC policy updated if new capabilities are introduced

## ðŸ“Ž Related Issues

Closes # <!-- issue number -->

## ðŸ“ Wiki / Docs Updated?

- [ ] `SHARDS.manifest` updated
- [ ] `CHANGELOG.md` entry added
- [ ] Wiki page updated (if applicable)
