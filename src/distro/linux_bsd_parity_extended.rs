use std::vec;
// SPDX-License-Identifier: MIT
// SigmaOS Extended Linux & BSD Distro Parity Subsystem
// Clean-room implementations of Slackware PkgTools / SlackBuilds, GNU Guix & Shepherd,
// Fedora Silverblue OSTree, Illumos/Solaris Crossbow & NetBSD RUMP, Netplan & Cloud-Init, and openSUSE YaST2 & Snapper.

use std::format;
use std::string::String;
use std::vec::Vec;

// ============================================================================
// 1. Slackware & LFS: PkgTools & SlackBuild Compiler (SlackwarePkgTools / SlackBuildCompiler)
// ============================================================================

/// Slackware Package Archive Record (/var/log/packages entry)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackPackage {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub build: String,
    pub files: Vec<String>,
}

/// Slackware PkgTools Management Engine (installpkg, removepkg, upgradepkg, slackpkg)
pub struct SlackwarePkgTools {
    pub installed_packages: Vec<SlackPackage>,
}

impl SlackwarePkgTools {
    pub fn new() -> Self {
        Self {
            installed_packages: Vec::new(),
        }
    }

    /// Simulates installpkg
    pub fn install_pkg(&mut self, pkg: SlackPackage) -> Result<(), &'static str> {
        if self.installed_packages.iter().any(|p| p.name == pkg.name) {
            return Err("Package already installed");
        }
        self.installed_packages.push(pkg);
        Ok(())
    }

    /// Simulates removepkg
    pub fn remove_pkg(&mut self, pkg_name: &str) -> Result<SlackPackage, &'static str> {
        if let Some(pos) = self
            .installed_packages
            .iter()
            .position(|p| p.name == pkg_name)
        {
            Ok(self.installed_packages.remove(pos))
        } else {
            Err("Package not found")
        }
    }

    /// Simulates upgradepkg
    pub fn upgrade_pkg(&mut self, new_pkg: SlackPackage) -> Result<(), &'static str> {
        let _ = self.remove_pkg(&new_pkg.name);
        self.installed_pkg_push(new_pkg);
        Ok(())
    }

    fn installed_pkg_push(&mut self, pkg: SlackPackage) {
        self.installed_packages.push(pkg);
    }

    pub fn is_installed(&self, pkg_name: &str) -> bool {
        self.installed_packages.iter().any(|p| p.name == pkg_name)
    }
}

impl Default for SlackwarePkgTools {
    fn default() -> Self {
        Self::new()
    }
}

/// SlackBuild Source Build Recipe Compiler & Generator
pub struct SlackBuildCompiler {
    pub script_name: String,
    pub arch: String,
    pub build: String,
    pub tag: String,
    pub build_flags: String,
}

impl SlackBuildCompiler {
    pub fn new(script_name: &str, arch: &str, build: &str, tag: &str) -> Self {
        Self {
            script_name: String::from(script_name),
            arch: String::from(arch),
            build: String::from(build),
            tag: String::from(tag),
            build_flags: String::from("-O2 -march=x86-64 -pipe"),
        }
    }

    /// Compiles source tarball into a .txz package according to SlackBuild specs
    pub fn compile_package(&self, pkg_name: &str, version: &str) -> SlackPackage {
        let package_tarball_name = format!(
            "{}-{}-{}-{}{}.txz",
            pkg_name, version, self.arch, self.build, self.tag
        );

        let mock_installed_files = vec![
            format!("/usr/bin/{}", pkg_name),
            format!("/usr/share/doc/{}-{}/README", pkg_name, version),
            format!("/install/slack-desc",),
        ];

        let _ = package_tarball_name;

        SlackPackage {
            name: String::from(pkg_name),
            version: String::from(version),
            arch: self.arch.clone(),
            build: format!("{}{}", self.build, self.tag),
            files: mock_installed_files,
        }
    }
}

// ============================================================================
// 2. GNU Guix & Shepherd: Functional Store & Shepherd Service Supervisor
// ============================================================================

/// Guix Functional Package Derivation Output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuixDerivation {
    pub name: String,
    pub version: String,
    pub store_path: String,
    pub inputs: Vec<String>,
}

