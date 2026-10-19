//! SigmaOS Channel Management
//!
//! Inspired by: Omarchy channel management (omancy-channel-current, omancy-channel-set)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, VERSION_CONTROL_PRACTICES, PRACTICAL_LICENSING_AWARENESS,
//! COMMUNITY_ORIENTED_DEVELOPMENT.

/// Release channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    Edge,
    Rc,
    Stable,
}

/// Channel configuration
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    pub name: String,
    pub description: String,
    pub requires_signature: bool,
    pub auto_update: bool,
    pub stability_level: StabilityLevel,
}

/// Stability level
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabilityLevel {
    Experimental,
    Testing,
    Stable,
    LongTermSupport,
}

/// Channel manager
pub struct ChannelManager {
    pub channels: HashMap<Channel, ChannelConfig>,
    pub current_channel: Channel,
    pub available_channels: Vec<Channel>,
}

impl ChannelManager {
    pub fn new() -> Self {
        let mut channels = HashMap::new();

        channels.insert(
            Channel::Edge,
            ChannelConfig {
                name: "edge".to_string(),
                description: "Bleeding edge - built from every PR, unsigned".to_string(),
                requires_signature: false,
                auto_update: true,
                stability_level: StabilityLevel::Experimental,
            },
        );

        channels.insert(
            Channel::Rc,
            ChannelConfig {
                name: "rc".to_string(),
                description: "Release candidate - tested, signed".to_string(),
                requires_signature: true,
                auto_update: true,
                stability_level: StabilityLevel::Testing,
            },
        );

        channels.insert(
            Channel::Stable,
            ChannelConfig {
                name: "stable".to_string(),
                description: "Production-ready - signed, long-term support".to_string(),
                requires_signature: true,
                auto_update: false,
                stability_level: StabilityLevel::LongTermSupport,
            },
        );

        Self {
            channels,
            current_channel: Channel::Stable,
            available_channels: vec![Channel::Edge, Channel::Rc, Channel::Stable],
        }
    }

    /// Get current channel
    pub fn current(&self) &Channel {
        &self.current_channel
    }

    /// Set current channel
    pub fn set(&mut self, channel: Channel) -> Result<(), String> {
        if !self.available_channels.contains(&channel) {
            return Err(format!("Channel {:?} not available", channel));
        }
        self.current_channel = channel;
        Ok(())
    }

    /// Get channel config
    pub fn get_config(&self, channel: &Channel) -> Option<&ChannelConfig> {
        self.channels.get(channel)
    }

    /// List all channels
    pub fn list(&self) -> Vec<&Channel> {
        self.available_channels.iter().collect()
    }

    /// Is signature required for channel
    pub fn requires_signature(&self, channel: &Channel) -> bool {
        self.channels
            .get(channel)
            .map(|c| c.requires_signature)
            .unwrap_or(false)
    }

    /// Can promote from source to target
    pub fn can_promote(&self, source: &Channel, target: &Channel) -> bool {
        match (source, target) {
            (Channel::Edge, Channel::Rc) => true,
            (Channel::Rc, Channel::Stable) => true,
            (Channel::Edge, Channel::Stable) => false,
            _ => false,
        }
    }

    /// Get promotion path
    pub fn promotion_path(&self, target: &Channel) -> Vec<Channel> {
        match target {
            Channel::Edge => vec![Channel::Edge],
            Channel::Rc => vec![Channel::Edge, Channel::Rc],
            Channel::Stable => vec![Channel::Edge, Channel::Rc, Channel::Stable],
        }
    }

    /// Set auto-update for channel
    pub fn set_auto_update(&mut self, channel: &Channel, enabled: bool) -> Result<(), String> {
        let config = self
            .channels
            .get_mut(channel)
            .ok_or_else(|| format!("Channel {:?} not found", channel))?;
        config.auto_update = enabled;
        Ok(())
    }

    /// Channel count
    pub fn channel_count(&self) -> usize {
        self.available_channels.len()
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_channels() {
        let mgr = ChannelManager::new();
        assert_eq!(mgr.channel_count(), 3);
        assert_eq!(*mgr.current(), Channel::Stable);
    }

    #[test]
    fn test_set_channel() {
        let mut mgr = ChannelManager::new();
        assert!(mgr.set(Channel::Edge).is_ok());
        assert_eq!(*mgr.current(), Channel::Edge);
    }

    #[test]
    fn test_requires_signature() {
        let mgr = ChannelManager::new();
        assert!(!mgr.requires_signature(&Channel::Edge));
        assert!(mgr.requires_signature(&Channel::Rc));
        assert!(mgr.requires_signature(&Channel::Stable));
    }

    #[test]
    fn test_can_promote() {
        let mgr = ChannelManager::new();
        assert!(mgr.can_promote(&Channel::Edge, &Channel::Rc));
        assert!(mgr.can_promote(&Channel::Rc, &Channel::Stable));
        assert!(!mgr.can_promote(&Channel::Edge, &Channel::Stable));
    }

    #[test]
    fn test_promotion_path() {
        let mgr = ChannelManager::new();
        let path = mgr.promotion_path(&Channel::Stable);
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Channel::Edge);
        assert_eq!(path[1], Channel::Rc);
        assert_eq!(path[2], Channel::Stable);
    }

    #[test]
    fn test_set_auto_update() {
        let mut mgr = ChannelManager::new();
        assert!(mgr.set_auto_update(&Channel::Edge, false).is_ok());
        assert!(!mgr.get_config(&Channel::Edge).unwrap().auto_update);
    }
}