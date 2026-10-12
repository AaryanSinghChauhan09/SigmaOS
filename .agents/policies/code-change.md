# Code Change Policy for Agents

## Rules for Code Modifications

1. **Pull Request Isolation**: Every change must arrive via a dedicated feature branch (`feat/`, `fix/`, `test/`, `docs/`, `chore/`) and Pull Request.
2. **Conventional Commits**: Commit messages must follow the standard format: `<type>(<scope>): <short description>`.
3. **Smallest Safe Increment**: Agents must modify the minimum number of lines required to complete a task. Avoid reformatting unrelated files.
4. **No Interface Breaks**: Subsystem agents must not alter public module traits or cross-subsystem ABI contracts without an explicit integration review.
5. **Mandatory Tests**: Every new feature or bug fix must include unit tests and regression tests proving correctness.
