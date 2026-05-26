# 🛡 Security Architecture

SigmaOS takes a radical approach to security. By eschewing POSIX compliance, we inherently block the thousands of CVEs associated with decades-old API surfaces.

## 1. Sovereign Cryptographic Primitives

SigmaOS avoids large external libraries like OpenSSL or libsodium, which are prone to supply-chain attacks and buffer overflows. 
- **`sigma_sha256.cpp`**: Bare-metal SHA-256 implementation adhering strictly to FIPS 180-4. Features 64-byte block size transformations and padding rules natively.
- **`sigma_aes.cpp`**: Bare-metal AES-256 implementation adhering to FIPS 197. Features 14-round block encryption/decryption.

*These primitives form the bedrock for higher-level security features like Secure Boot verification, disk encryption, and SSH handshakes.*

## 2. Mandatory Access Control (MAC)

SigmaOS drops traditional Discretionary Access Control (DAC) like `chmod` numerical bits for core operations, preferring a strict Mandatory Access Control.

### `sigma_mac.cpp`
Absorbing the design principles of SELinux and AppArmor, `sigma_mac` assigns explicit contexts to both subjects (processes) and objects (files/sockets).

**Built-in Contexts:**
- `SEC_CONTEXT_SYSTEM (0x0001)`: Equivalent to Ring 0 / root.
- `SEC_CONTEXT_USER (0x0002)`: Standard unprivileged user.
- `SEC_CONTEXT_GUEST (0x0004)`: Ephemeral, volatile user.
- `SEC_CONTEXT_RESTRICT (0x0008)`: Locked-down network/file access.

## 3. Privilege Escalation
We provide `sigma_sudo` and `sigma_su` to elevate a process's security context gracefully, mediated directly through the MAC checking system rather than the vulnerable `setuid` binary bits.
