# Future Development

**Status:** This page is a planning overview, not an implementation report. Ideas from Linux distributions and BSD systems are references for design study; they do not imply that SigmaOS provides equivalent behavior.

## Strategic objective

Develop SigmaOS into a competitive operating system by improving reliability, security, performance, hardware support, and usability. Evaluate one component at a time and describe its current limitations before claiming parity with another system.

## Development principles

- Inspect the relevant SigmaOS source and define the behavior that is missing before proposing a new subsystem.
- Treat Linux and BSD designs as references. Adapt only the parts that fit SigmaOS's architecture, threat model, licensing requirements, and hardware targets.
- Separate policy models and metadata from runtime enforcement. A capability is supported only when the runtime path performs it and errors are handled safely.
- Fail closed when a required cryptographic, authentication, or hardware provider is unavailable. Do not use fabricated success values, signatures, checksums, or benchmark results.
- Preserve existing APIs where possible. When an interface must change, document migration and compatibility effects.
- Keep performance claims tied to repeatable benchmarks that include the machine, workload, configuration, and baseline.

## Component roadmaps

Roadmaps live with their owning component so implementation status, reference comparisons, validation, and future work stay together:

- [Kernel](04-Kernel.md), [memory management](Memory-Management.md), [scheduler](Scheduler.md)
- [Filesystems and VFS](05-Filesystems.md), [networking](06-Networking.md), [security](07-Security.md)
- [Drivers](Hardware-Drivers.md), [desktop](08-Desktop.md), [packaging and updates](09-Packaging.md)
- [Enterprise Productivity Suite Roadmap](18-Enterprise-Productivity-Suite-Roadmap.md), [Pull Request Gateway Workflows](19-Pull-Request-Gateway-Workflows.md)
- [Init and services](Init-and-Services.md), [IPC and syscalls](IPC-and-Syscalls.md)
- [Virtualization and containers](Container-Virtualization.md), [power and time](Power-Management.md)

## Work status vocabulary

Use these terms consistently in wiki pages and release notes:

| Status | Meaning |
|---|---|
| Proposed | Design idea only; no implementation is implied. |
| Prototype | Code exists, but runtime integration, error handling, or validation is incomplete. |
| Integrated | The feature is connected to its intended runtime path; supported configurations and limitations are documented. |
| Supported | Integration has repeatable validation for the listed configurations and documented recovery behavior. |

Do not label a feature supported based only on a type, API, example, or standalone model.

## Development cycle

1. Describe the current behavior and link the relevant source files.
2. Record the gap, threat or failure cases, and the Linux/BSD design being studied.
3. Implement a small, reviewable change with explicit resource ownership and failure behavior.
4. Validate the change at the relevant subsystem boundary and document the configuration used.
5. Update the component roadmap and user documentation to match what is actually integrated.

## Maintenance

Keep this page focused on planning rules and links to component plans. Put detailed work and acceptance criteria on the corresponding component page. Remove claims that cannot be tied to source, runtime integration, or recorded validation.
