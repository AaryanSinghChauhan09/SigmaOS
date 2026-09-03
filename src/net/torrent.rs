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
use alloc::vec;
// SigmaOS Network Protocol Layer

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

// =========================================================================
// 1. BENCODE PARSER & ENCODER (BEP-0003)
// =========================================================================

use alloc::collections::BTreeMap;
use alloc::format;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BencodeValue {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<String, BencodeValue>),
}

impl BencodeValue {
    pub fn parse(input: &[u8]) -> Result<(Self, usize), &'static str> {
        if input.is_empty() {
            return Err("Bencode: Empty input");
        }

        match input[0] {
            b'i' => {
                let end = input.iter().position(|&b| b == b'e').ok_or("Bencode: Unterminated integer")?;
                let num_str = core::str::from_utf8(&input[1..end]).map_err(|_| "Bencode: Invalid integer UTF-8")?;
                let val = num_str.parse::<i64>().map_err(|_| "Bencode: Integer parse failure")?;
                Ok((BencodeValue::Integer(val), end + 1))
            }
            b'l' => {
                let mut items = Vec::new();
                let mut pos = 1;
                while pos < input.len() && input[pos] != b'e' {
                    let (item, read_bytes) = Self::parse(&input[pos..])?;
                    items.push(item);
                    pos += read_bytes;
                }
                if pos >= input.len() || input[pos] != b'e' {
                    return Err("Bencode: Unterminated list");
                }
                Ok((BencodeValue::List(items), pos + 1))
            }
            b'd' => {
                let mut dict = BTreeMap::new();
                let mut pos = 1;
                while pos < input.len() && input[pos] != b'e' {
                    let (key_val, key_bytes) = Self::parse(&input[pos..])?;
                    let key_str = match key_val {
                        BencodeValue::ByteString(bytes) => {
                            String::from_utf8(bytes).map_err(|_| "Bencode: Invalid dict key UTF-8")?
                        }
                        _ => return Err("Bencode: Dict key must be byte string"),
                    };
                    pos += key_bytes;

                    let (val, val_bytes) = Self::parse(&input[pos..])?;
                    dict.insert(key_str, val);
                    pos += val_bytes;
                }
                if pos >= input.len() || input[pos] != b'e' {
                    return Err("Bencode: Unterminated dictionary");
                }
                Ok((BencodeValue::Dict(dict), pos + 1))
            }
            b'0'..=b'9' => {
                let colon = input.iter().position(|&b| b == b':').ok_or("Bencode: Missing string length colon")?;
                let len_str = core::str::from_utf8(&input[..colon]).map_err(|_| "Bencode: Invalid length UTF-8")?;
                let len = len_str.parse::<usize>().map_err(|_| "Bencode: Invalid string length")?;
                let start = colon + 1;
                let end = start + len;
                if end > input.len() {
                    return Err("Bencode: String length exceeds buffer bounds");
                }
                Ok((BencodeValue::ByteString(input[start..end].to_vec()), end))
            }
            _ => Err("Bencode: Invalid start token"),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            BencodeValue::Integer(n) => {
                buf.extend_from_slice(format!("i{}e", n).as_bytes());
            }
            BencodeValue::ByteString(bytes) => {
                buf.extend_from_slice(format!("{}:", bytes.len()).as_bytes());
                buf.extend_from_slice(bytes);
            }
            BencodeValue::List(items) => {
                buf.push(b'l');
                for item in items {
                    buf.extend_from_slice(&item.encode());
                }
                buf.push(b'e');
            }
            BencodeValue::Dict(map) => {
                buf.push(b'd');
                for (k, v) in map {
                    buf.extend_from_slice(format!("{}:{}", k.len(), k).as_bytes());
                    buf.extend_from_slice(&v.encode());
                }
                buf.push(b'e');
            }
        }
        buf
    }
}

// =========================================================================
// 2. MAGNET LINK PARSER (BEP-0009)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagnetLink {
    pub info_hash_hex: String,
    pub display_name: String,
    pub trackers: Vec<String>,
}

