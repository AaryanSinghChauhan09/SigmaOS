// SigmaOS Distro Compatibility Layer
/// Gentoo Linux & SysVinit runlevels Architecture Absorption for SigmaOS
/// Implements Portage-grade ebuild compilation recipes, global & local compile-time USE Flags,
/// and OpenRC runlevel dependency-resolved parallel process/daemon supervision.
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

// =========================================================================
// 1. USE FLAGS (Gentoo-grade Compile-Time Feature Optimization)
// =========================================================================
#[derive(Debug, Clone)]
pub struct UseFlagManager {
    pub enabled_flags: Vec<String>,
}

impl UseFlagManager {
    pub fn parse(use_env: &str) -> Self {
        let mut enabled_flags = Vec::new();
        for flag in use_env.split_whitespace() {
            if !flag.starts_with('-') {
                enabled_flags.push(flag.to_string());
            }
        }
        UseFlagManager { enabled_flags }
    }

    /// Queries if a given feature flag is active under the current optimization profile
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.enabled_flags.contains(&flag.to_string())
    }
}

// =========================================================================
// 2. OPENRC INIT SYSTEM (OpenRC Dependency-Based Service Supervisor)
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenRcRunlevel {
    PowerOff = 0,   // Runlevel 0: Halt / PowerOff / poweroff.target
    SingleUser = 1, // Runlevel 1: Single-user rescue mode / rescue.target (minimal services)
    MultiUser = 3, // Runlevel 3: Multi-user command-line console mode / multi-user.target (networking active)
    Graphical = 5, // Runlevel 5: Multi-user graphical display mode / graphical.target (X11 / Wayland / Zenith)
    Reboot = 6,    // Runlevel 6: Reboot / reboot.target
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Started,
    Failed,
}

#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub dependencies: Vec<String>, // Services required before starting this one
    pub runlevels: Vec<OpenRcRunlevel>,
    pub status: ServiceStatus,
}

impl OpenRcService {
    pub fn new(name: &str) -> Self {
        OpenRcService {
            name: name.to_string(),
            dependencies: Vec::new(),
            runlevels: Vec::new(),
            status: ServiceStatus::Stopped,
        }
    }

    pub fn with_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }

    pub fn with_runlevel(mut self, runlevel: OpenRcRunlevel) -> Self {
        self.runlevels.push(runlevel);
        self
    }
}

pub struct OpenRcManager {
    pub services: Vec<OpenRcService>,
    pub current_runlevel: OpenRcRunlevel,
}

impl OpenRcManager {
    pub fn new() -> Self {
        OpenRcManager {
            services: Vec::new(),
            current_runlevel: OpenRcRunlevel::SingleUser,
        }
    }

    pub fn register_service(&mut self, service: OpenRcService) {
        self.services.push(service);
    }

    /// Transitions init state runlevels, resolving and starting services in parallel dependency orders
    pub fn transition_to_runlevel(
        &mut self,
        target_runlevel: OpenRcRunlevel,
    ) -> Result<(), &'static str> {
        self.current_runlevel = target_runlevel;

        // If transitioning to PowerOff (0) or Reboot (6), we stop all services in reverse dependency order
        if target_runlevel == OpenRcRunlevel::PowerOff || target_runlevel == OpenRcRunlevel::Reboot
        {
            for i in (0..self.services.len()).rev() {
                self.services[i].status = ServiceStatus::Stopped;
            }
            return Ok(());
        }

        // Collect all services targeted for this runlevel and any preceding runlevel
        // e.g. Graphical runlevel 5 should also include all standard MultiUser runlevel 3 and SingleUser runlevel 1 services!
        let mut target_services = Vec::new();
        for s in &self.services {
            let mut include = false;
            for &rl in &s.runlevels {
                if (rl as u8) <= (target_runlevel as u8) {
                    include = true;
                    break;
                }
            }
            if include {
                target_services.push(s.name.clone());
            }
        }

        let mut started_something = true;
        while started_something {
            started_something = false;

            for i in 0..self.services.len() {
                // If service is stopped and belongs to target set
                if self.services[i].status == ServiceStatus::Stopped
                    && target_services.contains(&self.services[i].name)
                {
                    // Verify if all dependencies are already started
                    let mut deps_satisfied = true;
                    for dep in &self.services[i].dependencies {
                        let mut dep_started = false;
                        for s in &self.services {
                            if &s.name == dep && s.status == ServiceStatus::Started {
                                dep_started = true;
                                break;
                            }
                        }
                        if !dep_started {
                            deps_satisfied = false;
                            break;
                        }
                    }

                    if deps_satisfied {
                        self.services[i].status = ServiceStatus::Started;
                        started_something = true;
                    }
                }
            }
        }

        // Verify if any target service failed to satisfy dependencies
        for s in &self.services {
            if s.status == ServiceStatus::Stopped && target_services.contains(&s.name) {
                return Err("Circular dependency or missing service dependencies in OpenRC!");
            }
        }

        Ok(())
    }
}

impl Default for OpenRcManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. PORTAGE & EBUILDS (Gentoo Portage-grade Emerge Engine)
// =========================================================================
#[derive(Debug, Clone)]
pub struct EbuildPackage {
    pub name: String,
    pub version: String,
    pub use_conditional_deps: Vec<(String, String)>, // (USE-flag, dependent package)
    pub compile_flags: Vec<String>,
}

impl EbuildPackage {
    pub fn new(name: &str, version: &str) -> Self {
        EbuildPackage {
            name: name.to_string(),
            version: version.to_string(),
            use_conditional_deps: Vec::new(),
            compile_flags: Vec::new(),
        }
    }

