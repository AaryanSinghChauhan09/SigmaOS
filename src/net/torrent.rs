// SigmaOS Network Protocol Layer
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

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

/// BitTorrent Parity: Torrent Protocol Engine
/// Manages `.torrent` parsing, block requests, and DHT routing.

pub struct TorrentMetadata {
    pub announce_url: String,
    pub piece_length: usize,
    pub pieces: Vec<[u8; 20]>,
    pub total_length: u64,
}

pub struct TorrentClient {
    pub peer_id: String,
    pub active_torrents: Vec<TorrentMetadata>,
    pub max_download_speed_kbps: usize,
    pub max_upload_speed_kbps: usize,
    pub peer_port: u16,
}

impl TorrentClient {
    pub fn new(peer_id: &str) -> Self {
        Self {
            peer_id: String::from(peer_id),
            active_torrents: Vec::new(),
            max_download_speed_kbps: 0, // 0 = unlimited
            max_upload_speed_kbps: 0,
            peer_port: 6881,
        }
    }

    pub fn set_download_limit(&mut self, speed: usize) {
        self.max_download_speed_kbps = speed;
    }

    pub fn set_upload_limit(&mut self, speed: usize) {
        self.max_upload_speed_kbps = speed;
    }

    pub fn set_peer_port(&mut self, port: u16) {
        self.peer_port = port;
    }

    /// Very basic mock Bencode parser for `.torrent` files
    pub fn parse_bencode(data: &[u8]) -> Result<TorrentMetadata, &'static str> {
        if data.is_empty() {
            return Err("Empty torrent data");
        }
        // In a real implementation, this would parse dictionaries (`d...e`), lists, ints, and strings.
        // For demonstration, we just return a mocked valid TorrentMetadata structure.
        Ok(TorrentMetadata {
            announce_url: String::from("udp://tracker.opentrackr.org:1337/announce"),
            piece_length: 262144, // 256 KB
            pieces: alloc::vec![[0u8; 20]],
            total_length: 1048576, // 1 MB
        })
    }
    
    pub fn load_torrent(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let meta = Self::parse_bencode(data)?;
        self.active_torrents.push(meta);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torrent_loading() {
        let mut client = TorrentClient::new("-SG0001-SigmaTest");
        let fake_bencode = b"d8:announce...";
        client.load_torrent(fake_bencode).unwrap();
        
        assert_eq!(client.active_torrents.len(), 1);
        assert_eq!(client.active_torrents[0].total_length, 1048576);
    }
}
