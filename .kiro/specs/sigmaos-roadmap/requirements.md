# Requirements Document

## Introduction

This document captures the requirements for the SigmaOS full-platform roadmap — a structured, actionable plan for building all major OS components absent from or partially implemented in the current SigmaOS codebase, compared against a production-grade Linux distribution. SigmaOS is a sovereign, Rust-first operating system targeting x86_64 hardware, with a web-based UI shell, formal-security principles, deterministic package management, and a modern filesystem stack.

The 15 component areas below are prioritized to bring SigmaOS to parity with a minimal but complete Linux distribution. The 90-day priority deliverables are: finalize the sigpkg format and sigma_pkg_registry API, expand sigma_libc to a stable libc subset, implement a minimal package client integrated with sigmad, establish QEMU boot tests and reproducible build CI, and deliver a basic service manager spec and sigmad supervisor.

---

## Glossary

- **SigmaOS**: The sovereign, AI-native x86_64 operating system that is the subject of this roadmap.
- **sigma_pkg**: The SigmaOS native package format providing deterministic, signed, content-addressed software distribution.
- **sigpkg**: Shorthand for the sigma_pkg binary format specification and its associated metadata schema.
- **sigma_pkg_registry**: The HTTP API server providing the authoritative package index and binary artifact storage for SigmaOS.
- **Pkg_Client**: The SigmaOS command-line package management client responsible for resolving, downloading, verifying, and installing sigma_pkg packages.
- **sigma_libc**: The SigmaOS-native C library implementation providing a stable POSIX-compatible ABI for userland programs.
- **Dynamic_Loader**: The SigmaOS runtime dynamic linker and loader (`ld-sigma.so`) responsible for resolving shared library dependencies at process launch.
- **sigmad**: The SigmaOS PID 1 supervisor and service manager, analogous to systemd.
- **Service_Unit**: A declarative configuration file describing a managed SigmaOS service, its dependencies, restart policy, and socket-activation parameters.
- **SigmaVFS**: The SigmaOS virtual filesystem layer that abstracts concrete filesystem implementations behind a uniform kernel interface.
- **SigmaFS**: The primary SigmaOS journaling, encrypted filesystem backed by a copy-on-write block layer.
- **Device_Manager**: The SigmaOS kernel subsystem that detects hardware events, manages device nodes under `/dev`, and dispatches hotplug events to userland.
- **Net_Manager**: The SigmaOS userland network configuration daemon responsible for interface bring-up, DHCP, DNS, and firewall rule application.
- **DRM_Driver**: The SigmaOS Direct Rendering Manager kernel driver providing GPU memory management and display pipeline control.
- **Compositor**: The SigmaOS Wayland-protocol display compositor responsible for window compositing, input routing, and display output.
- **Audio_Server**: The SigmaOS userland audio mixing and routing daemon providing a stable API to applications.
- **Secure_Boot_Chain**: The sequence from UEFI firmware through sigma-boot bootloader to the signed SigmaOS kernel, enforcing cryptographic verification at each step.
- **MAC_Policy**: The SigmaOS mandatory access control policy engine enforcing subject-to-object capability constraints independently of DAC permissions.
- **TPM**: Trusted Platform Module — hardware security chip used for attestation and sealing secrets to platform state.
- **Toolchain**: The SigmaOS cross-compilation and build toolchain including the Rust compiler configuration, linker scripts, and sysroot layout.
- **Sigma_SDK**: The SigmaOS application developer kit providing build tooling, API bindings, manifest schema, and documentation for third-party application authors.
- **OCI_Runtime**: The SigmaOS container runtime implementing the Open Container Initiative runtime specification to launch OCI-formatted container images.
- **Installer**: The SigmaOS guided installation program that partitions a target disk, writes the OS image, configures the bootloader, and collects initial user settings.
- **CI_Pipeline**: The SigmaOS continuous integration system executing build, test, and verification jobs on every commit and pull request.
- **A11y_Subsystem**: The SigmaOS accessibility infrastructure providing screen reader support, keyboard navigation, and high-contrast rendering.


---

## Requirements

---

### Requirement 1: sigpkg Format Specification

**User Story:** As a package maintainer, I want a fully specified, versioned sigpkg binary format with a machine-readable schema, so that I can produce packages that any conforming SigmaOS tool can parse, verify, and install deterministically.

#### Acceptance Criteria

1. THE sigma_pkg build tool SHALL produce sigpkg archives conforming to the sigpkg format specification version 1.0.
2. WHEN a sigpkg archive is provided to the Pkg_Client, THE Pkg_Client SHALL parse the archive header, metadata manifest, and payload sections without error for any archive produced by the build tool.
3. THE sigpkg format specification SHALL define required fields: package name, semantic version, target architecture, content hash (SHA-256), maintainer identity, and dependency list.
4. IF a sigpkg archive header contains an unrecognized format version, THEN THE Pkg_Client SHALL reject the archive and return a descriptive version-mismatch error.
5. THE sigma_pkg build tool SHALL embed a detached Ed25519 signature over the content hash in every produced archive.
6. FOR ALL valid sigpkg archives, parsing the archive then serializing its metadata then parsing the serialized metadata SHALL produce an equivalent metadata object (round-trip property).

