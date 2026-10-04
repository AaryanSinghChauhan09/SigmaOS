> Imported repository document from [`Agents/COMPONENT_AGENTS_TEMPLATE.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/COMPONENT_AGENTS_TEMPLATE.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# [COMPONENT_NAME] AI Agent Guidelines

This document provides specialized guidelines for AI agents working on the [COMPONENT_NAME] subsystem of SigmaOS.

## Component Overview

[COMPONENT_NAME] is responsible for [BRIEF_DESCRIPTION].

## Operational Boundaries

### Always Do
- Run relevant tests before submitting changes
- Add documentation for new APIs
- Follow SigmaOS coding standards (safe Rust, no external dependencies)
- Measure performance impact for optimizations

### Ask First
- Major architectural changes
- Adding external dependencies
- Modifying interfaces used by other components

### Never Do
- Commit hardcoded secrets or keys
- Introduce memory safety violations
- Break compatibility without documentation

## Open Source Inspiration

### Primary Competitors
- **Linux**: [RELEVANT_FEATURE] - [LINK/DESCRIPTION]
- **FreeBSD**: [RELEVANT_FEATURE] - [LINK/DESCRIPTION]
- **OpenBSD**: [RELEVANT_FEATURE] - [LINK/DESCRIPTION]

### Key Improvements Opportunities
1. [IMPROVEMENT_1]: [DESCRIPTION]
2. [IMPROVEMENT_2]: [DESCRIPTION]
3. [IMPROVEMENT_3]: [DESCRIPTION]

## Implementation Status

### Current State
- **Implemented**: [LIST_IMPLEMENTED_FEATURES]
- **In Progress**: [LIST_IN_PROGRESS_FEATURES]
- **Planned**: [LIST_PLANNED_FEATURES]

### Testing
- **Unit Tests**: [STATUS]
- **Integration Tests**: [STATUS]
- **Performance Tests**: [STATUS]

## Architecture Notes

### Key Structures
- [STRUCT_1]: [PURPOSE]
- [STRUCT_2]: [PURPOSE]
- [STRUCT_3]: [PURPOSE]

### Dependencies
- **Internal**: [LIST_INTERNAL_DEPENDENCIES]
- **External**: [LIST_EXTERNAL_DEPENDENCIES]

## Development Workflow

### Verification Commands
```bash
# Run component tests
cargo test --package [COMPONENT_NAME]

# Check compilation
cargo check --lib

# Format code
cargo fmt
```

### Common Patterns
[PATTERN_EXAMPLES]

## Known Issues

- [ISSUE_1]: [DESCRIPTION]
- [ISSUE_2]: [DESCRIPTION]

## Future Roadmap

### Short Term
- [GOAL_1]
- [GOAL_2]

### Long Term
- [GOAL_3]
- [GOAL_4]

## References

- [REFERENCE_1]: [LINK]
- [REFERENCE_2]: [LINK]

---

*Generated for SigmaOS [COMPONENT_NAME] component*
