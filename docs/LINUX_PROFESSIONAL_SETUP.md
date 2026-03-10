# SigmaOS Enterprise Linux Deployment Guide

## Overview
SigmaOS has achieved parity with premium enterprise Linux distributions. This guide outlines how to leverage the new Automation and Security subsystems, configuring SigmaOS for production-grade environments.

## 1. Zero-Touch Automation
SigmaOS now includes `SigmaAutomationLayer`, directly mimicking `systemd` timers, `cron`, and enterprise Identity Management (LDAP parity).

### User Provisioning
To provision new users scriptably:
```python
from kernel.linux_automation import SigmaAutomationLayer

auto = SigmaAutomationLayer(kernel)
auto.provision_user("syso_admin", groups=["wheel", "docker", "sigma-mesh"])
```

### Scheduled Backups (Timeshift / Borg Parity)
Enterprise-grade immutable backups run automatically via the automation daemon.
To schedule:
```python
auto.schedule_backup(target_dir="/etc/sigma", cron_expr="0 2 * * *", retention_days=30)
```

## 2. Hardened Security Layer
We've integrated the Holy Trinity of Linux security: `ufw` (firewall), `SELinux` (access control), and `fail2ban` (intrusion prevention).

### UFW Enforcement
```python
from kernel.linux_security_layer import SigmaSecurityLayer

sec = SigmaSecurityLayer(kernel)
sec.ufw_enable()
sec.ufw_allow("22/tcp")
sec.ufw_allow("8000/tcp") # Aether ports
```

### SELinux Access Controls
SigmaOS supports Enforcing, Permissive, or Disabled policies.
```python
# Force immediate lock-down
sec.selinux_setenforce(1) 
print(sec.selinux_getenforce()) # Output: Enforcing
```

### Fail2Ban Jail Deployment
Fail2Ban is enabled by default to protect Sovereign data nodes.
```python
print(sec.fail2ban_status("ssh"))
```

## 3. Visual Customization Toolkit
We have added a **Visual Custom Tool** within the OS Dashboard. This allows system administrators to adjust kernel parameters, theme syntax, and automation schedules visually without touching config files—ensuring maximum ease of use while maintaining Linux-grade flexibility.

Access this via the GUI intent bar by typing: `Deploy Custom Server`.