---

### Requirement 2: sigma_pkg_registry API

**User Story:** As a Pkg_Client operator, I want a versioned HTTP API for the sigma_pkg_registry, so that I can query available packages, resolve dependencies, and download signed artifacts reliably.

#### Acceptance Criteria

1. THE sigma_pkg_registry SHALL expose a `/v1/packages` endpoint returning a paginated JSON index of all published packages.
2. WHEN a GET request is issued to `/v1/packages/{name}/{version}`, THE sigma_pkg_registry SHALL return the package metadata and a pre-signed download URL within 500 milliseconds.
3. WHEN a package artifact is uploaded to the sigma_pkg_registry, THE sigma_pkg_registry SHALL verify the Ed25519 signature against the registered maintainer key before storing the artifact.
4. IF signature verification fails during upload, THEN THE sigma_pkg_registry SHALL reject the upload with HTTP 422 and a rejection reason in the response body.
5. THE sigma_pkg_registry SHALL return HTTP 404 with a structured error body for requests to unknown package names or versions.
6. THE sigma_pkg_registry SHALL support TLS 1.3 for all client connections and reject connections using TLS 1.1 or earlier.


---

### Requirement 3: Package Client Tooling

**User Story:** As a system operator, I want a command-line package client that can install, upgrade, remove, and verify packages transactionally, so that I can manage software on SigmaOS without leaving the system in a partially modified state on failure.

#### Acceptance Criteria

1. WHEN the user runs `sigpkg install <name>`, THE Pkg_Client SHALL resolve the full dependency graph, download all required sigpkg archives, verify each signature, and apply the installation atomically.
2. IF any signature verification step fails during installation, THEN THE Pkg_Client SHALL abort the entire transaction, roll back all filesystem changes, and report which package failed verification.
3. WHEN the user runs `sigpkg upgrade`, THE Pkg_Client SHALL compare installed package versions against the registry index and upgrade all packages with newer available versions in a single atomic transaction.
4. WHEN the user runs `sigpkg remove <name>`, THE Pkg_Client SHALL uninstall the package and any packages that depend solely on it, removing all installed files.
5. THE Pkg_Client SHALL display a summary of planned changes (packages to add, upgrade, or remove) and require explicit user confirmation before modifying the filesystem.
6. IF the system loses power during an installation transaction, THEN THE Pkg_Client SHALL restore the pre-transaction filesystem state on the next invocation without requiring manual intervention.

---

### Requirement 4: sigma_libc Stable POSIX Subset

**User Story:** As a userland developer, I want sigma_libc to expose a stable POSIX-compatible C library ABI, so that I can compile standard C programs and port existing POSIX software to SigmaOS without a host glibc dependency.

#### Acceptance Criteria

1. THE sigma_libc SHALL implement the C11 standard library interfaces for memory allocation (`malloc`, `free`, `realloc`, `calloc`), string manipulation, formatted I/O, and math functions.
2. THE sigma_libc SHALL implement the POSIX.1-2017 subset covering file I/O (`open`, `read`, `write`, `close`, `lseek`), process management (`fork`, `exec`, `wait`), signal handling, and POSIX threads (`pthread_create`, `pthread_join`, `pthread_mutex_*`).
3. WHEN a program compiled against sigma_libc calls `malloc` with a positive size, THE sigma_libc allocator SHALL return a pointer to memory of at least the requested size, or return NULL if the allocation cannot be satisfied.
4. FOR ALL pointer values `p` returned by `malloc(n)` where n > 0, freeing `p` with `free(p)` and then calling `malloc(n)` SHALL succeed without crashing (allocator stability property).
5. THE sigma_libc SHALL provide a stable ABI documented by a versioned symbol map; removing or changing the signature of any versioned symbol SHALL require a major version increment.
6. THE sigma_libc build SHALL produce a freestanding binary containing no symbols from the host system's glibc, verified by the CI_Pipeline.


---

### Requirement 5: Dynamic Loader and Runtime ABI

**User Story:** As a userland developer, I want the SigmaOS dynamic loader to resolve shared library dependencies at process launch, so that dynamically linked programs can run on SigmaOS without static linking every dependency.

#### Acceptance Criteria

