# Inspiration and Reference Projects

Use these projects to study proven design choices, then adapt the smallest useful idea to SigmaOS. The presence of a similar component name does not demonstrate feature parity.

| Project | Design areas to study | SigmaOS adaptation |
|---|---|---|
| [Linux Kernel](https://www.kernel.org/doc/html/latest/) | Scheduler, memory, VFS, driver lifecycle, networking, security enforcement | Define clear subsystem contracts and validate end-to-end behavior before claiming support. |
| [Omarchy Linux](https://github.com/omacom/omarchy) | Opinionated desktop defaults, keyboard workflow, themes, installation and updates | Make everyday workflows discoverable; validate from a clean install and keyboard-only operation. |
| [os-tutorial](https://github.com/cfenollosa/os-tutorial) | Staged boot, interrupts, memory and syscall explanations | Keep small runnable examples and test each boot stage in a documented emulator. |
| [Linux Mint](https://github.com/linuxmint) | Onboarding, familiar tools, defaults, recovery and user-facing documentation | Validate first-session tasks with a clean profile and present actionable failure messages. |
| [Arch Linux](https://wiki.archlinux.org/) | Transparent configuration, reproducible build recipes and practical documentation | Document exact supported configurations and separate user-controlled choices from defaults. |
| [Redox OS](https://doc.redox-os.org/book/) | Rust system interfaces, service boundaries and resource-oriented APIs | Prototype isolation at one boundary and measure fault containment and IPC cost. |
| [xv6 RISC-V](https://github.com/mit-pdos/xv6-riscv) | Compact process, trap, filesystem and locking designs | Use as a review reference; explicitly document where SigmaOS invariants differ. |

## Engineering workflow

1. Identify the upstream design and the specific behavior worth adapting.
2. State the SigmaOS constraint, threat boundary, and intended runtime path.
3. Add or update tests before describing the capability as working.
4. Run formatting, targeted tests, broader affected suites, and relevant QEMU/device checks.
5. Update only the owning component page with source locations, validation evidence, limitations, and future roadmap.
