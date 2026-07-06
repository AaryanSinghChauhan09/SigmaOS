// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/ui/sigma_wayland.rs — Sigma Display Server (Wayland)
//
// Implements Wayland-style display server with compositor management,
// surface handling, input processing, and output configuration.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Display Server Types ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Surface {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub title: String,
    pub app_id: String,
    pub visible: bool,
    pub focused: bool,
    pub fullscreen: bool,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub scale: f32,
    pub primary: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Seat {
    pub name: String,
    pub capabilities: Vec<String>,
    pub keyboard: bool,
    pub pointer: bool,
    pub touch: bool,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub z_index: i32,
    pub surfaces: Vec<String>,
}

// ─── Wayland Manager ─────────────────────────────────────────────────────

pub struct WaylandManager {
    pub surfaces: HashMap<String, Surface>,
    pub outputs: HashMap<String, Output>,
    pub seats: HashMap<String, Seat>,
    pub layers: Vec<Layer>,
    pub active_surface: Option<String>,
    pub compositor_running: bool,
}

impl WaylandManager {
    pub fn new() -> Self {
        let mut manager = WaylandManager {
            surfaces: HashMap::new(),
            outputs: HashMap::new(),
            seats: HashMap::new(),
            layers: Vec::new(),
            active_surface: None,
            compositor_running: true,
        };
        
        manager.init_sample_outputs();
        manager.init_sample_seats();
        manager.init_sample_layers();
        manager
    }

    /// Initialize sample outputs
    fn init_sample_outputs(&mut self) {
        self.outputs.insert("HDMI-1".to_string(), Output {
            name: "HDMI-1".to_string(),
            width: 1920,
            height: 1080,
            refresh_rate: 60,
            scale: 1.0,
            primary: true,
            enabled: true,
        });

        self.outputs.insert("DP-1".to_string(), Output {
            name: "DP-1".to_string(),
            width: 2560,
            height: 1440,
            refresh_rate: 144,
            scale: 1.5,
            primary: false,
            enabled: true,
        });
    }

    /// Initialize sample seats
    fn init_sample_seats(&mut self) {
        self.seats.insert("seat0".to_string(), Seat {
            name: "seat0".to_string(),
            capabilities: vec!["keyboard".to_string(), "pointer".to_string(), "touch".to_string()],
            keyboard: true,
            pointer: true,
            touch: true,
        });
    }

    /// Initialize sample layers
    fn init_sample_layers(&mut self) {
        self.layers.push(Layer {
            name: "background".to_string(),
            z_index: 0,
            surfaces: vec![],
        });

        self.layers.push(Layer {
            name: "bottom".to_string(),
            z_index: 1,
            surfaces: vec![],
        });

        self.layers.push(Layer {
            name: "top".to_string(),
            z_index: 2,
            surfaces: vec![],
        });

        self.layers.push(Layer {
            name: "overlay".to_string(),
            z_index: 3,
            surfaces: vec![],
        });

        self.layers.push(Layer {
            name: "popups".to_string(),
            z_index: 4,
            surfaces: vec![],
        });
    }

    /// Create surface
    pub fn create_surface(&mut self, width: u32, height: u32, title: String, app_id: String) -> Surface {
        let surface = Surface {
            id: format!("surface_{}", self.surfaces.len()),
            width,
            height,
            x: 100 + (self.surfaces.len() as i32 * 50),
            y: 100 + (self.surfaces.len() as i32 * 50),
            title,
            app_id,
            visible: true,
            focused: false,
            fullscreen: false,
        };
        
        self.surfaces.insert(surface.id.clone(), surface.clone());
        
        // Add to top layer by default
        if let Some(layer) = self.layers.iter_mut().find(|l| l.name == "top") {
            layer.surfaces.push(surface.id.clone());
        }
        
        surface
    }