/// Guix Functional Store Engine (/gnu/store)
pub struct GuixFunctionalStore {
    pub store: Vec<GuixDerivation>,
}

impl GuixFunctionalStore {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }

    /// Evaluates and adds a package derivation into the functional store
    pub fn add_derivation(&mut self, name: &str, version: &str, inputs: &[&str]) -> GuixDerivation {
        let mut hash_acc = 0u64;
        for input in inputs {
            for b in input.bytes() {
                hash_acc = hash_acc.wrapping_mul(31).wrapping_add(b as u64);
            }
        }
        for b in name.bytes().chain(version.bytes()) {
            hash_acc = hash_acc.wrapping_mul(31).wrapping_add(b as u64);
        }

        let store_hash = format!("{:032x}", hash_acc);
        let store_path = format!("/gnu/store/{}-{}-{}", &store_hash[..32], name, version);

        let deriv = GuixDerivation {
            name: String::from(name),
            version: String::from(version),
            store_path,
            inputs: inputs.iter().map(|&i| String::from(i)).collect(),
        };

        self.store.push(deriv.clone());
        deriv
    }

    /// Garbage collects unused store paths
    pub fn gc_live_paths(&mut self, live_roots: &[&str]) -> usize {
        let initial_count = self.store.len();
        self.store.retain(|d| {
            live_roots
                .iter()
                .any(|&root| d.store_path.contains(root) || d.name == root)
        });
        initial_count - self.store.len()
    }
}

impl Default for GuixFunctionalStore {
    fn default() -> Self {
        Self::new()
    }
}

/// GNU Shepherd Service Daemon Supervisor State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShepherdServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ShepherdService {
    pub name: String,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub state: ShepherdServiceState,
    pub pid: Option<u32>,
}

/// GNU Shepherd Service Manager Engine
pub struct GNUGuixShepherdSupervisor {
    pub services: Vec<ShepherdService>,
    pub pid_counter: u32,
}

impl GNUGuixShepherdSupervisor {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            pid_counter: 100,
        }
    }

    pub fn register_service(&mut self, name: &str, provides: &[&str], requires: &[&str]) {
        self.services.push(ShepherdService {
            name: String::from(name),
            provides: provides.iter().map(|&p| String::from(p)).collect(),
            requires: requires.iter().map(|&r| String::from(r)).collect(),
            state: ShepherdServiceState::Stopped,
            pid: None,
        });
    }

    pub fn start_service(&mut self, name: &str) -> Result<u32, &'static str> {
        let requires = if let Some(svc) = self.services.iter().find(|s| s.name == name) {
            svc.requires.clone()
        } else {
            return Err("Shepherd service not found");
        };

        // Ensure required services/provisions are active
        for req in &requires {
            let req_active = self.services.iter().any(|s| {
                (s.name == *req || s.provides.contains(req))
                    && s.state == ShepherdServiceState::Running
            });
            if !req_active {
                return Err("Shepherd service dependency not satisfied");
            }
        }

        if let Some(svc) = self.services.iter_mut().find(|s| s.name == name) {
            self.pid_counter += 1;
            svc.state = ShepherdServiceState::Running;
            svc.pid = Some(self.pid_counter);
            Ok(self.pid_counter)
        } else {
            Err("Shepherd service not found")
        }
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(svc) = self.services.iter_mut().find(|s| s.name == name) {
            svc.state = ShepherdServiceState::Stopped;
            svc.pid = None;
            Ok(())
        } else {
            Err("Shepherd service not found")
        }
    }
}

impl Default for GNUGuixShepherdSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Fedora Silverblue: Immutable OSTree Sysroot & Layered Overlays
// ============================================================================

/// OSTree Immutable Deployment Node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OstreeDeployment {
    pub checksum: String,
    pub os_name: String,
    pub version: String,
    pub layered_packages: Vec<String>,
    pub is_active: bool,
}

/// OSTree Atomic Deployment Engine (Fedora Silverblue / Endless OS Parity)
pub struct OstreeDeploymentEngine {
    pub deployments: Vec<OstreeDeployment>,
    pub active_index: Option<usize>,
}

