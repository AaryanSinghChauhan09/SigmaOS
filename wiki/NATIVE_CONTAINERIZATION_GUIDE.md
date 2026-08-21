# Native Containerization Guide for SigmaOS

## Overview

SigmaOS implements a native containerization system that provides lightweight, secure isolation without the overhead of traditional containers. This approach leverages the kernel's capability-based security model and microkernel architecture.

## Architecture

### Container Namespace Isolation

```rust
pub struct ContainerNamespace {
    pub pid_namespace: PidNamespace,
    pub mount_namespace: MountNamespace,
    pub network_namespace: NetworkNamespace,
    pub user_namespace: UserNamespace,
    pub uts_namespace: UtsNamespace,
    pub ipc_namespace: IpcNamespace,
}

pub struct Container {
    pub id: ContainerId,
    pub namespace: ContainerNamespace,
    pub capabilities: CapabilitySet,
    pub cgroup: CgroupController,
    pub rootfs: RootFilesystem,
    pub state: ContainerState,
}
```

### Capability-Based Container Security

Unlike traditional container systems that rely on cgroups and namespaces alone, SigmaOS containers use hardware-enforced capability tokens:

```rust
pub struct ContainerCapabilityToken {
    pub container_id: u64,
    pub allowed_syscalls: [SyscallNumber; 64],
    pub resource_limits: ResourceLimits,
    pub network_policy: NetworkPolicy,
    pub filesystem_access: FsAccessPolicy,
}
```

## Container Lifecycle Management

### Container Creation

```rust
impl ContainerManager {
    pub fn create_container(
        &mut self,
        image: ContainerImage,
        config: ContainerConfig,
    ) -> Result<ContainerId, ContainerError> {
        // 1. Generate unique container ID
        let container_id = self.generate_container_id();
        
        // 2. Create isolated namespaces
        let namespaces = self.create_namespaces(&config)?;
        
        // 3. Set up root filesystem
        let rootfs = self.setup_rootfs(&image)?;
        
        // 4. Generate capability token
        let capabilities = self.generate_container_capabilities(&config)?;
        
        // 5. Create cgroup for resource limits
        let cgroup = self.setup_cgroup(&config.resource_limits)?;
        
        let container = Container {
            id: container_id,
            namespace: namespaces,
            capabilities,
            cgroup,
            rootfs,
            state: ContainerState::Created,
        };
        
        self.containers.insert(container_id, container);
        Ok(container_id)
    }
}
```

### Container Execution

```rust
impl Container {
    pub fn start(&mut self) -> Result<(), ContainerError> {
        // Verify capability token
        self.verify_capabilities()?;
        
        // Enter namespaces
        self.enter_namespaces()?;
        
        // Set up cgroup limits
        self.apply_cgroup_limits()?;
        
        // Execute container init process
        self.execute_init_process()?;
        
        self.state = ContainerState::Running;
        Ok(())
    }
}
```

## Resource Management

### Native Cgroup v2 Implementation

```rust
pub struct SigmaCgroup {
    pub cpu_controller: CpuController,
    pub memory_controller: MemoryController,
    pub io_controller: IoController,
    pub pids_controller: PidsController,
}

pub struct CpuController {
    pub cpu_shares: u64,
    pub cpu_quota: u64,
    pub cpu_period: u64,
    pub cpus: Vec<u32>,
}

impl CpuController {
    pub fn set_cpu_limit(&mut self, quota_us: u64, period_us: u64) {
        self.cpu_quota = quota_us;
        self.cpu_period = period_us;
    }
    
    pub fn set_cpu_affinity(&mut self, cpus: Vec<u32>) {
        self.cpus = cpus;
    }
}
```

## Networking

### Container Network Interface

```rust
pub struct ContainerNetwork {
    pub veth_pair: VethPair,
    pub bridge: Bridge,
    pub ip_address: IpAddr,
    pub firewall_rules: Vec<FirewallRule>,
}

impl ContainerNetwork {
    pub fn create_veth_pair(&mut self) -> Result<(), NetworkError> {
        // Create virtual ethernet pair
        let (host_veth, container_veth) = VethPair::new()?;
        
        // Attach container veth to container namespace
        self.attach_to_namespace(container_veth)?;
        
        // Attach host veth to bridge
        self.bridge.attach(host_veth)?;
        
        // Configure IP addresses
        self.configure_ip_addresses()?;
        
        // Set up firewall rules
        self.apply_firewall_rules()?;
        
        Ok(())
    }
}
```

## Storage

### Layered Filesystem

```rust
pub struct LayeredFilesystem {
    pub layers: Vec<FilesystemLayer>,
    pub upper_layer: FilesystemLayer,
    pub work_dir: PathBuf,
}

pub struct FilesystemLayer {
    pub id: LayerId,
    pub digest: String,
    pub size: u64,
    pub files: HashMap<PathBuf, FileMetadata>,
}

impl LayeredFilesystem {
    pub fn mount(&self, target: &Path) -> Result<(), FsError> {
        // Use overlay filesystem for layering
        let options = MountOptions {
            lower: self.layers.iter().map(|l| l.path.clone()).collect(),
            upper: self.upper_layer.path.clone(),
            work: self.work_dir.clone(),
        };
        
        mount_overlay(&options, target)?;
        Ok(())
    }
}
```

## Security Features

### Seccomp-Like Syscall Filtering