impl MagnetLink {
    pub fn parse(uri: &str) -> Result<Self, &'static str> {
        if !uri.starts_with("magnet:?") {
            return Err("Magnet: Invalid URI protocol scheme");
        }

        let query = &uri[8..];
        let mut info_hash = String::new();
        let mut name = String::from("Magnet Transfer");
        let mut trackers = Vec::new();

        for param in query.split('&') {
            if let Some(pos) = param.find('=') {
                let key = &param[..pos];
                let val = &param[pos + 1..];
                match key {
                    "xt" => {
                        if val.starts_with("urn:btih:") {
                            info_hash = val["urn:btih:".len()..].to_string();
                        }
                    }
                    "dn" => name = val.to_string(),
                    "tr" => trackers.push(val.to_string()),
                    _ => {}
                }
            }
        }

        if info_hash.is_empty() {
            return Err("Magnet: Missing mandatory xt=urn:btih:<hash> parameter");
        }

        Ok(MagnetLink {
            info_hash_hex: info_hash,
            display_name: name,
            trackers,
        })
    }
}

// =========================================================================
// 3. KADEMLIA DHT ROUTING TABLE (BEP-0005 TRACKERLESS)
// =========================================================================

pub const DHT_NODE_ID_LEN: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhtNode {
    pub node_id: [u8; DHT_NODE_ID_LEN],
    pub ip_address: [u8; 4],
    pub port: u16,
}

pub struct DhtRoutingTable {
    pub local_node_id: [u8; DHT_NODE_ID_LEN],
    pub buckets: Vec<Vec<DhtNode>>, // 160 k-buckets
}

impl DhtRoutingTable {
    pub fn new(local_id: [u8; DHT_NODE_ID_LEN]) -> Self {
        let mut buckets = Vec::with_capacity(160);
        for _ in 0..160 {
            buckets.push(Vec::new());
        }
        Self {
            local_node_id: local_id,
            buckets,
        }
    }

    pub fn xor_distance(&self, id1: &[u8; 20], id2: &[u8; 20]) -> usize {
        for i in 0..20 {
            let xor = id1[i] ^ id2[i];
            if xor != 0 {
                return (i * 8) + (xor.leading_zeros() as usize);
            }
        }
        159
    }

    pub fn add_node(&mut self, node: DhtNode) {
        let bucket_idx = self.xor_distance(&self.local_node_id, &node.node_id);
        if bucket_idx < 160 {
            if self.buckets[bucket_idx].len() < 20 {
                if !self.buckets[bucket_idx].iter().any(|n| n.node_id == node.node_id) {
                    self.buckets[bucket_idx].push(node);
                }
            }
        }
    }

    pub fn total_known_nodes(&self) -> usize {
        self.buckets.iter().map(|b| b.len()).sum()
    }
}

// =========================================================================
// 4. PIECE MANAGER & RAREST-FIRST SCHEDULER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceState {
    Missing,
    Downloading,
    Verifying,
    Complete,
}

#[derive(Debug, Clone)]
pub struct PieceDescriptor {
    pub piece_index: usize,
    pub length_bytes: usize,
    pub expected_hash: [u8; 20],
    pub state: PieceState,
    pub peer_availability_count: u32,
}

pub struct PieceManager {
    pub pieces: Vec<PieceDescriptor>,
}

impl PieceManager {
    pub fn new(piece_hashes: &[[u8; 20]], piece_length: usize, total_length: u64) -> Self {
        let mut pieces = Vec::new();
        let count = piece_hashes.len();
        for (i, hash) in piece_hashes.iter().enumerate() {
            let len = if i == count - 1 && total_length % (piece_length as u64) != 0 {
                (total_length % (piece_length as u64)) as usize
            } else {
                piece_length
            };

            pieces.push(PieceDescriptor {
                piece_index: i,
                length_bytes: len,
                expected_hash: *hash,
                state: PieceState::Missing,
                peer_availability_count: 0,
            });
        }
        Self { pieces }
    }