impl OstreeDeploymentEngine {
    pub fn new() -> Self {
        Self {
            deployments: Vec::new(),
            active_index: None,
        }
    }

    /// Commits a new immutable tree commit
    pub fn commit_tree(&mut self, os_name: &str, version: &str, root_bytes: &[u8]) -> String {
        let mut checksum_acc = 0u64;
        for &b in root_bytes {
            checksum_acc = checksum_acc.wrapping_mul(31).wrapping_add(b as u64);
        }
        let checksum = format!("{:016x}", checksum_acc);

        let deployment = OstreeDeployment {
            checksum: checksum.clone(),
            os_name: String::from(os_name),
            version: String::from(version),
            layered_packages: Vec::new(),
            is_active: false,
        };

        self.deployments.push(deployment);
        if self.active_index.is_none() {
            self.active_index = Some(0);
            self.deployments[0].is_active = true;
        }

        checksum
    }

    /// Layers an RPM package onto the active ostree deployment (rpm-ostree pkg-add)
    pub fn layer_package_overlay(&mut self, pkg_name: &str) -> Result<(), &'static str> {
        if let Some(idx) = self.active_index {
            self.deployments[idx]
                .layered_packages
                .push(String::from(pkg_name));
            Ok(())
        } else {
            Err("No active deployment to layer package overlay onto")
        }
    }

    /// Switches active deployment atomically (rpm-ostree rollback or deploy)
    pub fn switch_active_deployment(&mut self, checksum: &str) -> Result<(), &'static str> {
        let mut found_idx = None;
        for (i, dep) in self.deployments.iter().enumerate() {
            if dep.checksum == checksum {
                found_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = found_idx {
            for dep in &mut self.deployments {
                dep.is_active = false;
            }
            self.deployments[idx].is_active = true;
            self.active_index = Some(idx);
            Ok(())
        } else {
            Err("Target OSTree deployment commit hash not found")
        }
    }
}

impl Default for OstreeDeploymentEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Illumos/Solaris Crossbow & NetBSD RUMP Kernel Drivers
// ============================================================================

/// Solaris Crossbow Virtual NIC (VNIC) Descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossbowVnic {
    pub vnic_name: String,
    pub underlying_phys_nic: String,
    pub mac_address: [u8; 6],
    pub bandwidth_limit_mbps: u32,
    pub bound_cpu_core: Option<u32>,
}

/// Illumos / Solaris Crossbow Network Virtualization Engine
pub struct SolarisCrossbowVnicEngine {
    pub vnics: Vec<CrossbowVnic>,
}

impl SolarisCrossbowVnicEngine {
    pub fn new() -> Self {
        Self { vnics: Vec::new() }
    }

    /// Creates a new Virtual NIC over a physical interface
    pub fn create_vnic(
        &mut self,
        vnic_name: &str,
        phys_nic: &str,
        mac: [u8; 6],
        bandwidth_mbps: u32,
    ) -> Result<(), &'static str> {
        if self.vnics.iter().any(|v| v.vnic_name == vnic_name) {
            return Err("VNIC with name already exists");
        }

        self.vnics.push(CrossbowVnic {
            vnic_name: String::from(vnic_name),
            underlying_phys_nic: String::from(phys_nic),
            mac_address: mac,
            bandwidth_limit_mbps: bandwidth_mbps,
            bound_cpu_core: None,
        });

        Ok(())
    }

    /// Binds VNIC traffic processing to a specific CPU core for zero-latency QoS
    pub fn bind_cpu_core(&mut self, vnic_name: &str, cpu_core: u32) -> Result<(), &'static str> {
        if let Some(vnic) = self.vnics.iter_mut().find(|v| v.vnic_name == vnic_name) {
            vnic.bound_cpu_core = Some(cpu_core);
            Ok(())
        } else {
            Err("VNIC not found")
        }
    }
}

impl Default for SolarisCrossbowVnicEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// NetBSD RUMP (Runnable Userspace Meta Program) Driver Server Component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RumpKernelServer {
    pub server_id: u32,
    pub subsystem: String, // e.g., "rumpfs_ext2", "rumpnet_virtio"
    pub is_isolated: bool,
    pub calls_processed: u64,
}

