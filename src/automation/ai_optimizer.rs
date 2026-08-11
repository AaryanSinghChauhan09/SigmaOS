#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationCategory {
    Performance,
    Power,
    Thermal,
    Network,
    Storage,
    Security,
}

#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: OptimizationCategory,
    pub description: String,
    pub impact: f64,
    pub confidence: f64,
    pub action: String,
    pub estimated_benefit: String,
}

#[derive(Debug, Clone)]
pub struct SystemState {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub temperature: f64,
    pub power_consumption: f64,
    pub timestamp: u64,
}

pub struct AiOptimizer {
    pub system_history: Vec<SystemState>,
    pub learning_rate: f64,
    pub ema_cpu: f64,
    pub ema_mem: f64,
}

impl AiOptimizer {
    pub fn new() -> Self {
        Self {
            system_history: Vec::new(),
            learning_rate: 0.1, // EMA alpha
            ema_cpu: 0.0,
            ema_mem: 0.0,
        }
    }

    pub fn record_state(&mut self, state: SystemState) {
        if self.system_history.is_empty() {
            self.ema_cpu = state.cpu_usage;
            self.ema_mem = state.memory_usage;
        } else {
            // Update Exponential Moving Average
            self.ema_cpu = (state.cpu_usage * self.learning_rate) + (self.ema_cpu * (1.0 - self.learning_rate));
            self.ema_mem = (state.memory_usage * self.learning_rate) + (self.ema_mem * (1.0 - self.learning_rate));
        }

        self.system_history.push(state);
        if self.system_history.len() > 1000 {
            self.system_history.remove(0);
        }
    }

    pub fn analyze_current_state(&self, current_state: &SystemState) -> Vec<OptimizationRecommendation> {
        let mut recs = Vec::new();

        // Use EMA for anomaly detection (Z-score simplified)
        let diff_cpu = current_state.cpu_usage - self.ema_cpu;
        if diff_cpu > 30.0 {
            recs.push(OptimizationRecommendation {
                category: OptimizationCategory::Performance,
                description: "CPU spike anomaly detected".to_string(),
                impact: 0.9,
                confidence: 0.92,
                action: "Throttle low-priority cgroups".to_string(),
                estimated_benefit: "Expected 20% CPU normalization".to_string(),
            });
        }

        // Linear regression for short-term CPU prediction
        if self.system_history.len() > 10 {
            let last_idx = self.system_history.len() - 1;
            let slope = self.system_history[last_idx].cpu_usage - self.system_history[last_idx - 10].cpu_usage;
            if slope > 2.0 {
                recs.push(OptimizationRecommendation {
                    category: OptimizationCategory::Performance,
                    description: "Upward CPU trend predicted".to_string(),
                    impact: 0.8,
                    confidence: 0.85,
                    action: "Pre-scale CPU frequency".to_string(),
                    estimated_benefit: "Reduced latency for upcoming workload".to_string(),
                });
            }
        }
        
        recs
    }
}
