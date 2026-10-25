// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Kali Linux penetration testing compatibility
// Principles: MODULARITY, SECURITY, LEAST PRIVILEGE
use alloc::{string::String, vec::Vec};
#[derive(Debug,Clone,PartialEq)] pub enum ToolCategory { Recon, Scanning, Exploitation, PostExploit, Forensics, Wireless, Sdr, Reporting, Password, Reverse }
#[derive(Debug,Clone)] pub struct PenTestTool { pub name: String, pub category: ToolCategory, pub path: String, pub installed: bool }
pub struct KaliToolRegistry { pub tools: Vec<PenTestTool> }
impl KaliToolRegistry {
    pub fn new() -> Self { let mut s = Self { tools: Vec::new() }; s.seed_defaults(); s }
    fn seed_defaults(&mut self) {
        for (n,c,p) in &[("nmap","Scanning","/usr/bin/nmap"),("metasploit","Exploitation","/usr/bin/msfconsole"),("aircrack-ng","Wireless","/usr/bin/aircrack-ng"),("volatility","Forensics","/usr/bin/vol.py")] {
            self.tools.push(PenTestTool { name: n.to_string().into(), category: ToolCategory::Scanning, path: p.to_string().into(), installed: true });
        }
    }
    pub fn register_tool(&mut self, t: PenTestTool) { self.tools.push(t); }
    pub fn find_by_category(&self, cat: &ToolCategory) -> Vec<&PenTestTool> { self.tools.iter().filter(|t| &t.category == cat).collect() }
    pub fn run_tool(&self, name: &str, _args: &[&str]) -> Result<Vec<u8>,&'static str> { if self.tools.iter().any(|t| t.name == name) { Ok(alloc::vec![0]) } else { Err("tool not found") } }
}
pub struct KaliUndercoverMode { pub active: bool }
impl KaliUndercoverMode {
    pub fn new() -> Self { Self { active: false } }
    pub fn activate(&mut self) { self.active = true; }
    pub fn deactivate(&mut self) { self.active = false; }
}
pub struct SdrTools { pub device_found: bool }
impl SdrTools {
    pub fn new() -> Self { Self { device_found: false } }
    pub fn detect_sdr_hardware(&mut self) { self.device_found = true; }
    pub fn start_gnuradio(&self) -> Result<(),&'static str> { if self.device_found { Ok(()) } else { Err("no SDR device") } }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_registry() { let reg = KaliToolRegistry::new(); assert!(!reg.tools.is_empty()); }
    #[test] fn test_undercover() { let mut m = KaliUndercoverMode::new(); m.activate(); assert!(m.active); m.deactivate(); assert!(!m.active); }
}
