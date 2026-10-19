//! SigmaCAD - CAD and Technical Drawing System
//!
//! Provides comprehensive CAD and technical drawing management inspired by CAD Craft.
//! Supports geometric primitives, constraints, and dimensioning.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Geometric entity type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Point,
    Line,
    Circle,
    Arc,
    Polyline,
    Rectangle,
    Polygon,
}

/// Constraint type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    Horizontal,
    Vertical,
    Parallel,
    Perpendicular,
    Tangent,
    Coincident,
    EqualLength,
    Fixed,
}

/// Dimension type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionType {
    Linear,
    Angular,
    Radial,
    Diameter,
}

/// 3D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Geometric entity
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub entity_type: EntityType,
    pub points: Vec<Point3D>,
    pub parameters: Vec<f64>,
}

impl Entity {
    pub fn new(id: String, entity_type: EntityType) -> Self {
        Self {
            id,
            entity_type,
            points: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn add_point(&mut self, point: Point3D) {
        self.points.push(point);
    }

    pub fn add_parameter(&mut self, param: f64) {
        self.parameters.push(param);
    }

    pub fn get_length(&self) -> Option<f64> {
        match self.entity_type {
            EntityType::Line if self.points.len() >= 2 => {
                Some(self.points[0].distance_to(&self.points[1]))
            }
            EntityType::Circle if !self.parameters.is_empty() => {
                Some(self.parameters[0]) // radius
            }
            _ => None,
        }
    }
}

/// Constraint between entities
#[derive(Debug, Clone)]
pub struct Constraint {
    pub id: String,
    pub constraint_type: ConstraintType,
    pub entity_ids: Vec<String>,
    pub parameters: Vec<f64>,
}

impl Constraint {
    pub fn new(id: String, constraint_type: ConstraintType, entity_ids: Vec<String>) -> Self {
        Self {
            id,
            constraint_type,
            entity_ids,
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, param: f64) {
        self.parameters.push(param);
    }
}

/// Dimension annotation
#[derive(Debug, Clone)]
pub struct Dimension {
    pub id: String,
    pub dimension_type: DimensionType,
    pub entity_id: String,
    pub value: f64,
    pub position: Point3D,
}

impl Dimension {
    pub fn new(
        id: String,
        dimension_type: DimensionType,
        entity_id: String,
        value: f64,
        position: Point3D,
    ) -> Self {
        Self {
            id,
            dimension_type,
            entity_id,
            value,
            position,
        }
    }
}

/// CAD drawing layer
#[derive(Debug, Clone)]
pub struct DrawingLayer {
    pub id: String,
    pub name: String,
    pub entities: BTreeMap<String, Entity>,
    pub visible: bool,
    pub locked: bool,
}

impl DrawingLayer {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            entities: BTreeMap::new(),
            visible: true,
            locked: false,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }

    pub fn remove_entity(&mut self, id: &str) -> bool {
        self.entities.remove(id).is_some()
    }

    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

/// CAD drawing
#[derive(Debug, Clone)]
pub struct Drawing {
    pub id: String,
    pub name: String,
    pub layers: BTreeMap<String, DrawingLayer>,
    pub constraints: BTreeMap<String, Constraint>,
    pub dimensions: BTreeMap<String, Dimension>,
    pub active_layer: Option<String>,
    pub units: String,
}

impl Drawing {
    pub fn new(id: String, name: String, units: String) -> Self {
        Self {
            id,
            name,
            layers: BTreeMap::new(),
            constraints: BTreeMap::new(),
            dimensions: BTreeMap::new(),
            active_layer: None,
            units,
        }
    }

    pub fn add_layer(&mut self, layer: DrawingLayer) {
        self.layers.insert(layer.id.clone(), layer);
    }

    pub fn remove_layer(&mut self, id: &str) -> bool {
        if self.active_layer.as_ref() == Some(&id.to_string()) {
            self.active_layer = None;
        }
        self.layers.remove(id).is_some()
    }

    pub fn get_layer(&self, id: &str) -> Option<&DrawingLayer> {
        self.layers.get(id)
    }

    pub fn get_layer_mut(&mut self, id: &str) -> Option<&mut DrawingLayer> {
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

    pub fn get_active_layer(&self) -> Option<&DrawingLayer> {
        self.active_layer
            .as_ref()
            .and_then(|id| self.layers.get(id))
    }

    pub fn get_active_layer_mut(&mut self) -> Option<&mut DrawingLayer> {
        self.active_layer
            .as_ref()
            .and_then(|id| self.layers.get_mut(id.as_str()))
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.insert(constraint.id.clone(), constraint);
    }

    pub fn remove_constraint(&mut self, id: &str) -> bool {
        self.constraints.remove(id).is_some()
    }

    pub fn add_dimension(&mut self, dimension: Dimension) {
        self.dimensions.insert(dimension.id.clone(), dimension);
    }

    pub fn remove_dimension(&mut self, id: &str) -> bool {
        self.dimensions.remove(id).is_some()
    }

    pub fn set_units(&mut self, units: String) {
        self.units = units;
    }

    pub fn total_entity_count(&self) -> usize {
        self.layers.values().map(|l| l.entity_count()).sum()
    }

    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    pub fn dimension_count(&self) -> usize {
        self.dimensions.len()
    }
}

/// SigmaCAD - CAD system manager
#[derive(Debug)]
pub struct SigmaCAD {
    drawings: BTreeMap<String, Drawing>,
    current_drawing: Option<String>,
}

impl SigmaCAD {
    pub fn new() -> Self {
        Self {
            drawings: BTreeMap::new(),
            current_drawing: None,
        }
    }

    pub fn add_drawing(&mut self, drawing: Drawing) {
        self.drawings.insert(drawing.id.clone(), drawing);
    }

    pub fn remove_drawing(&mut self, id: &str) -> bool {
        if self.current_drawing.as_ref() == Some(&id.to_string()) {
            self.current_drawing = None;
        }
        self.drawings.remove(id).is_some()
    }

    pub fn get_drawing(&self, id: &str) -> Option<&Drawing> {
        self.drawings.get(id)
    }

    pub fn get_drawing_mut(&mut self, id: &str) -> Option<&mut Drawing> {
        self.drawings.get_mut(id)
    }

    pub fn set_current_drawing(&mut self, id: &str) -> bool {
        if self.drawings.contains_key(id) {
            self.current_drawing = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_drawing(&self) -> Option<&Drawing> {
        self.current_drawing
            .as_ref()
            .and_then(|id| self.drawings.get(id))
    }

    pub fn get_current_drawing_mut(&mut self) -> Option<&mut Drawing> {
        self.current_drawing
            .as_ref()
            .and_then(|id| self.drawings.get_mut(id.as_str()))
    }

    pub fn list_drawings(&self) -> Vec<&Drawing> {
        self.drawings.values().collect()
    }

    pub fn drawing_count(&self) -> usize {
        self.drawings.len()
    }
}

impl Default for SigmaCAD {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaCAD instance
static mut SIGMACAD_INSTANCE: Option<SigmaCAD> = None;

/// Initialize the global SigmaCAD instance
pub fn init_sigmacad() {
    unsafe {
        SIGMACAD_INSTANCE = Some(SigmaCAD::new());
    }
}

/// Get the global SigmaCAD instance
pub fn get_sigmacad() -> Option<&'static SigmaCAD> {
    unsafe { SIGMACAD_INSTANCE.as_ref() }
}

/// Get the global SigmaCAD instance (mutable)
pub fn get_sigmacad_mut() -> Option<&'static mut SigmaCAD> {
    unsafe { SIGMACAD_INSTANCE.as_mut() }
}

/// Convenience function to add a drawing
pub fn add_drawing(drawing: Drawing) -> bool {
    if let Some(cad) = get_sigmacad_mut() {
        cad.add_drawing(drawing);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_creation() {
        let point = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn test_point3d_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);
        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new("entity1".to_string(), EntityType::Line);
        assert_eq!(entity.id, "entity1");
        assert_eq!(entity.entity_type, EntityType::Line);
    }

    #[test]
    fn test_entity_length() {
        let mut entity = Entity::new("line1".to_string(), EntityType::Line);
        entity.add_point(Point3D::new(0.0, 0.0, 0.0));
        entity.add_point(Point3D::new(3.0, 4.0, 0.0));
        let length = entity.get_length();
        assert!(length.is_some());
        assert!((length.unwrap() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_constraint_creation() {
        let constraint = Constraint::new(
            "constraint1".to_string(),
            ConstraintType::Horizontal,
            vec!["entity1".to_string()],
        );
        assert_eq!(constraint.id, "constraint1");
        assert_eq!(constraint.constraint_type, ConstraintType::Horizontal);
    }

    #[test]
    fn test_dimension_creation() {
        let dimension = Dimension::new(
            "dim1".to_string(),
            DimensionType::Linear,
            "entity1".to_string(),
            10.0,
            Point3D::new(0.0, 0.0, 0.0),
        );
        assert_eq!(dimension.id, "dim1");
        assert_eq!(dimension.value, 10.0);
    }

    #[test]
    fn test_drawing_layer_creation() {
        let layer = DrawingLayer::new("layer1".to_string(), "Layer 1".to_string());
        assert_eq!(layer.id, "layer1");
        assert!(layer.visible);
        assert!(!layer.locked);
    }

    #[test]
    fn test_drawing_layer_entity_management() {
        let mut layer = DrawingLayer::new("layer1".to_string(), "Test".to_string());
        let entity = Entity::new("entity1".to_string(), EntityType::Line);
        layer.add_entity(entity);
        assert_eq!(layer.entity_count(), 1);
        assert!(layer.remove_entity("entity1"));
        assert_eq!(layer.entity_count(), 0);
    }

    #[test]
    fn test_drawing_creation() {
        let drawing = Drawing::new(
            "drawing1".to_string(),
            "Test Drawing".to_string(),
            "mm".to_string(),
        );
        assert_eq!(drawing.id, "drawing1");
        assert_eq!(drawing.units, "mm");
    }

    #[test]
    fn test_drawing_layer_management() {
        let mut drawing =
            Drawing::new("drawing1".to_string(), "Test".to_string(), "mm".to_string());
        let layer = DrawingLayer::new("layer1".to_string(), "Layer 1".to_string());
        drawing.add_layer(layer);
        assert_eq!(drawing.total_entity_count(), 0);
    }

    #[test]
    fn test_sigmacad_creation() {
        let cad = SigmaCAD::new();
        assert_eq!(cad.drawing_count(), 0);
    }

    #[test]
    fn test_sigmacad_drawing_management() {
        let mut cad = SigmaCAD::new();
        let drawing = Drawing::new("drawing1".to_string(), "Test".to_string(), "mm".to_string());
        cad.add_drawing(drawing);
        assert_eq!(cad.drawing_count(), 1);
    }
}