1. WHEN a dynamically linked ELF binary is executed, THE Dynamic_Loader SHALL resolve all `DT_NEEDED` entries against the sigma_libc sysroot and load each required shared object into the process address space.
2. THE Dynamic_Loader SHALL apply ELF relocations for all supported relocation types (R_X86_64_64, R_X86_64_GLOB_DAT, R_X86_64_JUMP_SLOT, R_X86_64_RELATIVE) before transferring control to the program entry point.
3. IF a required shared object cannot be found in the library search path, THEN THE Dynamic_Loader SHALL print a diagnostic identifying the missing library and exit the process with code 127.
4. THE Dynamic_Loader SHALL support the `LD_LIBRARY_PATH` and `LD_PRELOAD` environment variables for development and debugging purposes.
5. WHEN the same shared object is required by multiple loaded libraries, THE Dynamic_Loader SHALL load the object exactly once and share its text segment across all dependents.

---

### Requirement 6: Coreutils Equivalents in Rust

**User Story:** As a system operator, I want a set of standard POSIX utility programs implemented in Rust, so that SigmaOS provides a complete base userland without depending on GNU coreutils or BusyBox.

#### Acceptance Criteria

1. THE SigmaOS base userland SHALL include Rust implementations of the following utilities: `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`, `rmdir`, `echo`, `printf`, `grep`, `sed`, `awk`, `find`, `sort`, `uniq`, `head`, `tail`, `wc`, `chmod`, `chown`, `ln`, `stat`, `df`, `du`, `ps`, `kill`, `env`, `pwd`, `date`, and `id`.
2. WHEN each utility is invoked with a valid POSIX-specified option set, THE utility SHALL produce output conforming to the POSIX.1-2017 specification for that utility.
3. WHEN each utility is invoked with an invalid option, THE utility SHALL print a usage message to stderr and exit with code 1.
4. THE coreutils build SHALL pass the SigmaOS coreutils test suite covering at least 90% of documented POSIX behaviors for each utility.


---

### Requirement 7: Init System and Service Manager

**User Story:** As a system integrator, I want sigmad to manage system services declaratively using Service_Unit files, so that services start in dependency order, restart on failure, and their lifecycle is observable without writing custom init scripts.

#### Acceptance Criteria

1. THE sigmad SHALL parse Service_Unit files from `/etc/sigma/services/` at boot and construct a directed acyclic graph of service dependencies.
2. WHEN the system reaches the default target, THE sigmad SHALL start all services whose dependencies are satisfied, in dependency order, before marking the target as reached.
3. WHEN a managed service process exits with a non-zero code and its Service_Unit specifies `Restart=on-failure`, THE sigmad SHALL restart the service after the configured restart delay (default: 1 second).
4. THE sigmad SHALL expose a control socket at `/run/sigmad/control.sock` accepting `start`, `stop`, `restart`, `status`, and `list` commands for runtime service management.
5. WHEN socket activation is configured in a Service_Unit, THE sigmad SHALL open and hold the declared socket before starting the service, passing the socket file descriptor to the service process via the standard socket activation protocol.
6. THE sigmad SHALL write structured service lifecycle events (start, stop, crash, restart) to the system journal with ISO 8601 timestamps and service unit names.
7. IF a service's dependency fails to start within 30 seconds, THEN THE sigmad SHALL mark that service as failed and log the dependency name and failure reason.

---

### Requirement 8: Journal and Log Subsystem

**User Story:** As a system administrator, I want a structured, persistent system journal, so that I can query logs by service, time range, or severity level after the fact.

#### Acceptance Criteria

1. THE sigmad journal SHALL store log entries in a structured binary format with fields for: timestamp (nanosecond precision), service unit name, priority level, and message text.
2. WHEN a service writes to its standard output or standard error, THE sigmad SHALL capture the output and store it in the journal attributed to that service unit.
3. THE sigmad SHALL provide a `sigjournal` command-line tool that queries journal entries by service name, time range, and minimum priority level.
4. WHEN `sigjournal` is invoked with `--follow`, THE sigjournal tool SHALL stream new journal entries to stdout in real time as they are written.
5. THE journal storage SHALL rotate log files when the journal exceeds a configurable maximum size (default: 512 MB), preserving the most recent entries.


---

### Requirement 9: SigmaFS Journaling and Encryption

**User Story:** As a storage engineer, I want SigmaFS to provide journaling and transparent block-level encryption, so that the filesystem recovers cleanly from unclean shutdowns and all at-rest data is protected.

#### Acceptance Criteria

1. WHEN the SigmaFS driver mounts a volume, THE SigmaVFS SHALL replay the journal to restore the filesystem to a consistent state before making it available to userland.
2. WHEN a write transaction is committed to SigmaFS, THE SigmaFS driver SHALL write the journal entry to stable storage before writing the data blocks, ensuring crash consistency.
3. THE SigmaFS driver SHALL encrypt all data blocks using AES-256-XTS with a volume key derived from a user passphrase and a per-volume salt via Argon2id.
4. IF the passphrase provided at mount time produces the wrong derived key, THEN THE SigmaFS driver SHALL refuse to mount the volume and return a clear authentication-failure error.
5. THE `mkfs.sigmafs` tool SHALL initialize a new SigmaFS volume with a journal of configurable size (default: 128 MB) and write the encrypted volume header.
6. THE `fsck.sigmafs` tool SHALL detect and report journal inconsistencies, orphaned inodes, and block allocation bitmap errors without requiring the volume to be mounted.

