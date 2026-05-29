# Welcome to the SigmaOS Wiki

SigmaOS is a sovereign, zero-dependency operating system built on a **Lattice Architecture**. It prioritizes extreme simplicity, explicit zero-trust security, and native orchestration.

## 🚀 Key Technologies
*   **Sovereign Orchestrator**: Native lightweight containerization engine. [Read more here.](Container-Orchestrator.md)
*   **Zenith Desktop & SDK**: Secure containerized desktop environment and application toolkit. [Read more here.](Zenith-Desktop-SDK.md)
*   **Sovereign System Profiles**: Adaptation profiles & Hybrid C/Rust Interop. [Read more here.](Sovereign-System-Profiles.md)
*   **Resilience & Control Center**: NixOS declarative profiles and safe diagnostics. [Read more here.](Resilience-and-Control-Center.md)
*   **Proton Bridge**: Opt-in POSIX/Linux syscall translation for running legacy binaries.
*   **Zero-Trust VFS**: Explicit Role-Based Access Control (RBAC) enforced deeply at the filesystem level.

## 🛠️ Contributing
We enforce a strict quality gateway. Please ensure you run tests locally before submitting a PR.
*   `make test` to run the unit test suite.
*   `make valgrind_check` to detect memory leaks.
*   The CI/CD pipeline will automatically build all variants and execute static analysis.

## 📚 Resources
*   [Developer Guide](../DEVELOPER_GUIDE.md)
*   [Architecture Blueprint](../Architecture.md)
*   [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
