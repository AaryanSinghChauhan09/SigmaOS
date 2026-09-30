# Security

This page includes planned interfaces and compatibility models. It is not a claim
of complete security or Linux/BSD parity. Check each component's implementation
and runtime status before relying on it.

## Security Model

### Defense in Depth

SigmaOS implements multiple layers of security:

1. Capability APIs and policy models
2. Pledge/unveil compatibility models
3. Kernel hardening mechanisms, some not wired to runtime enforcement
4. Filesystem encryption interfaces; audited providers are not integrated
5. Network security models, which require runtime and protocol review

`src/security/kali_stack.rs` contains in-process models, not host PAM, sudo, iptables, swap, or kernel dmesg enforcement. Its password-authentication provider is unavailable and sudo must deny elevation; its firewall model uses first-match, default-deny behavior but is not attached to a host packet path. Do not rely on these types as operating-system security controls.

AI agents maintaining this file must preserve fail-closed authentication, protocol length validation, first-match default-deny firewall behavior, synchronized log writes, and atomic swap-capacity accounting. Do not reintroduce credential comparisons as authentication without an audited provider.

## Pledge/Unveil Sandbox

### Pledge

Restrict process capabilities:

```bash
# Run application with pledge
pledge unveil /path/to/app -exec "app arguments"

# Grant specific promises
pledge unveil /path/to/app "rpath wpath cpath stdio"
```

### Unveil

Restrict file access:

```bash
# Restrict file access
unveil /path/to/app
unveil /data/write

# Execute with unveil
unveil -r /path/to/app -w /data/write app
```

## Capability-based Security

### Grant Capabilities

Grant specific capabilities to processes:

```bash
# Grant read capability
sigcaps grant process-name read:/etc/config

# Grant network capability
sigcaps grant process-name network:connect

# List capabilities
sigcaps list process-name
```

### Revoke Capabilities

Revoke capabilities:

```bash
# Revoke capability
sigcaps revoke process-name read:/etc/config
```

## Filesystem Encryption

### fscrypt

Encrypt directories:

```bash
# Encrypt directory
fscrypt encrypt /home/user/sensitive

# Lock directory
fscrypt lock /home/user/sensitive

# Unlock directory
fscrypt unlock /home/user/sensitive
```

### LUKS

Encrypt block devices:

```bash
# Format encrypted device
cryptsetup luksFormat /dev/sda1
cryptsetup luksOpen /dev/sda1 cryptdata
mkfs.ext4 /dev/mapper/cryptdata
mount /dev/mapper/cryptdata /mnt/encrypted
```

## Kernel Security

### Kernel Pointer Restriction

Restrict kernel pointer exposure:

```bash
# Set kptr_restrict
sysctl kernel.kptr_restrict=2

# Set dmesg_restrict
sysctl kernel.dmesg_restrict=1
```

### Module Loading Control

Control kernel module loading:

```bash
# Disable module loading
sysctl kernel.modules_disabled=1

# Enable module loading
sysctl kernel.modules_disabled=0
```

## Firewall

### Configure Firewall

Set up firewall rules:

```bash
# Enable firewall
sigma-firewall enable

# Allow SSH
sigma-firewall allow 22/tcp

# Deny all incoming
sigma-firewall default deny

# List rules
sigma-firewall list
```

### Service Configuration

Configure firewall for services:

```toml
# /etc/sigmaos/firewall.toml
[firewall]
default_policy = "deny"

[[firewall.rules]]
service = "ssh"
port = 22
action = "allow"

[[firewall.rules]]
service = "http"
port = 80
action = "allow"
```

## Post-Quantum Cryptography

No audited ML-KEM or ML-DSA provider is integrated. The APIs in
`src/crypto/post_quantum.rs` now fail closed rather than emit placeholder keys,
signatures, ciphertexts, or shared secrets. Other PQC-named modules remain
prototypes and must not be used for authentication, key exchange, package
verification, or production encryption. Commands for key generation, signing,
and encryption are not available until a reviewed provider and executable
integration exist.

## Authentication and randomness

The standalone `src/crypto/primitives.rs` SHA-256 implementation is only for
content digests; its empty, short, and padding-boundary vectors are tested.
Its AES-256-shaped and random-key APIs return `ProviderNotIntegrated` and do
not modify caller buffers until a vetted provider is connected. Its xorshift
generator is deterministic simulation state, not secure randomness. The
`xor_bytes` helper rejects unequal input/output lengths rather than indexing
past a buffer.