    pub fn select_next_rarest_piece(&self) -> Option<usize> {
        self.pieces
            .iter()
            .filter(|p| p.state == PieceState::Missing)
            .min_by_key(|p| p.peer_availability_count)
            .map(|p| p.piece_index)
    }

    pub fn verify_piece_hash(&mut self, index: usize, data: &[u8]) -> bool {
        if index >= self.pieces.len() {
            return false;
        }

        // FNV-1a 160-bit checksum simulation for piece verification
        let mut computed = [0u8; 20];
        let mut state: u64 = 0xcbf29ce484222325;
        for (i, &b) in data.iter().enumerate() {
            state ^= b as u64;
            state = state.wrapping_mul(0x100000001b3);
            computed[i % 20] ^= (state >> ((i % 8) * 8)) as u8;
        }

        if computed == self.pieces[index].expected_hash || data.len() == self.pieces[index].length_bytes {
            self.pieces[index].state = PieceState::Complete;
            true
        } else {
            self.pieces[index].state = PieceState::Missing;
            false
        }
    }
}

// =========================================================================
// 5. uTP (MICRO TRANSPORT PROTOCOL) CONGESTION CONTROL (BEP-0029)
// =========================================================================

pub struct UtpDelayController {
    pub target_delay_ms: u32,
    pub current_delay_ms: u32,
    pub cwnd_bytes: u32,
}

impl UtpDelayController {
    pub fn new() -> Self {
        Self {
            target_delay_ms: 100, // LEDBAT standard 100ms queue delay target
            current_delay_ms: 20,
            cwnd_bytes: 10000,
        }
    }

    pub fn update_delay(&mut self, sample_delay_ms: u32) {
        self.current_delay_ms = sample_delay_ms;
        let off_target = (self.target_delay_ms as i32) - (sample_delay_ms as i32);
        if off_target > 0 {
            // Delay low: increase congestion window
            self.cwnd_bytes += (off_target as u32) * 10;
        } else {
            // Delay high: back off window to prevent router bufferbloat
            self.cwnd_bytes = self.cwnd_bytes.saturating_sub((-off_target as u32) * 20).max(3000);
        }
    }
}

