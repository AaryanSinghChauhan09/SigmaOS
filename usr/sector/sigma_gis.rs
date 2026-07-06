// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/sector/sigma_gis.rs — Sigma GIS (QGIS)
//
// Implements QGIS-style GIS with map layers, spatial analysis,
// geoprocessing, and data visualization.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── GIS Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub id: String,
    pub geometry_type: String,  // point, line, polygon
    pub coordinates: Vec<Coordinate>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub layer_type: String,  // vector, raster
    pub features: Vec<Feature>,
    pub visible: bool,
    pub style: String,
    pub crs: String,  // coordinate reference system
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub layers: Vec<Layer>,
    pub crs: String,
    pub extent: (f64, f64, f64, f64),  // min_x, min_y, max_x, max_y
}

#[derive(Debug, Clone)]
pub struct GeoprocessingResult {
    pub operation: String,
    pub result_features: Vec<Feature>,
    pub statistics: HashMap<String, f64>,
}

// ─── GIS Manager ────────────────────────────────────────────────────

pub struct GISManager {
    pub projects: HashMap<String, Project>,
    pub current_project: Option<String>,
    pub current_layer: Option<String>,
}

impl GISManager {
    pub fn new() -> Self {
        let mut manager = GISManager {
            projects: HashMap::new(),
            current_project: None,
            current_layer: None,
        };
        
        manager.init_sample_project();
        manager
    }

    /// Initialize sample project
    fn init_sample_project(&mut self) {
        let feature1 = Feature {
            id: "feat_001".to_string(),
            geometry_type: "point".to_string(),
            coordinates: vec![Coordinate { x: 77.2090, y: 28.6139, z: None }],
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("name".to_string(), "New Delhi".to_string());
                attrs.insert("population".to_string(), "30000000".to_string());
                attrs
            },
        };
        
        let layer1 = Layer {
            id: "layer_001".to_string(),
            name: "Cities".to_string(),
            layer_type: "vector".to_string(),
            features: vec![feature1],
            visible: true,
            style: "circle".to_string(),
            crs: "EPSG:4326".to_string(),
        };
        
        let project = Project {
            id: "project_001".to_string(),
            name: "India Map".to_string(),
            layers: vec![layer1],
            crs: "EPSG:4326".to_string(),
            extent: (68.0, 8.0, 97.0, 37.0),
        };
        