---

### Requirement 10: initramfs Integration

**User Story:** As a boot engineer, I want the SigmaOS initramfs to unlock the encrypted root volume and pivot to it before launching sigmad, so that the full system boots from an encrypted root without manual intervention.

#### Acceptance Criteria

1. WHEN the kernel hands off to the initramfs, THE initramfs SHALL prompt the user for the root volume passphrase via the console or Plymouth if a display is available.
2. WHEN the correct passphrase is entered, THE initramfs SHALL invoke the SigmaFS unlock path and mount the decrypted root volume at `/new_root`.
3. WHEN the root volume is mounted, THE initramfs SHALL perform a pivot-root to `/new_root` and exec `/sbin/sigmad` as PID 1.
4. IF the root volume unlock fails after three consecutive incorrect passphrase attempts, THEN THE initramfs SHALL drop to a recovery shell with a diagnostic message rather than looping indefinitely.


---

### Requirement 11: Device Management Subsystem

**User Story:** As a hardware engineer, I want a udev-equivalent device manager for SigmaOS, so that hardware events are detected, device nodes are created under `/dev`, and userland daemons are notified of hotplug changes automatically.

#### Acceptance Criteria

1. WHEN the Device_Manager starts, THE Device_Manager SHALL enumerate all devices currently present by reading the kernel device tree and creating corresponding nodes under `/dev` with correct permissions.
2. WHEN a new hardware device is connected, THE Device_Manager SHALL receive the kernel uevent, create the device node under `/dev`, and broadcast a hotplug event to all registered userland listeners within 500 milliseconds.
3. WHEN a hardware device is removed, THE Device_Manager SHALL remove the corresponding `/dev` node and broadcast a device-removed event within 500 milliseconds.
4. THE Device_Manager SHALL load firmware files from `/lib/firmware` for devices that request firmware via the kernel firmware loading interface.
5. THE Device_Manager SHALL apply device permission rules from `/etc/sigma/udev.rules.d/` specifying owner, group, and mode for matching device nodes.
6. IF a device requires a kernel module that is not loaded, THEN THE Device_Manager SHALL invoke module loading for the matching module and retry device initialization.

---

### Requirement 12: Networking Stack and Network Manager

**User Story:** As a network engineer, I want SigmaOS to manage network interfaces automatically via DHCP and provide a firewall rule engine, so that connected hardware obtains addresses and network connectivity is secured without manual configuration.

#### Acceptance Criteria

1. WHEN a network interface transitions to the link-up state, THE Net_Manager SHALL issue DHCP discovery on the interface and configure the assigned address, subnet mask, gateway, and DNS resolvers within 10 seconds.
2. THE Net_Manager SHALL support static interface configuration via declarative files in `/etc/sigma/network.d/` specifying address, prefix length, gateway, and DNS servers.
3. THE Net_Manager SHALL apply IPv4 packet filter rules from `/etc/sigma/firewall.rules` using the kernel netfilter interface on every interface bring-up.
4. WHEN a firewall rule is added or removed via the `signet firewall` command, THE Net_Manager SHALL apply the change to the running kernel netfilter state within 200 milliseconds without requiring an interface restart.
5. THE kernel TCP/IP stack SHALL correctly reassemble fragmented IPv4 and IPv6 packets before delivering them to the socket layer.
6. IF a DHCP lease expires, THEN THE Net_Manager SHALL attempt lease renewal at 50% of the lease lifetime and fall back to DHCP rediscovery if the renewal is rejected.


---

### Requirement 13: Graphics, Windowing, and Desktop Stack

**User Story:** As a desktop user, I want SigmaOS to provide a functioning Wayland compositor with DRM/KMS output, so that graphical applications render to a physical display through a standard, hardware-accelerated display pipeline.

#### Acceptance Criteria

1. WHEN SigmaOS boots on hardware with a DRM-capable GPU, THE DRM_Driver SHALL initialize the display pipeline, enumerate connected outputs, and set the preferred display mode within 5 seconds.
2. THE Compositor SHALL implement the core Wayland protocol (`wl_compositor`, `wl_surface`, `wl_shell`, `xdg_wm_base`) to allow Wayland client applications to create and display windows.
3. WHEN a Wayland client submits a new buffer, THE Compositor SHALL composite it to the display output within two display refresh cycles (≤ 33 milliseconds at 60 Hz).
4. THE Compositor SHALL route keyboard and pointer input events from evdev device nodes to the focused Wayland surface.
5. WHEN the primary display output is a 4K panel, THE Compositor SHALL support fractional scaling factors (1.25×, 1.5×, 2.0×) and render surfaces at the selected scale without clipping.
6. IF the DRM_Driver cannot set the requested display mode, THEN THE DRM_Driver SHALL fall back to the highest supported resolution and log the requested and actual modes.

