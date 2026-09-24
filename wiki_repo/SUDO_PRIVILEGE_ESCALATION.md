# SigmaOS Privilege Escalation Architecture Specification (`sigsudo` & `sigdoas`)

## 1. Executive Summary

SigmaOS provides a unified, zero-dependency privilege escalation framework (`sigsudo` / `sigdoas`) that synthesizes key design principles from Linux `sudo`, OpenBSD `doas`, PolicyKit (`pkexec` GUI agent), and FreeBSD `wheel` group enforcement. It delivers fine-grained privilege delegation, environment variable scrubbing, time-bound credential caching, and audit logging to the system journal.

## 2. Architecture & Design Principles

```
+------------------------------------------------------------------+
|                    User / Process Request                        |
|  +--------------------------+      +---------------------------+ |
|  | CLI: sigsudo / sigdoas   |      | GUI Application           | |
|  +------------+-------------+      +-------------+-------------+ |
+---------------+----------------------------------+---------------+
                |                                  |
                |                                  | D-Bus / Zenith IPC
+---------------v----------------------------------v---------------+
|                   Sigma Privilege Daemon (`sigauthd`)            |
|  +-------------------------------------------------------------+ |
|  | 1. Rule Matcher (/etc/sudoers & /etc/doas.conf)            | |
|  | 2. Credential Validator (PAM / PQC Enclave Token / Password)| |
|  | 3. Environment Variable Scrubber (keepenv whitelist)        | |
|  | 4. Capability / Setuid Sandbox Enforcer                     | |
|  +-------------------------------------------------------------+ |
+-------------------------------+----------------------------------+
                                |
+-------------------------------+----------------------------------+
|               Target Execution (Root / Service UID)              |
|               Audit Event -> Journald Log Stream                 |
+------------------------------------------------------------------+
```

## 3. Configuration & Rule Engine

SigmaOS supports dual rule syntax engines:

### 3.1 Linux `sudoers` Compatibility Syntax (`/etc/sudoers`)
```sudoers
# User privilege specification
root    ALL=(ALL:ALL) ALL
jules   ALL=(ALL:ALL) ALL

# Group rules with NOPASSWD options
%wheel  ALL=(ALL:ALL) ALL
%admin  ALL=(ALL:ALL) NOPASSWD: /usr/bin/systemctl restart *
```

### 3.2 OpenBSD `doas.conf` Concise Syntax (`/etc/doas.conf`)
```doas
# Permitwheel users with environment preservation
permit keepenv :wheel

# Permit specific user without password prompt for reboot
permit nopass jules as root cmd /usr/bin/reboot
```

## 4. Environment Scrubbing & Security Controls

1. **Environment Variable Sanitization**: Strips dangerous dynamic linker environment variables (`LD_PRELOAD`, `LD_LIBRARY_PATH`, `PYTHONPATH`) prior to privilege drop or setuid execution.
2. **Credential Ticket Caching**: Time-limited session tickets (5-minute window) stored in secure `/run/sigsudo/ts/<uid>` tmpfs paths protected by `0700` permissions.
3. **FreeBSD Wheel Enforcement**: Optional strict mode requiring users to be explicit members of GID 0 (`wheel`) before root elevation is permitted.
4. **GUI Authentication Dialog (`siggksu` / `polkit`)**: Displays a secure Zenith Desktop popup window when non-interactive GUI applications request elevated authorization.
