# SigmaAdmin - System Administration Tools

## Overview

SigmaAdmin is the centralized system administration suite for SigmaOS, inspired by openSUSE's YaST. It provides both GUI and CLI interfaces for managing networking, users, storage, security, services, and system settings.

## Architecture

### Admin Modules

SigmaAdmin organizes administration into logical modules:

```rust
pub enum AdminModule {
    Network = 0,
    Users = 1,
    Storage = 2,
    Security = 3,
    Services = 4,
    Software = 5,
    System = 6,
    Hardware = 7,
    Logs = 8,
    Backup = 9,
}
```

### Main Structure

```rust
pub struct SigmaAdmin {
    pub current_module: AdminModule,
    pub initialized: SigmaBool,
    pub requires_root: SigmaBool,
}
```

## User Management

### User Operations

```rust
// Create user
user_manager.create_user(user: *const UserAccount) -> AdminResult

// Delete user
user_manager.delete_user(username: *const SigmaU8) -> AdminResult

// Modify user
user_manager.modify_user(user: *const UserAccount) -> AdminResult

// List users
user_manager.list_users(users: *mut *mut UserAccount, max_count: SigmaU32) -> AdminResult
```

### Group Operations

```rust
// Create group
user_manager.create_group(group: *const Group) -> AdminResult

// Add user to group
user_manager.add_user_to_group(username, groupname) -> AdminResult

// Remove user from group
user_manager.remove_user_from_group(username, groupname) -> AdminResult
```

## Network Management

### Interface Operations

```rust
// List interfaces
network_manager.list_interfaces(interfaces, max_count) -> AdminResult

// Configure interface
network_manager.configure_interface(interface: *const NetworkInterface) -> AdminResult

// Enable/disable interface
network_manager.enable_interface(name) -> AdminResult
network_manager.disable_interface(name) -> AdminResult
```

### DHCP and DNS

```rust
// Set DHCP
network_manager.set_dhcp(name, enabled) -> AdminResult

// Add DNS server
network_manager.add_dns_server(name, dns) -> AdminResult

// Test connection
network_manager.test_connection(host) -> SigmaBool
```

## Storage Management

### Device Operations

```rust
// List devices
storage_manager.list_devices(devices, max_count) -> AdminResult

// Mount device
storage_manager.mount_device(device, mount_point) -> AdminResult

// Unmount device
storage_manager.unmount_device(mount_point) -> AdminResult

// Format device
storage_manager.format_device(device, filesystem) -> AdminResult
```

### Partition Management

```rust
// Create partition
storage_manager.create_partition(device, size_bytes) -> AdminResult

// Delete partition
storage_manager.delete_partition(device, partition_number) -> AdminResult

// Get disk usage
storage_manager.get_disk_usage(path, used, total) -> AdminResult
```

## Service Management

### Service Operations

```rust
// List services
service_manager.list_services(services, max_count) -> AdminResult

// Start/stop/restart service
service_manager.start_service(name) -> AdminResult
service_manager.stop_service(name) -> AdminResult
service_manager.restart_service(name) -> AdminResult

// Enable/disable service
service_manager.enable_service(name) -> AdminResult
service_manager.disable_service(name) -> AdminResult
```

### Service Status

```rust
// Get status
service_manager.get_service_status(name) -> ServiceStatus

// Reload configuration
service_manager.reload_service(name) -> AdminResult
```

## System Information

### System Operations

```rust
// Get system info
system_info_manager.get_system_info() -> SystemInfo

// Set hostname
system_info_manager.set_hostname(hostname) -> AdminResult

// Get uptime
system_info_manager.get_uptime() -> SigmaU64

// Get load average
system_info_manager.get_load_average(load1, load5, load15) -> AdminResult
```

### Power Management

```rust
// Shutdown
system_info_manager.shutdown(delay_seconds) -> AdminResult

// Reboot
system_info_manager.reboot(delay_seconds) -> AdminResult
```

## API

### Initialization

```rust
// Initialize SigmaAdmin
sigma_admin_init() -> SigmaI32

// Get instance
sigma_admin_get() -> *mut SigmaAdmin

// Set current module
sigma_admin_set_module(module: AdminModule) -> SigmaI32
```

### Module Access

```rust
// Get user manager
sigma_admin_user_manager() -> *mut UserManager

// Get network manager
sigma_admin_network_manager() -> *mut NetworkManager

// Get storage manager
sigma_admin_storage_manager() -> *mut StorageManager

// Get service manager
sigma_admin_service_manager() -> *mut ServiceManager

// Get system info manager
sigma_admin_system_info_manager() -> *mut SystemInfoManager
```

## CLI Usage

```bash
# User management
sigma-admin user create --username john --shell /bin/bash
sigma-admin user delete --username john
sigma-admin user list

# Network management
sigma-admin network list
sigma-admin network configure --interface eth0 --ip 192.168.1.100
sigma-admin network enable --interface eth0

# Storage management
sigma-admin storage list
sigma-admin storage mount --device /dev/sda1 --mountpoint /mnt/data
sigma-admin storage format --device /dev/sda1 --filesystem ext4

# Service management
sigma-admin service list
sigma-admin service start --name nginx
sigma-admin service enable --name nginx

# System information
sigma-admin system info
sigma-admin system set-hostname --name sigmaos-desktop
```

## GUI Features

### Control Center

The GUI Control Center provides:

- **Network Panel** - Interface configuration, DNS, DHCP
- **User Panel** - User and group management
- **Storage Panel** - Disk management, partitioning
- **Service Panel** - Service status and control
- **System Panel** - System information, power management
- **Security Panel** - Firewall, MAC policies
- **Updates Panel** - System updates and package management

### Dashboard

Unified dashboard showing:
- System resource usage
- Network status
- Service status
- Recent logs
- Security alerts

## Integration

### Package Manager

SigmaAdmin integrates with the package manager for:
- Software installation/removal
- System updates
- Dependency management

### Security System

SigmaAdmin integrates with the MAC policy engine for:
- Security policy management
- Capability management
- Audit log viewing

## Future Enhancements

- **Remote Administration** - Web-based remote management
- **Automation** - Scriptable administration tasks
- **Templates** - Pre-configured system profiles
- **Backup/Restore** - System configuration backup
- **Cluster Management** - Multi-node administration

## References

- [Package Manager Documentation](Package-Manager.md)
- [MAC Policy Engine](MAC-Policy-Engine.md)
- [Network Configuration](Network-Configuration.md)