---

### Requirement 14: Audio Stack

**User Story:** As a multimedia user, I want SigmaOS to play and capture audio through a mixing server that multiple applications share simultaneously, so that audio does not require exclusive hardware access per application.

#### Acceptance Criteria

1. THE Audio_Server SHALL expose a UNIX domain socket API at `/run/sigma-audio/server.sock` accepting connect, stream-open, write, read, and disconnect messages.
2. WHEN two applications simultaneously write PCM data to the Audio_Server, THE Audio_Server SHALL mix the streams and output the combined audio to the hardware device without audible glitches.
3. THE Audio_Server SHALL support PCM sample formats: S16LE, S24LE, S32LE, and F32LE, at sample rates of 44100 Hz, 48000 Hz, and 96000 Hz.
4. WHEN a client application opens an audio stream, THE Audio_Server SHALL begin playback within 20 milliseconds of the first write.
5. THE kernel audio driver SHALL support at minimum the Intel HDA controller and virtio-sound virtual device.
6. IF all hardware audio outputs are unavailable, THEN THE Audio_Server SHALL return an error to connecting clients identifying that no output device is present.


---

### Requirement 15: Secure Boot Chain

**User Story:** As a security-conscious operator, I want the SigmaOS boot chain to verify each stage cryptographically from UEFI to the kernel, so that any tampering with the bootloader or kernel is detected before execution.

#### Acceptance Criteria

1. THE sigma-boot bootloader SHALL be signed with the SigmaOS Secure Boot key and verifiable by UEFI Secure Boot firmware without requiring custom MOK enrollment on production hardware.
2. WHEN sigma-boot loads the SigmaOS kernel image, THE sigma-boot SHALL verify the kernel image's Ed25519 signature against the embedded SigmaOS release public key before transferring execution.
3. IF the kernel image signature verification fails, THEN THE sigma-boot SHALL halt the boot process, display a clear tamper-detection message, and not execute the kernel.
4. THE SigmaOS kernel SHALL verify the initramfs archive signature before extracting it into memory.
5. THE Secure_Boot_Chain SHALL seal the volume encryption key to the TPM PCR values corresponding to the UEFI firmware, sigma-boot, and kernel, so that the key is only released when all three components are in their known-good state.

---

### Requirement 16: Kernel Module Signing

**User Story:** As a security engineer, I want all loadable kernel modules to carry a cryptographic signature verified at load time, so that unsigned or tampered modules cannot be inserted into the running kernel.

#### Acceptance Criteria

1. THE SigmaOS kernel SHALL reject any kernel module whose embedded signature does not verify against the kernel module signing key at module load time.
2. THE SigmaOS module build system SHALL sign every compiled kernel module with the release module signing key during the build process.
3. WHEN the kernel is booted in module-signing enforcement mode, THE Kernel SHALL not load modules presented without a valid signature regardless of the caller's privilege level.
4. IF a module load is rejected due to signature failure, THEN THE Kernel SHALL write an audit event identifying the module path, the rejection reason, and the requesting process.

---

### Requirement 17: Mandatory Access Control Policy

**User Story:** As a security engineer, I want a MAC policy engine that enforces capability constraints on all processes independently of discretionary access control, so that a compromised process cannot exceed its declared security policy even if running as root.

#### Acceptance Criteria

1. THE MAC_Policy engine SHALL enforce subject-to-object access rules on filesystem operations (read, write, execute, unlink), IPC, and network socket creation for all processes on the system.
2. WHEN a process attempts an operation not permitted by its MAC label, THE MAC_Policy engine SHALL deny the operation, return the appropriate error to the caller, and record a policy violation event in the audit log.
3. THE MAC_Policy engine SHALL ship a default policy covering all SigmaOS system services with least-privilege labels that prevent any single compromised service from accessing another service's data.
4. THE MAC_Policy compiler SHALL parse MAC policy source files and produce a binary policy blob; for all valid policy source files, compiling and then loading the binary policy SHALL produce equivalent enforcement behavior as the source (round-trip property).
5. WHILE a process is running with a confined MAC label, THE MAC_Policy engine SHALL not allow that process to change its own label to a less-restrictive one.


---

### Requirement 18: TPM Attestation and Secret Sealing

**User Story:** As a security engineer, I want SigmaOS to use the TPM to attest platform state and seal secrets to measured boot values, so that cryptographic material is only accessible when the system is in a verified state.

#### Acceptance Criteria

1. WHEN SigmaOS boots with a TPM 2.0 device present, THE Kernel SHALL extend the TPM PCR chain with measurements of the kernel image, initramfs, and kernel command line during boot.
2. THE sigmad attestation service SHALL generate a TPM Quote signed by the Attestation Key and make it available to remote verifiers at a configurable endpoint.
3. WHEN a secret is sealed to the TPM with a PCR policy, THE TPM driver SHALL only release the secret when the current PCR values match the sealing policy.
4. IF the TPM is not present or not functional, THEN THE SigmaOS security subsystem SHALL fall back to software-based key storage with a warning logged to the system journal, and SHALL NOT silently disable attestation features.

