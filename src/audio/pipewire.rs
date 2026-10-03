//! PipeWire-inspired Audio/Video Server
//! Modern audio routing and low-latency audio graph processing
//! Reference: PipeWire architecture and Linux audio subsystem

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Audio sample format
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    S16Le = 1,            // 16-bit signed little-endian
    S24Le = 2,            // 24-bit signed little-endian
    S32Le = 3,            // 32-bit signed little-endian
    F32Le = 4,            // 32-bit float little-endian
    F64Le = 5,            // 64-bit float little-endian
}

impl AudioFormat {
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            AudioFormat::S16Le => 2,
            AudioFormat::S24Le => 3,
            AudioFormat::S32Le => 4,
            AudioFormat::F32Le => 4,
            AudioFormat::F64Le => 8,
        }
    }
}

/// Audio stream parameters
#[derive(Debug, Clone, Copy)]
pub struct AudioParams {
    pub format: AudioFormat,
    pub rate: u32,            // Sample rate in Hz
    pub channels: u32,
    pub buffer_frames: u32,   // Buffer size in frames
}

impl AudioParams {
    pub fn default_params() -> Self {
        Self {
            format: AudioFormat::F32Le,
            rate: 48000,
            channels: 2,
            buffer_frames: 1024,
        }
    }

    pub fn buffer_bytes(&self) -> usize {
        self.buffer_frames as usize * self.channels as usize * self.format.bytes_per_sample()
    }
}

/// Node types in the audio graph
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Source = 1,           // Audio source (microphone, file)
    Sink = 2,             // Audio sink (speakers, file)
    Filter = 3,           // Audio processor (EQ, compressor)
    Adapter = 4,          // Format converter
    Profiler = 5,         // Latency profiler
}

/// Port direction
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input = 1,
    Output = 2,
}

/// Audio port structure
#[derive(Debug, Clone)]
pub struct AudioPort {
    pub port_id: u32,
    pub node_id: u32,
    pub direction: PortDirection,
    pub params: AudioParams,
    pub connected_to: Option<u32>,   // Connected port ID
}

impl AudioPort {
    pub fn new(port_id: u32, node_id: u32, direction: PortDirection, params: AudioParams) -> Self {
        Self {
            port_id,
            node_id,
            direction,
            params,
            connected_to: None,
        }
    }
}

/// Audio node in the processing graph
pub struct AudioNode {
    pub node_id: u32,
    pub node_type: NodeType,
    pub name: Vec<u8>,
    pub input_ports: Vec<u32>,
    pub output_ports: Vec<u32>,
    pub state: NodeState,
    pub process_callback: Option<ProcessCallback>,
}

/// Node processing state
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Idle = 0,
    Creating = 1,
    Suspended = 2,
    Running = 3,
    Error = 4,
}

/// Audio processing callback type
pub type ProcessCallback = fn(&[f32], &mut [f32], u32) -> Result<(), AudioError>;

impl AudioNode {
    pub fn new(node_id: u32, node_type: NodeType, name: &[u8]) -> Self {
        Self {
            node_id,
            node_type,
            name: name.to_vec(),
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            state: NodeState::Creating,
            process_callback: None,
        }
    }

    /// Add input port
    pub fn add_input(&mut self, port_id: u32) {
        self.input_ports.push(port_id);
    }

    /// Add output port
    pub fn add_output(&mut self, port_id: u32) {
        self.output_ports.push(port_id);
    }

    /// Process audio buffer
    pub fn process(&self, input: &[f32], output: &mut [f32], frames: u32) -> Result<(), AudioError> {
        if let Some(callback) = self.process_callback {
            callback(input, output, frames)
        } else {
            // Default passthrough
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            Ok(())
        }
    }
}

/// Audio graph link (connection between ports)
#[derive(Debug, Clone, Copy)]
pub struct GraphLink {
    pub link_id: u32,
    pub output_port: u32,
    pub input_port: u32,
    pub active: bool,
}

/// PipeWire core structure
pub struct PipeWireCore {
    pub nodes: BTreeMap<u32, AudioNode>,
    pub ports: BTreeMap<u32, AudioPort>,
    pub links: BTreeMap<u32, GraphLink>,
    next_id: AtomicU32,
    sample_clock: AtomicU64,
    pub quantum: u32,         // Processing quantum (buffer size)
    pub rate: u32,            // Sample rate
}

