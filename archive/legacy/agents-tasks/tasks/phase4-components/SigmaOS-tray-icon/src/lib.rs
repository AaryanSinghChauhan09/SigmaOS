//! SigmaOS system tray icon management (libappindicator-compatible API).

#![allow(dead_code)]

use std::collections::BTreeMap;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum TrayIconState { Active, Passive, NeedsAttention }
#[derive(Debug,Clone)]
pub struct TrayIcon { pub id: String, pub title: String, pub icon_name: String, pub state: TrayIconState, pub tooltip: String }
impl TrayIcon {
    pub fn new(id: impl Into<String>, title: impl Into<String>, icon: impl Into<String>) -> Self {
        Self { id: id.into(), title: title.into(), icon_name: icon.into(), state: TrayIconState::Passive, tooltip: String::new() }
    }
}
#[derive(Default)]
pub struct SystemTray { icons: BTreeMap<String, TrayIcon> }
impl SystemTray {
    pub fn register(&mut self, icon: TrayIcon) { self.icons.insert(icon.id.clone(), icon); }
    pub fn unregister(&mut self, id: &str) -> bool { self.icons.remove(id).is_some() }
    pub fn get(&self, id: &str) -> Option<&TrayIcon> { self.icons.get(id) }
    pub fn len(&self) -> usize { self.icons.len() }
    pub fn active(&self) -> Vec<&TrayIcon> { self.icons.values().filter(|i| i.state != TrayIconState::Passive).collect() }
}
#[cfg(test)] mod tests { use super::*;
  #[test] fn register_unregister() { let mut t = SystemTray::default(); t.register(TrayIcon::new("net","Network","network-idle")); assert_eq!(t.len(),1); assert!(t.unregister("net")); assert_eq!(t.len(),0); }
}