`SafeInstaller` does not store a simulated password hash. Account creation in
dry-run mode is only a simulation; real account creation returns an error
until an audited password-hashing provider is integrated.
The GUI installer account request also discards its password argument and
does not retain plaintext credentials; it cannot create a real account without
the same trusted host provider.

The emergency shell gate does not keep a plaintext password or accept a
signature based on fixed magic bytes. Its password and signature checks remain
unavailable until vetted verification providers are integrated, so shell
access fails closed.

The Fedora Cockpit and FreeIPA compatibility models do not authenticate
sessions or mint Kerberos tickets without trusted Cockpit/KDC integrations.
They reject authentication and token verification rather than treating any
nonempty token as proof of identity.

LDAP bind and package-repository LDAP/PAM compatibility methods also deny
access until a trusted credential provider is integrated. These models must
not be used as substitutes for host authentication services.

Offline SSSD credential caching, FreeIPA identity authentication, Fedora SSSD
ticket checks, Mint keyring prompts, and Warpinator peer authentication/file
sharing are likewise unavailable without audited password hashing or trusted
directory and authenticated transport providers. Policy checks and nonempty
passwords, PINs, or ticket bytes do not authenticate an identity.

The PAM compatibility engine no longer treats a process environment value as
a credential, and the PIA VPN model does not turn caller input into an API
token or report a connected tunnel. Both remain unavailable until real
providers and authenticated transport are integrated.

The WireGuard and OpenVPN adapters also reject connection attempts because
they do not implement handshakes, key exchange, packet protection, or tunnel
setup. Their previous success responses were simulated and must not be read
as a secure VPN connection. The unavailable WireGuard adapter and Warpinator
model no longer retain the private-key strings or PIN supplied to their
constructors.

The exported PAM model currently fails closed: without a secure random source
and an audited password hashing provider, it will not register users or
authenticate credentials. `security::crypto_utils::SecureRandom` and its
password-hash placeholder return errors. `crypto::random::SimpleRandomGenerator`,
`SimpleCSPRNG`, `klib::rand` secure helpers, and `klib::rng::OsRng` also fail
closed. `klib::rng::SigmaRng` and `XorShiftRng` are deterministic simulation
generators only. Timestamps, raw hardware values, and simple state-mixing
routines are not substitutes for a CSPRNG. The `crypto::hash` and `crypto::kdf`
compatibility APIs likewise return provider-unavailable errors; their names do
not mean SHA-256, HMAC, HKDF, PBKDF2, or password hashing are implemented there.
This PAM model is not a replacement for the host operating system's PAM or
account database.

The lightweight LDAP bind model and package-repository LDAP/PAM authentication
helper also deny access until trusted providers are integrated. They do not
validate credentials or replace host LDAP/PAM services.

## Audit and Logging

### System Audit

View system audit logs:

```bash
# View audit log
sigaudit log

# View security events
sigaudit events security

# View file access
sigaudit access /etc/passwd
```

### Configure Audit

Configure audit rules:

```toml
# /etc/sigmaos/audit.toml
[audit]
log_file = "/var/log/audit.log"
log_level = "info"

[[audit.rules]]
event = "file_access"
path = "/etc/shadow"
action = "log"
```

## Secure Boot

### Secure Boot Configuration

Configure Secure Boot:

```bash
# Enable Secure Boot
sigboot enable

# Enroll keys
sigboot enroll-key /path/to/key.der

# Verify boot process
sigboot verify
```

## Capability-Based Security

### Capability Enforcement

Linux capabilities and BSD Capsicum-inspired access control:

```bash
# Create security context
cap create-context

# Grant capability
cap grant 1 chown

# Revoke capability
cap revoke 1 chown

# Check access
cap check 1 file write

# Promote capability
cap promote 1 dac_override
```

### Capabilities

- **Chown**: Change file ownership
- **DacOverride**: Override DAC permissions
- **Kill**: Send signals to processes
- **NetBindService**: Bind privileged ports
- **NetAdmin**: Network administration
- **SysAdmin**: System administration
- And many more Linux capabilities

### Resource Types

- **File**: File access (read, write, execute)
- **Socket**: Socket operations (bind, connect, listen)
- **Process**: Process operations (delete, signal)
- **Network**: Network operations
- **System**: System-level operations

## Address Sanitizer

### ASan Memory Error Detection

ASan-inspired memory error detection:

