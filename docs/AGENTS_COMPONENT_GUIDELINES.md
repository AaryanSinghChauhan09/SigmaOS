# SOVEREIGN AI AGENT COMPONENT GUIDELINES & COMPONENT SUBSYSTEM SPECIFICATION

## 1. Executive Overview & Architectural Intent

Autonomous AI engineering agents in **SigmaOS** operate within a Genode-inspired recursive component tree hierarchy (`src/kernel/component.rs`). Subsystems, drivers, userland services, and AI runtimes are isolated as discrete, capability-bounded components rather than monolithic, unconstrained binaries.

This specification details mandatory component management protocols, Genode-style parent-child resource delegation, FreeBSD Capsicum file descriptor rights, OpenBSD Pledge/Unveil path scoping, Fedora AppStream/modulemd component grouping, and component lifecycle state machines.

---

## 2. Genode-Style Hierarchical Component Architecture

Component ownership in SigmaOS follows a strict tree hierarchy rooted at Component ID 0 (`root`):

```
                       +-------------------+
                       | Root Component 0  |
                       +---------+---------+
                                 |
         +-----------------------+-----------------------+
         |                                               |
         v                                               v
+------------------+                           +------------------+
| Network Shard    |                           | Userland Shard   |
| Component        |                           | Component        |
+--------+---------+                           +--------+---------+
         |                                               |
         v                                               v
+------------------+                           +------------------+
| Capsicum Sandbox |                           | Runit Service    |
| Component        |                           | Supervisor       |
+------------------+                           +------------------+
```

### 2.1 Parent-Child Delegation Invariants
1. **Child Creation Privilege**: A parent component can only spawn child components if it possesses the `can_create_child` capability right (`CapabilityRights`).
2. **Resource Proportionality**: Resource allocations (Memory, CPU time, I/O ports, IRQs, DMA channels) propagate downward from parent to child via `propagate_resource_limits()`. No child component may consume resources exceeding its parent's quota envelope.
3. **Capability Delegation**: Parent components delegate capabilities to child components with equal or reduced rights. Rights escalation across delegation boundaries is strictly prohibited.

---

## 3. Linux & BSD Distro-Inspired Security & Isolation

### 3.1 FreeBSD Capsicum Sandboxing (`FreeBsdCapsicumComponentSandbox`)
Components receiving open file descriptors enter capability mode (`enter_capability_mode()`), stripping ambient process privileges. Rights on individual file descriptors are restricted to explicit bitmasks (`limit_fd_rights(fd, rights_mask)`).

### 3.2 OpenBSD Pledge & Unveil Path Scoping (`OpenBsdPledgeUnveilComponentSandboxEngine`)
* **System Call Promises**: Components drop system call promises using `pledge("stdio rpath")`.
* **Filesystem Scoping**: File paths are unveiled with explicit access permissions (`unveil("/etc/sigmaos", "r")`). Accessing non-unveiled filesystem locations returns permission errors.

### 3.3 Fedora AppStream & Modularity (`FedoraModulemdComponentEngine`)
* **Component Grouping**: Software packages are grouped via Fedora `comps.xml` definitions (`Mandatory`, `Default`, `Optional`, `Conditional`).
* **Stream Switching**: Components support multi-stream version selection (`ModulemdStream`) enabling parallel component runtimes (e.g., Node.js 18 vs Node.js 20).
* **Automated Rollback Engine**: CoreOS update health checks and automatic rollback (`FedoraCoreosAutomatedRollbackEngine`).
* **Koji & Bodhi Integration**: RPC build clients (`FedoraKojiBuildSystemClientEngine`) and update feedback / karma submission (`FedoraBodhiUpdateFeedbackEngine`).
* **Automated Packaging**: Spec file generation and RPM packaging engine (`FedoraPaugusAutomatedPackagingEngine`).
* **OSTree Sysroot Staging**: rpm-ostree deployment sysroot, pending deployment commit, and bootloader entry staging (`FedoraOstreeSysrootStagingEngine`).
* **SSSD Kerberos Realm**: Active Directory / FreeIPA Kerberos ticket granting service client (`FedoraSssdKerberosRealmClientEngine`).
* **PipeWire WirePlumber Policy**: Audio session wireplumber Lua policy & device node routing governor (`FedoraPipewireWireplumberPolicyGovernor`).
* **RPM Seccomp Syscall Filter**: Post-install Seccomp BPF syscall filter validator (`FedoraRPMSeccompFilterEngine`).

### 3.4 Arch Linux Parity & Keyring Components (`src/compatibility/arch_linux.rs`)
* **GnuPG Keyring & Trust Database**: Web of Trust keyring initialization, key import, and revocation (`ArchKeyringTrustDatabaseEngine`).
* **Reflector Mirrorlist Ranking**: Dynamic mirror ranking by download speed, country code, and completion rate (`ArchReflectorMirrorlistEngine`).
* **Archiso Bootstrap Generator**: Headless chroot bootstrap TAR and profile builder (`ArchisoBootstrapGeneratorEngine`).
* **AUR v5 RPC Client**: JSON RPC API client and dependency tree solver (`ArchUserRepositoryRpcClientEngine`).

---

## 4. Component Lifecycle State Machine

Component state transitions are monitored by Void Linux runit-inspired service supervisors (`VoidRunitComponentServiceEngine`):

```
+----------------+       +----------------+       +----------------+
|    Created     | ----> |    Running     | ----> |   Suspended    |
+----------------+       +-------+--------+       +-------+--------+
                                 |                        |
                                 v                        v
                         +---------------+        +---------------+
                         |   Destroyed   | <----- |     Down      |
                         +---------------+        +---------------+
```

1. **Created**: Component allocated in `ComponentTree` with assigned parent and capability space.
2. **Running**: Executable memory mapped and scheduled with active process ID.
3. **Suspended**: Execution context paused; memory limits held in passive state.
4. **Down / Destroyed**: Component terminated; child tree recursively cleaned up and resources returned to parent pool.

---

## 5. Implementation Checklist for AI Agents

When authoring or modifying components in SigmaOS, AI agents must verify:

- [ ] Component registered in `ComponentTree` under valid parent ID.
- [ ] Sandboxing enforced via Capsicum FD limits or OpenBSD Pledge/Unveil.
- [ ] Memory and CPU allocations explicitly delegated without exceeding parent quotas.
- [ ] Capabilities assigned with least privilege (`can_write` and `can_delegate` disabled by default).
- [ ] Unit tests added under `src/kernel/component.rs` and verified via `rustc --test`.