---

### Requirement 19: Cross-Toolchain and Reproducible Builds

**User Story:** As a build engineer, I want the SigmaOS toolchain to produce bit-for-bit reproducible build artifacts when given identical source inputs, so that any party can independently verify that distributed binaries match published source code.

#### Acceptance Criteria

1. THE Toolchain build scripts SHALL produce SigmaOS kernel and userland binaries where the SHA-256 hash of each artifact is identical across two independent builds from the same source commit on the same architecture.
2. THE CI_Pipeline SHALL execute a reproducibility verification step that builds the same commit twice in separate environments and fails the pipeline if any artifact hash differs.
3. THE Toolchain SHALL document the exact versions of the Rust compiler, LLVM backend, and linker required to reproduce each release, pinned in a `toolchain.toml` file at the repository root.
4. WHEN cross-compiling SigmaOS for a non-native host, THE Toolchain SHALL produce a functionally correct binary without requiring the target architecture hardware.
5. THE Toolchain documentation SHALL describe all steps required to bootstrap the cross-compilation environment from a standard Debian or Fedora Linux host.


---

### Requirement 20: System Shells

**User Story:** As a developer, I want a POSIX-compliant shell and an interactive Rust-native shell available on SigmaOS, so that I can write portable shell scripts and use an ergonomic interactive command line.

#### Acceptance Criteria

1. THE SigmaOS base userland SHALL include a POSIX sh-compatible shell that executes shell scripts conforming to the POSIX.1-2017 Shell Command Language specification.
2. WHEN the POSIX shell executes a script file, THE shell SHALL interpret variable expansion, command substitution, pipelines, redirection, and control flow constructs as specified by POSIX.1-2017.
3. THE SigmaOS base userland SHALL include an interactive shell with syntax highlighting, history search, and tab completion for filesystem paths and installed commands.
4. WHEN the interactive shell is launched on a terminal, THE shell SHALL initialize from `/etc/sigma/shell.rc` and the user's `~/.sigmarc` configuration file.

---

### Requirement 21: Developer SDK

**User Story:** As an application developer, I want the Sigma_SDK to provide build tooling, API documentation, a manifest schema, and a working template project, so that I can develop, test, and publish a SigmaOS application without requiring access to project maintainers.

#### Acceptance Criteria

1. THE Sigma_SDK SHALL include a `sigma-new` command that scaffolds a new application project from a template, producing a compilable project with a valid manifest, a build script, and a README.
2. THE Sigma_SDK SHALL include API documentation covering all public stable APIs with function signatures, parameter descriptions, return types, and at least one usage example per function.
3. THE Sigma_SDK manifest schema SHALL be expressed as a JSON Schema document; WHEN an application manifest is validated against the schema, THE Sigma_SDK validator SHALL report all violations with field paths and human-readable error messages.
4. WHEN `sigma-sdk build` is invoked in a valid project directory, THE Sigma_SDK SHALL produce a sigpkg archive ready for submission to the sigma_pkg_registry.
5. THE Sigma_SDK SHALL include a local emulation mode allowing applications to run against a simulated SigmaOS API surface on a Linux development host without a physical SigmaOS installation.


---

### Requirement 22: Virtualization — virtio Maturity

**User Story:** As a virtualization engineer, I want the SigmaOS kernel's virtio drivers to be fully functional for block, network, and console devices, so that SigmaOS runs reliably as a guest on KVM and QEMU without device I/O errors.

#### Acceptance Criteria

1. THE SigmaOS virtio-blk driver SHALL correctly handle all virtqueue operations (add buffer, notify device, process used ring) for block read and write requests without data corruption.
2. THE SigmaOS virtio-net driver SHALL send and receive Ethernet frames at line rate up to 1 Gbps virtual NIC speed without dropped frames under sustained load.
3. WHEN SigmaOS boots as a QEMU guest with a virtio-console device, THE Kernel SHALL expose the console as a TTY device accessible to the init system and userland.
4. THE virtio driver implementations SHALL handle device reset and re-initialization gracefully without kernel panics when the hypervisor performs a device reset.
5. THE CI_Pipeline SHALL include a QEMU boot test that verifies SigmaOS reaches the sigmad default target within 60 seconds when launched as a QEMU KVM guest.

---

### Requirement 23: OCI Container Runtime

**User Story:** As a developer, I want to run OCI-compliant container images on SigmaOS, so that I can use containerized workloads and existing container ecosystem tooling on the platform.

#### Acceptance Criteria

