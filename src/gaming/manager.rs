//! SigmaOS Gaming Manager
//!
//! Inspired by: Omarchy gaming (omancy-install-gaming-steam, omancy-install-gaming-heroic, omancy-install-gaming-lutris, omancy-install-gaming-retroarch, omancy-install-gaming-geforce-now, omancy-install-gaming-battlenet, omancy-install-gaming-xbox-cloud)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, PRACTICAL_LICENSING_AWARENESS, COMMUNITY_ORIENTED_DEVELOPMENT,
//! PLUG_IN_DRIVER_MODEL.

/// Gaming platform
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamingPlatform {
    Steam,
    Heroic,
    Lutris,
    RetroArch,
    GeForceNow,
    BattleNet,
    XboxCloud,
    Custom(String),
}

/// Gaming platform status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformStatus {
    NotInstalled,
    Installed,
    Running,
    Configured,
    Error(String),
}

/// Gaming platform config
#[derive(Debug, Clone)]
pub struct GamingPlatformConfig {
    pub platform: GamingPlatform,
    pub status: PlatformStatus,
    pub install_date: Option<u64>,
    pub config_path: Option<String>,
    pub requires_sudo: bool,
}

/// Gaming manager
pub struct GamingManager {
    pub platforms: HashMap<GamingPlatform, GamingPlatformConfig>,
}

impl GamingManager {
    pub fn new() -> Self {
        Self {
            platforms: HashMap::new(),
        }
    }

    /// Install a gaming platform
    pub fn install(&mut self, platform: GamingPlatform) -> Result<(), String> {
        if self.platforms.contains_key(&platform) {
            return Err(format!("Platform {:?} already installed", platform));
        }

        let config = GamingPlatformConfig {
            platform: platform.clone(),
            status: PlatformStatus::Installed,
            install_date: Some(now_secs()),
            config_path: Some(format!("/etc/sigmaos/gaming/{}", platform_name(&platform))),
            requires_sudo: requires_sudo(&platform),
        };

        self.platforms.insert(platform, config);
        Ok(())
    }

    /// Remove a gaming platform
    pub fn remove(&mut self, platform: &GamingPlatform) -> Result<(), String> {
        if !self.platforms.contains_key(platform) {
            return Err(format!("Platform {:?} not installed", platform));
        }
        self.platforms.remove(platform);
        Ok(())
    }

    /// Launch a platform
    pub fn launch(&mut self, platform: &GamingPlatform) -> Result<(), String> {
        let config = self
            .platforms
            .get_mut(platform)
            .ok_or_else(|| format!("Platform {:?} not installed", platform))?;

        if config.status == PlatformStatus::Running {
            return Err(format!("Platform {:?} already running", platform));
        }

        config.status = PlatformStatus::Running;
        Ok(())
    }

    /// Stop a platform
    pub fn stop(&mut self, platform: &GamingPlatform) -> Result<(), String> {
        let config = self
            .platforms
            .get_mut(platform)
            .ok_or_else(|| format!("Platform {:?} not installed", platform))?;

        config.status = PlatformStatus::Installed;
        Ok(())
    }

    /// Configure a platform
    pub fn configure(&mut self, platform: &GamingPlatform) -> Result<(), String> {
        let config = self
            .platforms
            .get_mut(platform)
            .ok_or_else(|| format!("Platform {:?} not installed", platform))?;

        config.status = PlatformStatus::Configured;
        Ok(())
    }

    /// List installed platforms
    pub fn list_installed(&self) -> Vec<&GamingPlatformConfig> {
        self.platforms.values().collect()
    }

    /// Get platform status
    pub fn get_status(&self, platform: &GamingPlatform) -> Option<&PlatformStatus> {
        self.platforms.get(platform).map(|c| &c.status)
    }

    /// Is platform running
    pub fn is_running(&self, platform: &GamingPlatform) -> bool {
        matches!(
            self.get_status(platform),
            Some(PlatformStatus::Running)
        )
    }

    /// Platform count
    pub fn platform_count(&self) -> usize {
        self.platforms.len()
    }
}

impl Default for GamingManager {
    fn default() -> Self {
        Self::new()
    }
}

fn platform_name(platform: &GamingPlatform) -> String {
    match platform {
        GamingPlatform::Steam => "steam".to_string(),
        GamingPlatform::Heroic => "heroic".to_string(),
        GamingPlatform::Lutris => "lutris".to_string(),
        GamingPlatform::RetroArch => "retroarch".to_string(),
        GamingPlatform::GeForceNow => "geforce-now".to_string(),
        GamingPlatform::BattleNet => "battlenet".to_string(),
        GamingPlatform::XboxCloud => "xbox-cloud".to_string(),
        GamingPlatform::Custom(name) => name.clone(),
    }
}

fn requires_sudo(platform: &GamingPlatform) -> bool {
    matches!(
        platform,
        GamingPlatform::Steam
            | GamingPlatform::Lutris
            | GamingPlatform::RetroArch
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
    fn test_install_platform() {
        let mut mgr = GamingManager::new();
        assert!(mgr.install(GamingPlatform::Steam).is_ok());
        assert_eq!(mgr.platform_count(), 1);
    }

    #[test]
    fn test_install_duplicate_fails() {
        let mut mgr = GamingManager::new();
        mgr.install(GamingPlatform::Steam).ok();
        let result = mgr.install(GamingPlatform::Steam);
        assert!(result.is_err());
    }

    #[test]
    fn test_launch_stop_platform() {
        let mut mgr = GamingManager::new();
        mgr.install(GamingPlatform::Heroic).unwrap();
        assert!(mgr.launch(&GamingPlatform::Heroic).is_ok());
        assert!(mgr.is_running(&GamingPlatform::Heroic));
        assert!(mgr.stop(&GamingPlatform::Heroic).is_ok());
        assert!(!mgr.is_running(&GamingPlatform::Heroic));
    }

    #[test]
    fn test_launch_not_installed_fails() {
        let mut mgr = GamingManager::new();
        assert!(mgr.launch(&GamingPlatform::Steam).is_err());
    }

    #[test]
    fn test_remove_platform() {
        let mut mgr = GamingManager::new();
        mgr.install(GamingPlatform::Lutris).unwrap();
        assert!(mgr.remove(&GamingPlatform::Lutris).is_ok());
        assert_eq!(mgr.platform_count(), 0);
    }

    #[test]
    fn test_configure_platform() {
        let mut mgr = GamingManager::new();
        mgr.install(GamingPlatform::RetroArch).unwrap();
        assert!(mgr.configure(&GamingPlatform::RetroArch).is_ok());
        assert_eq!(
            mgr.get_status(&GamingPlatform::RetroArch),
            Some(&PlatformStatus::Configured)
        );
    }

    #[test]
    fn test_requires_sudo() {
        assert!(requires_sudo(&GamingPlatform::Steam));
        assert!(!requires_sudo(&GamingPlatform::Heroic));
    }
}