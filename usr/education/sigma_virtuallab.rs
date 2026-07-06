// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_virtuallab.rs — Sigma Virtual Lab
//
// Implements AI-powered virtual laboratory simulations for physics,
// chemistry, and biology experiments aligned with CBSE curriculum.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Experiment Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subject {
    Physics,
    Chemistry,
    Biology,
}

#[derive(Debug, Clone)]
pub struct Experiment {
    pub id: String,
    pub title: String,
    pub subject: Subject,
    pub chapter: String,  // NCERT chapter reference
    pub description: String,
    pub difficulty: String,  // Easy, Medium, Hard
    pub duration_minutes: u32,
    pub equipment: Vec<String>,
    pub objectives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SimulationState {
    pub experiment_id: String,
    pub parameters: HashMap<String, f64>,
    pub measurements: Vec<Measurement>,
    pub completed: bool,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub parameter: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}

// ─── Virtual Lab Application ─────────────────────────────────────────────────

pub struct VirtualLab {
    pub experiments: HashMap<String, Experiment>,
    pub current_simulation: Option<SimulationState>,
    pub chapter_filter: Option<String>,
    pub subject_filter: Option<Subject>,
}

impl VirtualLab {
    pub fn new() -> Self {
        let mut lab = VirtualLab {
            experiments: HashMap::new(),
            current_simulation: None,
            chapter_filter: None,
            subject_filter: None,
        };
        
        lab.init_cbse_experiments();
        lab
    }

    /// Initialize CBSE-aligned experiments
    fn init_cbse_experiments(&mut self) {
        // Physics Experiments
        self.add_experiment(Experiment {
            id: "phy_ohm_law".to_string(),
            title: "Ohm's Law Verification".to_string(),
            subject: Subject::Physics,
            chapter: "Chapter 12: Electricity".to_string(),
            description: "Verify the relationship between voltage, current, and resistance".to_string(),
            difficulty: "Easy".to_string(),
            duration_minutes: 30,
            equipment: vec!["Ammeter".to_string(), "Voltmeter".to_string(), "Resistor".to_string(), "Battery".to_string()],
            objectives: vec![
                "Measure current through resistor".to_string(),
                "Measure voltage across resistor".to_string(),
                "Calculate resistance using V=IR".to_string(),
            ],
        });

        self.add_experiment(Experiment {
            id: "phy_pendulum".to_string(),
            title: "Simple Pendulum".to_string(),
            subject: Subject::Physics,
            chapter: "Chapter 14: Oscillations".to_string(),
            description: "Determine acceleration due to gravity using simple pendulum".to_string(),
            difficulty: "Medium".to_string(),
            duration_minutes: 45,
            equipment: vec!["Pendulum bob".to_string(), "String".to_string(), "Stopwatch".to_string(), "Meter scale".to_string()],
            objectives: vec![
                "Measure time period for different lengths".to_string(),
                "Plot T² vs L graph".to_string(),
                "Calculate g from slope".to_string(),
            ],
        });

        // Chemistry Experiments
        self.add_experiment(Experiment {
            id: "chem_ph".to_string(),
            title: "pH Measurement".to_string(),
            subject: Subject::Chemistry,
            chapter: "Chapter 2: Acids, Bases and Salts".to_string(),
            description: "Measure pH of various solutions using pH paper and indicator".to_string(),
            difficulty: "Easy".to_string(),
            duration_minutes: 30,
            equipment: vec!["pH paper".to_string(), "Universal indicator".to_string(), "Test tubes".to_string()],
            objectives: vec![
                "Identify acidic and basic solutions".to_string(),
                "Compare pH of different solutions".to_string(),
                "Understand pH scale".to_string(),
            ],
        });

        self.add_experiment(Experiment {
            id: "chem_titration".to_string(),
            title: "Acid-Base Titration".to_string(),
            subject: Subject::Chemistry,
            chapter: "Chapter 2: Acids, Bases and Salts".to_string(),
            description: "Determine concentration of unknown acid using standard base".to_string(),
            difficulty: "Medium".to_string(),
            duration_minutes: 45,
            equipment: vec!["Burette".to_string(), "Pipette".to_string(), "Conical flask".to_string(), "Indicator".to_string()],
            objectives: vec![
                "Perform titration accurately".to_string(),
                "Identify endpoint using indicator".to_string(),
                "Calculate concentration using formula".to_string(),
            ],
        });

        // Biology Experiments
        self.add_experiment(Experiment {
            id: "bio_mitosis".to_string(),
            title: "Mitosis in Onion Root Tips".to_string(),
            subject: Subject::Biology,
            chapter: "Chapter 10: Cell Cycle and Cell Division".to_string(),
            description: "Observe and identify different stages of mitosis".to_string(),
            difficulty: "Medium".to_string(),
            duration_minutes: 60,
            equipment: vec!["Microscope".to_string(), "Onion root tips".to_string(), "Slide".to_string(), "Stain".to_string()],
            objectives: vec![
                "Prepare temporary mount".to_string(),
                "Observe under microscope".to_string(),
                "Identify prophase, metaphase, anaphase, telophase".to_string(),
            ],
        });

        self.add_experiment(Experiment {
            id: "bio_photosynthesis".to_string(),
            title: "Photosynthesis in Leaves".to_string(),
            subject: Subject::Biology,
            chapter: "Chapter 13: Photosynthesis".to_string(),
            description: "Demonstrate starch formation in leaves during photosynthesis".to_string(),
            difficulty: "Easy".to_string(),
            duration_minutes: 45,
            equipment: vec!["Potted plant".to_string(), "Alcohol".to_string(), "Iodine solution".to_string(), "Beaker".to_string()],
            objectives: vec![
                "Destarch leaves".to_string(),
                "Expose to sunlight".to_string(),
                "Test for starch using iodine".to_string(),
            ],
        });
    }

