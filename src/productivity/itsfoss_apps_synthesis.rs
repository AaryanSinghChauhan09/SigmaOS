// SPDX-License-Identifier: MIT
// SigmaOS ItsFOSS Productivity Applications Subsystem
// Zero-dependency Rust desktop applications inspired by Linux Mint Hypnotix IPTV, Warpinator LAN sharing, Bulky batch renamer, and Sticky notes

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. Linux Mint Hypnotix IPTV Player Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct IptvChannelNode {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub stream_url: String,
    pub logo_url: String,
}

#[derive(Debug, Clone)]
pub struct ItsFossHypnotixIptvEngine {
    pub channels: Vec<IptvChannelNode>,
    pub active_channel_id: Option<u32>,
}

impl ItsFossHypnotixIptvEngine {
    pub fn new() -> Self {
        let sample_channels = vec![
            IptvChannelNode {
                id: 1,
                name: "FreeNews 24/7".to_string(),
                category: "News".to_string(),
                stream_url: "https://iptv.example.org/freenews.m3u8".to_string(),
                logo_url: "https://iptv.example.org/logos/freenews.png".to_string(),
            },
            IptvChannelNode {
                id: 2,
                name: "TechTV Live".to_string(),
                category: "Technology".to_string(),
                stream_url: "https://iptv.example.org/techtv.m3u8".to_string(),
                logo_url: "https://iptv.example.org/logos/techtv.png".to_string(),
            },
        ];

        Self {
            channels: sample_channels,
            active_channel_id: None,
        }
    }

    pub fn filter_by_category(&self, category: &str) -> Vec<IptvChannelNode> {
        let cat_lower = category.to_lowercase();
        self.channels
            .iter()
            .filter(|c| c.category.to_lowercase() == cat_lower)
            .cloned()
            .collect()
    }

    pub fn play_channel(&mut self, id: u32) -> Result<String, &'static str> {
        if let Some(chan) = self.channels.iter().find(|c| c.id == id) {
            self.active_channel_id = Some(id);
            Ok(format!("HYPNOTIX_STREAM: Streaming '{}' ({})", chan.name, chan.stream_url))
        } else {
            Err("HYPNOTIX_STREAM: Channel ID not found in playlist")
        }
    }
}

impl Default for ItsFossHypnotixIptvEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Linux Mint Warpinator LAN File Sharing Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct WarpinatorLanPeer {
    pub peer_id: String,
    pub name: String,
    pub ip_address: [u8; 4],
    pub port: u16,
    pub is_trusted: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ItsFossWarpinatorLanSharingEngine {
    pub discovered_peers: BTreeMap<String, WarpinatorLanPeer>,
    pub port: u16,
}

impl ItsFossWarpinatorLanSharingEngine {
    pub fn new(port: u16) -> Self {
        Self {
            discovered_peers: BTreeMap::new(),
            port,
        }
    }

    pub fn register_lan_peer(&mut self, peer_id: &str, name: &str, ip: [u8; 4], is_trusted: bool) {
        self.discovered_peers.insert(
            peer_id.to_string(),
            WarpinatorLanPeer {
                peer_id: peer_id.to_string(),
                name: name.to_string(),
                ip_address: ip,
                port: self.port,
                is_trusted,
            },
        );
    }

    pub fn send_file_payload(&self, peer_id: &str, filename: &str, size_bytes: u64) -> Result<String, &'static str> {
        if let Some(peer) = self.discovered_peers.get(peer_id) {
            Ok(format!(
                "WARPINATOR_TRANSFER: Transmitted '{}' ({} bytes) to peer '{}' ({:?})",
                filename, size_bytes, peer.name, peer.ip_address
            ))
        } else {
            Err("WARPINATOR_TRANSFER: Target LAN peer not found")
        }
    }

    pub fn get_peer_count(&self) -> usize {
        self.discovered_peers.len()
    }
}

// ============================================================================
// 3. Linux Mint Bulky Batch File Renamer Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct RenameRuleResult {
    pub original_path: String,
    pub new_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct ItsFossBulkyBatchRenamerEngine;

impl ItsFossBulkyBatchRenamerEngine {
    pub fn apply_replace_rule(files: &[&str], find_str: &str, replace_str: &str) -> Vec<RenameRuleResult> {
        let mut results = Vec::new();
        for &file in files {
            let new_name = file.replace(find_str, replace_str);
            results.push(RenameRuleResult {
                original_path: file.to_string(),
                new_path: new_name,
            });
        }
        results
    }
}

