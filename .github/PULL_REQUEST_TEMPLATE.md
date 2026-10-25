# Pull Request Template

## Component
Describe the proposed changes, affected micro-engines, and components.

## Shell component
Component classification and shell subsystem impacts.

## Upstream study & Reference study
- **Upstream Study / Reference study:** Details of architecture research and inspiration.
- **What SigmaOS adopts:** Safe-Rust micro-engine implementations and zero-dependency abstractions.

## Safety and rollback
- **Risks and rollback:** Failure recovery mechanisms, rollback steps, and fallback paths.

## Kernel safety review
- [ ] Kernel safety review completed
- [ ] 100% test pass rate verified via `./run_sigma_tests.sh`
- [ ] Clean compilation with zero warnings or errors

## Shell safety review
- [ ] Shell safety review completed
