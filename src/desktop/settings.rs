#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Desktop Settings for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 776
/// Implements desktop settings and preferences

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type SettingID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingType { String = 0, Integer = 1, Boolean = 2, Color = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsError { Success = 0, NotFound = 1, InvalidType = 2 }

pub trait Setting {
    fn id(&self) -> SettingID;
    fn key(&self) -> &[u8];
    fn setting_type(&self) -> SettingType;
    fn value(&self) -> &[u8];
    fn set_value(&mut self, value: &[u8]);
}

#[repr(C)]
pub struct SimpleSetting {
    pub id: SettingID,
    pub key: [u8; 128],
    pub setting_type: AtomicUsize,
    pub value: [u8; 256],
}

impl SimpleSetting {
    pub fn new(id: SettingID, key: &[u8], setting_type: SettingType, value: &[u8]) -> Self {
        let mut key_array = [0u8; 128];
        let mut value_array = [0u8; 256];
        let key_len = key.len().min(127);
        let value_len = value.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(key.as_ptr(), key_array.as_mut_ptr(), key_len);
            core::ptr::copy_nonoverlapping(value.as_ptr(), value_array.as_mut_ptr(), value_len);
        }
        SimpleSetting {
            id,
            key: key_array,
            setting_type: AtomicUsize::new(setting_type as usize),
            value: value_array,
        }
    }
}

impl Setting for SimpleSetting {
    fn id(&self) -> SettingID { self.id }
    fn key(&self) -> &[u8] {
        let len = self.key.iter().position(|&b| b == 0).unwrap_or(128);
        &self.key[..len]
    }
    fn setting_type(&self) -> SettingType {
        match self.setting_type.load(Ordering::SeqCst) {
            0 => SettingType::String,
            1 => SettingType::Integer,
            2 => SettingType::Boolean,
            _ => SettingType::Color,
        }
    }
    fn value(&self) -> &[u8] {
        let len = self.value.iter().position(|&b| b == 0).unwrap_or(256);
        &self.value[..len]
    }

    fn set_value(&mut self, value: &[u8]) {
        let value_len = value.len().min(255);
        self.value = [0u8; 256];
        unsafe {
            core::ptr::copy_nonoverlapping(value.as_ptr(), self.value.as_mut_ptr(), value_len);
        }
    }
}

pub trait SettingsManager {
    fn get_setting(&self, key: &[u8]) -> Option<&dyn Setting>;
    fn set_setting(&mut self, key: &[u8], value: &[u8]) -> Result<(), SettingsError>;
    fn reset_default(&mut self, key: &[u8]) -> Result<(), SettingsError>;
    fn save_settings(&self) -> Result<(), SettingsError>;
}

