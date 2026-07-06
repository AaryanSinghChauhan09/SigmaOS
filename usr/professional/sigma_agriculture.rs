// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/professional/sigma_agriculture.rs — Sigma Agriculture Tools
//
// Implements crop yield prediction, soil health analysis, and weather-linked
// advisory system for farmers and agricultural professionals.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Agriculture Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CropType {
    Wheat,
    Rice,
    Maize,
    Cotton,
    Sugarcane,
    Pulses,
    Oilseeds,
}

#[derive(Debug, Clone)]
pub struct SoilSample {
    pub id: String,
    pub location: String,
    pub ph_level: f64,
    pub nitrogen: f64,
    pub phosphorus: f64,
    pub potassium: f64,
    pub organic_matter: f64,
    pub soil_type: String,
}

#[derive(Debug, Clone)]
pub struct CropYieldPrediction {
    pub crop: CropType,
    pub area_hectares: f64,
    pub expected_yield: f64,
    pub confidence: f64,
    pub factors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WeatherAdvisory {
    pub date: String,
    pub temperature: f64,
    pub humidity: f64,
    pub rainfall: f64,
    pub advisory: String,
}

// ─── Agriculture Analytics Engine ───────────────────────────────────────────

pub struct AgricultureManager {
    pub soil_samples: HashMap<String, SoilSample>,
    pub yield_predictions: Vec<CropYieldPrediction>,
    pub weather_advisories: Vec<WeatherAdvisory>,
}

impl AgricultureManager {
    pub fn new() -> Self {
        let mut manager = AgricultureManager {
            soil_samples: HashMap::new(),
            yield_predictions: Vec::new(),
            weather_advisories: Vec::new(),
        };
        
        manager.init_sample_data();
        manager
    }

    /// Initialize sample data
    fn init_sample_data(&mut self) {
        // Sample soil data
        self.soil_samples.insert("soil_001".to_string(), SoilSample {
            id: "soil_001".to_string(),
            location: "Punjab".to_string(),
            ph_level: 7.2,
            nitrogen: 280.0,
            phosphorus: 22.0,
            potassium: 180.0,
            organic_matter: 1.2,
            soil_type: "Alluvial".to_string(),
        });

        self.soil_samples.insert("soil_002".to_string(), SoilSample {
            id: "soil_002".to_string(),
            location: "Maharashtra".to_string(),
            ph_level: 6.5,
            nitrogen: 220.0,
            phosphorus: 18.0,
            potassium: 150.0,
            organic_matter: 0.8,
            soil_type: "Black Soil".to_string(),
        });
    }

    /// Add soil sample
    pub fn add_soil_sample(&mut self, sample: SoilSample) {
        self.soil_samples.insert(sample.id.clone(), sample);
    }

    /// Get soil sample by ID
    pub fn get_soil_sample(&self, id: &str) -> Option<&SoilSample> {
        self.soil_samples.get(id)
    }

    /// Analyze soil health
    pub fn analyze_soil_health(&self, sample_id: &str) -> HashMap<String, String> {
        let mut analysis = HashMap::new();
        
        if let Some(sample) = self.get_soil_sample(sample_id) {
            // pH analysis
            let ph_status = if sample.ph_level < 5.5 {
                "Acidic - Add lime".to_string()
            } else if sample.ph_level > 8.5 {
                "Alkaline - Add sulfur".to_string()
            } else {
                "Optimal".to_string()
            };
            analysis.insert("ph_status".to_string(), ph_status);
            
            // Nitrogen analysis
            let n_status = if sample.nitrogen < 200.0 {
                "Low - Add nitrogen fertilizer".to_string()
            } else if sample.nitrogen > 500.0 {
                "High - Reduce nitrogen".to_string()
            } else {
                "Optimal".to_string()
            };
            analysis.insert("nitrogen_status".to_string(), n_status);
            
            // Phosphorus analysis
            let p_status = if sample.phosphorus < 15.0 {
                "Low - Add phosphorus fertilizer".to_string()
            } else if sample.phosphorus > 60.0 {
                "High - Reduce phosphorus".to_string()
            } else {
                "Optimal".to_string()
            };
            analysis.insert("phosphorus_status".to_string(), p_status);
            
            // Potassium analysis
            let k_status = if sample.potassium < 100.0 {
                "Low - Add potassium fertilizer".to_string()
            } else if sample.potassium > 300.0 {
                "High - Reduce potassium".to_string()
            } else {
                "Optimal".to_string()
            };
            analysis.insert("potassium_status".to_string(), k_status);
            
            // Organic matter analysis
            let om_status = if sample.organic_matter < 0.5 {
                "Low - Add organic matter".to_string()
            } else if sample.organic_matter > 2.0 {
                "High - Good for soil health".to_string()
            } else {
                "Moderate".to_string()
            };
            analysis.insert("organic_matter_status".to_string(), om_status);
        }
        
        analysis
    }

    /// Predict crop yield
    pub fn predict_yield(&mut self, crop: CropType, area: f64, soil_quality: f64, rainfall: f64) -> CropYieldPrediction {
        let base_yield_per_hectare = match crop {
            CropType::Wheat => 3.5,
            CropType::Rice => 4.0,
            CropType::Maize => 2.5,
            CropType::Cotton => 1.8,
            CropType::Sugarcane => 70.0,
            CropType::Pulses => 1.2,
            CropType::Oilseeds => 1.5,
        };
        
        let quality_factor = soil_quality / 100.0;
        let rainfall_factor = if rainfall > 500.0 { 1.0 } else { rainfall / 500.0 };
        
        let expected_yield = base_yield_per_hectare * area * quality_factor * rainfall_factor;
        let confidence = (soil_quality + (rainfall.min(500.0) / 5.0)) / 2.0;
        
        let mut factors = Vec::new();
        if soil_quality < 50.0 {
            factors.push("Soil quality below optimal".to_string());
        }
        if rainfall < 300.0 {
            factors.push("Low rainfall may affect yield".to_string());
        }
        if rainfall > 800.0 {
            factors.push("Excessive rainfall may cause waterlogging".to_string());
        }
        
        let prediction = CropYieldPrediction {
            crop,
            area_hectares: area,
            expected_yield,
            confidence: confidence.min(100.0),
            factors,
        };
        
        self.yield_predictions.push(prediction.clone());
        prediction
    }

