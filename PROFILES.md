# SigmaOS Profession Profiles

SigmaOS adopts a highly specialized, intent-driven OS model through its **Profession Profiles**. Instead of a generic desktop environment, the OS dynamically adapts its UI, underlying kernel policies, toolchain, and shortcuts based on the active profession profile.

## The Profile Architecture

The repository's `/profiles/` directory structure dictates the available configurations:

* `/profiles/cashier/`
* `/profiles/accountant/`
* `/profiles/doctor/`
* `/profiles/engineer/`
* `/profiles/lawyer/`
* `/profiles/farmer/`

Each directory contains:

* `tools.md`: A specific list of allowed and pre-configured tools.
* `config.json`: The specific configuration activating the profile.
* `shortcuts/`: Symbolic links to shared modules specific to that profession.

## Profile Details and Tools

### Cashier

* **Environment**: Point-of-Sale (POS) focus.
* **Tools**: Barcode scanner abstraction, cash register integration, high-availability ledger API.
* **UI Context**: Distraction-free till interface, touch-optimized.

### Accountant

* **Environment**: High-precision ledger manipulation and auditing.
* **Tools**: Spreadsheet optimization plugins, encrypted ledger integrations, local tax software sandboxes.
* **UI Context**: Multi-monitor spreadsheet layout, high-contrast text.

### Doctor

* **Environment**: HIPAA-compliant telemetry and patient management.
* **Tools**: EHR (Electronic Health Record) software, medical imaging hardware support (DICOM).
* **UI Context**: Privacy-first screen blurring (for sensitive data), quick-search diagnostics.

### Engineer

* **Environment**: High-performance compute, CAD, and development.
* **Tools**: Compiler toolchains, Docker/Podman integration, GPU pass-through for 3D modeling.
* **UI Context**: Tiling window manager, telemetry dashboards.

### Lawyer

* **Environment**: Secure document management and communication.
* **Tools**: E-discovery tools, encrypted email client, secure document signing (PQC integrated).
* **UI Context**: Document review split-screen, redaction shortcuts.

### Farmer

* **Environment**: Agricultural telemetry and drone control.
* **Tools**: IoT sensor dashboards, drone pathfinding SDK, offline weather and satellite sync.
* **UI Context**: High visibility outdoor UI, offline-first sync alerts.

## Customizing Profiles (Tutorial)

1. **Create a new folder** under `/profiles/` (e.g., `/profiles/architect/`).
2. **Add `config.json`**: Define the profession name and active status.
3. **Link Tools**: Add symlinks to binaries in `/profiles/architect/shortcuts/`.
4. **Define Tools**: Write `/profiles/architect/tools.md` explaining the workflow.
5. **Restart Context Manager**: Run `agent.task run` and the OS will automatically adapt layout and quotas.
