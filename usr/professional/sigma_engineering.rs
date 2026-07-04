// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/professional/sigma_engineering.rs — Sigma Engineering Tools
//
// Implements CAD integration, circuit simulators, and structural analysis
// plugins for engineers and engineering students.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Engineering Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineeringDomain {
    Civil,
    Mechanical,
    Electrical,
    Chemical,
    Software,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub component_type: String,
    pub parameters: HashMap<String, f64>,
    pub position: (f64, f64, f64),
}

#[derive(Debug, Clone)]
pub struct CircuitNode {
    pub id: String,
    pub voltage: f64,
    pub current: f64,
    pub components: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StructuralElement {
    pub id: String,
    pub element_type: String,
    pub material: String,
    pub length: f64,
    pub cross_section: f64,
    pub load: f64,
    pub stress: f64,
}

// ─── Engineering Simulator ───────────────────────────────────────────────────

pub struct EngineeringSimulator {
    pub components: HashMap<String, Component>,
    pub circuit_nodes: HashMap<String, CircuitNode>,
    pub structural_elements: HashMap<String, StructuralElement>,
}

impl EngineeringSimulator {
    pub fn new() -> Self {
        let mut simulator = EngineeringSimulator {
            components: HashMap::new(),
            circuit_nodes: HashMap::new(),
            structural_elements: HashMap::new(),
        };
        
        simulator.init_standard_components();
        simulator
    }

    /// Initialize standard electrical components
    fn init_standard_components(&mut self) {
        // Resistor
        let mut params = HashMap::new();
        params.insert("resistance".to_string(), 1000.0);
        params.insert("power_rating".to_string(), 0.25);
        self.components.insert("comp_resistor".to_string(), Component {
            id: "comp_resistor".to_string(),
            name: "Resistor".to_string(),
            component_type: "Passive".to_string(),
            parameters: params,
            position: (0.0, 0.0, 0.0),
        });

        // Capacitor
        let mut params = HashMap::new();
        params.insert("capacitance".to_string(), 100e-6);
        params.insert("voltage_rating".to_string(), 16.0);
        self.components.insert("comp_capacitor".to_string(), Component {
            id: "comp_capacitor".to_string(),
            name: "Capacitor".to_string(),
            component_type: "Passive".to_string(),
            parameters: params,
            position: (0.0, 0.0, 0.0),
        });

        // Inductor
        let mut params = HashMap::new();
        params.insert("inductance".to_string(), 10e-3);
        params.insert("current_rating".to_string(), 1.0);
        self.components.insert("comp_inductor".to_string(), Component {
            id: "comp_inductor".to_string(),
            name: "Inductor".to_string(),
            component_type: "Passive".to_string(),
            parameters: params,
            position: (0.0, 0.0, 0.0),
        });

        // Diode
        let mut params = HashMap::new();
        params.insert("forward_voltage".to_string(), 0.7);
        params.insert("reverse_breakdown".to_string(), 50.0);
        self.components.insert("comp_diode".to_string(), Component {
            id: "comp_diode".to_string(),
            name: "Diode".to_string(),
            component_type: "Semiconductor".to_string(),
            parameters: params,
            position: (0.0, 0.0, 0.0),
        });
    }

    /// Add component
    pub fn add_component(&mut self, component: Component) {
        self.components.insert(component.id.clone(), component);
    }

    /// Get component by ID
    pub fn get_component(&self, id: &str) -> Option<&Component> {
        self.components.get(id)
    }

    /// Calculate Ohm's Law
    pub fn calculate_ohms_law(&self, voltage: f64, resistance: f64) -> (f64, f64, f64) {
        let current = voltage / resistance;
        let power = voltage * current;
        (current, power, resistance)
    }

    /// Calculate RC time constant
    pub fn calculate_rc_time_constant(&self, resistance: f64, capacitance: f64) -> f64 {
        resistance * capacitance
    }

    /// Calculate resonant frequency
    pub fn calculate_resonant_frequency(&self, inductance: f64, capacitance: f64) -> f64 {
        1.0 / (2.0 * std::f64::consts::PI * (inductance * capacitance).sqrt())
    }

    /// Add circuit node
    pub fn add_circuit_node(&mut self, node: CircuitNode) {
        self.circuit_nodes.insert(node.id.clone(), node);
    }

    /// Analyze simple series circuit
    pub fn analyze_series_circuit(&self, voltage: f64, resistances: &[f64]) -> HashMap<String, f64> {
        let total_resistance: f64 = resistances.iter().sum();
        let current = voltage / total_resistance;
        
        let mut results = HashMap::new();
        results.insert("total_resistance".to_string(), total_resistance);
        results.insert("current".to_string(), current);
        results.insert("total_power".to_string(), voltage * current);
        
        for (i, &r) in resistances.iter().enumerate() {
            results.insert(format!("voltage_r{}", i + 1), current * r);
            results.insert(format!("power_r{}", i + 1), current * current * r);
        }
        
        results
    }

    /// Add structural element
    pub fn add_structural_element(&mut self, element: StructuralElement) {
        self.structural_elements.insert(element.id.clone(), element);
    }

