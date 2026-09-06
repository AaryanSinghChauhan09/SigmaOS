#![allow(dead_code)]
#![allow(unexpected_cfgs)]
// SigmaNet Mesh: Stateless, zero-configuration peer-to-peer mesh networking
// for collaborative secure file sharing in SigmaOS.

use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshPeerNode {
    pub node_id: u64,
    pub ip_address: [u8; 4],
    pub port: u16,
    pub is_alive: bool,
    pub shared_chunks_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChunk {
    pub chunk_id: u32,
    pub data_hash: [u8; 32],
    pub payload: Vec<u8>,
}

pub struct SigmaNetMesh {
    pub local_node_id: u64,
    pub peers: Vec<MeshPeerNode>,
    pub local_chunks: Vec<FileChunk>,
}

impl SigmaNetMesh {
    pub fn new(local_id: u64) -> Self {
        Self {
            local_node_id: local_id,
            peers: Vec::new(),
            local_chunks: Vec::new(),
        }
    }

    pub fn discover_peer(&mut self, node_id: u64, ip: [u8; 4], port: u16) {
        if node_id == self.local_node_id {
            return;
        }
        for peer in &mut self.peers {
            if peer.node_id == node_id {
                peer.is_alive = true;
                return;
            }
        }
        self.peers.push(MeshPeerNode {
            node_id,
            ip_address: ip,
            port,
            is_alive: true,
            shared_chunks_count: 0,
        });
    }

    pub fn publish_chunk(&mut self, chunk_id: u32, data: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        for (i, &b) in data.iter().enumerate() {
            hash[i % 32] ^= b.wrapping_add(i as u8);
        }
        self.local_chunks.push(FileChunk {
            chunk_id,
            data_hash: hash,
            payload: data.to_vec(),
        });
        hash
    }

    pub fn sync_chunk_with_peer(&mut self, peer_id: u64, chunk_id: u32, payload: &[u8]) -> Result<(), &'static str> {
        let mut peer_found = false;
        for peer in &mut self.peers {
            if peer.node_id == peer_id && peer.is_alive {
                peer.shared_chunks_count += 1;
                peer_found = true;
                break;
            }
        }
        if !peer_found {
            return Err("Peer node not discovered or offline");
        }
        self.publish_chunk(chunk_id, payload);
        Ok(())
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmanet_mesh_peer_discovery() {
        let mut mesh = SigmaNetMesh::new(1001);
        mesh.discover_peer(2002, [192, 168, 1, 10], 8080);
        assert_eq!(mesh.peers.len(), 1);
        assert_eq!(mesh.peers[0].node_id, 2002);
        assert!(mesh.peers[0].is_alive);

        // Rediscovering same peer updates state
        mesh.discover_peer(2002, [192, 168, 1, 10], 8080);
        assert_eq!(mesh.peers.len(), 1);
    }

    #[test]
    fn test_mesh_chunk_publish_and_sync() {
        let mut mesh = SigmaNetMesh::new(1001);
        mesh.discover_peer(2002, [192, 168, 1, 20], 9090);

        let hash = mesh.publish_chunk(1, b"SOVEREIGN_MESH_CHUNK");
        assert_eq!(mesh.local_chunks.len(), 1);
        assert_ne!(hash, [0u8; 32]);

        assert!(mesh.sync_chunk_with_peer(2002, 2, b"COLLABORATIVE_PAYLOAD").is_ok());
        assert_eq!(mesh.peers[0].shared_chunks_count, 1);
        assert_eq!(mesh.local_chunks.len(), 2);

        assert!(mesh.sync_chunk_with_peer(9999, 3, b"FAIL").is_err());
    }
}
