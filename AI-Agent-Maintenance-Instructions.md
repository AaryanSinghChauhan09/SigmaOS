# AI Agent Maintenance Instructions

## Purpose
This document provides binding instructions for autonomous AI development agents to maintain, improve, and consolidate the **SigmaOS** repository (https://github.com/AaryanSinghChauhan09/SigmaOS) and its official GitHub Wiki (https://github.com/AaryanSinghChauhan09/SigmaOS/wiki).

---

## Mandatory Rule: Component Wiki Pages & Single Agent File Invariant

AI agents MUST strictly adhere to the following rules:
1. **Exclusive Component Wiki Creation**: AI agents are permitted to create and maintain ONLY various component-specific GitHub wiki pages and canonical navigation portals. Every component wiki page MUST be created by the AI agent by taking inspiration for current architecture and future development based on open-source competitors (Linux kernel, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, illumos, systemd, CachyOS, Omarchy, Linux Mint, Arch Linux, etc.).
2. **One Agent File Per Component Invariant in `Agents/`**: For every component of the operating system (including those components which are currently missing in SigmaOS, but are critical to SigmaOS, taking inspiration from Linux & BSD distributions), exactly ONE `[COMPONENT]_AGENT.md` file MUST exist in the `Agents/` folder. There must NEVER be multiple agent `.md` files for the same component. Any duplicates must be consolidated into the single canonical agent file.
3. **Consolidation & Uniqueness**: Exactly ONE wiki page shall exist per OS component (e.g., `Scheduler-Component-Guide.md`, `Memory-Management-Component-Guide.md`, `Filesystem-Vfs-Component-Guide.md`). When modifying, improving, or adding ideas to a component, the AI agent MUST update the existing component wiki page rather than creating duplicate or fragmented pages.
4. **Future Development Roadmap**: Each component wiki page must document concrete future development roadmaps, feature gap analyses against Linux/BSD distributions, architectural diagrams, implementation priorities, and testing strategies.
5. **Executable Code Integrity**: Ensure `.md` files contain executable code that works if directly implemented. If a `.md` specification file is fully implemented, migrate its comprehensive documentation into the GitHub Wiki.
6. **Single-Branch Invariant on Main Repository**: Ensure there is ONLY the `main` branch active on the GitHub repository. All pull requests and remote branches must be merged via `-X theirs` into `main` and deleted immediately.

---

## Core Programming & Engineering Principles

AI agents MUST maximize the usage of modern low-level systems languages (**Rust, Zig, Nim**) and adhere to the Master Engineering Principles Codex defined in [`AI-Development-Agent-Operating-Manual.md`](./AI-Development-Agent-Operating-Manual.md):

### 1. Object-Oriented Principles (OOPS) & Structural Discipline
- **Objects, Classes & Instances**: Model kernel state, drivers, and subsystems as concrete typed Rust structs implementing clear lifecycle behaviors.
- **Encapsulation & Information Hiding**: Enforce strict private field boundaries; state mutations are permitted exclusively via validated public APIs.
- **Clear Abstraction & Blackbox Design**: Conceal MMIO, assembly primitives, and hardware details behind clean abstract trait interfaces.
- **Inheritance, Polymorphism & Composition**: Leverage Rust trait polymorphism (`impl Trait`, `dyn Trait`), trait inheritance (`trait Sub: Super`), and favor composition over inheritance.
- **Booch Method & GRASP**: Systematic object-oriented assignment of responsibilities with low coupling and high cohesion.
- **Demeter Law & Command-Query Separation (CQS)**: Methods either mutate state or return queries, never chain arbitrary external dependencies.
- **Uniform Access & Inversion of Control (IoC)**: Standardized access semantics; drivers register with centralized dispatchers.

### 2. SOLID Architecture
- **S - Single Responsibility Principle (SRP)**: Every struct, function, and file must have one single, well-defined responsibility.
- **O - Open/Closed Principle (OCP)**: Systems must be open for extension (via traits and plugins) but closed for modification of core dispatch engines.
- **L - Liskov Substitution Principle (LSP)**: Derived implementations (e.g., specific block device drivers) must be interchangeable without violating callers' expectations.
- **I - Interface Segregation Principle (ISP)**: Create small, focused traits rather than monolithic, bloated interfaces.
- **D - Dependency Inversion Principle (DIP)**: Depend on abstractions rather than low-level concrete implementations.

### 3. Pragmatic Industry Principles & Clean Code
- **DRY (Don't Repeat Yourself)**: Unify duplicate logic across system tools and kernel services.
- **KISS (Keep It Simple, Stupid)**: Favor simple, deterministic, readable implementations over unnecessarily complex abstractions.
- **YAGNI (You Aren't Gonna Need It)**: Implement only what is required to fulfill the component specification and competitor parity.
- **Separation of Concerns & Isolation**: Isolate drivers, filesystems, network protocols, and process management into distinct modular domains.
- **Design by Contract**: Explicitly define and verify preconditions, postconditions, and system invariants.
- **Boy Scout Rule & Refactor Regularly**: Continually clean up obsolete code paths, remove warnings, and modernize implementations.
- **Clean Code Mindset & Zen of Python**: Explicit is better than implicit; readable, expressive code with self-documenting identifiers.
- **Optimise for Deletion & Rule of Three**: Build modular subsystems that can be cleanly removed; wait for three instances before abstracting.
- **Ninety-Ninety Rule & Power of Ten**: Rigorously engineer the critical details; bound loops and limit scoping.
- **Gall's Law & Worse is Better**: Build from proven, simple components; pragmatic simplicity trumps theoretical complexity.
- **Theirs Strategy for Merges**: Resolve branch integration conflicts by prioritizing the modern, enhanced SigmaOS implementation over deprecated legacy code.

### 4. Low-Level Control & Operating System Fundamentals
- **Low-Level Languages & Direct Device Control**: Pure Rust (`#![no_std]`), Zig, and Nim interacting safely with MMIO and CPU control registers.
- **Process Management & Efficient Scheduling**: Preemptive scheduling (EEVDF/CFS/ULE inspired), task switching, PID 1 init supervision, POSIX signal handling, process groups, and sessions.
- **Memory Discipline & Virtual Memory Abstraction**: Physical page allocation (Buddy system), kernel object caching (UMA/Slab), demand paging, CoW fork, guard pages, ASLR, and manual memory control.
- **Zero-Copy & Cache Conscious Design**: Zero-copy network/storage/IPC pipelines (shared memory, DMA-BUF) and cache-line aligned data structures.
- **Interrupt Handling Discipline**: Fast top-half interrupt service routines with work deferred to bottom-halves/worker threads.
- **File Management & Transactional Storage**: Virtual File System (VFS), directory entries, inode tables, page caches, journaling, and modern transactional storage (ZFS/HAMMER2 inspired).
- **Deadlock Avoidance & Safe Concurrency**: Hierarchical lock ordering, lockless ring buffers, atomic primitives, and race-free compile-time safety.
- **System Security & Least Privilege**: Principle of least privilege (PoLP), principle of least astonishment (POLA), secure defaults, memory safety invariants, constant-time cryptography, access control lists, capability tokens, and sandbox isolation.
- **Bootloader Integration & Cross Compilation**: Seamless bootloader-to-kernel handoff and multi-architecture HAL portability (`x86_64`, `aarch64`, `riscv64`).
- **Deterministic Testing & FIRST**: Unit tests following Arrange-Act-Assert (AAA), running fast, isolated, repeatable, self-validating, and timely.

---

## Repository & Branch Management Strategy

### Single-Branch Invariant
- **The repository must ONLY have the `main` branch active.**
- All pull requests from `https://github.com/AaryanSinghChauhan09/SigmaOS/pulls` must be reviewed, tested, merged into `main`, and closed.
- All remote branches from `https://github.com/AaryanSinghChauhan09/SigmaOS/branches` must be merged into `main` and then deleted immediately.
- Regularly execute `git remote prune origin` to keep the remote tracking references clean.

### Branch Merging Routine
```bash
# 1. Fetch all updates and prune stale references
git fetch --all --prune

# 2. Iterate and merge all non-main remote branches using -X theirs
for ref in $(git for-each-ref --format='%(refname:short)' refs/remotes/origin/ | grep -v "^origin$\|^origin/main$\|^origin/HEAD$\|wiki-origin"); do
    echo "Merging $ref..."
    if ! git merge-base --is-ancestor "$ref" HEAD 2>/dev/null; then
        git merge --no-edit -X theirs "$ref" || git merge --abort
    fi
    branch_name=$(echo "$ref" | sed 's|^origin/||')
    echo "Deleting remote branch $branch_name..."
    git push origin --delete "$branch_name" 2>&1 || true
done

# 3. Push consolidated main
git push origin main
```

---

## Security & Quality Assurance Directives

1. **GitHub Security Scans**: Continuously monitor and fix vulnerabilities reported in https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning:
   - Remediate access of invalid pointers and dangling pointers.
   - Prevent DOM text reinterpretation as HTML in UI components.
   - Fix prototype-polluting functions and property overwrites.
   - Eliminate hardcoded cryptographic keys and unverified PRNG seeds.
   - Prevent buffer overflows, format string vulnerabilities, and command injection.
2. **GitHub Actions Workflows**: Validate that workflows in https://github.com/AaryanSinghChauhan09/SigmaOS/actions pass cleanly.
3. **Standalone Testing**: Run tests with `--test-threads=1` to ensure thread safety when testing global singleton kernel states.

---

## Code & Documentation Synchronization

1. **Agent Files in `Agents/`**:
   - Every OS component must have exactly one `.md` file in `Agents/` (e.g., `KERNEL_SCHEDULER_AGENT.md`).
   - Every operational role must have a guide in `Agents/` (e.g., `TESTING_AGENT.md`, `FIXING_AGENT.md`, `IMPROVING_AGENT.md`, `RESEARCHING_AGENT.md`, `STABILISING_AGENT.md`, `COMMENTING_AGENT.md`, `PLANNING_AGENT.md`, `ITERATING_AGENT.md`, `MODULARISING_AGENT.md`, `REVIEWING_AGENT.md`, `COMPARING_AGENT.md`, `COMPETITOR_ADVANTAGE_ANALYSING_AGENT.md`, `COMPETITOR_ABSORBING_AGENT.md`, `MAINTENANCE_AGENT.md`, `INSPIRATION_TAKING_AGENT.md`).
2. **GitHub Wiki Synchronization**:
   - Keep the standalone wiki repository (`SigmaOS.wiki`) synchronized with the main code repository.
   - Fully implemented `.md` specifications must be elevated into Arch Linux Wiki style component guides on the GitHub Wiki.
