#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS AI-Driven Optimization System
// Copilot-style assistants for system tuning and automation

/// Optimization category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationCategory {
    Performance,
    Power,
    Thermal,
    Network,
    Storage,
    Security,
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: OptimizationCategory,
    pub description: String,
    pub impact: f64,     // 0.0 to 1.0
    pub confidence: f64, // 0.0 to 1.0
    pub action: String,
    pub estimated_benefit: String,
}

impl OptimizationRecommendation {
    pub fn new(category: OptimizationCategory, description: String) -> Self {
        Self {
            category,
            description,
            impact: 0.5,
            confidence: 0.5,
            action: String::new(),
            estimated_benefit: String::new(),
        }
    }

    pub fn with_impact(mut self, impact: f64) -> Self {
        self.impact = impact.clamp(0.0, 1.0);
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn with_action(mut self, action: String) -> Self {
        self.action = action;
        self
    }

    pub fn with_benefit(mut self, benefit: String) -> Self {
        self.estimated_benefit = benefit;
        self
    }
}

/// System state snapshot
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

impl SystemState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_usage: 0.0,
            temperature: 0.0,
            power_consumption: 0.0,
            timestamp: 0,
        }
    }

    pub fn with_cpu(mut self, usage: f64) -> Self {
        self.cpu_usage = usage.clamp(0.0, 100.0);
        self
    }

    pub fn with_memory(mut self, usage: f64) -> Self {
        self.memory_usage = usage.clamp(0.0, 100.0);
        self
    }

    pub fn with_temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }
}

impl Default for SystemState {
    fn default() -> Self {
        Self::new()
    }
}

/// AI optimizer
pub struct AiOptimizer {
    pub system_history: Vec<SystemState>,
    pub recommendations: Vec<OptimizationRecommendation>,
    pub learning_enabled: bool,
    pub optimization_threshold: f64,
}

impl AiOptimizer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            system_history: Vec::new(),
            recommendations: Vec::new(),
            learning_enabled: true,
            optimization_threshold: 0.7,
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.optimization_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    pub fn record_state(&mut self, state: SystemState) {
        self.system_history.push(state);

        // Keep only last 1000 states
        if self.system_history.len() > 1000 {
            self.system_history.remove(0);
        }
    }

    pub fn analyze_current_state(
        &self,
        current_state: &SystemState,
    ) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();

        // CPU optimization
        if current_state.cpu_usage > 80.0 {
            recommendations.push(
                OptimizationRecommendation::new(
                    OptimizationCategory::Performance,
                    "High CPU usage detected".to_string(),
                )
                .with_impact(0.8)
                .with_confidence(0.9)
                .with_action("Reduce background processes".to_string())
                .with_benefit("Expected 15-20% CPU reduction".to_string()),
            );
        }

        // Memory optimization
        if current_state.memory_usage > 85.0 {
            recommendations.push(
                OptimizationRecommendation::new(
                    OptimizationCategory::Performance,
                    "High memory usage detected".to_string(),
                )
                .with_impact(0.7)
                .with_confidence(0.85)
                .with_action("Clear cache and inactive applications".to_string())
                .with_benefit("Expected 10-15% memory reduction".to_string()),
            );
        }

        // Thermal optimization
        if current_state.temperature > 75.0 {
            recommendations.push(
                OptimizationRecommendation::new(
                    OptimizationCategory::Thermal,
                    "High temperature detected".to_string(),
                )
                .with_impact(0.9)
                .with_confidence(0.95)
                .with_action("Reduce CPU frequency and enable cooling".to_string())
                .with_benefit("Expected 5-10°C temperature reduction".to_string()),
            );
        }

        // Power optimization
        if current_state.power_consumption > 50.0 && !self.system_history.is_empty() {
            let avg_power: f64 = self
                .system_history
                .iter()
                .map(|s| s.power_consumption)
                .sum::<f64>()
                / self.system_history.len() as f64;

            if current_state.power_consumption > avg_power * 1.5 {
                recommendations.push(
                    OptimizationRecommendation::new(
                        OptimizationCategory::Power,
                        "Unusual power consumption detected".to_string(),
                    )
                    .with_impact(0.6)
                    .with_confidence(0.7)
                    .with_action("Enable power saving mode".to_string())
                    .with_benefit("Expected 20-30% power reduction".to_string()),
                );
            }
        }

        // Filter by threshold
        recommendations.retain(|r| r.confidence >= self.optimization_threshold);

        // Sort by impact
        recommendations.sort_by(|a, b| {
            b.impact
                .partial_cmp(&a.impact)
                .unwrap_or(core::cmp::Ordering::Equal)
        });

        recommendations
    }

    pub fn generate_recommendations(
        &mut self,
        current_state: &SystemState,
    ) -> Vec<OptimizationRecommendation> {
        let new_recommendations = self.analyze_current_state(current_state);
        self.recommendations = new_recommendations.clone();
        new_recommendations
    }

    pub fn apply_recommendation(
        &mut self,
        recommendation: &OptimizationRecommendation,
    ) -> Result<(), OptimizationError> {
        if !self.learning_enabled {
            return Err(OptimizationError::LearningDisabled);
        }

        println!("Applying recommendation: {}", recommendation.action);

        // Simulate applying the recommendation
        Ok(())
    }

    pub fn enable_learning(&mut self) {
        self.learning_enabled = true;
    }

    pub fn disable_learning(&mut self) {
        self.learning_enabled = false;
    }

    pub fn get_recommendations(&self) -> &[OptimizationRecommendation] {
        &self.recommendations
    }

    pub fn get_system_history(&self) -> &[SystemState] {
        &self.system_history
    }
}

impl Default for AiOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimization errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationError {
    LearningDisabled,
    InvalidRecommendation,
    SystemError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = AiOptimizer::new();
        assert!(optimizer.learning_enabled);
        assert_eq!(optimizer.optimization_threshold, 0.7);
    }

    #[test]
    fn test_state_recording() {
        let mut optimizer = AiOptimizer::new();
        let state = SystemState::new().with_cpu(50.0);
        optimizer.record_state(state);
        assert_eq!(optimizer.system_history.len(), 1);
    }

    #[test]
    fn test_high_cpu_recommendation() {
        let mut optimizer = AiOptimizer::new();
        let state = SystemState::new().with_cpu(90.0);
        let recommendations = optimizer.generate_recommendations(&state);
        assert!(!recommendations.is_empty());
        assert_eq!(
            recommendations[0].category,
            OptimizationCategory::Performance
        );
    }

    #[test]
    fn test_high_temperature_recommendation() {
        let mut optimizer = AiOptimizer::new();
        let state = SystemState::new().with_temperature(80.0);
        let recommendations = optimizer.generate_recommendations(&state);
        assert!(!recommendations.is_empty());
        assert_eq!(recommendations[0].category, OptimizationCategory::Thermal);
    }

    #[test]
    fn test_learning_toggle() {
        let mut optimizer = AiOptimizer::new();
        optimizer.disable_learning();
        assert!(!optimizer.learning_enabled);
        optimizer.enable_learning();
        assert!(optimizer.learning_enabled);
    }
}
