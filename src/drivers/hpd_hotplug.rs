// SigmaOS Display Hotplug Detection (HPD) Engine
// Monitors HDMI / DisplayPort HPD line interrupts and debounces connection events.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Display Connector Protocol Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorType {
    Hdmi,
    DisplayPort,
    EDp,
    UsbCDisplayPort,
}

/// Physical Connector Connection State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpdConnectionStatus {
    Connected,
    Disconnected,
}

/// HPD Event delivered to Wayland Compositor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HpdEvent {
    pub connector_id: u32,
    pub connector_type: ConnectorType,
    pub status: HpdConnectionStatus,
    pub timestamp_us: u64,
}

/// Display Connector State Tracker
#[derive(Debug, Clone)]
pub struct HpdConnector {
    pub id: u32,
    pub connector_type: ConnectorType,
    pub current_status: HpdConnectionStatus,
    pub last_pulse_time_us: u64,
}

/// Hotplug Detection Engine
#[derive(Debug, Clone, Default)]
pub struct HpdHotplugEngine {
    pub connectors: Vec<HpdConnector>,
    pub pending_events: Vec<HpdEvent>,
    pub debounce_interval_us: u64,
}

impl HpdHotplugEngine {
    pub fn new() -> Self {
        Self {
            connectors: Vec::new(),
            pending_events: Vec::new(),
            debounce_interval_us: 50_000, // 50ms debounce
        }
    }

    /// Register a physical video connector
    pub fn register_connector(&mut self, id: u32, connector_type: ConnectorType) {
        self.connectors.push(HpdConnector {
            id,
            connector_type,
            current_status: HpdConnectionStatus::Disconnected,
            last_pulse_time_us: 0,
        });
    }

    /// Interrupt Service Routine handling raw hardware HPD pin transition
    pub fn handle_hpd_irq(
        &mut self,
        connector_id: u32,
        pin_level_high: bool,
        now_us: u64,
    ) -> Option<HpdEvent> {
        let connector = self.connectors.iter_mut().find(|c| c.id == connector_id)?;

        // Debounce check
        if now_us.saturating_sub(connector.last_pulse_time_us) < self.debounce_interval_us {
            return None; // Glitch / debounce suppression
        }

        connector.last_pulse_time_us = now_us;
        let new_status = if pin_level_high {
            HpdConnectionStatus::Connected
        } else {
            HpdConnectionStatus::Disconnected
        };

        if connector.current_status != new_status {
            connector.current_status = new_status;
            let event = HpdEvent {
                connector_id,
                connector_type: connector.connector_type,
                status: new_status,
                timestamp_us: now_us,
            };
            self.pending_events.push(event);
            Some(event)
        } else {
            None
        }
    }

    /// Pop next pending hotplug event for display server
    pub fn pop_event(&mut self) -> Option<HpdEvent> {
        if self.pending_events.is_empty() {
            None
        } else {
            Some(self.pending_events.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpd_hotplug_detection_and_debounce() {
        let mut engine = HpdHotplugEngine::new();
        engine.register_connector(1, ConnectorType::Hdmi);

        // Plug-in event at t=100_000us
        let ev1 = engine.handle_hpd_irq(1, true, 100_000);
        assert!(ev1.is_some());
        assert_eq!(ev1.unwrap().status, HpdConnectionStatus::Connected);

        // Glitch within debounce interval (t=110_000us) -> suppressed
        let ev2 = engine.handle_hpd_irq(1, false, 110_000);
        assert!(ev2.is_none());

        // Unplug event after debounce interval (t=200_000us)
        let ev3 = engine.handle_hpd_irq(1, false, 200_000);
        assert!(ev3.is_some());
        assert_eq!(ev3.unwrap().status, HpdConnectionStatus::Disconnected);
    }
}