impl PipeWireCore {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            ports: BTreeMap::new(),
            links: BTreeMap::new(),
            next_id: AtomicU32::new(1),
            sample_clock: AtomicU64::new(0),
            quantum: 1024,
            rate: 48000,
        }
    }

    /// Allocate new object ID
    fn alloc_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Create audio node
    pub fn create_node(&mut self, node_type: NodeType, name: &[u8]) -> u32 {
        let node_id = self.alloc_id();
        let node = AudioNode::new(node_id, node_type, name);
        self.nodes.insert(node_id, node);
        node_id
    }

    /// Create audio port
    pub fn create_port(
        &mut self,
        node_id: u32,
        direction: PortDirection,
        params: AudioParams,
    ) -> Result<u32, AudioError> {
        let node = self.nodes.get_mut(&node_id)
            .ok_or(AudioError::InvalidNode)?;

        let port_id = self.alloc_id();
        let port = AudioPort::new(port_id, node_id, direction, params);
        
        match direction {
            PortDirection::Input => node.add_input(port_id),
            PortDirection::Output => node.add_output(port_id),
        }

        self.ports.insert(port_id, port);
        Ok(port_id)
    }

    /// Link two ports (output -> input)
    pub fn link_ports(&mut self, output_port: u32, input_port: u32) -> Result<u32, AudioError> {
        // Validate ports exist and have correct directions
        let out_port = self.ports.get(&output_port)
            .ok_or(AudioError::InvalidPort)?;
        let in_port = self.ports.get(&input_port)
            .ok_or(AudioError::InvalidPort)?;

        if out_port.direction != PortDirection::Output {
            return Err(AudioError::InvalidDirection);
        }
        if in_port.direction != PortDirection::Input {
            return Err(AudioError::InvalidDirection);
        }

        let link_id = self.alloc_id();
        let link = GraphLink {
            link_id,
            output_port,
            input_port,
            active: true,
        };

        // Update port connections
        self.ports.get_mut(&output_port).unwrap().connected_to = Some(input_port);
        self.ports.get_mut(&input_port).unwrap().connected_to = Some(output_port);

        self.links.insert(link_id, link);
        Ok(link_id)
    }

    /// Remove link
    pub fn unlink_ports(&mut self, link_id: u32) -> Result<(), AudioError> {
        let link = self.links.remove(&link_id)
            .ok_or(AudioError::InvalidLink)?;

        // Clear port connections
        if let Some(port) = self.ports.get_mut(&link.output_port) {
            port.connected_to = None;
        }
        if let Some(port) = self.ports.get_mut(&link.input_port) {
            port.connected_to = None;
        }

        Ok(())
    }

    /// Start node processing
    pub fn start_node(&mut self, node_id: u32) -> Result<(), AudioError> {
        let node = self.nodes.get_mut(&node_id)
            .ok_or(AudioError::InvalidNode)?;
        node.state = NodeState::Running;
        Ok(())
    }

    /// Stop node processing
    pub fn stop_node(&mut self, node_id: u32) -> Result<(), AudioError> {
        let node = self.nodes.get_mut(&node_id)
            .ok_or(AudioError::InvalidNode)?;
        node.state = NodeState::Suspended;
        Ok(())
    }

    /// Process audio graph (one quantum)
    pub fn process_graph(&mut self) -> Result<(), AudioError> {
        // Process all active nodes in topological order
        // In real implementation: use dependency graph for correct ordering
        
        for (_id, node) in &self.nodes {
            if node.state != NodeState::Running {
                continue;
            }

            // Get input buffers
            let input_buffer = vec![0.0f32; self.quantum as usize * 2]; // Stereo
            let mut output_buffer = vec![0.0f32; self.quantum as usize * 2];

            // Process
            node.process(&input_buffer, &mut output_buffer, self.quantum)?;

            // Route output to connected nodes
            // In real implementation: transfer data to connected input ports
        }

        // Advance sample clock
        self.sample_clock.fetch_add(self.quantum as u64, Ordering::SeqCst);

        Ok(())
    }

    /// Get current sample time
    pub fn get_time(&self) -> u64 {
        self.sample_clock.load(Ordering::SeqCst)
    }

    /// Set graph quantum (buffer size)
    pub fn set_quantum(&mut self, quantum: u32) -> Result<(), AudioError> {
        if quantum < 32 || quantum > 8192 {
            return Err(AudioError::InvalidQuantum);
        }
        self.quantum = quantum;
        Ok(())
    }
}

/// Audio session (client connection)
pub struct AudioSession {
    pub session_id: u32,
    pub client_name: Vec<u8>,
    pub nodes: Vec<u32>,
}

impl AudioSession {
    pub fn new(session_id: u32, client_name: &[u8]) -> Self {
        Self {
            session_id,
            client_name: client_name.to_vec(),
            nodes: Vec::new(),
        }
    }
}

/// Audio device abstraction
pub struct AudioDevice {
    pub device_id: u32,
    pub name: Vec<u8>,
    pub is_source: bool,      // true = input device, false = output device
    pub params: AudioParams,
    pub node_id: Option<u32>, // Associated PipeWire node
}

/// Error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioError {
    InvalidNode,
    InvalidPort,
    InvalidLink,
    InvalidDirection,
    InvalidQuantum,
    GraphCycle,
    BufferUnderrun,
    BufferOverrun,
    FormatMismatch,
}

/// Built-in audio processors
pub mod processors {
    use super::*;

    /// Simple gain/volume control
    pub fn gain_processor(input: &[f32], output: &mut [f32], frames: u32) -> Result<(), AudioError> {
        let gain = 0.5f32; // 50% volume
        for i in 0..(frames as usize * 2) {
            output[i] = input[i] * gain;
        }
        Ok(())
    }

    /// Simple low-pass filter (smoothing)
    pub fn lowpass_processor(input: &[f32], output: &mut [f32], frames: u32) -> Result<(), AudioError> {
        let alpha = 0.1f32; // Filter coefficient
        let mut prev = 0.0f32;
        for i in 0..(frames as usize * 2) {
            prev = prev + alpha * (input[i] - prev);
            output[i] = prev;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipewire_create_node() {
        let mut pw = PipeWireCore::new();
        let node_id = pw.create_node(NodeType::Source, b"test_source");
        assert!(pw.nodes.contains_key(&node_id));
    }

    #[test]
    fn test_audio_params() {
        let params = AudioParams::default_params();
        assert_eq!(params.rate, 48000);
        assert_eq!(params.channels, 2);
    }

    #[test]
    fn test_port_linking() {
        let mut pw = PipeWireCore::new();
        let node1 = pw.create_node(NodeType::Source, b"source");
        let node2 = pw.create_node(NodeType::Sink, b"sink");
        
        let out_port = pw.create_port(node1, PortDirection::Output, AudioParams::default_params()).unwrap();
        let in_port = pw.create_port(node2, PortDirection::Input, AudioParams::default_params()).unwrap();
        
        let link_id = pw.link_ports(out_port, in_port).unwrap();
        assert!(pw.links.contains_key(&link_id));
    }
}
