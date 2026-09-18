// Unified Control Center for SigmaOS
// Inspired by elementaryOS Switchboard, GNOME Control Center, and openSUSE YaST2

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlCenterCategory {
    Network,
    Display,
    Sound,
    Power,
    Users,
    Security,
    Storage,
    Printers,
    System,
}

pub trait SwitchboardPlug: Send + Sync {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn category(&self) -> ControlCenterCategory;
    fn get_setting(&self, key: &str) -> Option<String>;
    fn set_setting(&mut self, key: &str, value: &str) -> Result<(), String>;
}

/// Network Settings Plug
pub struct NetworkSettingsPlug {
    pub wifi_enabled: bool,
    pub ethernet_up: bool,
    pub hostname: String,
}

impl SwitchboardPlug for NetworkSettingsPlug {
    fn id(&self) -> &str { "network" }
    fn title(&self) -> &str { "Network & Wi-Fi" }
    fn category(&self) -> ControlCenterCategory { ControlCenterCategory::Network }
    fn get_setting(&self, key: &str) -> Option<String> {
        match key {
            "wifi_enabled" => Some(self.wifi_enabled.to_string()),
            "ethernet_up" => Some(self.ethernet_up.to_string()),
            "hostname" => Some(self.hostname.clone()),
            _ => None,
        }
    }
    fn set_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "wifi_enabled" => {
                self.wifi_enabled = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            "hostname" => {
                self.hostname = value.to_string();
                Ok(())
            }
            _ => Err(format!("Unknown key {}", key)),
        }
    }
}

/// Display Settings Plug
pub struct DisplaySettingsPlug {
    pub resolution: String,
    pub scale_factor: f32,
    pub night_light: bool,
}

impl SwitchboardPlug for DisplaySettingsPlug {
    fn id(&self) -> &str { "display" }
    fn title(&self) -> &str { "Display & Scaling" }
    fn category(&self) -> ControlCenterCategory { ControlCenterCategory::Display }
    fn get_setting(&self, key: &str) -> Option<String> {
        match key {
            "resolution" => Some(self.resolution.clone()),
            "scale_factor" => Some(self.scale_factor.to_string()),
            "night_light" => Some(self.night_light.to_string()),
            _ => None,
        }
    }
    fn set_setting(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "resolution" => {
                self.resolution = value.to_string();
                Ok(())
            }
            "scale_factor" => {
                self.scale_factor = value.parse().map_err(|_| "Invalid float")?;
                Ok(())
            }
            "night_light" => {
                self.night_light = value.parse().map_err(|_| "Invalid boolean")?;
                Ok(())
            }
            _ => Err(format!("Unknown key {}", key)),
        }
    }
}

/// Unified Control Center Manager
#[cfg(not(target_os = "none"))]
use std::sync::Mutex;

#[cfg(target_os = "none")]
pub struct Mutex<T>(core::cell::UnsafeCell<T>);

#[cfg(target_os = "none")]
pub struct BareGuard<'a, T>(&'a mut T);

#[cfg(target_os = "none")]
impl<'a, T> core::ops::Deref for BareGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { self.0 }
}

#[cfg(target_os = "none")]
impl<'a, T> core::ops::DerefMut for BareGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0 }
}

#[cfg(target_os = "none")]
unsafe impl<T: Send> Send for Mutex<T> {}
#[cfg(target_os = "none")]
unsafe impl<T: Send> Sync for Mutex<T> {}

#[cfg(target_os = "none")]
impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Mutex(core::cell::UnsafeCell::new(value))
    }
}

#[cfg(target_os = "none")]
impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> Result<BareGuard<'_, T>, ()> {
        unsafe { Ok(BareGuard(&mut *self.0.get())) }
    }
}

pub struct UnifiedControlCenter {
    pub plugs: BTreeMap<String, std::sync::Arc<Mutex<dyn SwitchboardPlug>>>,
}

impl UnifiedControlCenter {
    pub fn new() -> Self {
        let mut center = Self {
            plugs: BTreeMap::new(),
        };

        let net_plug = NetworkSettingsPlug {
            wifi_enabled: true,
            ethernet_up: true,
            hostname: "sigmaos-desktop".to_string(),
        };
        center.register_plug(std::sync::Arc::new(Mutex::new(net_plug)));

        let display_plug = DisplaySettingsPlug {
            resolution: "1920x1080".to_string(),
            scale_factor: 1.0,
            night_light: false,
        };
        center.register_plug(std::sync::Arc::new(Mutex::new(display_plug)));

        center
    }

    pub fn register_plug(&mut self, plug: std::sync::Arc<Mutex<dyn SwitchboardPlug>>) {
        let id = {
            #[cfg(not(target_os = "none"))]
            {
                plug.lock().unwrap().id().to_string()
            }
            #[cfg(target_os = "none")]
            {
                plug.lock().unwrap().id().to_string()
            }
        };
        self.plugs.insert(id, plug);
    }

    pub fn get_plug_setting(&self, plug_id: &str, key: &str) -> Option<String> {
        let plug = self.plugs.get(plug_id)?;
        #[cfg(not(target_os = "none"))]
        {
            plug.lock().unwrap().get_setting(key)
        }
        #[cfg(target_os = "none")]
        {
            plug.lock().ok()?.get_setting(key)
        }
    }

    pub fn set_plug_setting(&mut self, plug_id: &str, key: &str, value: &str) -> Result<(), String> {
        let plug = self.plugs.get(plug_id).ok_or("Plug not found")?;
        #[cfg(not(target_os = "none"))]
        {
            plug.lock().unwrap().set_setting(key, value)
        }
        #[cfg(target_os = "none")]
        {
            plug.lock().map_err(|_| "Lock error")?.set_setting(key, value)
        }
    }
}

impl Default for UnifiedControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_control_center() {
        let mut center = UnifiedControlCenter::new();
        assert_eq!(center.get_plug_setting("network", "hostname").unwrap(), "sigmaos-desktop");

        assert!(center.set_plug_setting("network", "hostname", "sigmaos-workstation").is_ok());
        assert_eq!(center.get_plug_setting("network", "hostname").unwrap(), "sigmaos-workstation");

        assert_eq!(center.get_plug_setting("display", "resolution").unwrap(), "1920x1080");
        assert!(center.set_plug_setting("display", "scale_factor", "1.25").is_ok());
        assert_eq!(center.get_plug_setting("display", "scale_factor").unwrap(), "1.25");
    }
}
