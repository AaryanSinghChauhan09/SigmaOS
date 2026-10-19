//! SigmaOS Cloud Services Manager
//!
//! Inspired by: Omarchy services (omancy-install-service-tailscale, omancy-install-service-dropbox, omancy-install-service-1password, omancy-install-service-signal, omancy-install-service-spotify, omancy-install-service-sunshine, omancy-install-service-nordvpn)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, PRACTICAL_LICENSING_AWARENESS, COMMUNITY_ORIENTED_DEVELOPMENT,
//! PLUG_IN_DRIVER_MODEL, COMPONENT_ISOLATION.

/// Cloud service type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudService {
    Tailscale,
    Dropbox,
    OnePassword,
    Signal,
    Spotify,
    Sunshine,
    NordVpn,
    Custom(String),
}

/// Service status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStatus {
    NotInstalled,
    Installed,
    Running,
    Stopped,
    Error(String),
}

/// Cloud service config
#[derive(Debug, Clone)]
pub struct CloudServiceConfig {
    pub service: CloudService,
    pub status: ServiceStatus,
    pub install_date: Option<u64>,
    pub config_path: Option<String>,
    pub auto_start: bool,
    pub requires_sudo: bool,
}

/// Cloud services manager
pub struct CloudServicesManager {
    pub services: HashMap<CloudService, CloudServiceConfig>,
}

impl CloudServicesManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Install a service
    pub fn install(&mut self, service: CloudService) -> Result<(), String> {
        if self.services.contains_key(&service) {
            return Err(format!("Service {:?} already installed", service));
        }

        let config = CloudServiceConfig {
            service: service.clone(),
            status: ServiceStatus::Installed,
            install_date: Some(now_secs()),
            config_path: Some(format!("/etc/sigmaos/services/{}", service_name(&service))),
            auto_start: false,
            requires_sudo: requires_sudo(&service),
        };

        self.services.insert(service, config);
        Ok(())
    }

    /// Remove a service
    pub fn remove(&mut self, service: &CloudService) -> Result<(), String> {
        if !self.services.contains_key(service) {
            return Err(format!("Service {:?} not installed", service));
        }
        self.services.remove(service);
        Ok(())
    }

    /// Start a service
    pub fn start(&mut self, service: &CloudService) -> Result<(), String> {
        let config = self
            .services
            .get_mut(service)
            .ok_or_else(|| format!("Service {:?} not installed", service))?;

        if config.status == ServiceStatus::Running {
            return Err(format!("Service {:?} already running", service));
        }

        config.status = ServiceStatus::Running;
        Ok(())
    }

    /// Stop a service
    pub fn stop(&mut self, service: &CloudService) -> Result<(), String> {
        let config = self
            .services
            .get_mut(service)
            .ok_or_else(|| format!("Service {:?} not installed", service))?;

        if config.status == ServiceStatus::Stopped || config.status == ServiceStatus::NotInstalled {
            return Err(format!("Service {:?} not running", service));
        }

        config.status = ServiceStatus::Stopped;
        Ok(())
    }

    /// Set auto-start
    pub fn set_auto_start(&mut self, service: &CloudService, enabled: bool) -> Result<(), String> {
        let config = self
            .services
            .get_mut(service)
            .ok_or_else(|| format!("Service {:?} not installed", service))?;
        config.auto_start = enabled;
        Ok(())
    }

    /// List installed services
    pub fn list_installed(&self) -> Vec<&CloudServiceConfig> {
        self.services.values().collect()
    }

    /// Get service status
    pub fn get_status(&self, service: &CloudService) -> Option<&ServiceStatus> {
        self.services.get(service).map(|c| &c.status)
    }

    /// Is service running
    pub fn is_running(&self, service: &CloudService) -> bool {
        matches!(
            self.get_status(service),
            Some(ServiceStatus::Running)
        )
    }

    /// Service count
    pub fn service_count(&self) -> usize {
        self.services.len()
    }
}

impl Default for CloudServicesManager {
    fn default() -> Self {
        Self::new()
    }
}

fn service_name(service: &CloudService) -> String {
    match service {
        CloudService::Tailscale => "tailscale".to_string(),
        CloudService::Dropbox => "dropbox".to_string(),
        CloudService::OnePassword => "1password".to_string(),
        CloudService::Signal => "signal".to_string(),
        CloudService::Spotify => "spotify".to_string(),
        CloudService::Sunshine => "sunshine".to_string(),
        CloudService::NordVpn => "nordvpn".to_string(),
        CloudService::Custom(name) => name.clone(),
    }
}

fn requires_sudo(service: &CloudService) -> bool {
    matches!(
        service,
        CloudService::Tailscale
            | CloudService::OnePassword
            | CloudService::Signal
            | CloudService::Sunshine
            | CloudService::NordVpn
    )
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_service() {
        let mut mgr = CloudServicesManager::new();
        assert!(mgr.install(CloudService::Tailscale).is_ok());
        assert_eq!(mgr.service_count(), 1);
    }

    #[test]
    fn test_install_duplicate_fails() {
        let mut mgr = CloudServicesManager::new();
        mgr.install(CloudService::Tailscale).ok();
        let result = mgr.install(CloudService::Tailscale);
        assert!(result.is_err());
    }

    #[test]
    fn test_start_stop_service() {
        let mut mgr = CloudServicesManager::new();
        mgr.install(CloudService::Spotify).unwrap();
        assert!(mgr.start(&CloudService::Spotify).is_ok());
        assert!(mgr.is_running(&CloudService::Spotify));
        assert!(mgr.stop(&CloudService::Spotify).is_ok());
        assert!(!mgr.is_running(&CloudService::Spotify));
    }

    #[test]
    fn test_start_not_installed_fails() {
        let mut mgr = CloudServicesManager::new();
        assert!(mgr.start(&CloudService::Tailscale).is_err());
    }

    #[test]
    fn test_remove_service() {
        let mut mgr = CloudServicesManager::new();
        mgr.install(CloudService::Dropbox).unwrap();
        assert!(mgr.remove(&CloudService::Dropbox).is_ok());
        assert_eq!(mgr.service_count(), 0);
    }

    #[test]
    fn test_auto_start() {
        let mut mgr = CloudServicesManager::new();
        mgr.install(CloudService::Signal).unwrap();
        assert!(mgr.set_auto_start(&CloudService::Signal, true).is_ok());
        assert!(mgr.services.get(&CloudService::Signal).unwrap().auto_start);
    }

    #[test]
    fn test_requires_sudo() {
        assert!(requires_sudo(&CloudService::Tailscale));
        assert!(!requires_sudo(&CloudService::Spotify));
    }
}