    /// Add experiment to lab
    fn add_experiment(&mut self, experiment: Experiment) {
        self.experiments.insert(experiment.id.clone(), experiment);
    }

    /// Start simulation for experiment
    pub fn start_simulation(&mut self, experiment_id: &str) -> Result<(), String> {
        if self.experiments.contains_key(experiment_id) {
            self.current_simulation = Some(SimulationState {
                experiment_id: experiment_id.to_string(),
                parameters: HashMap::new(),
                measurements: Vec::new(),
                completed: false,
                score: 0.0,
            });
            Ok(())
        } else {
            Err("Experiment not found".to_string())
        }
    }

    /// Set simulation parameter
    pub fn set_parameter(&mut self, key: String, value: f64) -> Result<(), String> {
        if let Some(sim) = &mut self.current_simulation {
            sim.parameters.insert(key, value);
            Ok(())
        } else {
            Err("No active simulation".to_string())
        }
    }

    /// Add measurement
    pub fn add_measurement(&mut self, parameter: String, value: f64, unit: String) -> Result<(), String> {
        if let Some(sim) = &mut self.current_simulation {
            sim.measurements.push(Measurement {
                parameter,
                value,
                unit,
                timestamp: "now".to_string(),
            });
            Ok(())
        } else {
            Err("No active simulation".to_string())
        }
    }

    /// Complete simulation and calculate score
    pub fn complete_simulation(&mut self) -> Result<f64, String> {
        if let Some(sim) = &mut self.current_simulation {
            if let Some(exp) = self.experiments.get(&sim.experiment_id) {
                // Simple scoring: check if objectives were met
                let objectives_met = sim.measurements.len() as f64;
                let total_objectives = exp.objectives.len() as f64;
                sim.score = (objectives_met / total_objectives) * 100.0;
                sim.completed = true;
                Ok(sim.score)
            } else {
                Err("Experiment not found".to_string())
            }
        } else {
            Err("No active simulation".to_string())
        }
    }