    /// Add weather advisory
    pub fn add_weather_advisory(&mut self, advisory: WeatherAdvisory) {
        self.weather_advisories.push(advisory);
    }

    /// Get weather advisories
    pub fn get_weather_advisories(&self) -> &[WeatherAdvisory] {
        &self.weather_advisories
    }

    /// Generate fertilizer recommendation
    pub fn generate_fertilizer_recommendation(&self, sample_id: &str) -> String {
        if let Some(sample) = self.get_soil_sample(sample_id) {
            let mut recommendations = Vec::new();
            
            if sample.nitrogen < 200.0 {
                recommendations.push(format!("Apply {} kg/ha of nitrogen fertilizer", (250.0 - sample.nitrogen).ceil()));
            }
            if sample.phosphorus < 15.0 {
                recommendations.push(format!("Apply {} kg/ha of phosphorus fertilizer", (25.0 - sample.phosphorus).ceil()));
            }
            if sample.potassium < 100.0 {
                recommendations.push(format!("Apply {} kg/ha of potassium fertilizer", (150.0 - sample.potassium).ceil()));
            }
            if sample.organic_matter < 0.5 {
                recommendations.push("Add organic compost or farmyard manure".to_string());
            }
            
            if recommendations.is_empty() {
                "Soil nutrients are balanced. Maintain current practices.".to_string()
            } else {
                recommendations.join("\n")
            }
        } else {
            "Sample not found".to_string()
        }
    }

    /// Get all soil samples
    pub fn get_all_soil_samples(&self) -> Vec<&SoilSample> {
        self.soil_samples.values().collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = AgricultureManager::new();
    
    println!("Sigma Agriculture Tools v0.1 - Crop Yield & Soil Health");
    
    loop {
        println!("\nCommands: soil <id>, analyze <id>, fertilizer <id>, predict <crop> <area> <soil_quality> <rainfall>, weather, samples, quit");
        println!("Crops: wheat, rice, maize, cotton, sugarcane, pulses, oilseeds");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "soil" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(sample) = manager.get_soil_sample(arg) {
                        println!("--- Soil Sample ---");
                        println!("Location: {}", sample.location);
                        println!("Soil Type: {}", sample.soil_type);
                        println!("pH Level: {:.1}", sample.ph_level);
                        println!("Nitrogen: {:.1} kg/ha", sample.nitrogen);
                        println!("Phosphorus: {:.1} kg/ha", sample.phosphorus);
                        println!("Potassium: {:.1} kg/ha", sample.potassium);
                        println!("Organic Matter: {:.1}%", sample.organic_matter);
                    }
                }
            }
            "analyze" => {
                if let Some(arg) = parts.get(1) {
                    let analysis = manager.analyze_soil_health(arg);
                    println!("--- Soil Health Analysis ---");
                    for (key, value) in &analysis {
                        println!("{}: {}", key, value);
                    }
                }
            }
            "fertilizer" => {
                if let Some(arg) = parts.get(1) {
                    let recommendation = manager.generate_fertilizer_recommendation(arg);
                    println!("--- Fertilizer Recommendation ---");
                    println!("{}", recommendation);
                }
            }
            "predict" => {
                if parts.len() >= 5 {
                    let crop = match parts[1] {
                        "wheat" => CropType::Wheat,
                        "rice" => CropType::Rice,
                        "maize" => CropType::Maize,
                        "cotton" => CropType::Cotton,
                        "sugarcane" => CropType::Sugarcane,
                        "pulses" => CropType::Pulses,
                        "oilseeds" => CropType::Oilseeds,
                        _ => {
                            println!("Unknown crop");
                            continue;
                        }
                    };
                    
                    if let (Ok(area), Ok(quality), Ok(rainfall)) = (
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>(),
                        parts[4].parse::<f64>()
                    ) {
                        let prediction = manager.predict_yield(crop, area, quality, rainfall);
                        println!("--- Yield Prediction ---");
                        println!("Crop: {:?}", prediction.crop);
                        println!("Area: {:.2} hectares", prediction.area_hectares);
                        println!("Expected Yield: {:.2} tonnes", prediction.expected_yield);
                        println!("Confidence: {:.1}%", prediction.confidence);
                        if !prediction.factors.is_empty() {
                            println!("\nFactors:");
                            for factor in &prediction.factors {
                                println!("- {}", factor);
                            }
                        }
                    }
                }
            }
            "weather" => {
                println!("--- Weather Advisories ---");
                for advisory in manager.get_weather_advisories() {
                    println!("Date: {}", advisory.date);
                    println!("Temperature: {:.1}°C", advisory.temperature);
                    println!("Humidity: {:.1}%", advisory.humidity);
                    println!("Rainfall: {:.1} mm", advisory.rainfall);
                    println!("Advisory: {}", advisory.advisory);
                    println!();
                }
            }
            "samples" => {
                println!("--- All Soil Samples ---");
                for sample in manager.get_all_soil_samples() {
                    println!("{} - {} ({})", sample.id, sample.location, sample.soil_type);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
