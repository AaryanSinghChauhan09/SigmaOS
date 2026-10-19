//! SigmaFig - Figure and Diagram System
//!
//! Provides comprehensive figure and diagram management inspired by Fig Craft.
//! Supports diagram creation, node graphs, and visual connectors.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Node type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Rectangle,
    Circle,
    Diamond,
    Ellipse,
    Text,
    Image,
    Container,
}

/// Connector type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorType {
    Line,
    Arrow,
    Dashed,
    Curved,
    Orthogonal,
}

/// 2D position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FigSize {
    pub width: f64,
    pub height: f64,
}

impl FigSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

/// Diagram node
#[derive(Debug, Clone)]
pub struct DiagramNode {
    pub id: String,
    pub node_type: NodeType,
    pub position: Position,
    pub size: FigSize,
    pub text: String,
    pub color: String,
}

impl DiagramNode {
    pub fn new(id: String, node_type: NodeType, position: Position, size: FigSize) -> Self {
        Self {
            id,
            node_type,
            position,
            size,
            text: String::new(),
            color: "#FFFFFF".to_string(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_size(&mut self, size: FigSize) {
        self.size = size;
    }
}

/// Connector between nodes
#[derive(Debug, Clone)]
pub struct Connector {
    pub id: String,
    pub connector_type: ConnectorType,
    pub from_node: String,
    pub to_node: String,
    pub label: String,
}

impl Connector {
    pub fn new(
        id: String,
        connector_type: ConnectorType,
        from_node: String,
        to_node: String,
    ) -> Self {
        Self {
            id,
            connector_type,
            from_node,
            to_node,
            label: String::new(),
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

/// Diagram layer
#[derive(Debug, Clone)]
pub struct DiagramLayer {
    pub id: String,
    pub name: String,
    pub nodes: BTreeMap<String, DiagramNode>,
    pub connectors: BTreeMap<String, Connector>,
    pub visible: bool,
}

impl DiagramLayer {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            nodes: BTreeMap::new(),
            connectors: BTreeMap::new(),
            visible: true,
        }
    }

    pub fn add_node(&mut self, node: DiagramNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        self.nodes.remove(id).is_some()
    }

    pub fn get_node(&self, id: &str) -> Option<&DiagramNode> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut DiagramNode> {
        self.nodes.get_mut(id)
    }

    pub fn add_connector(&mut self, connector: Connector) {
        self.connectors.insert(connector.id.clone(), connector);
    }

    pub fn remove_connector(&mut self, id: &str) -> bool {
        self.connectors.remove(id).is_some()
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn connector_count(&self) -> usize {
        self.connectors.len()
    }
}

/// Diagram
#[derive(Debug, Clone)]
pub struct Diagram {
    pub id: String,
    pub name: String,
    pub layers: BTreeMap<String, DiagramLayer>,
    pub active_layer: Option<String>,
}

impl Diagram {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            layers: BTreeMap::new(),
            active_layer: None,
        }
    }

    pub fn add_layer(&mut self, layer: DiagramLayer) {
        self.layers.insert(layer.id.clone(), layer);
    }

    pub fn remove_layer(&mut self, id: &str) -> bool {
        if self.active_layer.as_ref() == Some(&id.to_string()) {
            self.active_layer = None;
        }
        self.layers.remove(id).is_some()
    }

    pub fn get_layer(&self, id: &str) -> Option<&DiagramLayer> {
        self.layers.get(id)
    }

    pub fn get_layer_mut(&mut self, id: &str) -> Option<&mut DiagramLayer> {
        self.layers.get_mut(id)
    }

    pub fn set_active_layer(&mut self, id: &str) -> bool {
        if self.layers.contains_key(id) {
            self.active_layer = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_active_layer(&self) -> Option<&DiagramLayer> {
        self.active_layer
            .as_ref()
            .and_then(|id| self.layers.get(id))
    }

    pub fn get_active_layer_mut(&mut self) -> Option<&mut DiagramLayer> {
        self.active_layer
            .as_ref()
            .and_then(|id| self.layers.get_mut(id.as_str()))
    }

    pub fn total_node_count(&self) -> usize {
        self.layers.values().map(|l| l.node_count()).sum()
    }

    pub fn total_connector_count(&self) -> usize {
        self.layers.values().map(|l| l.connector_count()).sum()
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

/// SigmaFig - Figure and diagram system manager
#[derive(Debug)]
pub struct SigmaFig {
    diagrams: BTreeMap<String, Diagram>,
    current_diagram: Option<String>,
}

impl SigmaFig {
    pub fn new() -> Self {
        Self {
            diagrams: BTreeMap::new(),
            current_diagram: None,
        }
    }

    pub fn add_diagram(&mut self, diagram: Diagram) {
        self.diagrams.insert(diagram.id.clone(), diagram);
    }

    pub fn remove_diagram(&mut self, id: &str) -> bool {
        if self.current_diagram.as_ref() == Some(&id.to_string()) {
            self.current_diagram = None;
        }
        self.diagrams.remove(id).is_some()
    }

    pub fn get_diagram(&self, id: &str) -> Option<&Diagram> {
        self.diagrams.get(id)
    }

    pub fn get_diagram_mut(&mut self, id: &str) -> Option<&mut Diagram> {
        self.diagrams.get_mut(id)
    }

    pub fn set_current_diagram(&mut self, id: &str) -> bool {
        if self.diagrams.contains_key(id) {
            self.current_diagram = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_diagram(&self) -> Option<&Diagram> {
        self.current_diagram
            .as_ref()
            .and_then(|id| self.diagrams.get(id))
    }

    pub fn get_current_diagram_mut(&mut self) -> Option<&mut Diagram> {
        self.current_diagram
            .as_ref()
            .and_then(|id| self.diagrams.get_mut(id.as_str()))
    }

    pub fn list_diagrams(&self) -> Vec<&Diagram> {
        self.diagrams.values().collect()
    }

    pub fn diagram_count(&self) -> usize {
        self.diagrams.len()
    }
}

impl Default for SigmaFig {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaFig instance
static mut SIGMAFIG_INSTANCE: Option<SigmaFig> = None;

/// Initialize the global SigmaFig instance
pub fn init_sigmafig() {
    unsafe {
        SIGMAFIG_INSTANCE = Some(SigmaFig::new());
    }
}

/// Get the global SigmaFig instance
pub fn get_sigmafig() -> Option<&'static SigmaFig> {
    unsafe { SIGMAFIG_INSTANCE.as_ref() }
}

/// Get the global SigmaFig instance (mutable)
pub fn get_sigmafig_mut() -> Option<&'static mut SigmaFig> {
    unsafe { SIGMAFIG_INSTANCE.as_mut() }
}

/// Convenience function to add a diagram
pub fn add_diagram(diagram: Diagram) -> bool {
    if let Some(fig) = get_sigmafig_mut() {
        fig.add_diagram(diagram);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    #[test]
    fn test_position_distance() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_diagram_node_creation() {
        let node = DiagramNode::new(
            "node1".to_string(),
            NodeType::Rectangle,
            Position::new(0.0, 0.0),
            FigSize::new(100.0, 50.0),
        );
        assert_eq!(node.id, "node1");
        assert_eq!(node.node_type, NodeType::Rectangle);
    }

    #[test]
    fn test_diagram_node_modification() {
        let mut node = DiagramNode::new(
            "node1".to_string(),
            NodeType::Circle,
            Position::new(0.0, 0.0),
            FigSize::new(50.0, 50.0),
        );
        node.set_text("Test Node".to_string());
        node.set_color("#FF0000".to_string());
        assert_eq!(node.text, "Test Node");
        assert_eq!(node.color, "#FF0000");
    }

    #[test]
    fn test_connector_creation() {
        let connector = Connector::new(
            "conn1".to_string(),
            ConnectorType::Arrow,
            "node1".to_string(),
            "node2".to_string(),
        );
        assert_eq!(connector.id, "conn1");
        assert_eq!(connector.connector_type, ConnectorType::Arrow);
    }

    #[test]
    fn test_diagram_layer_creation() {
        let layer = DiagramLayer::new("layer1".to_string(), "Layer 1".to_string());
        assert_eq!(layer.id, "layer1");
        assert!(layer.visible);
    }

    #[test]
    fn test_diagram_layer_management() {
        let mut layer = DiagramLayer::new("layer1".to_string(), "Test".to_string());
        let node = DiagramNode::new(
            "node1".to_string(),
            NodeType::Rectangle,
            Position::new(0.0, 0.0),
            FigSize::new(100.0, 50.0),
        );
        layer.add_node(node);
        assert_eq!(layer.node_count(), 1);
        assert!(layer.remove_node("node1"));
        assert_eq!(layer.node_count(), 0);
    }

    #[test]
    fn test_diagram_creation() {
        let diagram = Diagram::new("diagram1".to_string(), "Flowchart".to_string());
        assert_eq!(diagram.id, "diagram1");
        assert_eq!(diagram.name, "Flowchart");
    }

    #[test]
    fn test_sigmafig_creation() {
        let fig = SigmaFig::new();
        assert_eq!(fig.diagram_count(), 0);
    }

    #[test]
    fn test_sigmafig_diagram_management() {
        let mut fig = SigmaFig::new();
        let diagram = Diagram::new("diagram1".to_string(), "Test".to_string());
        fig.add_diagram(diagram);
        assert_eq!(fig.diagram_count(), 1);
    }
}