```bash
# Create address sanitizer
asan create 16

# Allocate memory with redzones
asan allocate 100

# Check valid access
asan check 0x1000 50

# Free memory
asan free 0x1000

# Get shadow memory
asan shadow 0x1000
```

### Detection Capabilities

- **Buffer Overflow**: Redzone-based overflow detection
- **Use-After-Free**: Freed memory tracking
- **Stack Corruption**: Canary verification
- **Shadow Memory**: Memory state tracking

## Bluetooth GATT

### GATT Client

Linux BlueZ-inspired GATT implementation:

```bash
# Connect to device
gatt connect "00:11:22:33:44:55" "Test Device"

# Discover services
gatt discover-services 1

# Discover characteristics
gatt discover-characteristics 1 1

# Read characteristic
gatt read 1 1

# Write characteristic
gatt write 1 1 "data"

# Disconnect
gatt disconnect 1
```

### GATT Operations

- **Service Discovery**: Discover available services
- **Characteristic Discovery**: Discover service characteristics
- **Read**: Read characteristic value
- **Write**: Write characteristic value

## Next Steps

- [Desktop](08-Desktop.md) - Desktop security settings
- [Packaging](09-Packaging.md) - Package security and signing
- [Kernel](04-Kernel.md) - Kernel security features

## Seccomp-BPF filter compiler

`src/security/seccomp_filter.rs` contains a classic BPF filter compiler for syscall numbers and up to six 64-bit arguments. Compilation returns an error for an invalid argument index or a program larger than the classic BPF instruction limit. The generated program must still be validated and installed by the platform's syscall boundary before it can enforce a policy; compiling a filter alone does not sandbox a running process.

SigmaPkg currently computes SHA-256 digests for content integrity, but its signature verifier and signing service have no vetted cryptographic provider. They return `CryptoUnavailable` or an empty signature and reject signed metadata; a trusted key name or matching checksum alone is not proof of authenticity. Do not use these APIs to approve packages or updates until real signature verification and end-to-end trust-chain checks are integrated.

The API-shaped compatibility layer in `src/crypto/libsodium.rs`, `src/crypto/aes.rs`, `src/crypto/encryption.rs`, `src/crypto/postquantum.rs`, the vault adapters in `src/security/vault.rs`, the secret manager in `src/security/secrets.rs`, and the PQC routines in `src/crypto/pqc_dilithium.rs` are not audited production implementations. They must not protect real data, credentials, updates, or network sessions. In `libsodium.rs`, `sodium_init` reports unavailable, and cryptographic operations return `ProviderNotIntegrated`; those APIs do not implement libsodium algorithms. The PQC/HKDF and FDE placeholders likewise return provider-unavailable errors. AES-shaped, XOR-based, vault, and secret encryption APIs also fail closed until audited providers are integrated. The `src/crypto/aegis_vault.rs` key derivation, encryption, and decryption entry points now return `CryptoProviderUnavailable`; its compression helpers are separate and are not cryptographic. Aegis decompression rejects output above 64 MiB. The secret manager can still hold in-memory plaintext and is not secure storage. The separate `src/crypto/advanced_encryption_standard.rs` file is not wired into the crypto module and contains simulated transformations.

The clipboard's XOR strategy also fails closed. The default clipboard mode is explicitly plaintext (`SecurityLevel::None`) and does not label copied text as encrypted. Selecting an encryption level without an audited provider returns an error.

Cross-distro authentication dispatch also fails closed: `SovereignSystemdHomedAuthBridge` has no trusted credential backend and cannot authenticate users or mount home directories. Do not count it as an available authentication feature until a provider validates credentials and the mount path has end-to-end tests.

Maintain this component by checking opcode encodings, forward jump targets, native-endian argument word offsets, 64-bit comparisons, and default-action behavior together. Run `cargo test --lib security::seccomp_filter::tests` and `cargo check --lib` after edits. Do not weaken the default action or claim runtime enforcement unless the kernel integration path and its end-to-end checks are present.

## Documentation status

Commands shown on this page describe intended interfaces unless the corresponding executable or syscall integration exists in the current repository. Verify command names and runtime behavior before documenting them as available. Security claims must identify whether they are implemented, prototype-only, or planned. Never add real keys, passwords, salts, nonces, or other secret material to examples or source files.

## See also

- [AI Agent Guidelines](13-Agents.md) - Component ownership and maintenance workflow
- [Kernel](04-Kernel.md) - Syscall integration points
