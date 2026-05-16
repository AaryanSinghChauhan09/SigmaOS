# SigmaOS Contributor Guidelines

Welcome to SigmaOS! These rules act like a constitution for the repo: they keep development disciplined, documentation clear, and collaboration smooth.

## 🛠 Development Rules

### Code Style
- Use consistent formatting enforced by `clang-format` or equivalent.
- Follow naming conventions: `snake_case` for functions, `PascalCase` for structs/classes.
- Comment every function with purpose, parameters, and return values.

### Modularity
- Write kernel modules and drivers as independent units.
- Expose APIs for user-defined functions through a syscall table.
- No hard-coding; use configuration files or constants.

### Testing
- Every new function/module must include unit tests.
- Run automated QEMU boot tests before merging.
- Stress-test concurrency and shard isolation.

### Security
- Implement post-quantum cryptography where relevant.
- Ensure memory is wiped after use (zero-data remanence).
- No unchecked pointer arithmetic; always validate inputs.

## 📚 Documentation Rules

### README
- Update whenever new features or modules are added.
- Include build instructions, dependencies, and usage examples.

### Wiki
- Each subsystem (kernel, memory, scheduler, drivers, security) gets its own page.
- Add diagrams for architecture and workflows.
- Maintain a glossary for technical terms (e.g., “shards,” “sovereign lattice”).

### Change Logs
- Document every release with a “What’s New” section.
- Track breaking changes separately.

## 🤝 Collaboration Rules

### Issues
- Use templates for bug reports and feature requests.
- Label issues clearly (bug, enhancement, documentation, security).

### Pull Requests
- Must include description, tests, and documentation updates.
- Require at least one reviewer approval before merging.

### Branching
- Use `main` for stable releases.
- Develop features in separate branches (`feature/memory-manager`, `fix/irq-handler`).

## ⚙️ Automation Rules

### CI/CD
- Every commit triggers automated builds and tests.
- Add static analysis (`clang-tidy`) and security scans.

### Documentation Automation
- Auto-generate API docs from code comments (`Doxygen`/`Sphinx`).
- Link CI status badges in the Wiki.

### Release Automation
- Use GitHub Actions to package ISO images and binaries.
- Publish tagged releases with changelogs.

## ✅ Contribution Rules
- **Coding Standards:** Follow kernel coding guidelines (C, C++, Rust).
- **Commit Messages:** Use imperative style (“Add memory allocator,” “Fix IRQ handler”).
- **Review Process:** No direct commits to `main`; all changes go through PR review.
- **Community Engagement:** Encourage contributors to add examples and tutorials in the Wiki.