#[repr(C)]
pub struct SimpleSettingsManager {
    pub settings: Vec<Option<Box<dyn Setting>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSettingsManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSettingsManager {
            settings: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SettingsManager for SimpleSettingsManager {
    fn get_setting(&self, key: &[u8]) -> Option<&dyn Setting> {
        for setting_option in &self.settings {
            if let Some(ref setting) = *setting_option {
                if setting.key() == key { return Some(setting.as_ref()); }
            }
        }
        None
    }

    fn set_setting(&mut self, key: &[u8], value: &[u8]) -> Result<(), SettingsError> {
        for setting_option in &mut self.settings {
            if let Some(ref mut setting) = *setting_option {
                if setting.key() == key {
                    setting.set_value(value);
                    return Ok(());
                }
            }
        }
        Err(SettingsError::NotFound)
    }

    fn reset_default(&mut self, key: &[u8]) -> Result<(), SettingsError> {
        for setting_option in &mut self.settings {
            if let Some(ref mut setting) = *setting_option {
                if setting.key() == key {
                    setting.set_value(b"default");
                    return Ok(());
                }
            }
        }
        Err(SettingsError::NotFound)
    }

    fn save_settings(&self) -> Result<(), SettingsError> {
        Ok(())
    }
}

pub trait SettingsCategory {
    fn get_category(&self, category: &[u8]) -> Vec<&dyn Setting>;
    fn add_to_category(&mut self, category: &[u8], setting: Box<dyn Setting>);
}

#[repr(C)]
pub struct SimpleSettingsCategory {
    pub categories: Vec<([u8; 64], Vec<SettingID>)>,
    pub manager: SimpleSettingsManager,
}

impl SimpleSettingsCategory {
    pub fn new(manager: SimpleSettingsManager) -> Self {
        SimpleSettingsCategory {
            categories: Vec::new(),
            manager,
        }
    }
}

impl SettingsCategory for SimpleSettingsCategory {
    fn get_category(&self, category: &[u8]) -> Vec<&dyn Setting> {
        let mut results = Vec::new();
        for &(ref cat, ref ids) in &self.categories {
            let cat_len = cat.iter().position(|&b| b == 0).unwrap_or(64);
            if &cat[..cat_len] == category {
                for &_id in ids {
                    for setting_option in &self.manager.settings {
                        if let Some(ref setting) = *setting_option {
                            if setting.id() == _id {
                                results.push(setting.as_ref());
                            }
                        }
                    }
                }
            }
        }
        results
    }

    fn add_to_category(&mut self, category: &[u8], setting: Box<dyn Setting>) {
        let id = setting.id();
        self.manager.settings.push(Some(setting));

        let mut cat_array = [0u8; 64];
        let cat_len = category.len().min(63);
        for i in 0..cat_len {
            cat_array[i] = category[i];
        }

        let mut found = false;
        for (cat, ids) in &mut self.categories {
            let cat_len = cat.iter().position(|&b| b == 0).unwrap_or(64);
            if &cat[..cat_len] == category {
                ids.push(id);
                found = true;
                break;
            }
        }
        if !found {
            let mut ids = Vec::new();
            ids.push(id);
            self.categories.push((cat_array, ids));
        }
    }
}

/// GNOME dconf / GSettings Schema Validator
pub struct GsettingsSchemaValidator;

impl GsettingsSchemaValidator {
    pub fn validate_setting(setting_type: SettingType, value: &[u8]) -> bool {
        match setting_type {
            SettingType::Boolean => value == b"true" || value == b"false",
            SettingType::Integer => {
                for &b in value {
                    if !b.is_ascii_digit() && b != b'-' {
                        return false;
                    }
                }
                !value.is_empty()
            }
            SettingType::Color => value.starts_with(b"#") && (value.len() == 7 || value.len() == 9),
            SettingType::String => true,
        }
    }
}

/// KDE KConfig Cascading Hierarchy (Defaults -> Global -> User)
pub struct KconfigCascadingStore {
    pub user_overrides: SimpleSettingsManager,
    pub global_defaults: SimpleSettingsManager,
}

impl KconfigCascadingStore {
    pub fn new(global_defaults: SimpleSettingsManager, user_overrides: SimpleSettingsManager) -> Self {
        KconfigCascadingStore {
            user_overrides,
            global_defaults,
        }
    }

    pub fn get_effective_setting(&self, key: &[u8]) -> Option<&dyn Setting> {
        if let Some(user_setting) = self.user_overrides.get_setting(key) {
            Some(user_setting)
        } else {
            self.global_defaults.get_setting(key)
        }
    }
}

/// XFCE xfconf Daemon IPC Notification Dispatcher
pub struct XfconfBusDispatcher {
    pub channel_name: [u8; 32],
    pub dispatch_count: usize,
}

impl XfconfBusDispatcher {
    pub fn new(channel: &[u8]) -> Self {
        let mut ch = [0u8; 32];
        let len = channel.len().min(31);
        ch[..len].copy_from_slice(&channel[..len]);
        XfconfBusDispatcher {
            channel_name: ch,
            dispatch_count: 0,
        }
    }

    pub fn notify_property_change(&mut self, _key: &[u8], _value: &[u8]) {
        self.dispatch_count += 1;
    }
}

/// FreeBSD sysctl / rc.conf System Desktop Override Schema
pub struct RcConfSettingsOverlay {
    pub sysctl_overrides: SimpleSettingsManager,
}

impl RcConfSettingsOverlay {
    pub fn new() -> Self {
        RcConfSettingsOverlay {
            sysctl_overrides: SimpleSettingsManager::new(),
        }
    }

    pub fn apply_override(&mut self, key: &[u8], value: &[u8]) -> Result<(), SettingsError> {
        let id = self.sysctl_overrides.next_id.fetch_add(1, Ordering::SeqCst);
        let setting = SimpleSetting::new(id, key, SettingType::String, value);
        self.sysctl_overrides.settings.push(Some(Box::new(setting)));
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gsettings_schema_validation() {
        assert!(GsettingsSchemaValidator::validate_setting(SettingType::Boolean, b"true"));
        assert!(!GsettingsSchemaValidator::validate_setting(SettingType::Boolean, b"invalid"));
        assert!(GsettingsSchemaValidator::validate_setting(SettingType::Integer, b"100"));
        assert!(GsettingsSchemaValidator::validate_setting(SettingType::Color, b"#FF0000"));
    }

    #[test]
    fn test_kconfig_cascading_store() {
        let global = SimpleSettingsManager::new();
        let mut user = SimpleSettingsManager::new();

        let id = user.next_id.fetch_add(1, Ordering::SeqCst);
        let s = SimpleSetting::new(id, b"theme", SettingType::String, b"dark");
        user.settings.push(Some(Box::new(s)));

        let store = KconfigCascadingStore::new(global, user);
        let eff = store.get_effective_setting(b"theme");
        assert!(eff.is_some());
        assert_eq!(eff.unwrap().value(), b"dark");
    }

    #[test]
    fn test_xfconf_bus_dispatcher() {
        let mut dispatcher = XfconfBusDispatcher::new(b"xsettings");
        dispatcher.notify_property_change(b"/Net/ThemeName", b"Adwaita-dark");
        assert_eq!(dispatcher.dispatch_count, 1);
    }

    #[test]
    fn test_rc_conf_overlay() {
        let mut overlay = RcConfSettingsOverlay::new();
        assert!(overlay.apply_override(b"kern.ipc.maxsockbuf", b"2097152").is_ok());
        assert!(overlay.sysctl_overrides.get_setting(b"kern.ipc.maxsockbuf").is_some());
    }
}