    /// Destroy surface
    pub fn destroy_surface(&mut self, surface_id: &str) -> Result<(), String> {
        if self.surfaces.remove(surface_id).is_some() {
            // Remove from all layers
            for layer in &mut self.layers {
                layer.surfaces.retain(|s| s != surface_id);
            }
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }

    /// Focus surface
    pub fn focus_surface(&mut self, surface_id: &str) -> Result<(), String> {
        if let Some(surface) = self.surfaces.get_mut(surface_id) {
            surface.focused = true;
            self.active_surface = Some(surface_id.to_string());
            
            // Unfocus others
            for (id, s) in self.surfaces.iter_mut() {
                if id != surface_id {
                    s.focused = false;
                }
            }
            
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }

    /// Move surface
    pub fn move_surface(&mut self, surface_id: &str, x: i32, y: i32) -> Result<(), String> {
        if let Some(surface) = self.surfaces.get_mut(surface_id) {
            surface.x = x;
            surface.y = y;
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }

    /// Resize surface
    pub fn resize_surface(&mut self, surface_id: &str, width: u32, height: u32) -> Result<(), String> {
        if let Some(surface) = self.surfaces.get_mut(surface_id) {
            surface.width = width;
            surface.height = height;
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self, surface_id: &str) -> Result<(), String> {
        if let Some(surface) = self.surfaces.get_mut(surface_id) {
            surface.fullscreen = !surface.fullscreen;
            if surface.fullscreen {
                surface.x = 0;
                surface.y = 0;
                if let Some(output) = self.outputs.values().find(|o| o.primary) {
                    surface.width = output.width;
                    surface.height = output.height;
                }
            }
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }

    /// Add surface to layer
    pub fn add_to_layer(&mut self, surface_id: &str, layer_name: &str) -> Result<(), String> {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.name == layer_name) {
            if self.surfaces.contains_key(surface_id) {
                // Remove from other layers first
                for l in &mut self.layers {
                    l.surfaces.retain(|s| s != surface_id);
                }
                layer.surfaces.push(surface_id.to_string());
                Ok(())
            } else {
                Err("Surface not found".to_string())
            }
        } else {
            Err("Layer not found".to_string())
        }
    }

    /// Get surface by ID
    pub fn get_surface(&self, id: &str) -> Option<&Surface> {
        self.surfaces.get(id)
    }

    /// Get all surfaces
    pub fn get_all_surfaces(&self) -> Vec<&Surface> {
        self.surfaces.values().collect()
    }

    /// Get output by name
    pub fn get_output(&self, name: &str) -> Option<&Output> {
        self.outputs.get(name)
    }

    /// Get all outputs
    pub fn get_all_outputs(&self) -> Vec<&Output> {
        self.outputs.values().collect()
    }

    /// Toggle compositor
    pub fn toggle_compositor(&mut self) {
        self.compositor_running = !self.compositor_running;
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = WaylandManager::new();
    
    println!("Sigma Display Server v0.1 - Wayland Style");
    
    loop {
        println!("\n--- Display Server Status ---");
        println!("Compositor: {}", if manager.compositor_running { "RUNNING" } else { "STOPPED" });
        println!("Surfaces: {}", manager.surfaces.len());
        println!("Outputs: {}", manager.outputs.len());
        println!("Seats: {}", manager.seats.len());
        if let Some(active) = &manager.active_surface {
            println!("Active Surface: {}", active);
        }
        
        println!("\nCommands: create_surface <w> <h> <title> <app_id>, destroy <surface_id>, focus <surface_id>, move <id> <x> <y>, resize <id> <w> <h>, fullscreen <id>, layer <id> <layer>, surfaces, outputs, toggle, quit");
        println!("Layers: background, bottom, top, overlay, popups");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create_surface" => {
                if parts.len() >= 5 {
                    if let (Ok(width), Ok(height)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                        let title = parts[3].to_string();
                        let app_id = parts[4].to_string();
                        let surface = manager.create_surface(width, height, title, app_id);
                        println!("Surface created: {}", surface.id);
                    }
                }
            }
            "destroy" => {
                if let Some(arg) = parts.get(1) {
                    match manager.destroy_surface(arg) {
                        Ok(_) => println!("Surface destroyed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "focus" => {
                if let Some(arg) = parts.get(1) {
                    match manager.focus_surface(arg) {
                        Ok(_) => println!("Surface focused"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "move" => {
                if parts.len() >= 4 {
                    if let (Ok(x), Ok(y)) = (parts[2].parse::<i32>(), parts[3].parse::<i32>()) {
                        match manager.move_surface(parts[1], x, y) {
                            Ok(_) => println!("Surface moved"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "resize" => {
                if parts.len() >= 4 {
                    if let (Ok(width), Ok(height)) = (parts[2].parse::<u32>(), parts[3].parse::<u32>()) {
                        match manager.resize_surface(parts[1], width, height) {
                            Ok(_) => println!("Surface resized"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "fullscreen" => {
                if let Some(arg) = parts.get(1) {
                    match manager.toggle_fullscreen(arg) {
                        Ok(_) => println!("Fullscreen toggled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "layer" => {
                if parts.len() >= 3 {
                    match manager.add_to_layer(parts[1], parts[2]) {
                        Ok(_) => println!("Surface moved to layer"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "surfaces" => {
                println!("--- All Surfaces ---");
                for surface in manager.get_all_surfaces() {
                    let status = if surface.focused { "[FOCUSED]" } else { "" };
                    let fs = if surface.fullscreen { "[FULLSCREEN]" } else { "" };
                    println!("{} - {}x{} at ({},{}) {} {} ({})", 
                        surface.id, surface.width, surface.height, surface.x, surface.y, status, fs, surface.title);
                }
            }
            "outputs" => {
                println!("--- All Outputs ---");
                for output in manager.get_all_outputs() {
                    let primary = if output.primary { "[PRIMARY]" } else { "" };
                    let enabled = if output.enabled { "[ENABLED]" } else { "" };
                    println!("{} - {}x{}@{}Hz (scale: {}) {} {}", 
                        output.name, output.width, output.height, output.refresh_rate, output.scale, primary, enabled);
                }
            }
            "toggle" => {
                manager.toggle_compositor();
                println!("Compositor {}", if manager.compositor_running { "started" } else { "stopped" });
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
