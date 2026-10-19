//! SigmaOS Restart Manager
//!
//! Inspired by: Omarchy restart (omancy-restart-app, omancy-restart-audio, omancy-restart-bluetooth, omancy-restart-btop, omancy-restart-gum, omancy-restart-helix, omancy-restart-herdr, omancy-restart-hyprctl, omancy-restart-hyprsunset, omancy-restart-opencode, omancy-restart-shell, omancy-restart-terminal, omancy-restart-tmux, omancy-restart-trackpad, omancy-restart-wifi, omancy-restart-xcompose)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, SAFE_CONCURRENCY, RACE_FREE DESIGN, DEADLOCK_AVOIDANCE,
/// EFFICIENT_SCHEDULING, COMPOSITION_OVER_INHERITANCE.

/// Restartable component
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartComponent {
    App,
    Audio,
    Bluetooth,
    Btop,
    Gum,
    Helix,
    Herdr,
    Hyprctl,
    Hyprsunset,
    Shell,
    Terminal,
    Tmux,
    Trackpad,
    Wifi,
    Xcompose,
    Custom(String),
}

/// Restart status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartStatus {
    Idle,
    Restarting,
    Restarted,
    Failed(String),
}

/// Restart record
#[derive(Debug, Clone)]
pub struct RestartRecord {
    pub component: RestartComponent,
    pub status: RestartStatus,
    pub restarted_at: Option<u64>,
    pub restart_count: u32,
}

/// Restart manager
pub struct RestartManager {
    pub records: HashMap<RestartComponent, RestartRecord>,
}

impl RestartManager {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Restart a component
    pub fn restart(&mut self, component: RestartComponent) -> Result<(), String> {
        let record = self.records.entry(component.clone()).or_insert(RestartRecord {
            component,
            status: RestartStatus::Idle,
            restarted_at: None,
            restart_count: 0,
        });

        record.status = RestartStatus::Restarting;
        record.restart_count += 1;
        record.restarted_at = Some(now_secs());

        // Simulate restart
        record.status = RestartStatus::Restarted;
        Ok(())
    }

    /// Get restart record
    pub fn get_record(&self, component: &RestartComponent) -> Option<&RestartRecord> {
        self.records.get(component)
    }

    /// List all records
    pub fn list_records(&self) -> Vec<&RestartRecord> {
        self.records.values().collect()
    }

    /// Get restart count for a component
    pub fn restart_count(&self, component: &RestartComponent) -> u32 {
        self.records.get(component).map(|r| r.restart_count).unwrap_or(0)
    }

    /// Is component currently restarting
    pub fn is_restarting(&self, component: &RestartComponent) -> bool {
        matches!(
            self.records.get(component),
            Some(RestartRecord { status: RestartStatus::Restarting, .. })
        )
    }

    /// Record count
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}

impl Default for RestartManager {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_restart_component() {
        let mut mgr = RestartManager::new();
        assert!(mgr.restart(RestartComponent::Audio).is_ok());
        let record = mgr.get_record(&RestartComponent::Audio).unwrap();
        assert_eq!(record.status, RestartStatus::Restarted);
        assert_eq!(record.restart_count, 1);
    }

    #[test]
    fn test_restart_count() {
        let mut mgr = RestartManager::new();
        mgr.restart(RestartComponent::Wifi).unwrap();
        mgr.restart(RestartComponent::Wifi).unwrap();
        assert_eq!(mgr.restart_count(&RestartComponent::Wifi), 2);
    }

    #[test]
    fn test_list_records() {
        let mut mgr = RestartManager::new();
        mgr.restart(RestartComponent::Shell).unwrap();
        mgr.restart(RestartComponent::Terminal).unwrap();
        assert_eq!(mgr.record_count(), 2);
    }

    #[test]
    fn test_is_restarting() {
        let mut mgr = RestartManager::new();
        assert!(!mgr.is_restarting(&RestartComponent::Audio));
        mgr.records.insert(
            RestartComponent::Audio,
            RestartRecord {
                component: RestartComponent::Audio,
                status: RestartStatus::Restarting,
                restarted_at: None,
                restart_count: 0,
            },
        );
        assert!(mgr.is_restarting(&RestartComponent::Audio));
    }

    #[test]
    fn test_custom_component() {
        let mut mgr = RestartManager::new();
        assert!(mgr.restart(RestartComponent::Custom("myapp".to_string())).is_ok());
        assert!(mgr.get_record(&RestartComponent::Custom("myapp".to_string())).is_some());
    }
}