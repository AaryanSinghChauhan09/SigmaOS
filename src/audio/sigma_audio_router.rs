use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Source,
    Sink,
    Stream,
}

pub struct AudioNode {
    pub id: usize,
    pub name: String,
    pub node_type: NodeType,
    pub volume: f32,
    pub muted: bool,
}

pub struct AudioLink {
    pub source_id: usize,
    pub sink_id: usize,
}

pub struct AudioRouter {
    pub nodes: Vec<AudioNode>,
    pub links: Vec<AudioLink>,
    pub quantum_size: u32,
}

static NODE_ID_GEN: AtomicUsize = AtomicUsize::new(1);

impl AudioRouter {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            quantum_size: 256,
        }
    }

    pub fn add_node(&mut self, name: String, node_type: NodeType) -> usize {
        let id = NODE_ID_GEN.fetch_add(1, Ordering::SeqCst);
        self.nodes.push(AudioNode {
            id,
            name,
            node_type,
            volume: 1.0,
            muted: false,
        });
        id
    }

    pub fn link_nodes(&mut self, source_id: usize, sink_id: usize) {
        self.links.push(AudioLink { source_id, sink_id });
    }

    pub fn set_volume(&mut self, id: usize, volume: f32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.volume = volume.clamp(0.0, 1.0);
        }
    }

    pub fn set_mute(&mut self, id: usize, muted: bool) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.muted = muted;
        }
    }

    pub fn set_quantum_size(&mut self, size: u32) {
        self.quantum_size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_init() {
        let router = AudioRouter::new();
        assert_eq!(router.nodes.len(), 0);
        assert_eq!(router.quantum_size, 256);
    }

    #[test]
    fn test_add_node() {
        let mut router = AudioRouter::new();
        let id = router.add_node(String::from("Speaker"), NodeType::Sink);
        assert_eq!(router.nodes.len(), 1);
        assert_eq!(router.nodes[0].id, id);
        assert_eq!(router.nodes[0].node_type, NodeType::Sink);
    }

    #[test]
    fn test_link_nodes() {
        let mut router = AudioRouter::new();
        let src = router.add_node(String::from("App"), NodeType::Stream);
        let sink = router.add_node(String::from("Speaker"), NodeType::Sink);
        router.link_nodes(src, sink);
        assert_eq!(router.links.len(), 1);
        assert_eq!(router.links[0].source_id, src);
        assert_eq!(router.links[0].sink_id, sink);
    }

    #[test]
    fn test_volume_control() {
        let mut router = AudioRouter::new();
        let id = router.add_node(String::from("Speaker"), NodeType::Sink);
        router.set_volume(id, 0.5);
        assert_eq!(router.nodes[0].volume, 0.5);
        router.set_volume(id, 1.5);
        assert_eq!(router.nodes[0].volume, 1.0);
    }

    #[test]
    fn test_mute_control() {
        let mut router = AudioRouter::new();
        let id = router.add_node(String::from("Speaker"), NodeType::Sink);
        router.set_mute(id, true);
        assert_eq!(router.nodes[0].muted, true);
    }
}