// ============================================================================
// 4. Linux Mint Sticky Desktop Notes Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct StickyNoteEntry {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub color_tag: String,
    pub canvas_x: u32,
    pub canvas_y: u32,
}

#[derive(Debug, Clone)]
pub struct ItsFossStickyNotesEngine {
    pub notes: Vec<StickyNoteEntry>,
}

impl ItsFossStickyNotesEngine {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn create_note(&mut self, id: u32, title: &str, content: &str, color_tag: &str) {
        self.notes.push(StickyNoteEntry {
            id,
            title: title.to_string(),
            content: content.to_string(),
            color_tag: color_tag.to_string(),
            canvas_x: 100,
            canvas_y: 100,
        });
    }

    pub fn get_note_count(&self) -> usize {
        self.notes.len()
    }
}

impl Default for ItsFossStickyNotesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Sovereign ItsFOSS Productivity Apps Master Suite
// ============================================================================

#[derive(Debug, Default)]
pub struct SovereignItsFossAppsSuite {
    pub hypnotix: ItsFossHypnotixIptvEngine,
    pub warpinator: ItsFossWarpinatorLanSharingEngine,
    pub bulky: ItsFossBulkyBatchRenamerEngine,
    pub sticky: ItsFossStickyNotesEngine,
}

impl SovereignItsFossAppsSuite {
    pub fn new() -> Self {
        Self {
            hypnotix: ItsFossHypnotixIptvEngine::new(),
            warpinator: ItsFossWarpinatorLanSharingEngine::new(42000),
            bulky: ItsFossBulkyBatchRenamerEngine,
            sticky: ItsFossStickyNotesEngine::new(),
        }
    }

    pub fn synthesize_and_verify_all(&mut self) -> bool {
        // Verify Hypnotix
        let play_res = self.hypnotix.play_channel(1);
        let hypnotix_ok = play_res.is_ok();

        // Verify Warpinator
        self.warpinator.register_lan_peer("p1", "laptop_mint", [192, 168, 1, 100], true);
        let send_res = self.warpinator.send_file_payload("p1", "ISO_image.iso", 1000000);
        let warpinator_ok = send_res.is_ok() && self.warpinator.get_peer_count() == 1;

        // Verify Bulky
        let renamed = ItsFossBulkyBatchRenamerEngine::apply_replace_rule(&["IMG_01.jpg", "IMG_02.jpg"], "IMG_", "VACATION_");
        let bulky_ok = renamed.len() == 2 && renamed[0].new_path == "VACATION_01.jpg";

        // Verify Sticky Notes
        self.sticky.create_note(10, "Todo", "Finish SigmaOS parity", "yellow");
        let sticky_ok = self.sticky.get_note_count() == 1;

        hypnotix_ok && warpinator_ok && bulky_ok && sticky_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypnotix_iptv_engine() {
        let mut hyp = ItsFossHypnotixIptvEngine::new();
        let news = hyp.filter_by_category("News");
        assert_eq!(news.len(), 1);
        assert!(hyp.play_channel(1).is_ok());
    }

    #[test]
    fn test_warpinator_lan_sharing_engine() {
        let mut warp = ItsFossWarpinatorLanSharingEngine::new(42000);
        warp.register_lan_peer("peer_alpha", "Alpha_PC", [10, 0, 0, 5], true);
        assert_eq!(warp.get_peer_count(), 1);
        assert!(warp.send_file_payload("peer_alpha", "doc.pdf", 2048).is_ok());
    }

    #[test]
    fn test_bulky_and_sticky_engines() {
        let files = vec!["track_01.mp3", "track_02.mp3"];
        let renamed = ItsFossBulkyBatchRenamerEngine::apply_replace_rule(&files, "track_", "song_");
        assert_eq!(renamed[0].new_path, "song_01.mp3");

        let mut sticky = ItsFossStickyNotesEngine::new();
        sticky.create_note(1, "Ideas", "Test notes", "blue");
        assert_eq!(sticky.get_note_count(), 1);
    }

    #[test]
    fn test_sovereign_itsfoss_apps_suite() {
        let mut suite = SovereignItsFossAppsSuite::new();
        assert!(suite.synthesize_and_verify_all());
    }
}