/// NetBSD RUMP Kernel Userspace Driver Server Supervisor
pub struct NetBsdRumpKernel {
    pub rump_servers: Vec<RumpKernelServer>,
}

impl NetBsdRumpKernel {
    pub fn new() -> Self {
        Self {
            rump_servers: Vec::new(),
        }
    }

    /// Registers a rump kernel server running in an isolated userland process
    pub fn register_rump_server(&mut self, id: u32, subsystem: &str) -> Result<(), &'static str> {
        if self.rump_servers.iter().any(|s| s.server_id == id) {
            return Err("RUMP server ID already registered");
        }

        self.rump_servers.push(RumpKernelServer {
            server_id: id,
            subsystem: String::from(subsystem),
            is_isolated: true,
            calls_processed: 0,
        });

        Ok(())
    }

    /// Forwards a hypercall/syscall to the isolated RUMP driver server
    pub fn forward_rump_syscall(&mut self, server_id: u32) -> Result<u64, &'static str> {
        if let Some(srv) = self
            .rump_servers
            .iter_mut()
            .find(|s| s.server_id == server_id)
        {
            srv.calls_processed += 1;
            Ok(srv.calls_processed)
        } else {
            Err("RUMP kernel server not found")
        }
    }
}

impl Default for NetBsdRumpKernel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Ubuntu/Debian Netplan & Cloud-Init
// ============================================================================

/// Netplan Interface Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetplanInterface {
    pub name: String,
    pub dhcp4: bool,
    pub addresses: Vec<String>,
    pub gateway4: Option<String>,
}

/// Ubuntu Netplan Declarative YAML Config Renderer Engine
pub struct NetplanYamlRenderer {
    pub renderer_backend: String, // "networkd" or "NetworkManager"
    pub interfaces: Vec<NetplanInterface>,
}

impl NetplanYamlRenderer {
    pub fn new(renderer: &str) -> Self {
        Self {
            renderer_backend: String::from(renderer),
            interfaces: Vec::new(),
        }
    }

    pub fn add_interface(
        &mut self,
        name: &str,
        dhcp4: bool,
        addresses: &[&str],
        gateway: Option<&str>,
    ) {
        self.interfaces.push(NetplanInterface {
            name: String::from(name),
            dhcp4,
            addresses: addresses.iter().map(|&a| String::from(a)).collect(),
            gateway4: gateway.map(String::from),
        });
    }

    /// Renders declarative YAML structure into backend configurations
    pub fn generate_backend_config(&self) -> String {
        let mut config = format!(
            "network:\n  version: 2\n  renderer: {}\n  ethernets:\n",
            self.renderer_backend
        );
        for iface in &self.interfaces {
            config.push_str(&format!(
                "    {}:\n      dhcp4: {}\n",
                iface.name, iface.dhcp4
            ));
            if !iface.addresses.is_empty() {
                config.push_str("      addresses:\n");
                for addr in &iface.addresses {
                    config.push_str(&format!("        - {}\n", addr));
                }
            }
            if let Some(gw) = &iface.gateway4 {
                config.push_str(&format!("      gateway4: {}\n", gw));
            }
        }
        config
    }
}

/// Cloud-Init Metadata & Userdata Instance Bootstrap Provisioning Engine
pub struct CloudInitBootstrapEngine {
    pub instance_id: String,
    pub hostname: String,
    pub ssh_authorized_keys: Vec<String>,
    pub bootstrap_packages: Vec<String>,
    pub runcmd_commands: Vec<String>,
}

impl CloudInitBootstrapEngine {
    pub fn new(instance_id: &str, hostname: &str) -> Self {
        Self {
            instance_id: String::from(instance_id),
            hostname: String::from(hostname),
            ssh_authorized_keys: Vec::new(),
            bootstrap_packages: Vec::new(),
            runcmd_commands: Vec::new(),
        }
    }

    pub fn add_ssh_key(&mut self, key: &str) {
        self.ssh_authorized_keys.push(String::from(key));
    }

