// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/sector/sigma_cad.rs — Sigma Engineering CAD (FreeCAD)
//
// Implements FreeCAD-style CAD with 3D modeling, parametric design,
// assembly management, and export capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── CAD Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct Sketch {
    pub id: String,
    pub name: String,
    pub plane: String,  // XY, XZ, YZ
    pub points: Vec<Point3D>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub material: String,
    pub sketches: Vec<Sketch>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Assembly {
    pub id: String,
    pub name: String,
    pub parts: Vec<AssemblyPart>,
    pub constraints: Vec<AssemblyConstraint>,
}

#[derive(Debug, Clone)]
pub struct AssemblyPart {
    pub part_id: String,
    pub position: Point3D,
    pub rotation: Point3D,
}

#[derive(Debug, Clone)]
pub struct AssemblyConstraint {
    pub id: String,
    pub type_: String,  // mate, align, distance, angle
    pub part1: String,
    pub part2: String,
    pub value: f64,
}

// ─── CAD Manager ────────────────────────────────────────────────────

pub struct CADManager {
    pub parts: HashMap<String, Part>,
    pub assemblies: HashMap<String, Assembly>,
    pub current_part: Option<String>,
    pub current_assembly: Option<String>,
}

impl CADManager {
    pub fn new() -> Self {
        let mut manager = CADManager {
            parts: HashMap::new(),
            assemblies: HashMap::new(),
            current_part: None,
            current_assembly: None,
        };
        
        manager.init_sample_part();
        manager
    }

    /// Initialize sample part
    fn init_sample_part(&mut self) {
        let sketch = Sketch {
            id: "sketch_001".to_string(),
            name: "Base Sketch".to_string(),
            plane: "XY".to_string(),
            points: vec![
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 100.0, y: 0.0, z: 0.0 },
                Point3D { x: 100.0, y: 50.0, z: 0.0 },
                Point3D { x: 0.0, y: 50.0, z: 0.0 },
            ],
            constraints: vec!["horizontal".to_string(), "vertical".to_string()],
        };
        
        let part = Part {
            id: "part_001".to_string(),
            name: "Base Plate".to_string(),
            material: "Steel".to_string(),
            sketches: vec![sketch],
            features: vec!["extrude".to_string(), "fillet".to_string()],
        };
        