```rust
pub struct SyscallFilter {
    pub allowed_syscalls: HashSet<SyscallNumber>,
    pub denied_syscalls: HashSet<SyscallNumber>,
    pub default_action: FilterAction,
}

impl SyscallFilter {
    pub fn check_syscall(&self, syscall: SyscallNumber) -> bool {
        if self.allowed_syscalls.contains(&syscall) {
            return true;
        }
        if self.denied_syscalls.contains(&syscall) {
            return false;
        }
        matches!(self.default_action, FilterAction::Allow)
    }
}
```

### AppArmor-Like Policy

```rust
pub struct ContainerSecurityPolicy {
    pub file_access_rules: Vec<FileAccessRule>,
    pub network_rules: Vec<NetworkRule>,
    pub capability_rules: Vec<CapabilityRule>,
    pub ptrace_scope: PtraceScope,
}

pub struct FileAccessRule {
    pub path: PathBuf,
    pub permissions: FilePermissions,
    pub mode: AccessMode,
}
```

## Container Images

### Image Format

```rust
pub struct ContainerImage {
    pub id: ImageId,
    pub layers: Vec<LayerId>,
    pub config: ImageConfig,
    pub manifest: ImageManifest,
}

pub struct ImageConfig {
    pub cmd: Vec<String>,
    pub entrypoint: Vec<String>,
    pub env: Vec<String>,
    pub working_dir: PathBuf,
    pub user: String,
    pub exposed_ports: Vec<u16>,
    pub volumes: Vec<PathBuf>,
}
```

### Image Distribution

```rust
pub struct ImageRegistry {
    pub images: HashMap<ImageId, ContainerImage>,
    pub storage: ImageStorage,
}

impl ImageRegistry {
    pub fn pull_image(&mut self, reference: &str) -> Result<ImageId, RegistryError> {
        // 1. Resolve reference to image ID
        let image_id = self.resolve_reference(reference)?;
        
        // 2. Download layers
        for layer_id in &self.images[&image_id].layers {
            self.download_layer(layer_id)?;
        }
        
        // 3. Verify layer digests
        self.verify_layers(&image_id)?;
        
        Ok(image_id)
    }
}
```

## Integration with Systemd

### Container Service Units

```ini
[Unit]
Description=SigmaOS Container Service
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/sigma-container start %i
ExecStop=/usr/bin/sigma-container stop %i
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

## Performance Optimization

### Memory Deduplication

```rust
pub struct KsmDeduplicator {
    pub pages_to_scan: u64,
    pub sleep_millisecs: u32,
    pub merge_across_nodes: bool,
}

impl KsmDeduplicator {
    pub fn enable(&mut self) {
        // Enable Kernel Samepage Merging
        self.write_sysctl_bool("vm.ksm_run", true)?;
        self.write_sysctl_int("vm.pages_to_scan", self.pages_to_scan)?;
        self.write_sysctl_int("vm.ksm_sleep_millisecs", self.sleep_millisecs)?;
    }
}
```

## Use Cases

### Web Application Container

```rust
let web_container = ContainerConfig {
    image: "nginx:latest",
    ports: vec![80],
    environment: vec![
        ("NGINX_PORT".to_string(), "80".to_string()),
    ],
    resource_limits: ResourceLimits {
        memory: 512 * 1024 * 1024, // 512MB
        cpu_shares: 1024,
    },
    capabilities: vec![
        Capability::NetBindService,
        Capability::Setgid,
        Capability::Setuid,
    ],
};
```

### Database Container

```rust
let db_container = ContainerConfig {
    image: "postgres:14",
    ports: vec![5432],
    volumes: vec![
        "/var/lib/postgresql/data".into(),
    ],
    resource_limits: ResourceLimits {
        memory: 2 * 1024 * 1024 * 1024, // 2GB
        cpu_shares: 2048,
    },
    capabilities: vec![
        Capability::Chown,
        Capability::Setgid,
        Capability::Setuid,
    ],
};
```

## Comparison with Docker

| Feature | SigmaOS Containers | Docker |
|---------|-------------------|--------|
| Isolation | Capability-based | Namespaces + Cgroups |
| Security | Hardware-enforced | Linux capabilities |
| Overhead | Minimal | Moderate |
| Image Format | Native | OCI compliant |
| Networking | Native bridge | Bridge/Overlay/Macvlan |
| Storage | Layered overlay | OverlayFS |

## Future Enhancements

1. **GPU Passthrough**: Direct GPU access for compute workloads
2. **Live Migration**: Container migration between hosts
3. **Snapshotting**: Container state snapshots
4. **Container Orchestration**: Native container orchestration
5. **FIPS Mode**: FIPS 140-2 compliant cryptographic operations

## Best Practices

1. **Minimal Base Images**: Use minimal base images to reduce attack surface
2. **Capability Drop**: Drop unnecessary capabilities
3. **Resource Limits**: Always set resource limits
4. **Read-only Root**: Mount root filesystem as read-only when possible
5. **Security Scanning**: Scan images for vulnerabilities before deployment

## Troubleshooting

### Container Won't Start

```bash
# Check container logs
sigma-container logs <container_id>

# Verify capability token
sigma-container verify <container_id>

# Check namespace isolation
sigma-container inspect <container_id>
```

### Performance Issues

```bash
# Check resource usage
sigma-container stats <container_id>

# Analyze cgroup limits
cat /sys/fs/cgroup/sigma/<container_id>/cpu.max
cat /sys/fs/cgroup/sigma/<container_id>/memory.max
```

## References

- [Linux Containers](https://linuxcontainers.org/)
- [OCI Runtime Specification](https://github.com/opencontainers/runtime-spec)
- [systemd-nspawn](https://www.freedesktop.org/software/systemd/man/systemd-nspawn.html)
- [Capabilities(7)](https://man7.org/linux/man-pages/man7/capabilities.7.html)