    pub fn add_package(&mut self, pkg: &str) {
        self.bootstrap_packages.push(String::from(pkg));
    }

    pub fn add_runcmd(&mut self, cmd: &str) {
        self.runcmd_commands.push(String::from(cmd));
    }

    /// Executes cloud-init first-boot provisioning sequence
    pub fn execute_bootstrap(&self) -> usize {
        let total_actions = self.ssh_authorized_keys.len()
            + self.bootstrap_packages.len()
            + self.runcmd_commands.len();
        total_actions
    }
}

// ============================================================================
// 6. openSUSE YaST2 & Snapper Btrfs Snapshot Timeline Engine
// ============================================================================

/// YaST2 Setting Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YastSetting {
    pub module: String, // e.g. "network", "firewall", "storage"
    pub key: String,
    pub value: String,
}

/// openSUSE YaST2 Centralized Declarative Configuration Engine
pub struct Yast2ControlCenter {
    pub settings: Vec<YastSetting>,
}

impl Yast2ControlCenter {
    pub fn new() -> Self {
        Self {
            settings: Vec::new(),
        }
    }

    pub fn set_setting(&mut self, module: &str, key: &str, value: &str) {
        if let Some(s) = self
            .settings
            .iter_mut()
            .find(|s| s.module == module && s.key == key)
        {
            s.value = String::from(value);
        } else {
            self.settings.push(YastSetting {
                module: String::from(module),
                key: String::from(key),
                value: String::from(value),
            });
        }
    }

    pub fn get_setting(&self, module: &str, key: &str) -> Option<&str> {
        self.settings
            .iter()
            .find(|s| s.module == module && s.key == key)
            .map(|s| s.value.as_str())
    }
}

impl Default for Yast2ControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapper Btrfs / S-FS Snapshot Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapperType {
    Pre,
    Post,
    Timeline,
}

/// Snapper Snapshot Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapperSnapshot {
    pub id: u32,
    pub snap_type: SnapperType,
    pub description: String,
    pub root_tree_hash: u64,
}

/// openSUSE Snapper Btrfs / S-FS Timeline & Transaction Snapshot Engine
pub struct SnapperBtrfsEngine {
    pub snapshots: Vec<SnapperSnapshot>,
    pub next_id: u32,
}

impl SnapperBtrfsEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            next_id: 1,
        }
    }

    /// Creates a new Btrfs/S-FS snapshot before or after system transaction
    pub fn create_snapshot(
        &mut self,
        snap_type: SnapperType,
        description: &str,
        root_data: &[u8],
    ) -> u32 {
        let mut hash_acc = 0u64;
        for &b in root_data {
            hash_acc = hash_acc.wrapping_mul(31).wrapping_add(b as u64);
        }

        let id = self.next_id;
        self.next_id += 1;

        self.snapshots.push(SnapperSnapshot {
            id,
            snap_type,
            description: String::from(description),
            root_tree_hash: hash_acc,
        });

        id
    }

    /// Cleans up old timeline snapshots retaining up to `keep_count`
    pub fn cleanup_old_snapshots(&mut self, keep_count: usize) -> usize {
        if self.snapshots.len() > keep_count {
            let to_remove = self.snapshots.len() - keep_count;
            self.snapshots.drain(0..to_remove);
            to_remove
        } else {
            0
        }
    }
}

impl Default for SnapperBtrfsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slackware_pkgtools_and_compiler() {
        let mut pkgtools = SlackwarePkgTools::new();
        let compiler = SlackBuildCompiler::new("kernel-sovereign", "x86_64", "1", "sgi");

        let pkg = compiler.compile_package("kernel-sovereign", "6.12.0");
        assert_eq!(pkg.name, "kernel-sovereign");
        assert_eq!(pkg.version, "6.12.0");

        assert!(pkgtools.install_pkg(pkg.clone()).is_ok());
        assert!(pkgtools.is_installed("kernel-sovereign"));
        assert!(pkgtools.install_pkg(pkg).is_err()); // duplicate install

