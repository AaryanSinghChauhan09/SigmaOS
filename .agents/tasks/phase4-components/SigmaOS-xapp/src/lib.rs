//! SigmaOS XApp — shared application utilities (file info, thumbnailing, IPC).

#![allow(dead_code)]

use std::path::{Path, PathBuf};
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum AppRole { FileManager, TextEditor, MediaPlayer, ImageViewer, Terminal, Other(String) }
#[derive(Debug,Clone)]
pub struct XAppInfo { pub app_id: String, pub name: String, pub icon: String, pub role: AppRole, pub supported_mime_types: Vec<String> }
impl XAppInfo {
    pub fn new(app_id: impl Into<String>, name: impl Into<String>, role: AppRole) -> Self {
        Self { app_id: app_id.into(), name: name.into(), icon: String::new(), role, supported_mime_types: Vec::new() }
    }
    pub fn with_mime(mut self, mime: impl Into<String>) -> Self { self.supported_mime_types.push(mime.into()); self }
    pub fn supports(&self, mime: &str) -> bool { self.supported_mime_types.iter().any(|m| m == mime || m.ends_with("/*") && mime.starts_with(&m[..m.len()-1])) }
}
#[cfg(test)] mod tests { use super::*;
  #[test] fn mime_support() { let app = XAppInfo::new("sigma-fm","Files",AppRole::FileManager).with_mime("inode/directory").with_mime("text/*"); assert!(app.supports("inode/directory")); assert!(app.supports("text/plain")); assert!(!app.supports("video/mp4")); }
}