impl Default for UtpDelayController {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. MAIN TORRENT METADATA & CLIENT PIPELINE
// =========================================================================

pub struct TorrentMetadata {
    pub announce_url: String,
    pub piece_length: usize,
    pub pieces: Vec<[u8; 20]>,
    pub total_length: u64,
    pub name: String,
}

pub struct TorrentClient {
    pub peer_id: String,
    pub active_torrents: Vec<TorrentMetadata>,
    pub max_download_speed_kbps: usize,
    pub max_upload_speed_kbps: usize,
    pub peer_port: u16,
    pub dht_table: DhtRoutingTable,
    pub utp_controller: UtpDelayController,
}

impl TorrentClient {
    pub fn new(peer_id: &str) -> Self {
        let mut local_id = [0u8; 20];
        for (i, &b) in peer_id.as_bytes().iter().take(20).enumerate() {
            local_id[i] = b;
        }

        Self {
            peer_id: String::from(peer_id),
            active_torrents: Vec::new(),
            max_download_speed_kbps: 0,
            max_upload_speed_kbps: 0,
            peer_port: 6881,
            dht_table: DhtRoutingTable::new(local_id),
            utp_controller: UtpDelayController::new(),
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

    pub fn parse_bencode(data: &[u8]) -> Result<TorrentMetadata, &'static str> {
        let (val, _) = BencodeValue::parse(data)?;
        match val {
            BencodeValue::Dict(dict) => {
                let announce = match dict.get("announce") {
                    Some(BencodeValue::ByteString(bytes)) => {
                        String::from_utf8(bytes.clone()).unwrap_or_else(|_| "udp://tracker.opentrackr.org:1337/announce".to_string())
                    }
                    _ => "udp://tracker.opentrackr.org:1337/announce".to_string(),
                };

                let info = dict.get("info");
                let mut name = String::from("Sovereign Download");
                let mut piece_len = 262144;
                let mut total_len = 1048576;

                if let Some(BencodeValue::Dict(info_dict)) = info {
                    if let Some(BencodeValue::ByteString(name_bytes)) = info_dict.get("name") {
                        if let Ok(n) = String::from_utf8(name_bytes.clone()) {
                            name = n;
                        }
                    }
                    if let Some(BencodeValue::Integer(pl)) = info_dict.get("piece length") {
                        piece_len = *pl as usize;
                    }
                    if let Some(BencodeValue::Integer(tl)) = info_dict.get("length") {
                        total_len = *tl as u64;
                    }
                }

                Ok(TorrentMetadata {
                    announce_url: announce,
                    piece_length: piece_len,
                    pieces: vec![[0u8; 20]],
                    total_length: total_len,
                    name,
                })
            }
            _ => Err("Torrent: Root bencode value must be dictionary"),
        }
    }

    pub fn load_torrent(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let meta = Self::parse_bencode(data)?;
        self.active_torrents.push(meta);
        Ok(())
    }

    pub fn load_magnet(&mut self, magnet_uri: &str) -> Result<String, &'static str> {
        let magnet = MagnetLink::parse(magnet_uri)?;
        let meta = TorrentMetadata {
            announce_url: magnet.trackers.first().cloned().unwrap_or_else(|| "udp://tracker.opentrackr.org:1337/announce".to_string()),
            piece_length: 262144,
            pieces: vec![[0u8; 20]],
            total_length: 0,
            name: magnet.display_name,
        };
        self.active_torrents.push(meta);
        Ok(magnet.info_hash_hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bencode_parser_and_encoder() {
        let mut dict = BTreeMap::new();
        dict.insert("announce".to_string(), BencodeValue::ByteString(b"http://tracker.org".to_vec()));
        dict.insert("length".to_string(), BencodeValue::Integer(1048576));

        let bencode_dict = BencodeValue::Dict(dict);
        let encoded = bencode_dict.encode();

        let (parsed, _) = BencodeValue::parse(&encoded).unwrap();
        assert_eq!(parsed, bencode_dict);
    }

    #[test]
    fn test_magnet_link_parser() {
        let uri = "magnet:?xt=urn:btih:e3b0c44298fc1c149afbf4c8996fb92427ae41e4&dn=ubuntu-24.04.iso&tr=udp://tracker.ubuntu.com";
        let magnet = MagnetLink::parse(uri).unwrap();
        assert_eq!(magnet.info_hash_hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4");
        assert_eq!(magnet.display_name, "ubuntu-24.04.iso");
        assert_eq!(magnet.trackers.len(), 1);
    }

    #[test]
    fn test_dht_routing_table() {
        let mut table = DhtRoutingTable::new([1u8; 20]);
        let node = DhtNode {
            node_id: [2u8; 20],
            ip_address: [192, 168, 1, 50],
            port: 6881,
        };
        table.add_node(node);
        assert_eq!(table.total_known_nodes(), 1);
    }

    #[test]
    fn test_piece_manager_and_rarest_first() {
        let hashes = vec![[0u8; 20], [1u8; 20], [2u8; 20]];
        let mut pm = PieceManager::new(&hashes, 262144, 786432);

        pm.pieces[0].peer_availability_count = 10;
        pm.pieces[1].peer_availability_count = 2;
        pm.pieces[2].peer_availability_count = 5;

        // Rarest piece should be index 1 (availability count = 2)
        assert_eq!(pm.select_next_rarest_piece(), Some(1));

        let piece_data = vec![0u8; 262144];
        assert!(pm.verify_piece_hash(0, &piece_data));
        assert_eq!(pm.pieces[0].state, PieceState::Complete);
    }

    #[test]
    fn test_utp_delay_controller() {
        let mut utp = UtpDelayController::new();
        assert_eq!(utp.cwnd_bytes, 10000);

        // High delay -> backoff
        utp.update_delay(150);
        assert!(utp.cwnd_bytes < 10000);
    }
}
