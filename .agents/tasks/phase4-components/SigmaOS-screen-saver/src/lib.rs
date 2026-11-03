//! SigmaOS screen saver — idle lock/blank policy with configurable timeout.

#![allow(dead_code)]

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ScreenSaverMode { Blank, Lock, Off }
pub struct ScreenSaver { pub mode: ScreenSaverMode, pub idle_timeout_s: u32, active: bool }
impl ScreenSaver {
    pub fn new(mode: ScreenSaverMode, timeout: u32) -> Self { Self { mode, idle_timeout_s: timeout, active: false } }
    pub fn activate(&mut self) { self.active = true; }
    pub fn deactivate(&mut self) { self.active = false; }
    pub fn is_active(&self) -> bool { self.active }
    pub fn should_activate(&self, idle_s: u32) -> bool { idle_s >= self.idle_timeout_s }
}
#[cfg(test)] mod tests { use super::*;
  #[test] fn activates_after_timeout() { let s = ScreenSaver::new(ScreenSaverMode::Lock, 300); assert!(s.should_activate(300)); assert!(!s.should_activate(60)); }
  #[test] fn deactivate_clears() { let mut s = ScreenSaver::new(ScreenSaverMode::Blank, 60); s.activate(); assert!(s.is_active()); s.deactivate(); assert!(!s.is_active()); }
}
