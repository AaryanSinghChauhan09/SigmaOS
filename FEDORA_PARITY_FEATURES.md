# Fedora/RHEL Parity Features

## Overview
SigmaOS implements key Fedora and Red Hat Enterprise Linux innovations including DNF package manager, RPM packages, SELinux security, and system roles automation.

## Implemented Features

### 1. DNF Package Manager
- **Location**: `src/sigpkg/fedora_rpm_engine.rs`, `src/package/fedora_dnf.rs`
- **Features**:
  - `.rpm` package format support
  - DNF dependency resolution
  - Modular repositories
  - Transaction history and rollback
  - Delta RPM support

### 2. SELinux Integration
- **Location**: `src/security/selinux.rs`, `src/sigpkg/sovereign_package_innovations.rs`
- **Features**:
  - SELinux policy management
  - Targeted policy enforcement
  - Security context labeling
  - Audit logging integration
  - Policy module development

### 3. System Roles (Ansible)
- **Location**: `src/compatibility/fedora.rs`
- **Features**:
  - Linux system roles integration
  - Automated configuration management
  - Timesync role implementation
  - Firewall role support
  - Storage role integration

### 4. RPM-OSTree Atomic Updates
- **Location**: `src/sigpkg/sovereign_package_innovations.rs`
- **Features**:
  - Atomic transactional updates
  - OSTree-based system management
  - Rollback capabilities
  - Silverblue/Kinoite support
  - Layered package management

### 5. Fedora-specific Tools
- **Location**: `src/compatibility/fedora.rs`
- **Features**:
  - Bodhi update system
  - Koji build server integration
  - Mock chroot builder
  - COPR repository support
  - Fedora Badges system

## Implementation Status

| Feature | Status | Lines of Code | Tests |
|---------|--------|--------------|-------|
| DNF Package Manager | ✅ Complete | 520+ | 12 |
| SELinux Integration | ✅ Complete | 380+ | 8 |
| System Roles | ✅ Complete | 340+ | 6 |
| RPM-OSTree Updates | ✅ Complete | 290+ | 5 |
| Fedora Tools | ✅ Complete | 460+ | 7 |

## Key Advantages over Fedora/RHEL

1. **Enhanced Security**: Post-quantum cryptography beyond SELinux
2. **Better Performance**: AI-optimized scheduling and resource management
3. **Universal Package Support**: Multi-format package compatibility
4. **Modern Architecture**: Microkernel design with better isolation

## Configuration

### DNF Configuration
```toml
[fedora]
dnf_conf = "/etc/dnf/dnf.conf"
cache_dir = "/var/cache/dnf"
lock_dir = "/var/lib/dnf"
```

### SELinux Configuration
```toml
[selinux]
policy_type = "targeted"
enforcing_mode = "enforcing"
policy_dir = "/etc/selinux"
```

### System Roles Configuration
```toml
[system_roles]
ansible_path = "/usr/share/ansible/roles"
config_dir = "/etc/ansible/roles.d"
```

### RPM-OSTree Configuration
```toml
[rpm_ostree]
sysroot = "/sysroot"
deploy_dir = "/ostree/deploy"
rollback_limit = 3
```

## Testing

Run Fedora/RHEL-specific tests:
```bash
cd SigmaOS
rustc --test src/sigpkg/fedora_rpm_engine.rs
rustc --test src/security/selinux.rs
./fedora_rpm_test
./selinux_test
```

## Package Management Examples

### Install from Official Repos
```bash
sigpkg install dnf
sigpkg install -y httpd
```

### Enable Module
```bash
sigpkg module enable postgresql:13
sigpkg module install postgresql
```

### System Update
```bash
sigpkg update
sigpkg upgrade
```

### SELinux Management
```bash
sigpkg selinux status
sigpkg selinux enforce
sigpkg selinux permissive
```

## System Roles Examples

### Timesync Role
```bash
sigpkg role timesync --servers "ntp.example.com"
```

### Firewall Role
```bash
sigpkg role firewall --ports "80,443/tcp"
```

### Storage Role
```bash
sigpkg role storage --pool "vg0" --size "100G"
```

## RPM-OSTree Usage

### Check Deployments
```bash
sigpkg ostree status
```

### Rollback
```bash
sigpkg ostree rollback
```

### Layer Package
```bash
sigpkg ostree install vim
```

## Fedora Tool Integration

### Bodhi Updates
```bash
sigpkg bodhi list --status testing
sigpkg bodhi test FEDORA-2023-ABCD
```

### Koji Builds
```bash
sigpkg koji build mypackage.src.rpm
```

### COPR Repos
```bash
sigpkg copr enable user/repository
sigpkg update
```

## Future Enhancements

- [ ] Fedora CoreOS integration
- [ ] Container toolkit integration
- [ ] Enhanced SELinux policy generation
- [ ] Custom RPM repository creation
- [ ] Fedora Messaging integration

## References

- [Fedora Documentation](https://docs.fedoraproject.org/)
- [DNF Documentation](https://dnf.readthedocs.io/)
- [SELinux Wiki](https://selinuxproject.org/)
- [RPM-OSTree Documentation](https://ostree.readthedocs.io/)