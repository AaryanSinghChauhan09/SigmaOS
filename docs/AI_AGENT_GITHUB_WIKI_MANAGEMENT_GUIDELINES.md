# AI Agent GitHub Wiki Management Guidelines

## Purpose
These guidelines define operational rules, synchronization patterns, and verification procedures for AI coding agents creating, editing, or managing GitHub Wiki documentation in SigmaOS.

---

## Directives for AI Agents

1. **Dual Synchronization**:
   - Always update both `wiki/<filename>.md` and `wiki_repo/<filename>.md`.
   - Ensure content parity between both directories.

2. **Home.md Index Registration**:
   - Add every new wiki document to `wiki/Home.md` and `wiki_repo/Home.md` in the feature index section.

3. **Pattern: Copying and Syncing Wiki Files**:
```bash
cp wiki/NEW_GUIDE.md wiki_repo/NEW_GUIDE.md
```

4. **Testing and Verification**:
   - Verify file existence using `read_file` or `ls` on both `wiki/` and `wiki_repo/`.
   - Run `./run_sigma_tests.sh` to confirm overall repository integrity.

---

## Related Files
- `docs/AI_AGENT_GITHUB_WIKI_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_GITHUB_WIKI_MANAGEMENT.md`
- `wiki/Home.md`
