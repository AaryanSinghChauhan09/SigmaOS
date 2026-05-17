# SigmaOS Contributor Guidelines

Welcome to SigmaOS! Every contributor must follow these rules. They act like a constitution for the repo, ensuring disciplined development, clear documentation, and smooth collaboration.

## 🛠 General Task Rules

### Consistency First

* Every task must follow the same naming conventions, formatting, and documentation style.

* **No shortcuts**: even small fixes require tests and documentation updates.

### Traceability & Commit Messages

* Each task must be linked to an issue or roadmap item.

*Commit messages must use the**imperative style** (e.g., "Add memory allocator," "Fix IRQ handler").

* Reference issue IDs for traceability to ensure every change maps back to the roadmap.

### Atomic Changes

* One task = one logical change.

* Avoid mixing bug fixes, new features, and documentation in a single PR.

---

## 📂 Repo Rules

### .MD Files

* Every `.md` file must be fully implemented (no placeholders).

* Once complete, its content should be migrated into the Wiki.

*After migration,**delete the `.md` file** from the main repository to avoid duplication (except for core files like `README.md` and `CONTRIBUTING.md`).

### Code Contributions

* Must include unit tests and CI validation.

* Kernel shards must be modular and documented in the Wiki.

* Security primitives must pass regression tests before merging.

### Branching & PRs

* Use `main` for stable releases only.

* Feature branches must follow the `feature/<name>` format.

* PRs require at least one reviewer approval.

---

## 📚 Wiki Rules

### Structure

* Each subsystem (kernel, memory, scheduler, drivers, security, CI/CD) gets its own page.

* Add diagrams, flowcharts, and examples where possible.

* Maintain a glossary for technical terms.

### Updates

*Any new feature or module must be documented in the Wiki**before** merging.

* Wiki pages must be kept in sync with repo changes.

* Changelogs and release notes must be mirrored in the Wiki.

### Contributor Guidelines

* Clearly state coding standards, testing requirements, and review process (this file).

* Provide templates for bug reports, feature requests, and PRs.

---

## ⚙️ Automation Rules

### CI/CD

* Every commit triggers automated builds, QEMU boot tests, and security scans.

* Cross-architecture builds (x86, ARM, RISC-V) must be validated.

### Docs Automation

* Auto-generate API docs from code comments using Doxygen or Sphinx.

* Push generated docs into the Wiki automatically.

### Release Automation

* Tagged releases must package ISO images and binaries.

* Publish changelogs both in the repo and Wiki.

---

## ✅ Enforcement

* These rules are enforced via GitHub Actions checks (`lint`, `test`, `docs`).

* **No merge is allowed if rules are violated.**

* Every contributor or AI tool knows exactly what to do: `implement` → `document` → `automate` → `enforce`.