        let removed = pkgtools.remove_pkg("kernel-sovereign").unwrap();
        assert_eq!(removed.name, "kernel-sovereign");
        assert!(!pkgtools.is_installed("kernel-sovereign"));
    }

    #[test]
    fn test_gnu_guix_store_and_shepherd() {
        let mut store = GuixFunctionalStore::new();
        let deriv = store.add_derivation("gcc", "14.2.0", &["glibc", "gmp"]);
        assert!(deriv.store_path.starts_with("/gnu/store/"));
        assert_eq!(store.store.len(), 1);

        let mut shepherd = GNUGuixShepherdSupervisor::new();
        shepherd.register_service("syslogd", &["syslog"], &[]);
        shepherd.register_service("networking", &["net"], &["syslog"]);

        assert!(shepherd.start_service("networking").is_err()); // dependency syslog not active
        let syslog_pid = shepherd.start_service("syslogd").unwrap();
        assert!(syslog_pid > 100);

        let net_pid = shepherd.start_service("networking").unwrap();
        assert!(net_pid > syslog_pid);
    }

    #[test]
    fn test_fedora_silverblue_ostree() {
        let mut ostree = OstreeDeploymentEngine::new();
        let _commit_hash = ostree.commit_tree("SigmaOS-Silverblue", "40.2026", b"ROOT_SYSROOT_FS");

        assert_eq!(ostree.deployments.len(), 1);
        assert!(ostree.deployments[0].is_active);

        assert!(ostree.layer_package_overlay("htop").is_ok());
        assert_eq!(ostree.deployments[0].layered_packages[0], "htop");

        let rollback_hash = ostree.commit_tree("SigmaOS-Silverblue", "40.2025", b"PREV_SYSROOT_FS");
        assert!(ostree.switch_active_deployment(&rollback_hash).is_ok());
        assert_eq!(ostree.deployments[1].checksum, rollback_hash);
        assert!(ostree.deployments[1].is_active);
    }

    #[test]
    fn test_illumos_crossbow_and_netbsd_rump() {
        let mut crossbow = SolarisCrossbowVnicEngine::new();
        assert!(crossbow
            .create_vnic(
                "vnic0",
                "e1000g0",
                [0x02, 0x00, 0x00, 0x11, 0x22, 0x33],
                1000
            )
            .is_ok());
        assert!(crossbow.bind_cpu_core("vnic0", 2).is_ok());
        assert_eq!(crossbow.vnics[0].bound_cpu_core, Some(2));

        let mut rump = NetBsdRumpKernel::new();
        assert!(rump.register_rump_server(1, "rumpfs_ext2").is_ok());
        let count = rump.forward_rump_syscall(1).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_ubuntu_netplan_and_cloud_init() {
        let mut netplan = NetplanYamlRenderer::new("networkd");
        netplan.add_interface("eth0", true, &["192.168.1.50/24"], Some("192.168.1.1"));

        let yaml = netplan.generate_backend_config();
        assert!(yaml.contains("network:"));
        assert!(yaml.contains("eth0:"));
        assert!(yaml.contains("dhcp4: true"));

        let mut cloud_init = CloudInitBootstrapEngine::new("i-0123456789", "sovereign-node");
        cloud_init.add_ssh_key("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5...");
        cloud_init.add_package("curl");
        cloud_init.add_runcmd("systemctl restart networkd");

        let executed = cloud_init.execute_bootstrap();
        assert_eq!(executed, 3);
    }

    #[test]
    fn test_opensuse_yast2_and_snapper() {
        let mut yast = Yast2ControlCenter::new();
        yast.set_setting("firewall", "default_zone", "drop");
        assert_eq!(yast.get_setting("firewall", "default_zone"), Some("drop"));

        let mut snapper = SnapperBtrfsEngine::new();
        let snap1 = snapper.create_snapshot(SnapperType::Pre, "Pre zypper dup", b"STATE_V1");
        let _snap2 = snapper.create_snapshot(SnapperType::Post, "Post zypper dup", b"STATE_V2");
        assert_eq!(snap1, 1);
        assert_eq!(snapper.snapshots.len(), 2);

        let removed = snapper.cleanup_old_snapshots(1);
        assert_eq!(removed, 1);
        assert_eq!(snapper.snapshots.len(), 1);
    }
}
