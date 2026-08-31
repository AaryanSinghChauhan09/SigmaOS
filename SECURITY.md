# SigmaOS Security Hardening Guide

## Table of Contents

1.  [Security Overview](#security-overview)
2.  [System Hardening](#system-hardening)
3.  [Network Security](#network-security)
4.  [Application Security](#application-security)
5.  [Kernel Security](#kernel-security)
6.  [Audit and Monitoring](#audit-and-monitoring)
7.  [Incident Response](#incident-response)

## Security Overview

SigmaOS is designed with security as a core principle, inspired by OpenBSD's "secure by default" philosophy. This guide covers essential security hardening steps for production deployments.

### Security Features

*   **Post-quantum cryptography**: Kyber-1024, Dilithium-5
*   **Mandatory Access Control**: SELinux-inspired policies
*   **Sandboxing**: Application containers
*   **ASLR**: Address space layout randomization
*   **Stack protection**: Stack canaries and DEP
*   **Secure boot**: UEFI secure boot support

## System Hardening

### Enable Security Features

```bash
# Enable ASLR
sysctl -w kernel.randomize_va_space=2
echo "kernel.randomize_va_space=2" >> /etc/sysctl.conf

# Enable stack protection
echo "kernel.exec-shield=1" >> /etc/sysctl.conf

# Enable DEP
echo "kernel.noexec=1" >> /etc/sysctl.conf
```

### Configure MAC Policy

```bash
# Enable MAC policy
sigmac enable

# Set enforcing mode
sigmac set-enforcing true

# Add security rules
sigmac add-rule subject:app object:/etc/passwd permissions:read action:allow
sigmac add-rule subject:app object:/etc/shadow permissions:deny action:deny
```

### Secure Boot

```bash
# Enable secure boot
sigsecureboot enable

# Configure keys
sigsecureboot add-key /path/to/key.der

# Verify boot integrity
sigsecureboot verify
```

### User Security

```bash
# Enforce strong passwords
sigpasswd policy min-length 12
sigpasswd policy require-upper
sigpasswd policy require-lower
sigpasswd policy require-number
sigpasswd policy require-special

# Enable account lockout
sigpasswd lockout 5 15

# Enable two-factor authentication
sig2fa enable
```

## Network Security

### Firewall Configuration

```bash
# Enable firewall
sigfirewall enable

# Default deny policy
sigfirewall default deny

# Allow essential services
sigfirewall allow 22/tcp  # SSH
sigfirewall allow 80/tcp  # HTTP
sigfirewall allow 443/tcp # HTTPS

# Enable logging
sigfirewall log enable
```

### Network Hardening

```bash
# Disable unnecessary services
siginit disable telnet
siginit disable ftp

# Enable TCP SYN cookies
echo "net.ipv4.tcp_syncookies=1" >> /etc/sysctl.conf

# Disable source routing
echo "net.ipv4.conf.all.accept_source_route=0" >> /etc/sysctl.conf

# Enable ICMP protection
echo "net.ipv4.icmp_echo_ignore_broadcasts=1" >> /etc/sysctl.conf
```

### SSH Security

```bash
# Disable root login
sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config

# Disable password authentication
sed -i 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config

# Enable key-based authentication
sed -i 's/PubkeyAuthentication no/PubkeyAuthentication yes/' /etc/ssh/sshd_config

# Restart SSH service
siginit restart sshd
```

## Application Security

### Sandbox Applications

```bash
# Create sandbox for application
sigsandbox create --name secure-app --memory 512M --cpu 50

# Add capabilities
sigsandbox add-capability secure-app network

# Run application in sandbox
sigsandbox run secure-app /usr/bin/app
```

### Application Whitelisting

```bash
# Enable application whitelisting
sigwhitelist enable

# Add trusted applications
sigwhitelist add /usr/bin/firefox
sigwhitelist add /usr/bin/terminal

# Remove untrusted applications
sigwhitelist remove /usr/bin/unknown-app
```

### File Permissions

```bash
# Set restrictive umask
echo "umask 077" >> /etc/profile

# Secure sensitive files
chmod 600 /etc/shadow
chmod 600 /etc/ssh/ssh_host_*
chmod 644 /etc/passwd

# Set appropriate permissions for directories
chmod 755 /home
chmod 700 /home/*
```

## Kernel Security

### Kernel Parameters

```bash
# Disable kernel module loading
echo "kernel.modules_disabled=1" >> /etc/sysctl.conf

# Disable kernel profiling
echo "kernel.perf_event_paranoid=3" >> /etc/sysctl.conf

# Enable kernel hardening
echo "kernel.kptr_restrict=2" >> /etc/sysctl.conf

# Disable magic sysrq
echo "kernel.sysrq=0" >> /etc/sysctl.conf
```

### Kernel Module Security

```bash
# Load only necessary modules
echo "modprobe blacklist usb-storage" >> /etc/modprobe.d/blacklist.conf
echo "modprobe blacklist firewire-core" >> /etc/modprobe.d/blacklist.conf

# Sign kernel modules
sigmodule sign /path/to/module.ko

# Verify module signatures
sigmodule verify /path/to/module.ko
```

### Secure Kernel Update

```bash
# Verify kernel signature before update
sigkernel verify signature.sig kernel.bin

# Update kernel securely
sigkernel update --verify kernel.bin

# Rebuild initramfs
sigmkinitramfs
```

## Audit and Monitoring

### Enable Security Audit

```bash
# Enable audit system
sigaudit enable

# Configure audit rules
sigaudit add-rule --subject app --action read --object /etc/passwd
sigaudit add-rule --subject app --action write --object /etc/shadow

# View audit logs
sigaudit log view
```

### System Monitoring

```bash
# Enable system monitoring
sigmonitor enable

# Configure alerts
sigmonitor alert cpu > 80
sigmonitor alert memory > 90
sigmonitor alert disk > 85

# View monitoring dashboard
sigmonitor dashboard
```

### Log Management

```bash
# Enable secure logging
siglog secure

# Configure log rotation
siglog rotate daily
siglog retain 30

# Send logs to remote server
siglog remote syslog.example.com
```

### Intrusion Detection

```bash
# Enable intrusion detection
sigids enable

# Configure detection rules
sigids add-rule --type scan --action block
sigids add-rule --type exploit --action alert

# View detection events
sigids events
```

## Incident Response

### Security Incident Response

```bash
# Lock down system
sigsecurity lockdown

# Collect forensic data
sigforensics collect

# Analyze security logs
sigaudit analyze

# Generate incident report
sigreport generate
```

### Recovery Procedures

```bash
# Restore from backup
sigbackup restore /path/to/backup

# Verify system integrity
sigintegrity verify

# Reapply security hardening
sigsecurity reapply
```

### Security Scanning

```bash
# Run vulnerability scan
sigscan vulnerability

# Run compliance scan
sigscan compliance

# Run penetration test
sigscan penetration

# Fix detected issues
sigscan fix
```

## Security Best Practices

### Regular Updates

```bash
# Update system regularly
sigpkg update && sigpkg upgrade

# Check for security updates
sigpkg security-check

# Apply security patches
sigpkg security-patch
```

### Backup Strategy

```bash
# Create regular backups
sigbackup create --daily
sigbackup create --weekly

# Encrypt backups
sigbackup encrypt /path/to/backup

# Test backup restoration
sigbackup test
```

### Security Training

*   Regular security awareness training
*   Phishing simulation exercises
*   Security policy reviews
*   Incident response drills

## Compliance

### Security Standards

SigmaOS can be configured to meet various security standards:

*   **CIS Benchmarks**: Industry-standard security configurations
*   **NIST**: National Institute of Standards and Technology guidelines
*   **PCI DSS**: Payment Card Industry Data Security Standard
*   **HIPAA**: Health Insurance Portability and Accountability Act

### Compliance Scanning

```bash
# Run CIS benchmark scan
sigcompliance cis

# Run NIST scan
sigcompliance nist

# Generate compliance report
sigcompliance report
```

## Additional Resources

*   [Installation Guide](./INSTALLATION)
*   [Configuration Guide](./CONFIGURATION)
*   [Package Management Guide](./PACKAGE_MANAGEMENT)
*   [Development Guide](./DEVELOPMENT)
*   [SigmaOS Security Documentation](../src/security/)