        self.parts.insert(part.id.clone(), part);
        self.current_part = Some("part_001".to_string());
    }

    /// Create new part
    pub fn create_part(&mut self, name: String, material: String) -> Part {
        let part = Part {
            id: format!("part_{}", self.parts.len()),
            name,
            material,
            sketches: Vec::new(),
            features: Vec::new(),
        };
        
        self.parts.insert(part.id.clone(), part.clone());
        self.current_part = Some(part.id.clone());
        part
    }

    /// Add sketch to part
    pub fn add_sketch(&mut self, part_id: &str, sketch: Sketch) -> Result<(), String> {
        if let Some(part) = self.parts.get_mut(part_id) {
            part.sketches.push(sketch);
            Ok(())
        } else {
            Err("Part not found".to_string())
        }
    }

    /// Add feature to part
    pub fn add_feature(&mut self, part_id: &str, feature: String) -> Result<(), String> {
        if let Some(part) = self.parts.get_mut(part_id) {
            part.features.push(feature);
            Ok(())
        } else {
            Err("Part not found".to_string())
        }
    }

    /// Create assembly
    pub fn create_assembly(&mut self, name: String) -> Assembly {
        let assembly = Assembly {
            id: format!("assembly_{}", self.assemblies.len()),
            name,
            parts: Vec::new(),
            constraints: Vec::new(),
        };
        
        self.assemblies.insert(assembly.id.clone(), assembly.clone());
        self.current_assembly = Some(assembly.id.clone());
        assembly
    }

    /// Add part to assembly
    pub fn add_part_to_assembly(&mut self, assembly_id: &str, part_id: String, position: Point3D, rotation: Point3D) -> Result<(), String> {
        if let Some(assembly) = self.assemblies.get_mut(assembly_id) {
            if self.parts.contains_key(&part_id) {
                assembly.parts.push(AssemblyPart {
                    part_id,
                    position,
                    rotation,
                });
                Ok(())
            } else {
                Err("Part not found".to_string())
            }
        } else {
            Err("Assembly not found".to_string())
        }
    }

    /// Add constraint to assembly
    pub fn add_constraint(&mut self, assembly_id: &str, constraint: AssemblyConstraint) -> Result<(), String> {
        if let Some(assembly) = self.assemblies.get_mut(assembly_id) {
            assembly.constraints.push(constraint);
            Ok(())
        } else {
            Err("Assembly not found".to_string())
        }
    }

    /// Extrude sketch (simulated)
    pub fn extrude_sketch(&mut self, part_id: &str, sketch_id: &str, height: f64) -> Result<(), String> {
        if let Some(part) = self.parts.get_mut(part_id) {
            if part.sketches.iter().any(|s| s.id == sketch_id) {
                part.features.push(format!("extrude {} from {}", height, sketch_id));
                Ok(())
            } else {
                Err("Sketch not found".to_string())
            }
        } else {
            Err("Part not found".to_string())
        }
    }

    /// Get part by ID
    pub fn get_part(&self, id: &str) -> Option<&Part> {
        self.parts.get(id)
    }

    /// Get all parts
    pub fn get_all_parts(&self) -> Vec<&Part> {
        self.parts.values().collect()
    }

    /// Get assembly by ID
    pub fn get_assembly(&self, id: &str) -> Option<&Assembly> {
        self.assemblies.get(id)
    }

    /// Get all assemblies
    pub fn get_all_assemblies(&self) -> Vec<&Assembly> {
        self.assemblies.values().collect()
    }

    /// Switch part
    pub fn switch_part(&mut self, id: &str) -> Result<(), String> {
        if self.parts.contains_key(id) {
            self.current_part = Some(id.to_string());
            Ok(())
        } else {
            Err("Part not found".to_string())
        }
    }

    /// Switch assembly
    pub fn switch_assembly(&mut self, id: &str) -> Result<(), String> {
        if self.assemblies.contains_key(id) {
            self.current_assembly = Some(id.to_string());
            Ok(())
        } else {
            Err("Assembly not found".to_string())
        }
    }

    /// Export (simulated)
    pub fn export(&self, id: &str, format: String) -> Result<String, String> {
        if self.parts.contains_key(id) || self.assemblies.contains_key(id) {
            Ok(format!("{}.{}", id, format))
        } else {
            Err("Item not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = CADManager::new();
    
    println!("Sigma Engineering CAD v0.1 - FreeCAD Style");
    
    loop {
        println!("\n--- CAD Status ---");
        if let Some(part_id) = &manager.current_part {
            if let Some(part) = manager.get_part(part_id) {
                println!("Current Part: {} ({})", part.name, part.material);
                println!("Sketches: {}", part.sketches.len());
                println!("Features: {}", part.features.len());
            }
        }
        if let Some(asm_id) = &manager.current_assembly {
            if let Some(asm) = manager.get_assembly(asm_id) {
                println!("Current Assembly: {}", asm.name);
                println!("Parts: {}", asm.parts.len());
                println!("Constraints: {}", asm.constraints.len());
            }
        }
        println!("Total Parts: {}", manager.parts.len());
        println!("Total Assemblies: {}", manager.assemblies.len());
        
        println!("\nCommands: create_part <name> <material>, add_sketch <part_id> <name> <plane>, add_feature <part_id> <feature>, create_assembly <name>, add_part_to_asm <asm_id> <part_id> <x> <y> <z>, add_constraint <asm_id> <type> <part1> <part2> <value>, extrude <part_id> <sketch_id> <height>, switch_part <id>, switch_asm <id>, export <id> <format>, parts, assemblies, part <id>, assembly <id>, quit");
        println!("Sketch planes: XY, XZ, YZ");
        println!("Constraint types: mate, align, distance, angle");
        println!("Export formats: step, stl, obj");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create_part" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let material = parts[2].to_string();
                    let part = manager.create_part(name, material);
                    println!("Part created: {}", part.name);
                }
            }
            "add_sketch" => {
                if parts.len() >= 4 {
                    let part_id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let plane = parts[3].to_string();
                    let sketch = Sketch {
                        id: format!("sketch_{}", rand_id()),
                        name,
                        plane,
                        points: Vec::new(),
                        constraints: Vec::new(),
                    };
                    match manager.add_sketch(&part_id, sketch) {
                        Ok(_) => println!("Sketch added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_feature" => {
                if parts.len() >= 3 {
                    match manager.add_feature(parts[1], parts[2].to_string()) {
                        Ok(_) => println!("Feature added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "create_assembly" => {
                if let Some(arg) = parts.get(1) {
                    let assembly = manager.create_assembly(arg.to_string());
                    println!("Assembly created: {}", assembly.name);
                }
            }
            "add_part_to_asm" => {
                if parts.len() >= 7 {
                    if let (Ok(x), Ok(y), Ok(z)) = (parts[3].parse::<f64>(), parts[4].parse::<f64>(), parts[5].parse::<f64>()) {
                        let position = Point3D { x, y, z };
                        let rotation = Point3D { x: 0.0, y: 0.0, z: 0.0 };
                        match manager.add_part_to_assembly(parts[1], parts[2].to_string(), position, rotation) {
                            Ok(_) => println!("Part added to assembly"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "add_constraint" => {
                if parts.len() >= 6 {
                    if let Ok(value) = parts[5].parse::<f64>() {
                        let constraint = AssemblyConstraint {
                            id: format!("constraint_{}", rand_id()),
                            type_: parts[2].to_string(),
                            part1: parts[3].to_string(),
                            part2: parts[4].to_string(),
                            value,
                        };
                        match manager.add_constraint(parts[1], constraint) {
                            Ok(_) => println!("Constraint added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "extrude" => {
                if parts.len() >= 4 {
                    if let Ok(height) = parts[3].parse::<f64>() {
                        match manager.extrude_sketch(parts[1], parts[2], height) {
                            Ok(_) => println!("Extrusion created"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "switch_part" => {
                if let Some(arg) = parts.get(1) {
                    match manager.switch_part(arg) {
                        Ok(_) => println!("Switched to part"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "switch_asm" => {
                if let Some(arg) = parts.get(1) {
                    match manager.switch_assembly(arg) {
                        Ok(_) => println!("Switched to assembly"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "export" => {
                if parts.len() >= 3 {
                    match manager.export(parts[1], parts[2].to_string()) {
                        Ok(path) => println!("Exported to: {}", path),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "parts" => {
                println!("--- All Parts ---");
                for part in manager.get_all_parts() {
                    println!("{} - {} ({})", part.name, part.material, part.features.len());
                }
            }
            "assemblies" => {
                println!("--- All Assemblies ---");
                for asm in manager.get_all_assemblies() {
                    println!("{} - {} parts, {} constraints", asm.name, asm.parts.len(), asm.constraints.len());
                }
            }
            "part" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(part) = manager.get_part(arg) {
                        println!("--- Part Details ---");
                        println!("Name: {}", part.name);
                        println!("Material: {}", part.material);
                        println!("Sketches: {}", part.sketches.len());
                        println!("Features: {}", part.features.join(", "));
                    }
                }
            }
            "assembly" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(asm) = manager.get_assembly(arg) {
                        println!("--- Assembly Details ---");
                        println!("Name: {}", asm.name);
                        println!("Parts: {}", asm.parts.len());
                        for ap in &asm.parts {
                            println!("  - {} at ({}, {}, {})", ap.part_id, ap.position.x, ap.position.y, ap.position.z);
                        }
                        println!("Constraints: {}", asm.constraints.len());
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