    /// Get experiments filtered by subject and chapter
    pub fn get_experiments(&self) -> Vec<&Experiment> {
        self.experiments.values()
            .filter(|exp| {
                if let Some(subject) = self.subject_filter {
                    if exp.subject != subject {
                        return false;
                    }
                }
                if let Some(chapter) = &self.chapter_filter {
                    if !exp.chapter.contains(chapter) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Get AI-suggested experiment variations
    pub fn get_suggested_variations(&self, experiment_id: &str) -> Vec<String> {
        let mut variations = Vec::new();
        
        if let Some(exp) = self.experiments.get(experiment_id) {
            match exp.subject {
                Subject::Physics => {
                    variations.push("Try different resistor values and plot I-V graph".to_string());
                    variations.push("Repeat with different battery voltages".to_string());
                    variations.push("Compare with parallel circuit configuration".to_string());
                }
                Subject::Chemistry => {
                    variations.push("Test with different acid concentrations".to_string());
                    variations.push("Compare with different indicators".to_string());
                    variations.push("Try weak acid vs strong acid titration".to_string());
                }
                Subject::Biology => {
                    variations.push("Compare mitosis in different plant tissues".to_string());
                    variations.push("Vary light intensity for photosynthesis".to_string());
                    variations.push("Test with different wavelengths of light".to_string());
                }
            }
        }
        
        variations
    }

    /// Filter by subject
    pub fn filter_by_subject(&mut self, subject: Subject) {
        self.subject_filter = Some(subject);
    }

    /// Filter by chapter
    pub fn filter_by_chapter(&mut self, chapter: String) {
        self.chapter_filter = Some(chapter);
    }

    /// Clear filters
    pub fn clear_filters(&mut self) {
        self.subject_filter = None;
        self.chapter_filter = None;
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut lab = VirtualLab::new();
    
    println!("Sigma Virtual Lab v0.1 - CBSE Science Experiments");
    
    loop {
        println!("\n--- Available Experiments ---");
        for exp in lab.get_experiments() {
            let subject_str = match exp.subject {
                Subject::Physics => "PHY",
                Subject::Chemistry => "CHEM",
                Subject::Biology => "BIO",
            };
            println!("[{}] {} - {} ({} min)", subject_str, exp.title, exp.chapter, exp.duration_minutes);
        }
        
        println!("\nCommands: start <id>, param <key> <value>, measure <param> <value> <unit>, complete, variations <id>, filter <subject|chapter>, clear, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "start" => {
                if let Some(arg) = parts.get(1) {
                    match lab.start_simulation(arg) {
                        Ok(_) => println!("Started simulation: {}", arg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "param" => {
                if parts.len() >= 3 {
                    if let Ok(value) = parts[2].parse::<f64>() {
                        match lab.set_parameter(parts[1].to_string(), value) {
                            Ok(_) => println!("Parameter set"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "measure" => {
                if parts.len() >= 4 {
                    if let Ok(value) = parts[2].parse::<f64>() {
                        match lab.add_measurement(parts[1].to_string(), value, parts[3].to_string()) {
                            Ok(_) => println!("Measurement recorded"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "complete" => {
                match lab.complete_simulation() {
                    Ok(score) => println!("Simulation complete! Score: {:.1}%", score),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "variations" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Suggested Variations ---");
                    for variation in lab.get_suggested_variations(arg) {
                        println!("- {}", variation);
                    }
                }
            }
            "filter" => {
                if parts.len() >= 2 {
                    match parts[1] {
                        "physics" => lab.filter_by_subject(Subject::Physics),
                        "chemistry" => lab.filter_by_subject(Subject::Chemistry),
                        "biology" => lab.filter_by_subject(Subject::Biology),
                        "chapter" => {
                            if parts.len() >= 3 {
                                lab.filter_by_chapter(parts[2..].join(" "));
                            }
                        }
                        _ => {}
                    }
                    println!("Filter applied");
                }
            }
            "clear" => {
                lab.clear_filters();
                println!("Filters cleared");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