1. THE OCI_Runtime SHALL implement the OCI Runtime Specification 1.0 lifecycle operations: `create`, `start`, `kill`, `delete`, and `state`.
2. WHEN a container is created from a valid OCI image bundle, THE OCI_Runtime SHALL set up a filesystem namespace (overlayfs), PID namespace, network namespace, and cgroup limits as specified in the bundle's `config.json`.
3. WHEN a container process exits, THE OCI_Runtime SHALL collect its exit code, tear down all namespaces, release cgroup resources, and report the exit status within 2 seconds.
4. IF an OCI bundle's `config.json` specifies a Linux capability to drop, THEN THE OCI_Runtime SHALL remove that capability from the container process's bounding set before exec.
5. THE OCI_Runtime SHALL be compatible with the `sigma-ctr` command-line client and the containerd shim v2 protocol for integration with higher-level container tooling.


---

### Requirement 24: Installer and Live Image Tooling

**User Story:** As a new user, I want a guided installer that partitions a target disk, installs SigmaOS, configures the bootloader, and collects locale and user settings, so that I can set up SigmaOS on bare metal without reading low-level documentation.

#### Acceptance Criteria

1. THE Installer SHALL guide the user through selecting a target disk, choosing between automatic and manual partitioning, setting a hostname, creating an initial user account, and selecting locale and timezone.
2. WHEN the user confirms the installation plan, THE Installer SHALL partition the target disk, format the partitions, write the OS image, and install sigma-boot to the EFI System Partition within a time proportional to the disk write speed.
3. IF any step of the disk write fails, THEN THE Installer SHALL display a clear error message, roll back any partial writes, and allow the user to retry or abort.
4. THE Installer SHALL produce bootable live ISO images that allow the user to try SigmaOS from RAM before committing to installation.
5. THE release build pipeline SHALL produce signed live ISO images with a detached Ed25519 signature published alongside the ISO; the CI_Pipeline SHALL verify the signature as part of the release step.

---

### Requirement 25: Signed Release Pipeline

**User Story:** As a distribution consumer, I want every SigmaOS release artifact to carry a verifiable cryptographic signature tied to the project's release key, so that I can confirm the authenticity and integrity of what I download.

#### Acceptance Criteria

1. WHEN the CI_Pipeline runs a release build for a version tag, THE CI_Pipeline SHALL sign every produced artifact (ISO, kernel image, initramfs, sigpkg index) with the SigmaOS release Ed25519 key stored in the CI secrets vault.
2. THE release artifacts SHALL be published to the project release page accompanied by a detached `.sig` file for each artifact.
3. WHEN a user verifies a release artifact using `sigverify`, THE sigverify tool SHALL confirm the signature against the project's published public key and report pass or fail with the artifact path and key fingerprint.
4. THE CI_Pipeline SHALL never embed the private signing key in repository source code, build scripts, or CI configuration files.


---

### Requirement 26: Accessibility Subsystem

**User Story:** As a user with visual or motor impairments, I want SigmaOS to provide screen reader support and keyboard-only navigation for all built-in applications, so that the platform is usable without relying on a pointer device or visual output alone.

#### Acceptance Criteria

1. THE A11y_Subsystem SHALL expose an AT-SPI2-compatible accessibility bus that screen reader applications can query for the widget tree, focus state, and text content of any SigmaOS application.
2. WHEN a SigmaOS built-in application renders a UI element with interactive function, THE application SHALL provide an accessible name, role, and state through the A11y_Subsystem.
3. THE A11y_Subsystem SHALL deliver focus-change and text-change events to registered screen readers within 100 milliseconds of the corresponding UI state change.
4. THE Compositor SHALL support a high-contrast rendering mode activatable from the accessibility settings panel, which inverts or replaces all theme colors with a high-contrast palette.
5. WHEN the user activates keyboard navigation mode, THE Compositor SHALL ensure that all interactive UI elements in every built-in application are reachable via Tab and arrow key navigation without requiring a pointer device.

---

### Requirement 27: Localization Infrastructure

**User Story:** As an international user, I want SigmaOS to display the UI in my preferred locale and correctly render locale-specific text, dates, and numbers, so that the system is usable without knowledge of English.

#### Acceptance Criteria

1. THE SigmaOS base system SHALL include locale data for at minimum: en_US, de_DE, fr_FR, ja_JP, zh_CN, and es_ES, covering character encodings, collation, date/time formatting, and number formatting.
2. WHEN the user's locale is set, THE system SHALL render all built-in application UI strings in the configured locale if a translation is available, falling back to en_US for untranslated strings.
3. THE localization build pipeline SHALL extract all user-visible strings from built-in applications into `.po` translation files that translators can update without modifying application source code.
4. WHEN the locale is set to a right-to-left language, THE Compositor and built-in applications SHALL mirror the UI layout to present a correct right-to-left reading order.


---

### Requirement 28: Subsystem Documentation and Manpages

**User Story:** As a contributor or system operator, I want every SigmaOS subsystem to have architecture documentation and every command-line tool to have a manpage, so that I can understand and operate the system without reading source code.

#### Acceptance Criteria

