## 📋 Pull Request Summary
<!-- What does this PR do? One paragraph max. -->

## 🏗️ Type of Change
- [ ] 🐛 Bug fix (correctness / safety / UB)
- [ ] ✨ New shard / feature
- [ ] ♻️ Refactor / modularization
- [ ] 📝 Documentation / Wiki
- [ ] ⚙️ CI / Build system
- [ ] 🛡️ Security hardening
- [ ] ⚡ Performance improvement

## 🧪 Testing
- [ ] New unit tests added
- [ ] Existing tests pass (`ctest`)
- [ ] QEMU smoke test passes
- [ ] Tested on: x86_64 / arm64 _(circle one)_

## 🔍 Code Quality Checklist
- [ ] `clang-format` passes with no diff
- [ ] `clang-tidy` reports no new warnings
- [ ] No raw machine opcodes or unsafe `void*` containers
- [ ] All new shards registered in `SHARDS.manifest`
- [ ] New public APIs documented with Doxygen `@brief`
- [ ] No blocking sleeps on the main thread

## 🛡️ Security Checklist
- [ ] No new unsafe casts or `reinterpret_cast` without justification
- [ ] AI-executing code runs within `SovereignSandbox` (level ≥ `STRICT`)
- [ ] New IPC uses `SovereignEventBus` (not direct shard calls)
- [ ] MAC policy updated if new capabilities are introduced

## 📎 Related Issues
Closes # <!-- issue number -->

## 📝 Wiki / Docs Updated?
- [ ] `SHARDS.manifest` updated
- [ ] `CHANGELOG.md` entry added
- [ ] Wiki page updated (if applicable)