        self.projects.insert(project.id.clone(), project);
        self.current_project = Some("project_001".to_string());
    }

    /// Create new project
    pub fn create_project(&mut self, name: String, crs: String) -> Project {
        let project = Project {
            id: format!("project_{}", self.projects.len()),
            name,
            layers: Vec::new(),
            crs,
            extent: (-180.0, -90.0, 180.0, 90.0),
        };
        
        self.projects.insert(project.id.clone(), project.clone());
        self.current_project = Some(project.id.clone());
        project
    }

    /// Add layer to project
    pub fn add_layer(&mut self, project_id: &str, layer: Layer) -> Result<(), String> {
        if let Some(project) = self.projects.get_mut(project_id) {
            project.layers.push(layer);
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }

    /// Add feature to layer
    pub fn add_feature(&mut self, project_id: &str, layer_id: &str, feature: Feature) -> Result<(), String> {
        if let Some(project) = self.projects.get_mut(project_id) {
            if let Some(layer) = project.layers.iter_mut().find(|l| l.id == layer_id) {
                layer.features.push(feature);
                Ok(())
            } else {
                Err("Layer not found".to_string())
            }
        } else {
            Err("Project not found".to_string())
        }
    }

    /// Buffer operation
    pub fn buffer(&self, layer_id: &str, distance: f64) -> GeoprocessingResult {
        let mut result_features = Vec::new();
        
        if let Some(project) = self.current_project.as_ref().and_then(|id| self.projects.get(id)) {
            if let Some(layer) = project.layers.iter().find(|l| l.id == layer_id) {
                for feature in &layer.features {
                    let mut buffered = feature.clone();
                    buffered.id = format!("buffered_{}", buffered.id);
                    result_features.push(buffered);
                }
            }
        }
        
        let mut stats = HashMap::new();
        stats.insert("features_processed".to_string(), result_features.len() as f64);
        stats.insert("buffer_distance".to_string(), distance);
        
        GeoprocessingResult {
            operation: "buffer".to_string(),
            result_features,
            statistics: stats,
        }
    }

    /// Intersect operation
    pub fn intersect(&self, layer1_id: &str, layer2_id: &str) -> GeoprocessingResult {
        let mut result_features = Vec::new();
        
        if let Some(project) = self.current_project.as_ref().and_then(|id| self.projects.get(id)) {
            if let (Some(layer1), Some(layer2)) = (
                project.layers.iter().find(|l| l.id == layer1_id),
                project.layers.iter().find(|l| l.id == layer2_id)
            ) {
                // Simulate intersection
                for f1 in &layer1.features {
                    for f2 in &layer2.features {
                        let mut intersected = f1.clone();
                        intersected.id = format!("intersect_{}_{}", f1.id, f2.id);
                        result_features.push(intersected);
                    }
                }
            }
        }
        
        let mut stats = HashMap::new();
        stats.insert("features_processed".to_string(), result_features.len() as f64);
        
        GeoprocessingResult {
            operation: "intersect".to_string(),
            result_features,
            statistics: stats,
        }
    }

    /// Calculate area (for polygons)
    pub fn calculate_area(&self, layer_id: &str) -> f64 {
        let mut total_area = 0.0;
        
        if let Some(project) = self.current_project.as_ref().and_then(|id| self.projects.get(id)) {
            if let Some(layer) = project.layers.iter().find(|l| l.id == layer_id) {
                for feature in &layer.features {
                    if feature.geometry_type == "polygon" {
                        // Shoelace formula for area
                        let coords = &feature.coordinates;
                        if coords.len() >= 3 {
                            let mut area = 0.0;
                            for i in 0..coords.len() {
                                let j = (i + 1) % coords.len();
                                area += coords[i].x * coords[j].y;
                                area -= coords[j].x * coords[i].y;
                            }
                            total_area += area.abs() / 2.0;
                        }
                    }
                }
            }
        }
        
        total_area
    }

    /// Calculate length (for lines)
    pub fn calculate_length(&self, layer_id: &str) -> f64 {
        let mut total_length = 0.0;
        
        if let Some(project) = self.current_project.as_ref().and_then(|id| self.projects.get(id)) {
            if let Some(layer) = project.layers.iter().find(|l| l.id == layer_id) {
                for feature in &layer.features {
                    if feature.geometry_type == "line" {
                        for i in 0..feature.coordinates.len() - 1 {
                            let c1 = &feature.coordinates[i];
                            let c2 = &feature.coordinates[i + 1];
                            let dx = c2.x - c1.x;
                            let dy = c2.y - c1.y;
                            total_length += (dx * dx + dy * dy).sqrt();
                        }
                    }
                }
            }
        }
        
        total_length
    }

    /// Toggle layer visibility
    pub fn toggle_layer(&mut self, project_id: &str, layer_id: &str) -> Result<(), String> {
        if let Some(project) = self.projects.get_mut(project_id) {
            if let Some(layer) = project.layers.iter_mut().find(|l| l.id == layer_id) {
                layer.visible = !layer.visible;
                Ok(())
            } else {
                Err("Layer not found".to_string())
            }
        } else {
            Err("Project not found".to_string())
        }
    }

    /// Get project by ID
    pub fn get_project(&self, id: &str) -> Option<&Project> {
        self.projects.get(id)
    }

    /// Get all projects
    pub fn get_all_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    /// Switch project
    pub fn switch_project(&mut self, id: &str) -> Result<(), String> {
        if self.projects.contains_key(id) {
            self.current_project = Some(id.to_string());
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = GISManager::new();
    
    println!("Sigma GIS v0.1 - QGIS Style");
    
    loop {
        println!("\n--- GIS Status ---");
        if let Some(proj_id) = &manager.current_project {
            if let Some(project) = manager.get_project(proj_id) {
                println!("Current Project: {} ({})", project.name, project.crs);
                println!("Layers: {}", project.layers.len());
                println!("Extent: ({}, {}, {}, {})", project.extent.0, project.extent.1, project.extent.2, project.extent.3);
            }
        }
        println!("Total Projects: {}", manager.projects.len());
        
        println!("\nCommands: create_project <name> <crs>, add_layer <type> <name> <crs>, add_feature <layer_id> <type>, buffer <layer_id> <distance>, intersect <layer1> <layer2>, area <layer_id>, length <layer_id>, toggle <layer_id>, switch_project <id>, projects, project <id>, layers, quit");
        println!("Layer types: vector, raster");
        println!("Geometry types: point, line, polygon");
        println!("CRS examples: EPSG:4326 (WGS84), EPSG:3857 (Web Mercator)");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create_project" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let crs = parts[2].to_string();
                    let project = manager.create_project(name, crs);
                    println!("Project created: {}", project.name);
                }
            }
            "add_layer" => {
                if parts.len() >= 4 {
                    let layer_type = parts[1].to_string();
                    let name = parts[2].to_string();
                    let crs = parts[3].to_string();
                    let layer = Layer {
                        id: format!("layer_{}", rand_id()),
                        name,
                        layer_type,
                        features: Vec::new(),
                        visible: true,
                        style: "default".to_string(),
                        crs,
                    };
                    if let Some(proj_id) = &manager.current_project {
                        match manager.add_layer(proj_id, layer) {
                            Ok(_) => println!("Layer added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "add_feature" => {
                if parts.len() >= 3 {
                    let layer_id = parts[1].to_string();
                    let geom_type = parts[2].to_string();
                    let feature = Feature {
                        id: format!("feat_{}", rand_id()),
                        geometry_type: geom_type,
                        coordinates: vec![Coordinate { x: 0.0, y: 0.0, z: None }],
                        attributes: HashMap::new(),
                    };
                    if let Some(proj_id) = &manager.current_project {
                        match manager.add_feature(proj_id, &layer_id, feature) {
                            Ok(_) => println!("Feature added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "buffer" => {
                if parts.len() >= 3 {
                    if let Ok(distance) = parts[2].parse::<f64>() {
                        let result = manager.buffer(parts[1], distance);
                        println!("Buffer operation: {} features processed", result.result_features.len());
                    }
                }
            }
            "intersect" => {
                if parts.len() >= 3 {
                    let result = manager.intersect(parts[1], parts[2]);
                    println!("Intersect operation: {} features processed", result.result_features.len());
                }
            }
            "area" => {
                if let Some(arg) = parts.get(1) {
                    let area = manager.calculate_area(arg);
                    println!("Total area: {:.2} square units", area);
                }
            }
            "length" => {
                if let Some(arg) = parts.get(1) {
                    let length = manager.calculate_length(arg);
                    println!("Total length: {:.2} units", length);
                }
            }
            "toggle" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(proj_id) = &manager.current_project {
                        match manager.toggle_layer(proj_id, arg) {
                            Ok(_) => println!("Layer visibility toggled"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "switch_project" => {
                if let Some(arg) = parts.get(1) {
                    match manager.switch_project(arg) {
                        Ok(_) => println!("Switched to project"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "projects" => {
                println!("--- All Projects ---");
                for project in manager.get_all_projects() {
                    println!("{} - {} ({})", project.name, project.crs, project.layers.len());
                }
            }
            "project" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(project) = manager.get_project(arg) {
                        println!("--- Project Details ---");
                        println!("Name: {}", project.name);
                        println!("CRS: {}", project.crs);
                        println!("Layers: {}", project.layers.len());
                        for layer in &project.layers {
                            println!("  {} - {} ({}) [{}]", layer.name, layer.layer_type, layer.crs, if layer.visible { "visible" } else { "hidden" });
                        }
                    }
                }
            }
            "layers" => {
                if let Some(proj_id) = &manager.current_project {
                    if let Some(project) = manager.get_project(proj_id) {
                        println!("--- Layers ---");
                        for layer in &project.layers {
                            println!("{} - {} ({}) [{}]", layer.name, layer.layer_type, layer.features.len(), if layer.visible { "visible" } else { "hidden" });
                        }
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u32
}
