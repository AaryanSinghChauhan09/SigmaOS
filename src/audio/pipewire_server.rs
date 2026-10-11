// SigmaOS PipeWire Audio & Video Graph Routing Server
// Coordinates low-latency audio processing nodes, ports, links, and format negotiation.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Audio Sample Format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSampleFormat {
    S16Le,
    S24Le,
    S32Le,
    F32Le,
}

/// Negotiated Audio Stream Format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioStreamFormat {
    pub sample_format: AudioSampleFormat,
    pub channels: u8,
    pub sample_rate_hz: u32,
}

/// Port Direction (Input / Output)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PwPortDirection {
    Input,
    Output,
}

/// Processing Node in the Audio Graph (e.g. App, Filter, Hardware Sink)
#[derive(Debug, Clone)]
pub struct PwNode {
    pub id: u32,
    pub name: String,
    pub media_class: String, // e.g. "Audio/Sink", "Audio/Source", "Stream/Output/Audio"
    pub input_ports: Vec<u32>,
    pub output_ports: Vec<u32>,
    pub is_active: bool,
}

/// Link connecting an Output Port to an Input Port
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PwLink {
    pub link_id: u32,
    pub out_port_id: u32,
    pub in_port_id: u32,
    pub is_active: bool,
}

/// PipeWire Core Graph Server
#[derive(Debug, Clone, Default)]
pub struct PipeWireGraphServer {
    pub nodes: Vec<PwNode>,
    pub links: Vec<PwLink>,
    pub next_id: u32,
    pub processed_cycles: u64,
}

impl PipeWireGraphServer {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            next_id: 1,
            processed_cycles: 0,
        }
    }

    /// Register a new audio processing node
    pub fn create_node(&mut self, name: &str, media_class: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let node = PwNode {
            id,
            name: String::from(name),
            media_class: String::from(media_class),
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            is_active: true,
        };

        self.nodes.push(node);
        id
    }

    /// Link an output port of a source node to an input port of a destination node
    pub fn link_ports(&mut self, out_port: u32, in_port: u32) -> Result<u32, &'static str> {
        let link_id = self.next_id;
        self.next_id += 1;

        self.links.push(PwLink {
            link_id,
            out_port_id: out_port,
            in_port_id: in_port,
            is_active: true,
        });

        Ok(link_id)
    }

    /// Process a quantum cycle across the audio graph
    pub fn process_quantum(&mut self, _frames: usize) -> usize {
        self.processed_cycles += 1;
        self.links.iter().filter(|l| l.is_active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipewire_graph_routing() {
        let mut server = PipeWireGraphServer::new();

        let app_node = server.create_node("Spotify", "Stream/Output/Audio");
        let sink_node = server.create_node("ALC892_Analog", "Audio/Sink");

        assert_eq!(server.nodes.len(), 2);
        assert_ne!(app_node, sink_node);

        let link = server.link_ports(app_node * 10, sink_node * 10).expect("Linked");
        assert_eq!(server.links.len(), 1);

        let active_links = server.process_quantum(512);
        assert_eq!(active_links, 1);
        assert_eq!(server.processed_cycles, 1);
    }
}