1. THE SigmaOS documentation SHALL include an architecture document for each of the following subsystems: kernel memory manager, scheduler, SigmaVFS, networking stack, security model, sigmad service manager, and Compositor.
2. EVERY SigmaOS command-line tool SHALL include a manpage in section 1 or section 8 covering the tool's synopsis, description, options, environment variables, exit codes, and at least two usage examples.
3. THE CI_Pipeline SHALL verify that every executable installed to `/usr/bin` or `/sbin` has a corresponding manpage entry, failing the build if any tool is missing its manpage.
4. THE contributor onboarding documentation SHALL describe the development environment setup, repository layout, coding conventions, and the process for submitting a pull request in sufficient detail for a developer unfamiliar with SigmaOS to make their first contribution.

---

### Requirement 29: CI/CD and Testing Infrastructure

**User Story:** As a project maintainer, I want a comprehensive CI/CD pipeline that builds, boots, and tests SigmaOS on every commit across multiple hardware configurations, so that regressions are caught before they reach the main branch.

#### Acceptance Criteria

1. THE CI_Pipeline SHALL build the SigmaOS kernel and base userland on every pull request and report pass/fail status on the PR within 15 minutes.
2. THE CI_Pipeline SHALL execute QEMU boot tests that launch SigmaOS as a QEMU KVM guest and verify the system reaches the sigmad default target without a kernel panic, on every PR targeting the main branch.
3. THE CI_Pipeline SHALL run the reproducible build verification step (as specified in Requirement 19) on every release tag.
4. THE CI_Pipeline SHALL run the sigma_pkg round-trip test (parse → serialize → parse equivalence) on every change to the sigma-pkg/ directory.
5. THE CI_Pipeline SHALL run the sigma_libc conformance test suite on every change to the userland/ directory.
6. WHERE a fuzzing target is defined for a subsystem, THE CI_Pipeline SHALL execute the fuzzer for at least 60 seconds per target in CI and fail the pipeline if the fuzzer finds a crash.
7. WHEN a CI_Pipeline job fails, THE CI_Pipeline SHALL produce a structured failure report identifying the failing job name, the failing test or build step, and a link to the full log.
8. THE CI_Pipeline SHALL support hardware-in-the-loop testing on at least one physical x86_64 machine via a self-hosted runner, executing a smoke test suite that verifies boot, network connectivity, and package installation.


---

### Requirement 30: Formal Verification and Fuzzing Integration

**User Story:** As a security engineer, I want critical SigmaOS subsystems to have property-based and fuzzing test coverage integrated into CI, so that input-space edge cases and memory safety bugs are discovered automatically before affecting users.

#### Acceptance Criteria

1. THE sigma_libc allocator SHALL have a property-based test suite verifying: allocations of arbitrary size return non-overlapping regions, free followed by malloc does not crash, and the allocator does not leak memory across a sequence of allocations and frees.
2. THE sigpkg parser SHALL have a property-based fuzz corpus; WHEN the fuzzer supplies arbitrary byte sequences as sigpkg input, THE Pkg_Client parser SHALL not crash, panic, or invoke undefined behavior — it SHALL return a structured parse error.
3. THE kernel TCP/IP stack SHALL have a fuzzing target that accepts arbitrary byte sequences as incoming network packets; WHEN the fuzzer supplies malformed or truncated packets, THE kernel TCP/IP stack SHALL handle them without a kernel panic.
4. THE MAC_Policy compiler SHALL have a property-based test verifying that compiling a valid policy and loading it produces the same enforcement decisions as evaluating the source policy directly (compiler correctness property).
5. THE CI_Pipeline SHALL run each fuzzing target in a sanitizer-enabled build (AddressSanitizer, UndefinedBehaviorSanitizer) to surface memory safety violations that would otherwise go undetected.

---

## 90-Day Priority Deliverables

The following requirements are designated as 90-day priority targets. Progress on these items SHALL be tracked in the project milestone tracker and reported in each sprint review.

| Priority | Requirement | Deliverable |
|----------|-------------|-------------|
| P0 | Requirement 1 | sigpkg format specification v1.0 finalized and documented |
| P0 | Requirement 2 | sigma_pkg_registry API v1 endpoint live in staging |
| P0 | Requirement 3 | Minimal Pkg_Client supporting install and verify commands |
| P0 | Requirement 4 | sigma_libc expanded to cover full C11 + POSIX.1-2017 thread/file subset |
| P0 | Requirement 7 | Basic Service_Unit spec finalized; sigmad supervisor boots to default target |
| P1 | Requirement 29 | QEMU boot tests and reproducible build checks running on every PR |
| P1 | Requirement 19 | Toolchain reproducibility verified in CI |
| P1 | Requirement 9 | mkfs.sigmafs and fsck.sigmafs tools functional |
| P2 | Requirement 10 | initramfs unlock and pivot-root functional |
| P2 | Requirement 11 | Device_Manager creates /dev nodes for enumerated hardware |