    /// Calculate stress
    pub fn calculate_stress(&self, force: f64, area: f64) -> f64 {
        force / area
    }

    /// Calculate strain
    pub fn calculate_strain(&self, stress: f64, youngs_modulus: f64) -> f64 {
        stress / youngs_modulus
    }

    /// Calculate beam deflection (simply supported, point load at center)
    pub fn calculate_beam_deflection(&self, load: f64, length: f64, modulus: f64, moment_of_inertia: f64) -> f64 {
        (load * length.powi(3)) / (48.0 * modulus * moment_of_inertia)
    }

    /// Get all components
    pub fn get_all_components(&self) -> Vec<&Component> {
        self.components.values().collect()
    }

    /// Get components by type
    pub fn get_components_by_type(&self, component_type: &str) -> Vec<&Component> {
        self.components.values()
            .filter(|c| c.component_type == component_type)
            .collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut simulator = EngineeringSimulator::new();
    
    println!("Sigma Engineering Tools v0.1 - CAD & Circuit Simulation");
    
    loop {
        println!("\n--- Available Components ---");
        for component in simulator.get_all_components() {
            println!("{} - {} ({})", component.id, component.name, component.component_type);
        }
        
        println!("\nCommands: ohm <voltage> <resistance>, rc <R> <C>, resonant <L> <C>, series <voltage> <R1> <R2> ..., stress <force> <area>, strain <stress> <E>, beam <load> <length> <E> <I>, components, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "ohm" => {
                if parts.len() >= 3 {
                    if let (Ok(voltage), Ok(resistance)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let (current, power, _) = simulator.calculate_ohms_law(voltage, resistance);
                        println!("--- Ohm's Law Results ---");
                        println!("Voltage: {:.2} V", voltage);
                        println!("Resistance: {:.2} Ω", resistance);
                        println!("Current: {:.4} A", current);
                        println!("Power: {:.4} W", power);
                    }
                }
            }
            "rc" => {
                if parts.len() >= 3 {
                    if let (Ok(r), Ok(c)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let tau = simulator.calculate_rc_time_constant(r, c);
                        println!("--- RC Time Constant ---");
                        println!("Resistance: {:.2} Ω", r);
                        println!("Capacitance: {:.6e} F", c);
                        println!("Time Constant (τ): {:.6e} s", tau);
                    }
                }
            }
            "resonant" => {
                if parts.len() >= 3 {
                    if let (Ok(l), Ok(c)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let freq = simulator.calculate_resonant_frequency(l, c);
                        println!("--- Resonant Frequency ---");
                        println!("Inductance: {:.6e} H", l);
                        println!("Capacitance: {:.6e} F", c);
                        println!("Resonant Frequency: {:.2} Hz", freq);
                    }
                }
            }
            "series" => {
                if parts.len() >= 3 {
                    if let Ok(voltage) = parts[1].parse::<f64>() {
                        let resistances: Vec<f64> = parts[2..].iter()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        
                        if !resistances.is_empty() {
                            let results = simulator.analyze_series_circuit(voltage, &resistances);
                            println!("--- Series Circuit Analysis ---");
                            println!("Source Voltage: {:.2} V", voltage);
                            for (key, value) in &results {
                                if key.starts_with("voltage") {
                                    println!("{}: {:.2} V", key, value);
                                } else if key.starts_with("power") {
                                    println!("{}: {:.4} W", key, value);
                                } else {
                                    println!("{}: {:.4}", key, value);
                                }
                            }
                        }
                    }
                }
            }
            "stress" => {
                if parts.len() >= 3 {
                    if let (Ok(force), Ok(area)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let stress = simulator.calculate_stress(force, area);
                        println!("--- Stress Calculation ---");
                        println!("Force: {:.2} N", force);
                        println!("Area: {:.6e} m²", area);
                        println!("Stress: {:.2} Pa", stress);
                    }
                }
            }
            "strain" => {
                if parts.len() >= 3 {
                    if let (Ok(stress), Ok(modulus)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let strain = simulator.calculate_strain(stress, modulus);
                        println!("--- Strain Calculation ---");
                        println!("Stress: {:.2} Pa", stress);
                        println!("Young's Modulus: {:.2e} Pa", modulus);
                        println!("Strain: {:.6e}", strain);
                    }
                }
            }
            "beam" => {
                if parts.len() >= 5 {
                    if let (Ok(load), Ok(length), Ok(modulus), Ok(moment)) = (
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                        parts[4].parse::<f64>()
                    ) {
                        let deflection = simulator.calculate_beam_deflection(load, length, modulus, moment);
                        println!("--- Beam Deflection ---");
                        println!("Load: {:.2} N", load);
                        println!("Length: {:.2} m", length);
                        println!("Modulus: {:.2e} Pa", modulus);
                        println!("Moment of Inertia: {:.6e} m⁴", moment);
                        println!("Deflection: {:.6e} m", deflection);
                    }
                }
            }
            "components" => {
                println!("--- All Components ---");
                for component in simulator.get_all_components() {
                    println!("{} - {}", component.name, component.component_type);
                    println!("  Parameters:");
                    for (key, value) in &component.parameters {
                        println!("    {}: {:.6e}", key, value);
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