    pub fn with_use_dep(mut self, flag: &str, dep_pkg: &str) -> Self {
        self.use_conditional_deps
            .push((flag.to_string(), dep_pkg.to_string()));
        self
    }

    pub fn with_compile_flag(mut self, flag: &str) -> Self {
        self.compile_flags.push(flag.to_string());
        self
    }
}

pub struct PortageEngine {
    pub use_manager: UseFlagManager,
    pub installed_packages: Vec<String>,
}

impl PortageEngine {
    pub fn new(use_flags: UseFlagManager) -> Self {
        PortageEngine {
            use_manager: use_flags,
            installed_packages: Vec::new(),
        }
    }

    /// Simulates 'emerge' package compilation and installation checking compile-time USE-flags
    pub fn emerge(&mut self, ebuild: &EbuildPackage) -> Result<(), &'static str> {
        // 1. Resolve conditional compile-time dependencies based on current USE configurations
        for (flag, dep) in &ebuild.use_conditional_deps {
            if self.use_manager.is_enabled(flag) {
                if !self.installed_packages.contains(dep) {
                    return Err("Portage error: compile-time dependency unsatisfied. Run emerge on it first.");
                }
            }
        }

        // 2. Simulate native optimization compilation (e.g. -march=native -O3)
        let mut compile_cmd = format!("gcc -O3 -march=native ");
        for flag in &ebuild.compile_flags {
            compile_cmd.push_str(flag);
            compile_cmd.push(' ');
        }

        // Compile and install successfully
        self.installed_packages.push(ebuild.name.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gentoo_use_flags() {
        let use_manager = UseFlagManager::parse("ssl x509 -ipv6 threads");
        assert!(use_manager.is_enabled("ssl"));
        assert!(use_manager.is_enabled("threads"));
        assert!(!use_manager.is_enabled("ipv6")); // explicitly disabled with "-"
    }

    #[test]
    fn test_openrc_init_runlevel_dependencies() {
        let mut manager = OpenRcManager::new();

        // Register hardware clock, network, and GUI services
        let udev = OpenRcService::new("udev").with_runlevel(OpenRcRunlevel::SingleUser);

        let localmount = OpenRcService::new("localmount")
            .with_dependency("udev")
            .with_runlevel(OpenRcRunlevel::SingleUser);

        let dhcpcd = OpenRcService::new("dhcpcd")
            .with_dependency("localmount")
            .with_runlevel(OpenRcRunlevel::MultiUser);

        let zenith = OpenRcService::new("zenith")
            .with_dependency("dhcpcd")
            .with_runlevel(OpenRcRunlevel::Graphical);

        manager.register_service(udev);
        manager.register_service(localmount);
        manager.register_service(dhcpcd);
        manager.register_service(zenith);

        // 1. Transition to SingleUser (Runlevel 1) (starts only udev then localmount)
        manager
            .transition_to_runlevel(OpenRcRunlevel::SingleUser)
            .unwrap();
        assert_eq!(manager.services[0].status, ServiceStatus::Started); // udev
        assert_eq!(manager.services[1].status, ServiceStatus::Started); // localmount
        assert_eq!(manager.services[2].status, ServiceStatus::Stopped); // dhcpcd (runlevel 3)
        assert_eq!(manager.services[3].status, ServiceStatus::Stopped); // zenith (runlevel 5)

        // 2. Transition to MultiUser (Runlevel 3) (starts dhcpcd network daemon)
        manager
            .transition_to_runlevel(OpenRcRunlevel::MultiUser)
            .unwrap();
        assert_eq!(manager.services[2].status, ServiceStatus::Started); // dhcpcd
        assert_eq!(manager.services[3].status, ServiceStatus::Stopped); // zenith still stopped

        // 3. Transition to Graphical (Runlevel 5) (starts display compositor zenith)
        manager
            .transition_to_runlevel(OpenRcRunlevel::Graphical)
            .unwrap();
        assert_eq!(manager.services[3].status, ServiceStatus::Started); // zenith

        // 4. Transition to PowerOff (Runlevel 0) (Halt - stops all services cleanly in reverse order)
        manager
            .transition_to_runlevel(OpenRcRunlevel::PowerOff)
            .unwrap();
        assert!(manager
            .services
            .iter()
            .all(|s| s.status == ServiceStatus::Stopped));
    }

    #[test]
    fn test_portage_emerge_use_conditional_compilation() {
        let use_flags = UseFlagManager::parse("ssl zlib");
        let mut portage = PortageEngine::new(use_flags);

        // Define openssl ebuild package
        let openssl = EbuildPackage::new("dev-libs/openssl", "3.1.2");
        portage.emerge(&openssl).unwrap();

        // Define nginx ebuild with compile-time dependency on openssl if "ssl" flag is enabled
        let nginx = EbuildPackage::new("www-servers/nginx", "1.25.1")
            .with_use_dep("ssl", "dev-libs/openssl")
            .with_compile_flag("-DHTTP_SSL");

        // Compiling and installing nginx should succeed because dev-libs/openssl was already emerged
        assert!(portage.emerge(&nginx).is_ok());
        assert!(portage
            .installed_packages
            .contains(&"www-servers/nginx".to_string()));
    }

    #[test]
    fn test_portage_emerge_failed_dependencies() {
        let use_flags = UseFlagManager::parse("ssl");
        let mut portage = PortageEngine::new(use_flags);

        // Define nginx with conditional "ssl" dependency on dev-libs/openssl
        let nginx = EbuildPackage::new("www-servers/nginx", "1.25.1")
            .with_use_dep("ssl", "dev-libs/openssl");

        // emerge nginx should fail immediately because dev-libs/openssl is not compiled or installed yet
        assert!(portage.emerge(&nginx).is_err());
    }
}
