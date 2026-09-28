# Security

SigmaOS provides comprehensive security features including sandboxing, encryption, and access control.

## Security Model

### Defense in Depth

SigmaOS implements multiple layers of security:

1. **Capability-based Security**: Fine-grained resource permissions
2. **Sandboxing**: Pledge/unveil for process isolation
3. **Kernel Hardening**: Kernel pointer restriction, stack protection
4. **Filesystem Encryption**: Per-directory encryption with fscrypt
5. **Network Security**: Firewall, VPN, secure protocols

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

### Kyber-1024

Use post-quantum encryption:

```bash
# Generate Kyber keypair
sigpqc keygen kyber1024

# Encrypt file
sigpqc encrypt file.txt --algorithm kyber1024

# Decrypt file
sigpqc decrypt file.txt.enc --algorithm kyber1024
```

### Dilithium-5

Use post-quantum signatures:

```bash
# Generate Dilithium keypair
sigpqc keygen dilithium5

# Sign file
sigpqc sign file.txt --algorithm dilithium5

# Verify signature
sigpqc verify file.txt.sig --algorithm dilithium5
```

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

## Next Steps

- [Desktop](08-Desktop.md) - Desktop security settings
- [Packaging](09-Packaging.md) - Package security and signing
- [Kernel](04-Kernel.md) - Kernel security features
