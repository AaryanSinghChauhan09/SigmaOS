# Kernel

SigmaOS kernel is a microkernel-inspired design with modular subsystems.

## Kernel Architecture

### Core Components

- **Task Scheduler**: CPU scheduling with multiple algorithms (CFS, RT, IDLE)
- **Memory Management**: Buddy allocator, slab allocator, transparent huge pages
- **IPC**: Inter-process communication with shared memory and message queues
- **VFS**: Virtual filesystem layer with pluggable filesystem drivers
- **Security**: Capability-based security, pledge/unveil sandboxing
- **Drivers**: Hardware abstraction layer for device drivers

## Kernel Modules

### Loading Modules

Load kernel modules dynamically:

```bash
# Load module
sigmod load module-name

# Unload module
sigmod unload module-name

# List loaded modules
sigmod list

# Show module information
sigmod info module-name
```

### Kernel Parameters

View and modify kernel parameters:

```bash
# View all parameters
sysctl -a

# View specific parameter
sysctl kernel.hostname

# Set parameter
sysctl kernel.hostname="new-hostname"

# Persist parameter
echo "kernel.hostname=new-hostname" >> /etc/sysctl.conf
```

## Process Management

### Process Monitoring

View process information:

```bash
# List all processes
sigps aux

# Show process tree
sigps tree

# Show process details
sigps show pid
```

### Process Control

```bash
# Kill process
sigkill pid

# Set process priority
renice priority pid

# Run process with nice level
nice -n 10 command
```

## Memory Management

### Memory Information

View memory usage:

```bash
# Show memory statistics
sigmem stat

# Show memory map for process
sigmem map pid

# Show swap usage
sigmem swap
```

### Transparent Huge Pages

Configure huge pages:

```bash
# Enable huge pages
echo 1 > /proc/sys/vm/nr_hugepages

# Set huge page size
echo 2048 > /proc/sys/vm/hugepages_treat_as
```

## Filesystem Encryption

### fscrypt

Enable per-directory encryption:

```bash
# Create encrypted directory
fscrypt encrypt /path/to/directory

# Lock directory
fscrypt lock /path/to/directory

# Unlock directory
fscrypt unlock /path/to/directory
```

### AutoFS

Configure on-demand mounting:

```bash
# Register mount trigger
autofs register /mnt/data /dev/sda1 ext4

# Trigger mount
autofs trigger /mnt/data

# Configure idle timeout
autofs set-timeout /mnt/data 300
```

## Security Mitigations

### Kernel Pointer Restriction

Restrict kernel pointer exposure:

```bash
# Set kptr_restrict level
sysctl kernel.kptr_restrict=2

# Set dmesg_restrict level
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

## Device Drivers

### PCI/PCIe

Manage PCIe devices:

```bash
# List PCIe devices
sigpci list

# Scan for new devices
sigpci scan

# Configure device
sigpci configure bus:device:function
```

### NVMe

Manage NVMe storage:

```bash
# List NVMe devices
sgnvme list

# Format NVMe device
sgnvme format /dev/nvme0n1

# Show NVMe SMART data
sgnvme smart /dev/nvme0n1
```

## Kernel Debugging

### System Logs

View kernel logs:

```bash
# View kernel messages
dmesg

# View systemd journal
journalctl -k

# Follow kernel logs
journalctl -kf
```

### Kernel Debugging

Enable kernel debugging:

```bash
# Enable debug symbols
echo 1 > /proc/sys/kernel/sysrq

# Trigger sysrq
echo t > /proc/sysrq-trigger
```

## Linux-Compatible Syscalls

### Syscall Compatibility

Linux syscall compatibility layer:

```bash
# Use Linux syscalls directly
# via compatibility layer

# Standard Linux syscalls supported:
# - read, write, open, close
# - fork, clone, execve, exit
# - mkdir, rmdir, unlink, chmod
# - getpid, getppid
# - And many more
```

## Process Descriptors

### Pidfd/Procdesc

Linux pidfd and FreeBSD Capsicum procdesc integration:

```bash
# Open pidfd for process
pidfd_open pid > /proc/self/fd/3

# Send signal via pidfd
pidfd_send_signal 3 SIGTERM

# Get file descriptor from process
pidfd_getfd 3 4
```

### Subreaper

Orphan process containment:

```bash
# Set process as subreaper
prctl --set-child-subreaper 1

# Reparent orphans
# Automatically handled by kernel
```

## Control Flow Integrity

### CFI Engine

Forward-edge CFI for control flow hijacking prevention:

```bash
# Register CFI target
cfi register 0x1000 function_name

# Validate indirect call
cfi validate 0x2000 0x1000

# View violations
cfi violations
```

## Next Steps

- [Filesystems](05-Filesystems.md) - Filesystem management
- [Networking](06-Networking.md) - Network configuration
- [Security](07-Security.md) - Security hardening
