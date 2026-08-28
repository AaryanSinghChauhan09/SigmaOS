// SigmaOS Distro Compatibility Layer
// SigmaOS LocalSend Protocol Compatibility Bridge
// Implements the LocalSend REST/UDP v2.1 protocol format
// to enable offline peer-to-peer secure file transfer within local networks.

#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


extern crate alloc;
use crate::security::capability::CapabilityToken;
use alloc::collections::{BTreeMap, BTreeSet};

/// Device types supported by LocalSend protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSendDeviceType {
    Mobile,
    Desktop,
    Web,
    Headless,
    Server,
}

impl LocalSendDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LocalSendDeviceType::Mobile => "mobile",
            LocalSendDeviceType::Desktop => "desktop",
            LocalSendDeviceType::Web => "web",
            LocalSendDeviceType::Headless => "headless",
            LocalSendDeviceType::Server => "server",
        }
    }
}

/// Metadata payload for LocalSend peer device discovery
#[derive(Debug, Clone)]
pub struct LocalSendDevice {
    pub alias: String,
    pub version: String,
    pub device_model: Option<String>,
    pub device_type: LocalSendDeviceType,
    pub fingerprint: String,
    pub port: u16,
    pub protocol: String, // "http" or "https"
    pub download: bool,   // Download (Reverse File Transfer) API active
}

/// Metadata for individual files managed by the LocalSend session
#[derive(Debug, Clone)]
pub struct LocalSendFileMetadata {
    pub id: String,
    pub file_name: String,
    pub size: u64, // bytes
    pub file_type: String,
    pub sha256: Option<String>,
    pub preview: Option<String>,
}

/// LocalSend File Transfer Session
#[derive(Debug, Clone)]
pub struct LocalSendSession {
    pub session_id: String,
    pub files: BTreeMap<String, LocalSendFileMetadata>,
    pub accepted_files_tokens: BTreeMap<String, String>, // file_id -> token
}

/// Sovereign LocalSend Protocol Bridge Manager
pub struct LocalSendBridgeManager {
    pub active_sessions: BTreeMap<String, LocalSendSession>,
    pub registered_peers: BTreeMap<String, LocalSendDevice>,
    pub local_device: LocalSendDevice,
    pub pin_code: Option<String>,
}

impl LocalSendBridgeManager {
    pub fn new(alias: &str, fingerprint: &str) -> Self {
        Self {
            active_sessions: BTreeMap::new(),
            registered_peers: BTreeMap::new(),
            local_device: LocalSendDevice {
                alias: alias.to_string(),
                version: "2.1".to_string(),
                device_model: Some("SigmaOS Node".to_string()),
                device_type: LocalSendDeviceType::Desktop,
                fingerprint: fingerprint.to_string(),
                port: 53317,
                protocol: "https".to_string(),
                download: true,
            },
            pin_code: None,
        }
    }

    /// Set an optional numeric PIN restriction for incoming prep-requests
    pub fn set_pin(&mut self, pin: String) {
        self.pin_code = Some(pin);
    }

    /// Handles an incoming UDP multicast advertisement/announce message or register payload
    pub fn handle_register_announcement(&mut self, peer: LocalSendDevice) {
        self.registered_peers.insert(peer.fingerprint.clone(), peer);
    }

    /// Process incoming preparation requests (POST /api/localsend/v2/prepare-upload)
    pub fn prepare_upload(
        &mut self,
        sender_info: LocalSendDevice,
        files_to_upload: Vec<LocalSendFileMetadata>,
        provided_pin: Option<&str>,
        _cap: &CapabilityToken,
    ) -> Result<LocalSendSession, &'static str> {
        // Enforce PIN validation
        if let Some(ref required_pin) = self.pin_code {
            if provided_pin != Some(required_pin.as_str()) {
                return Err("Invalid PIN code supplied or missing");
            }
        }

        let session_id = format!("sess_{}", sender_info.fingerprint);
        let mut files_map = BTreeMap::new();
        let mut tokens_map = BTreeMap::new();

        for file in files_to_upload {
            let token = format!("tok_{}", file.id);
            tokens_map.insert(file.id.clone(), token);
            files_map.insert(file.id.clone(), file);
        }

        let session = LocalSendSession {
            session_id: session_id.clone(),
            files: files_map,
            accepted_files_tokens: tokens_map,
        };

        self.active_sessions.insert(session_id, session.clone());
        Ok(session)
    }

    /// Process file chunks upload (POST /api/localsend/v2/upload)
    pub fn handle_upload_chunk(
        &self,
        session_id: &str,
        file_id: &str,
        token: &str,
        _chunk_payload: &[u8],
    ) -> Result<(), &'static str> {
        let session = self
            .active_sessions
            .get(session_id)
            .ok_or("Session not found")?;
        let expected_token = session
            .accepted_files_tokens
            .get(file_id)
            .ok_or("File not accepted in session")?;
        if expected_token != token {
            return Err("Invalid file validation token");
        }
        Ok(())
    }

    /// Cancel active LocalSend transfer session
    pub fn cancel_session(&mut self, session_id: &str) -> bool {
        self.active_sessions.remove(session_id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localsend_registration() {
        let mut manager = LocalSendBridgeManager::new("Nice Orange", "fp_orange123");
        let peer = LocalSendDevice {
            alias: "Secret Banana".to_string(),
            version: "2.1".to_string(),
            device_model: Some("Samsung Phone".to_string()),
            device_type: LocalSendDeviceType::Mobile,
            fingerprint: "fp_banana456".to_string(),
            port: 53317,
            protocol: "https".to_string(),
            download: true,
        };

        manager.handle_register_announcement(peer);
        assert_eq!(manager.registered_peers.len(), 1);
        assert!(manager.registered_peers.contains_key("fp_banana456"));
    }

    #[test]
    fn test_localsend_prepare_upload_and_chunk_handling() {
        let mut manager = LocalSendBridgeManager::new("Nice Orange", "fp_orange123");
        manager.set_pin("123456".to_string());

        let peer = LocalSendDevice {
            alias: "Secret Banana".to_string(),
            version: "2.1".to_string(),
            device_model: Some("Samsung Phone".to_string()),
            device_type: LocalSendDeviceType::Mobile,
            fingerprint: "fp_banana456".to_string(),
            port: 53317,
            protocol: "https".to_string(),
            download: true,
        };

        let file = LocalSendFileMetadata {
            id: "file_img1".to_string(),
            file_name: "my_screenshot.png".to_string(),
            size: 1024,
            file_type: "image/png".to_string(),
            sha256: Some("sha_mock_img".to_string()),
            preview: None,
        };

        let cap = CapabilityToken::new();
        // Try with missing/invalid PIN
        let bad_prep =
            manager.prepare_upload(peer.clone(), vec![file.clone()], Some("000000"), &cap);
        assert!(bad_prep.is_err());

        // Prepare with correct PIN
        let session = manager
            .prepare_upload(peer, vec![file], Some("123456"), &cap)
            .unwrap();
        assert_eq!(session.session_id, "sess_fp_banana456");

        // Upload chunk with valid token
        let token = session.accepted_files_tokens.get("file_img1").unwrap();
        let chunk_res =
            manager.handle_upload_chunk("sess_fp_banana456", "file_img1", token, &[0u8; 10]);
        assert!(chunk_res.is_ok());

        // Cancel session
        assert!(manager.cancel_session("sess_fp_banana456"));
        assert_eq!(manager.active_sessions.len(), 0);
    }
}
