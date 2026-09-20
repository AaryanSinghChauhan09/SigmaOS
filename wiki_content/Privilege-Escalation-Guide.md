# Privilege Escalation Guide: `sigsudo` & `sigdoas` in SigmaOS

## Introduction

In SigmaOS, administrative privilege escalation is managed by `sigsudo` and `sigdoas`. These commands allow authorized users to run commands as root or another user, while ensuring strict security auditing, environment sanitization, and time-bound password caching.

## Usage Overview

### 1. Basic CLI Usage

To execute a command with root privileges using `sigsudo`:
```bash
sigsudo systemctl restart zenith-compositor
```

To execute a command using the lightweight OpenBSD-style `sigdoas`:
```bash
sigdoas reboot
```

To switch to an interactive root shell:
```bash
sigsudo -i
# or
sigdoas -s
```

### 2. Configuration Files

#### `/etc/sudoers` Rules
Administrative access can be configured in `/etc/sudoers` using `visudo`:
```sudoers
# Allow members of group wheel to execute any command
%wheel ALL=(ALL:ALL) ALL

# Allow user jules to run package updates without password
jules ALL=(ALL) NOPASSWD: /usr/bin/sigpkg update
```

#### `/etc/doas.conf` Rules
Alternatively, concise `doas` rules can be specified in `/etc/doas.conf`:
```doas
# Permit wheel group users to execute commands with environment preserved
permit keepenv :wheel

# Permit jules to run system restart without password
permit nopass jules as root cmd /usr/bin/reboot
```

## Security Best Practices

1. **Wheel Group Membership**: Ensure administrative users are members of the `wheel` group (`usermod -aG wheel jules`).
2. **Environment Sanitization**: `sigsudo` automatically strips dangerous variables like `LD_PRELOAD` to prevent local privilege escalation.
3. **Audit Trails**: All failed and successful privilege escalation attempts are logged to the binary system journal (`journalctl -u sigauthd`).
