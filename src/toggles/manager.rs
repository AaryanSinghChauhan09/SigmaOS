//! SigmaOS System Toggles Manager
//!
//! Inspired by: Omarchy toggles (omancy-toggle-bar, omancy-toggle-idle, omancy-toggle-nightlight, omancy-toggle-notification-silencing, omancy-toggle-screensaver, omancy-toggle-suspend, omancy-toggle-touchpad, omancy-toggle-touchscreen)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! USER_FREEDOM_AND_CONTROL, SIMPLE_USER_INTERFACE, TOGGLE_STATE.

/// Toggle type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleType {
    Bar,
    Idle,
    NightLight,
    NotificationSilencing,
    Screensaver,
    Suspend,
    Touchpad,
    Touchscreen,
    HybridGpu,
    CrashCapture,
    InputDevice,
}

/// Toggle state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleState {
    On,
    Off,
}

/// Toggles manager
pub struct TogglesManager {
    pub states: HashMap<ToggleType, ToggleState>,
}

impl TogglesManager {
    pub fn new() -> Self {
        let mut states = HashMap::new();
        states.insert(ToggleType::Bar, ToggleState::On);
        states.insert(ToggleType::Idle, ToggleState::On);
        states.insert(ToggleType::NightLight, ToggleState::Off);
        states.insert(ToggleType::NotificationSilencing, ToggleState::Off);
        states.insert(ToggleType::Screensaver, ToggleState::On);
        states.insert(ToggleType::Suspend, ToggleState::On);
        states.insert(ToggleType::Touchpad, ToggleState::On);
        states.insert(ToggleType::Touchscreen, ToggleState::On);
        states.insert(ToggleType::HybridGpu, ToggleState::Off);
        states.insert(ToggleType::CrashCapture, ToggleState::On);
        states.insert(ToggleType::InputDevice, ToggleState::On);

        Self { states }
    }

    /// Toggle a setting
    pub fn toggle(&mut self, toggle_type: ToggleType) -> ToggleState {
        let current = self.states.get(&toggle_type).copied().unwrap_or(ToggleState::Off);
        let new_state = match current {
            ToggleState::On => ToggleState::Off,
            ToggleState::Off => ToggleState::On,
        };
        self.states.insert(toggle_type, new_state);
        new_state
    }

    /// Set a toggle state
    pub fn set(&mut self, toggle_type: ToggleType, state: ToggleState) {
        self.states.insert(toggle_type, state);
    }

    /// Get a toggle state
    pub fn get(&self, toggle_type: &ToggleType) -> ToggleState {
        self.states.get(toggle_type).copied().unwrap_or(ToggleState::Off)
    }

    /// Is enabled
    pub fn is_enabled(&self, toggle_type: &ToggleType) -> bool {
        matches!(self.get(toggle_type), ToggleState::On)
    }

    /// Is disabled
    pub fn is_disabled(&self, toggle_type: &ToggleType) -> bool {
        matches!(self.get(toggle_type), ToggleState::Off)
    }

    /// List all toggles
    pub fn list_all(&self) -> Vec<(ToggleType, ToggleState)> {
        self.states.iter().map(|(k, v)| (*k, *v)).collect()
    }

    /// Toggle count
    pub fn toggle_count(&self) -> usize {
        self.states.len()
    }
}

impl Default for TogglesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_bar() {
        let mut mgr = TogglesManager::new();
        assert!(mgr.is_enabled(&ToggleType::Bar));
        let new_state = mgr.toggle(ToggleType::Bar);
        assert_eq!(new_state, ToggleState::Off);
        assert!(mgr.is_disabled(&ToggleType::Bar));
    }

    #[test]
    fn test_toggle_nightlight() {
        let mut mgr = TogglesManager::new();
        assert!(!mgr.is_enabled(&ToggleType::NightLight));
        mgr.toggle(ToggleType::NightLight);
        assert!(mgr.is_enabled(&ToggleType::NightLight));
    }

    #[test]
    fn test_set_state() {
        let mut mgr = TogglesManager::new();
        mgr.set(ToggleType::Touchpad, ToggleState::Off);
        assert!(!mgr.is_enabled(&ToggleType::Touchpad));
    }

    #[test]
    fn test_list_all() {
        let mgr = TogglesManager::new();
        let all = mgr.list_all();
        assert_eq!(all.len(), 11);
    }

    #[test]
    fn test_toggle_count() {
        let mgr = TogglesManager::new();
        assert_eq!(mgr.toggle_count(), 11);
    }
}